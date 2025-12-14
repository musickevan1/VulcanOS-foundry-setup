use anyhow::{anyhow, Result};
use cairo::{Antialias, Context, Format, ImageSurface, Surface};
use chrono::{Local, Locale, Timelike, format::{StrftimeItems, Item as ChronoItem}};
use drm::control::ClipRect;
use freedesktop_icons::lookup;
use input::{
    event::{
        device::DeviceEvent,
        keyboard::{KeyState, KeyboardEvent, KeyboardEventTrait},
        touch::{TouchEvent, TouchEventPosition, TouchEventSlot},
        Event, EventTrait,
    },
    Device as InputDevice, Libinput, LibinputInterface,
};
use input_linux::{uinput::UInputHandle, EventKind, Key, SynchronizeKind};
use input_linux_sys::{input_event, input_id, timeval, uinput_setup};
use libc::{c_char, O_ACCMODE, O_RDONLY, O_RDWR, O_WRONLY};
use librsvg_rebind::{prelude::HandleExt, Handle, Rectangle};
use nix::{
    errno::Errno,
    sys::{
        epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags},
        signal::{SigSet, Signal},
    },
};
use privdrop::PrivDrop;
use std::{
    cmp::min,
    collections::HashMap,
    env,
    fs::{self, File, OpenOptions},
    os::{
        fd::{AsFd, AsRawFd},
        unix::{fs::OpenOptionsExt, io::OwnedFd},
    },
    panic::{self, AssertUnwindSafe},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use tracing::{info, warn, debug, error};
use tracing_subscriber::{EnvFilter, fmt};
use udev::MonitorBuilder;

mod config;
mod display;
mod events;
mod gestures;
mod modules;
mod pages;

use crate::config::ConfigManager;
use crate::display::{BacklightManager, Compositor, DrmBackend, PixelShiftManager, PIXEL_SHIFT_WIDTH_PX};
use crate::config::new_loader::ConfigLoader;
use crate::gestures::{GestureDetector, GestureConfig, Gesture, TouchPhase, Direction};
use crate::modules::{ModuleRegistry, Action};
use crate::pages::PageManager;
use config::{ButtonConfig, Config};

const BUTTON_SPACING_PX: i32 = 16;
const BUTTON_COLOR_INACTIVE: f64 = 0.200;
const BUTTON_COLOR_ACTIVE: f64 = 0.400;
const ICON_SIZE: i32 = 48;
const TIMEOUT_MS: i32 = 10 * 1000;

#[derive(Clone, Copy, PartialEq, Eq)]
enum BatteryState {
    NotCharging,
    Charging,
    Low,
}

struct BatteryImages {
    plain: Vec<Handle>,
    charging: Vec<Handle>,
    bolt: Handle,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum BatteryIconMode {
    Percentage,
    Icon,
    Both
}

/// Holds Hyprland socket info discovered before privilege drop
#[derive(Clone, Debug)]
struct HyprlandSocketInfo {
    /// The instance signature (directory name under /hypr/)
    signature: String,
    /// The XDG_RUNTIME_DIR where the socket was found (e.g., /run/user/1000)
    runtime_dir: String,
    /// The UID of the user who owns the Hyprland session
    uid: u32,
}

impl BatteryIconMode {
    fn should_draw_icon(self) -> bool {
        self != BatteryIconMode::Percentage
    }
    fn should_draw_text(self) -> bool {
        self != BatteryIconMode::Icon
    }
}

enum ButtonImage {
    Text(String),
    Svg(Handle),
    Bitmap(ImageSurface),
    Time(Vec<ChronoItem<'static>>, Locale),
    Battery(String, BatteryIconMode, BatteryImages),
}

struct Button {
    image: ButtonImage,
    changed: bool,
    active: bool,
    action: Key,
}

fn try_load_svg(path: &str) -> Result<ButtonImage> {
    Ok(ButtonImage::Svg(
        Handle::from_file(path)?.ok_or(anyhow!("failed to load image"))?,
    ))
}

fn try_load_png(path: impl AsRef<Path>) -> Result<ButtonImage> {
    let mut file = File::open(path)?;
    let surf = ImageSurface::create_from_png(&mut file)?;
    if surf.height() == ICON_SIZE && surf.width() == ICON_SIZE {
        return Ok(ButtonImage::Bitmap(surf));
    }
    let resized = ImageSurface::create(Format::ARgb32, ICON_SIZE, ICON_SIZE).unwrap();
    let c = Context::new(&resized).unwrap();
    c.scale(
        ICON_SIZE as f64 / surf.width() as f64,
        ICON_SIZE as f64 / surf.height() as f64,
    );
    c.set_source_surface(surf, 0.0, 0.0).unwrap();
    c.set_antialias(Antialias::Best);
    c.paint().unwrap();
    Ok(ButtonImage::Bitmap(resized))
}

fn try_load_image(name: impl AsRef<str>, theme: Option<impl AsRef<str>>) -> Result<ButtonImage> {
    let name = name.as_ref();
    let locations;

    // Load list of candidate locations
    if let Some(theme) = theme {
        // Freedesktop icons
        let theme = theme.as_ref();
        let candidates = vec![
            lookup(name)
                .with_cache()
                .with_theme(theme)
                .with_size(ICON_SIZE as u16)
                .force_svg()
                .find(),
            lookup(name)
                .with_cache()
                .with_theme(theme)
                .force_svg()
                .find(),
        ];

        // .flatten() removes `None` and unwraps `Some` values
        locations = candidates.into_iter().flatten().collect();
    } else {
        // Standard file icons
        locations = vec![
            PathBuf::from(format!("/etc/vulcanbar/{name}.svg")),
            PathBuf::from(format!("/etc/vulcanbar/{name}.png")),
            PathBuf::from(format!("/usr/share/vulcanbar/{name}.svg")),
            PathBuf::from(format!("/usr/share/vulcanbar/{name}.png")),
        ];
    };

    // Try to load each candidate
    let mut last_err = anyhow!("no suitable icon path was found"); // in case locations is empty

    for location in locations {
        let result = match location.extension().and_then(|s| s.to_str()) {
            Some("png") => try_load_png(&location),
            Some("svg") => try_load_svg(
                location
                    .to_str()
                    .ok_or(anyhow!("image path is not unicode"))?,
            ),
            _ => Err(anyhow!("invalid file extension")),
        };

        match result {
            Ok(image) => return Ok(image),
            Err(err) => {
                last_err = err.context(format!("while loading path {}", location.display()));
            }
        };
    }

    // if function hasn't returned by now, all sources have been exhausted
    Err(last_err.context(format!("failed loading all possible paths for icon {name}")))
}

fn find_battery_device() -> Option<String> {
    let power_supply_path = "/sys/class/power_supply";
    if let Ok(entries) = fs::read_dir(power_supply_path) {
        for entry in entries.flatten() {
            let dev_path = entry.path();
            let type_path = dev_path.join("type");
            if let Ok(typ) = fs::read_to_string(&type_path) {
                if typ.trim() == "Battery" {
                    if let Some(name) = dev_path.file_name().and_then(|n| n.to_str()) {
                        return Some(name.to_string());
                    }
                }
            }
        }
    }
    None
}

fn get_battery_state(battery: &str) -> (u32, BatteryState) {
    let status_path = format!("/sys/class/power_supply/{}/status", battery);
    let status = fs::read_to_string(&status_path)
        .unwrap_or_else(|_| "Unknown".to_string());

    #[cfg(target_arch = "x86_64")]
    let capacity = {
        let charge_now_path = format!("/sys/class/power_supply/{}/charge_now", battery);
        let charge_full_path = format!("/sys/class/power_supply/{}/charge_full", battery);
        let charge_now = fs::read_to_string(&charge_now_path)
            .ok()
            .and_then(|s| s.trim().parse::<f64>().ok());
        let charge_full = fs::read_to_string(&charge_full_path)
            .ok()
            .and_then(|s| s.trim().parse::<f64>().ok());
        match (charge_now, charge_full) {
            (Some(now), Some(full)) if full > 0.0 => ((now / full) * 100.0).round() as u32,
            _ => 100,
        }
    };

    #[cfg(target_arch = "aarch64")]
    let capacity = {
        let capacity_path = format!("/sys/class/power_supply/{}/capacity", battery);
        fs::read_to_string(&capacity_path)
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok())
            .unwrap_or(100)
    };

    let status = match status.trim() {
        "Charging" | "Full" => BatteryState::Charging,
        "Discharging" if capacity < 10 => BatteryState::Low,
        _ => BatteryState::NotCharging,
    };
    (capacity, status)
}

impl Button {
    fn with_config(cfg: ButtonConfig) -> Button {
        if let Some(text) = cfg.text {
            Button::new_text(text, cfg.action)
        } else if let Some(icon) = cfg.icon {
            Button::new_icon(&icon, cfg.theme, cfg.action)
        } else if let Some(time) = cfg.time {
            Button::new_time(cfg.action, &time, cfg.locale.as_deref())
        } else if let Some(battery_mode) = cfg.battery {
            if let Some(battery) = find_battery_device() {
                Button::new_battery(cfg.action, battery, battery_mode, cfg.theme)
            } else {
                Button::new_text("Battery N/A".to_string(), cfg.action)
            }
        } else {
            panic!("Invalid config, a button must have either Text, Icon or Time")
        }
    }
    fn new_text(text: String, action: Key) -> Button {
        Button {
            action,
            active: false,
            changed: false,
            image: ButtonImage::Text(text),
        }
    }
    fn new_icon(path: impl AsRef<str>, theme: Option<impl AsRef<str>>, action: Key) -> Button {
        let image = try_load_image(path, theme).expect("failed to load icon");
        Button {
            action,
            image,
            active: false,
            changed: false,
        }
    }
    fn load_battery_image(icon: &str, theme: Option<impl AsRef<str>>) -> Handle {
        if let ButtonImage::Svg(svg) = try_load_image(icon, theme).unwrap() {
            return svg;
        }
        panic!("failed to load icon");
    }
    fn new_battery(action: Key, battery: String, battery_mode: String, theme: Option<impl AsRef<str>>) -> Button {
        let bolt = Self::load_battery_image("bolt", theme.as_ref());
        let mut plain = Vec::new();
        let mut charging = Vec::new();
        for icon in [
            "battery_0_bar", "battery_1_bar", "battery_2_bar", "battery_3_bar",
            "battery_4_bar", "battery_5_bar", "battery_6_bar", "battery_full",
        ] {
            plain.push(Self::load_battery_image(icon, theme.as_ref()));
        }
        for icon in [
            "battery_charging_20", "battery_charging_30", "battery_charging_50",
            "battery_charging_60", "battery_charging_80",
            "battery_charging_90", "battery_charging_full",
        ] {
            charging.push(Self::load_battery_image(icon, theme.as_ref()));
        }
        let battery_mode = match battery_mode.as_str() {
            "icon" => BatteryIconMode::Icon,
            "percentage" => BatteryIconMode::Percentage,
            "both" => BatteryIconMode::Both,
            _ => panic!("invalid battery mode, accepted modes: icon, percentage, both"),
        };
        Button {
            action,
            active: false,
            changed: false,
            image: ButtonImage::Battery(battery, battery_mode, BatteryImages {
                plain, bolt, charging
            }),
        }
    }

    fn new_time(action: Key, format: &str, locale_str: Option<&str>) -> Button {
        let format_str = if format == "24hr" {
            "%H:%M    %a %-e %b"
        } else if format == "12hr" {
            "%-l:%M %p    %a %-e %b"
        } else {
            format
        };

        let format_items = match StrftimeItems::new(format_str).parse_to_owned() {
            Ok(s) => s,
            Err(e) => panic!("Invalid time format, consult the configuration file for examples of correct ones: {e:?}"),
        };

        let locale = locale_str.and_then(|l| Locale::try_from(l).ok()).unwrap_or(Locale::POSIX);
        Button {
            action,
            active: false,
            changed: false,
            image: ButtonImage::Time(format_items, locale),
        }
    }
    fn render(
        &self,
        c: &Context,
        height: i32,
        button_left_edge: f64,
        button_width: u64,
        y_shift: f64,
    ) {
        match &self.image {
            ButtonImage::Text(text) => {
                let extents = c.text_extents(text).unwrap();
                c.move_to(
                    button_left_edge + (button_width as f64 / 2.0 - extents.width() / 2.0).round(),
                    y_shift + (height as f64 / 2.0 + extents.height() / 2.0).round(),
                );
                c.show_text(text).unwrap();
            }
            ButtonImage::Svg(svg) => {
                let x =
                    button_left_edge + (button_width as f64 / 2.0 - (ICON_SIZE / 2) as f64).round();
                let y = y_shift + ((height as f64 - ICON_SIZE as f64) / 2.0).round();

                svg.render_document(c, &Rectangle::new(x, y, ICON_SIZE as f64, ICON_SIZE as f64))
                    .unwrap();
            }
            ButtonImage::Bitmap(surf) => {
                let x =
                    button_left_edge + (button_width as f64 / 2.0 - (ICON_SIZE / 2) as f64).round();
                let y = y_shift + ((height as f64 - ICON_SIZE as f64) / 2.0).round();
                c.set_source_surface(surf, x, y).unwrap();
                c.rectangle(x, y, ICON_SIZE as f64, ICON_SIZE as f64);
                c.fill().unwrap();
            }
            ButtonImage::Time(format, locale) => {
                let current_time = Local::now();
                let formatted_time = current_time.format_localized_with_items(format.iter(), *locale).to_string();
                let time_extents = c.text_extents(&formatted_time).unwrap();
                c.move_to(
                    button_left_edge + (button_width as f64 / 2.0 - time_extents.width() / 2.0).round(),
                    y_shift + (height as f64 / 2.0 + time_extents.height() / 2.0).round()
                );
                c.show_text(&formatted_time).unwrap();
            }
            ButtonImage::Battery(battery, battery_mode, icons) => {
                let (capacity, state) = get_battery_state(battery);
                let icon = if battery_mode.should_draw_icon() {
                    Some(match state {
                        BatteryState::Charging => match capacity {
                            0..=20 => &icons.charging[0],
                            21..=30 => &icons.charging[1],
                            31..=50 => &icons.charging[2],
                            51..=60 => &icons.charging[3],
                            61..=80 => &icons.charging[4],
                            81..=99 => &icons.charging[5],
                            _ => &icons.charging[6],
                        },
                        _ => match capacity {
                            0 => &icons.plain[0],
                            1..=20 => &icons.plain[1],
                            21..=30 => &icons.plain[2],
                            31..=50 => &icons.plain[3],
                            51..=60 => &icons.plain[4],
                            61..=80 => &icons.plain[5],
                            81..=99 => &icons.plain[6],
                            _ => &icons.plain[7],
                        },
                    })
                } else if state == BatteryState::Charging {
                    Some(&icons.bolt)
                } else {
                    None
                };
                let percent_str = format!("{:.0}%", capacity);
                let extents = c.text_extents(&percent_str).unwrap();
                let mut width = extents.width();
                let mut text_offset = 0;
                if let Some(svg) = icon {
                    if !battery_mode.should_draw_text() {
                        width = ICON_SIZE as f64;
                    } else {
                        width += ICON_SIZE as f64;
                    }
                    text_offset = ICON_SIZE;
                    let x =
                        button_left_edge + (button_width as f64 / 2.0 - width / 2.0).round();
                    let y = y_shift + ((height as f64 - ICON_SIZE as f64) / 2.0).round();

                    svg.render_document(c, &Rectangle::new(x, y, ICON_SIZE as f64, ICON_SIZE as f64))
                        .unwrap();
                }
                if battery_mode.should_draw_text() {
                    c.move_to(
                        button_left_edge + (button_width as f64 / 2.0 - width / 2.0 + text_offset as f64).round(),
                        y_shift + (height as f64 / 2.0 + extents.height() / 2.0).round(),
                    );
                    c.show_text(&percent_str).unwrap();
                }
            }
        }
    }
    fn set_active<F>(&mut self, uinput: &mut UInputHandle<F>, active: bool)
    where
        F: AsRawFd,
    {
        if self.active != active {
            self.active = active;
            self.changed = true;

            toggle_key(uinput, self.action, active as i32);
        }
    }
    fn set_backround_color(&self, c: &Context, color: f64) {
        if let ButtonImage::Battery(battery, _, _) = &self.image {
            let (_, state) = get_battery_state(battery);
            match state {
                BatteryState::NotCharging => c.set_source_rgb(color, color, color),
                BatteryState::Charging => c.set_source_rgb(0.0, color, 0.0),
                BatteryState::Low => c.set_source_rgb(color, 0.0, 0.0),
            }
        } else {
            c.set_source_rgb(color, color, color);
        }
    }
}

#[derive(Default)]
pub struct FunctionLayer {
    displays_time: bool,
    displays_battery: bool,
    buttons: Vec<(usize, Button)>,
    virtual_button_count: usize,
}

impl FunctionLayer {
    fn with_config(cfg: Vec<ButtonConfig>) -> FunctionLayer {
        if cfg.is_empty() {
            panic!("Invalid configuration, layer has 0 buttons");
        }

        let mut virtual_button_count = 0;
        FunctionLayer {
            displays_time: cfg.iter().any(|cfg| cfg.time.is_some()),
            displays_battery: cfg.iter().any(|cfg| cfg.battery.is_some()),
            buttons: cfg
                .into_iter()
                .scan(&mut virtual_button_count, |state, cfg| {
                    let i = **state;
                    let mut stretch = cfg.stretch.unwrap_or(1);
                    if stretch < 1 {
                        println!("Stretch value must be at least 1, setting to 1.");
                        stretch = 1;
                    }
                    **state += stretch;
                    Some((i, Button::with_config(cfg)))
                })
                .collect(),
            virtual_button_count,
        }
    }
    fn draw(
        &mut self,
        config: &Config,
        width: i32,
        height: i32,
        surface: &Surface,
        pixel_shift: (f64, f64),
        complete_redraw: bool,
    ) -> Vec<ClipRect> {
        let c = Context::new(surface).unwrap();
        let mut modified_regions = if complete_redraw {
            vec![ClipRect::new(0, 0, height as u16, width as u16)]
        } else {
            Vec::new()
        };
        c.translate(height as f64, 0.0);
        c.rotate((90.0f64).to_radians());
        let pixel_shift_width = if config.enable_pixel_shift {
            PIXEL_SHIFT_WIDTH_PX
        } else {
            0
        };
        let virtual_button_width = ((width - pixel_shift_width as i32)
            - (BUTTON_SPACING_PX * (self.virtual_button_count - 1) as i32))
            as f64
            / self.virtual_button_count as f64;
        let radius = 8.0f64;
        let bot = (height as f64) * 0.15;
        let top = (height as f64) * 0.85;
        let (pixel_shift_x, pixel_shift_y) = pixel_shift;

        if complete_redraw {
            c.set_source_rgb(0.0, 0.0, 0.0);
            c.paint().unwrap();
        }
        c.set_font_face(&config.font_face);
        c.set_font_size(32.0);

        for i in 0..self.buttons.len() {
            let end = if i + 1 < self.buttons.len() {
                self.buttons[i + 1].0
            } else {
                self.virtual_button_count
            };
            let (start, button) = &mut self.buttons[i];
            let start = *start;

            if !button.changed && !complete_redraw {
                continue;
            };

            let left_edge = (start as f64 * (virtual_button_width + BUTTON_SPACING_PX as f64))
                .floor()
                + pixel_shift_x
                + (pixel_shift_width / 2) as f64;

            let button_width = virtual_button_width
                + ((end - start - 1) as f64 * (virtual_button_width + BUTTON_SPACING_PX as f64))
                    .floor();

            let color = if button.active {
                BUTTON_COLOR_ACTIVE
            } else if config.show_button_outlines {
                BUTTON_COLOR_INACTIVE
            } else {
                0.0
            };
            if !complete_redraw {
                c.set_source_rgb(0.0, 0.0, 0.0);
                c.rectangle(
                    left_edge,
                    bot - radius,
                    button_width,
                    top - bot + radius * 2.0,
                );
                c.fill().unwrap();
            }
            button.set_backround_color(&c, color);
            // draw box with rounded corners
            c.new_sub_path();
            let left = left_edge + radius;
            let right = (left_edge + button_width.ceil()) - radius;
            c.arc(
                right,
                bot,
                radius,
                (-90.0f64).to_radians(),
                (0.0f64).to_radians(),
            );
            c.arc(
                right,
                top,
                radius,
                (0.0f64).to_radians(),
                (90.0f64).to_radians(),
            );
            c.arc(
                left,
                top,
                radius,
                (90.0f64).to_radians(),
                (180.0f64).to_radians(),
            );
            c.arc(
                left,
                bot,
                radius,
                (180.0f64).to_radians(),
                (270.0f64).to_radians(),
            );
            c.close_path();

            c.fill().unwrap();
            c.set_source_rgb(1.0, 1.0, 1.0);
            button.render(
                &c,
                height,
                left_edge,
                button_width.ceil() as u64,
                pixel_shift_y,
            );

            button.changed = false;

            if !complete_redraw {
                modified_regions.push(ClipRect::new(
                    height as u16 - top as u16 - radius as u16,
                    left_edge as u16,
                    height as u16 - bot as u16 + radius as u16,
                    left_edge as u16 + button_width as u16,
                ));
            }
        }

        modified_regions
    }

    fn hit(&self, width: u16, height: u16, x: f64, y: f64, i: Option<usize>) -> Option<usize> {
        let virtual_button_width =
            (width as i32 - (BUTTON_SPACING_PX * (self.virtual_button_count - 1) as i32)) as f64
                / self.virtual_button_count as f64;

        let i = i.unwrap_or_else(|| {
            let virtual_i = (x / (width as f64 / self.virtual_button_count as f64)) as usize;
            self.buttons
                .iter()
                .position(|(start, _)| *start > virtual_i)
                .unwrap_or(self.buttons.len())
                - 1
        });
        if i >= self.buttons.len() {
            return None;
        }

        let start = self.buttons[i].0;
        let end = if i + 1 < self.buttons.len() {
            self.buttons[i + 1].0
        } else {
            self.virtual_button_count
        };

        let left_edge = (start as f64 * (virtual_button_width + BUTTON_SPACING_PX as f64)).floor();

        let button_width = virtual_button_width
            + ((end - start - 1) as f64 * (virtual_button_width + BUTTON_SPACING_PX as f64))
                .floor();

        if x < left_edge
            || x > (left_edge + button_width)
            || y < 0.1 * height as f64
            || y > 0.9 * height as f64
        {
            return None;
        }

        Some(i)
    }
}

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        let mode = flags & O_ACCMODE;

        OpenOptions::new()
            .custom_flags(flags)
            .read(mode == O_RDONLY || mode == O_RDWR)
            .write(mode == O_WRONLY || mode == O_RDWR)
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        _ = File::from(fd);
    }
}

fn emit<F>(uinput: &mut UInputHandle<F>, ty: EventKind, code: u16, value: i32)
where
    F: AsRawFd,
{
    uinput
        .write(&[input_event {
            value,
            type_: ty as u16,
            code,
            time: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        }])
        .unwrap();
}

fn toggle_key<F>(uinput: &mut UInputHandle<F>, code: Key, value: i32)
where
    F: AsRawFd,
{
    emit(uinput, EventKind::Key, code as u16, value);
    emit(
        uinput,
        EventKind::Synchronize,
        SynchronizeKind::Report as u16,
        0,
    );
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("vulcanbar=info"));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .compact()
        .init();
}

fn print_help() {
    eprintln!("VulcanBar - Waybar-like Touch Bar daemon for T2 MacBooks");
    eprintln!();
    eprintln!("Usage: vulcanbar [OPTIONS]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --modular    Use new modular architecture (Waybar-style config)");
    eprintln!("  --legacy     Use legacy tiny-dfr mode (default)");
    eprintln!("  --help       Show this help message");
    eprintln!("  --version    Show version information");
    eprintln!();
    eprintln!("Environment:");
    eprintln!("  RUST_LOG=vulcanbar=debug    Enable debug logging");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for help/version flags
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        eprintln!("vulcanbar {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    // Check for modular mode
    let use_modular = args.iter().any(|a| a == "--modular");

    // Initialize tracing for modular mode
    if use_modular {
        init_tracing();
        info!("Starting VulcanBar in modular mode");
    }

    let mut drm = DrmBackend::open_card().unwrap();
    let (height, width) = drm.mode().size();

    // Select which main loop to run
    let result = if use_modular {
        panic::catch_unwind(AssertUnwindSafe(|| modular_main(&mut drm)))
    } else {
        panic::catch_unwind(AssertUnwindSafe(|| real_main(&mut drm)))
    };

    // Handle panic - show crash bitmap
    if result.is_err() {
        error!("VulcanBar crashed, showing crash bitmap");
    }

    let crash_bitmap = include_bytes!("crash_bitmap.raw");
    let mut map = drm.map().unwrap();
    let data = map.as_mut();
    let mut wptr = 0;
    for byte in crash_bitmap {
        for i in 0..8 {
            let bit = ((byte >> i) & 0x1) == 0;
            let color = if bit { 0xFF } else { 0x0 };
            data[wptr] = color;
            data[wptr + 1] = color;
            data[wptr + 2] = color;
            data[wptr + 3] = color;
            wptr += 4;
        }
    }
    drop(map);
    drm.dirty(&[ClipRect::new(0, 0, height, width)]).unwrap();
    let mut sigset = SigSet::empty();
    sigset.add(Signal::SIGTERM);
    sigset.wait().unwrap();
}

fn real_main(drm: &mut DrmBackend) {
    let (height, width) = drm.mode().size();
    let (db_width, db_height) = drm.fb_info().unwrap().size();
    let mut uinput = UInputHandle::new(OpenOptions::new().write(true).open("/dev/uinput").unwrap());
    let mut backlight = BacklightManager::new();
    let mut last_redraw_minute = Local::now().minute();
    let mut cfg_mgr = ConfigManager::new();
    let (mut cfg, mut layers) = cfg_mgr.load_config(width);
    let mut pixel_shift = PixelShiftManager::new();

    // drop privileges to input and video group
    let groups = ["input", "video"];

    PrivDrop::default()
        .user("nobody")
        .group_list(&groups)
        .apply()
        .unwrap_or_else(|e| panic!("Failed to drop privileges: {}", e));

    let mut surface =
        ImageSurface::create(Format::ARgb32, db_width as i32, db_height as i32).unwrap();
    let mut active_layer = 0;
    let mut needs_complete_redraw = true;

    let mut input_tb = Libinput::new_with_udev(Interface);
    let mut input_main = Libinput::new_with_udev(Interface);
    input_tb.udev_assign_seat("seat-touchbar").unwrap();
    input_main.udev_assign_seat("seat0").unwrap();
    let udev_monitor = MonitorBuilder::new()
        .unwrap()
        .match_subsystem("power_supply")
        .unwrap()
        .listen()
        .unwrap();
    let epoll = Epoll::new(EpollCreateFlags::empty()).unwrap();
    epoll
        .add(input_main.as_fd(), EpollEvent::new(EpollFlags::EPOLLIN, 0))
        .unwrap();
    epoll
        .add(input_tb.as_fd(), EpollEvent::new(EpollFlags::EPOLLIN, 1))
        .unwrap();
    epoll
        .add(cfg_mgr.fd(), EpollEvent::new(EpollFlags::EPOLLIN, 2))
        .unwrap();
    epoll
        .add(&udev_monitor, EpollEvent::new(EpollFlags::EPOLLIN, 3))
        .unwrap();
    uinput.set_evbit(EventKind::Key).unwrap();
    for layer in &layers {
        for button in &layer.buttons {
            uinput.set_keybit(button.1.action).unwrap();
        }
    }
    let mut dev_name_c = [0 as c_char; 80];
    let dev_name = "Dynamic Function Row Virtual Input Device".as_bytes();
    for i in 0..dev_name.len() {
        dev_name_c[i] = dev_name[i] as c_char;
    }
    uinput
        .dev_setup(&uinput_setup {
            id: input_id {
                bustype: 0x19,
                vendor: 0x1209,
                product: 0x316E,
                version: 1,
            },
            ff_effects_max: 0,
            name: dev_name_c,
        })
        .unwrap();
    uinput.dev_create().unwrap();

    let mut digitizer: Option<InputDevice> = None;
    let mut touches = HashMap::new();
    loop {
        if cfg_mgr.update_config(&mut cfg, &mut layers, width) {
            active_layer = 0;
            needs_complete_redraw = true;
        }

        let now = Local::now();
        let ms_left = ((60 - now.second()) * 1000) as i32;
        let mut next_timeout_ms = min(ms_left, TIMEOUT_MS);

        if cfg.enable_pixel_shift {
            let (pixel_shift_needs_redraw, pixel_shift_next_timeout_ms) = pixel_shift.update();
            if pixel_shift_needs_redraw {
                needs_complete_redraw = true;
            }
            next_timeout_ms = min(next_timeout_ms, pixel_shift_next_timeout_ms);
        }

        let current_minute = now.minute();
        if layers[active_layer].displays_time && (current_minute != last_redraw_minute) {
            needs_complete_redraw = true;
            last_redraw_minute = current_minute;
        }
        if layers[active_layer].displays_battery {
            for button in &mut layers[active_layer].buttons {
                if let ButtonImage::Battery(_, _, _) = button.1.image {
                    button.1.changed = true;
                }
            }
        }

        if needs_complete_redraw || layers[active_layer].buttons.iter().any(|b| b.1.changed) {
            let shift = if cfg.enable_pixel_shift {
                pixel_shift.get()
            } else {
                (0.0, 0.0)
            };
            let clips = layers[active_layer].draw(
                &cfg,
                width as i32,
                height as i32,
                &surface,
                shift,
                needs_complete_redraw,
            );
            let data = surface.data().unwrap();
            drm.map().unwrap().as_mut()[..data.len()].copy_from_slice(&data);
            drm.dirty(&clips).unwrap();
            needs_complete_redraw = false;
        }

        match epoll.wait(
            &mut [EpollEvent::new(EpollFlags::EPOLLIN, 0)],
            next_timeout_ms as u16,
        ) {
            Err(Errno::EINTR) | Ok(_) => 0,
            e => e.unwrap(),
        };

        _ = udev_monitor.iter().last();

        input_tb.dispatch().unwrap();
        input_main.dispatch().unwrap();
        for event in &mut input_tb.clone().chain(input_main.clone()) {
            backlight.process_event(&event);
            match event {
                Event::Device(DeviceEvent::Added(evt)) => {
                    let dev = evt.device();
                    if dev.name().contains(" Touch Bar") {
                        digitizer = Some(dev);
                    }
                }
                Event::Keyboard(KeyboardEvent::Key(key)) => {
                    if key.key() == Key::Fn as u32 {
                        let new_layer = match key.key_state() {
                            KeyState::Pressed => 1,
                            KeyState::Released => 0,
                        };
                        if active_layer != new_layer {
                            active_layer = new_layer;
                            needs_complete_redraw = true;
                        }
                    }
                }
                Event::Touch(te) => {
                    if Some(te.device()) != digitizer || backlight.current_bl() == 0 {
                        continue;
                    }
                    match te {
                        TouchEvent::Down(dn) => {
                            let x = dn.x_transformed(width as u32);
                            let y = dn.y_transformed(height as u32);
                            if let Some(btn) = layers[active_layer].hit(width, height, x, y, None) {
                                touches.insert(dn.seat_slot(), (active_layer, btn));
                                layers[active_layer].buttons[btn]
                                    .1
                                    .set_active(&mut uinput, true);
                            }
                        }
                        TouchEvent::Motion(mtn) => {
                            if !touches.contains_key(&mtn.seat_slot()) {
                                continue;
                            }

                            let x = mtn.x_transformed(width as u32);
                            let y = mtn.y_transformed(height as u32);
                            let (layer, btn) = *touches.get(&mtn.seat_slot()).unwrap();
                            let hit = layers[active_layer]
                                .hit(width, height, x, y, Some(btn))
                                .is_some();
                            layers[layer].buttons[btn].1.set_active(&mut uinput, hit);
                        }
                        TouchEvent::Up(up) => {
                            if !touches.contains_key(&up.seat_slot()) {
                                continue;
                            }
                            let (layer, btn) = *touches.get(&up.seat_slot()).unwrap();
                            layers[layer].buttons[btn].1.set_active(&mut uinput, false);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        backlight.update_backlight(&cfg);
    }
}

/// New modular main loop using Waybar-style configuration
fn modular_main(drm: &mut DrmBackend) {
    let (height, width) = drm.mode().size();
    let (db_width, db_height) = drm.fb_info().unwrap().size();

    info!("Touch Bar display: {}x{} (framebuffer: {}x{})", width, height, db_width, db_height);

    // Load configuration
    let mut config_loader = match ConfigLoader::new() {
        Ok(loader) => loader,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            panic!("Configuration error: {}", e);
        }
    };

    // Find Hyprland socket EARLY - before creating modules
    // This must happen before PageManager creation so modules can use hyprctl
    let hyprland_socket = find_hyprland_socket();
    if let Some(ref sock) = hyprland_socket {
        std::env::set_var("HYPRLAND_INSTANCE_SIGNATURE", &sock.signature);
        std::env::set_var("XDG_RUNTIME_DIR", &sock.runtime_dir);
        info!("Set Hyprland env vars: HYPRLAND_INSTANCE_SIGNATURE={}", sock.signature);
    }

    // Create module registry
    let registry = ModuleRegistry::with_builtins();
    info!("Registered modules: {:?}", registry.available_modules());

    // Create PageManager (supports both single-page and multi-page modes)
    let mut page_manager = match PageManager::from_config(
        config_loader.config(),
        &registry,
        width as i32,
        height as i32,
    ) {
        Ok(pm) => pm,
        Err(e) => {
            error!("Failed to create page manager: {}", e);
            panic!("PageManager error: {}", e);
        }
    };

    info!(
        "Created page manager with {} pages (current: {})",
        page_manager.page_count(),
        page_manager.current_page_name()
    );

    // Set up backlight and pixel shift
    let mut backlight = BacklightManager::new();
    let mut pixel_shift = PixelShiftManager::new();
    let config = config_loader.config();

    // Create drawing surface
    let mut surface = ImageSurface::create(Format::ARgb32, db_width as i32, db_height as i32)
        .expect("Failed to create surface");

    // Set up libinput for touch events
    let mut input_tb = Libinput::new_with_udev(Interface);
    let mut input_main = Libinput::new_with_udev(Interface);
    input_tb.udev_assign_seat("seat-touchbar").unwrap();
    input_main.udev_assign_seat("seat0").unwrap();

    // Set up epoll for event multiplexing
    let epoll = Epoll::new(EpollCreateFlags::empty()).unwrap();
    epoll.add(input_main.as_fd(), EpollEvent::new(EpollFlags::EPOLLIN, 0)).unwrap();
    epoll.add(input_tb.as_fd(), EpollEvent::new(EpollFlags::EPOLLIN, 1)).unwrap();
    epoll.add(config_loader.fd(), EpollEvent::new(EpollFlags::EPOLLIN, 2)).unwrap();

    // Set up uinput for key events
    let mut uinput = UInputHandle::new(
        OpenOptions::new().write(true).open("/dev/uinput").unwrap()
    );
    uinput.set_evbit(EventKind::Key).unwrap();

    // Register common function keys
    for key in [
        Key::BrightnessDown, Key::BrightnessUp,
        Key::PreviousSong, Key::PlayPause, Key::NextSong,
        Key::Mute, Key::VolumeDown, Key::VolumeUp,
        Key::F1, Key::F2, Key::F3, Key::F4, Key::F5, Key::F6,
        Key::F7, Key::F8, Key::F9, Key::F10, Key::F11, Key::F12,
    ] {
        uinput.set_keybit(key).unwrap();
    }

    // Create uinput device
    let mut dev_name_c = [0 as c_char; 80];
    let dev_name = "VulcanBar Touch Bar Input".as_bytes();
    for i in 0..dev_name.len() {
        dev_name_c[i] = dev_name[i] as c_char;
    }
    uinput.dev_setup(&uinput_setup {
        id: input_id {
            bustype: 0x19,
            vendor: 0x1209,
            product: 0x316E,
            version: 2,
        },
        ff_effects_max: 0,
        name: dev_name_c,
    }).unwrap();
    uinput.dev_create().unwrap();

    // Determine which user to drop privileges to
    // If we found a Hyprland socket, drop to that user so we can access the socket
    // Otherwise fall back to "nobody" for security
    let drop_user = if let Some(ref sock) = hyprland_socket {
        info!("Found Hyprland socket: {} in {} (uid {})", sock.signature, sock.runtime_dir, sock.uid);
        // Get username from UID
        let passwd = unsafe { libc::getpwuid(sock.uid) };
        if !passwd.is_null() {
            let username = unsafe { std::ffi::CStr::from_ptr((*passwd).pw_name) }
                .to_string_lossy()
                .into_owned();
            info!("Will drop privileges to user: {} (uid {})", username, sock.uid);
            username
        } else {
            warn!("Could not resolve uid {} to username, falling back to nobody", sock.uid);
            "nobody".to_string()
        }
    } else {
        warn!("Hyprland socket not found - workspace switching disabled");
        "nobody".to_string()
    };

    // Drop privileges
    let groups = ["input", "video"];
    PrivDrop::default()
        .user(&drop_user)
        .group_list(&groups)
        .apply()
        .unwrap_or_else(|e| panic!("Failed to drop privileges: {}", e));

    info!("VulcanBar modular mode running");

    // Main event loop state
    let mut needs_redraw = true;
    let mut digitizer: Option<InputDevice> = None;
    let mut last_update = Instant::now();

    // Create gesture detector with config (if provided)
    let gesture_config = if let Some(ref gestures) = config_loader.config().gestures {
        GestureConfig {
            swipe_threshold_px: gestures.swipe_threshold_px,
            swipe_velocity_threshold: gestures.swipe_velocity_threshold,
            long_press_duration_ms: gestures.long_press_duration_ms,
            ..GestureConfig::default()
        }
    } else {
        GestureConfig::default()
    };
    let mut gesture_detector = GestureDetector::with_config(gesture_config);

    // Calculate update interval from modules
    let update_interval = page_manager.min_update_interval()
        .unwrap_or(Duration::from_secs(1));

    loop {
        // Check for config reload
        if config_loader.check_reload() {
            info!("Configuration reloaded");
            if let Ok(new_page_manager) = PageManager::from_config(
                config_loader.config(),
                &registry,
                width as i32,
                height as i32,
            ) {
                page_manager = new_page_manager;
                needs_redraw = true;
            }
        }

        // Update page manager (handles transitions and module updates)
        let now = Instant::now();
        if page_manager.update() {
            needs_redraw = true;
        }
        if now.duration_since(last_update) >= update_interval {
            if let Ok(changed) = page_manager.update_modules() {
                needs_redraw = needs_redraw || changed;
            }
            last_update = now;
        }

        // Update pixel shift
        let config = config_loader.config();
        let shift = if config.general.enable_pixel_shift {
            let (shift_changed, _) = pixel_shift.update();
            if shift_changed {
                needs_redraw = true;
            }
            pixel_shift.get()
        } else {
            (0.0, 0.0)
        };

        // Render if needed
        if needs_redraw {
            debug!("Rendering frame");
            match page_manager.render(&surface, &config.general, shift) {
                Ok(clips) => {
                    let data = surface.data().unwrap();
                    drm.map().unwrap().as_mut()[..data.len()].copy_from_slice(&data);
                    drm.dirty(&clips).unwrap();
                }
                Err(e) => {
                    warn!("Render error: {}", e);
                }
            }
            needs_redraw = false;
        }

        // Wait for events
        let timeout_ms = update_interval.as_millis() as u16;
        match epoll.wait(
            &mut [EpollEvent::new(EpollFlags::EPOLLIN, 0)],
            timeout_ms.min(1000),
        ) {
            Err(Errno::EINTR) | Ok(_) => {}
            Err(e) => warn!("epoll error: {}", e),
        }

        // Process input events
        input_tb.dispatch().unwrap();
        input_main.dispatch().unwrap();

        for event in &mut input_tb.clone().chain(input_main.clone()) {
            backlight.process_event(&event);

            match event {
                Event::Device(DeviceEvent::Added(evt)) => {
                    let dev = evt.device();
                    debug!("Device added: {}", dev.name());
                    // Match the actual Touch Bar touchpad, not our virtual device
                    if dev.name().contains("Touch Bar") && dev.name().contains("Touchpad") {
                        debug!("Touch Bar digitizer found: {}", dev.name());
                        digitizer = Some(dev);
                    }
                }
                Event::Touch(te) => {
                    // Debug: log all touch events
                    debug!("Touch event from device: {}", te.device().name());

                    if Some(te.device()) != digitizer {
                        debug!("Touch ignored: device mismatch (expected Touch Bar)");
                        continue;
                    }
                    if backlight.current_bl() == 0 {
                        debug!("Touch ignored: backlight is off");
                        continue;
                    }

                    // Convert touch event to gesture detector input
                    let (phase, slot, x, y) = match te {
                        TouchEvent::Down(dn) => {
                            let x = dn.x_transformed(width as u32);
                            let y = dn.y_transformed(height as u32);
                            debug!("Touch down at ({:.1}, {:.1}) slot {}", x, y, dn.seat_slot());
                            (TouchPhase::Start, dn.seat_slot() as i32, x, y)
                        }
                        TouchEvent::Motion(mtn) => {
                            let x = mtn.x_transformed(width as u32);
                            let y = mtn.y_transformed(height as u32);
                            (TouchPhase::Move, mtn.seat_slot() as i32, x, y)
                        }
                        TouchEvent::Up(up) => {
                            debug!("Touch up slot {}", up.seat_slot());
                            // Position may not be available on up, use 0,0
                            (TouchPhase::End, up.seat_slot() as i32, 0.0, 0.0)
                        }
                        TouchEvent::Cancel(cancel) => {
                            (TouchPhase::Cancel, cancel.seat_slot() as i32, 0.0, 0.0)
                        }
                        _ => continue,
                    };

                    // Process through gesture detector
                    if let Some(gesture) = gesture_detector.process_touch(phase, slot, x, y) {
                        debug!("Gesture recognized: {:?}", gesture);
                        match gesture {
                            Gesture::Tap { x, y } => {
                                // Tap: Send to page_manager for module interaction
                                // Press event
                                if let Some(action) = page_manager.handle_touch(x, y, true, slot) {
                                    debug!("Action from press: {:?}", action);
                                    // Handle SwitchToPage actions here (needs page_manager access)
                                    if let Action::SwitchToPage(ref page_name) = action {
                                        if page_manager.switch_to_page_by_name(page_name) {
                                            info!("Switching to page: {}", page_name);
                                        }
                                    } else {
                                        handle_action(&mut uinput, action, hyprland_socket.as_ref());
                                    }
                                }
                                // Release event - IMPORTANT: modules return actions on release!
                                if let Some(action) = page_manager.handle_touch(x, y, false, slot) {
                                    debug!("Action from release: {:?}", action);
                                    // Handle SwitchToPage actions here (needs page_manager access)
                                    if let Action::SwitchToPage(ref page_name) = action {
                                        if page_manager.switch_to_page_by_name(page_name) {
                                            info!("Switching to page: {}", page_name);
                                        }
                                    } else {
                                        handle_action(&mut uinput, action, hyprland_socket.as_ref());
                                    }
                                }
                                needs_redraw = true;
                            }
                            Gesture::Swipe { direction, velocity, .. } => {
                                // Swipe: Switch pages
                                debug!("Swipe {:?} with velocity {:.1}px/s", direction, velocity);
                                let old_page = page_manager.current_page_name().to_string();
                                page_manager.switch_page(direction);
                                let new_page = page_manager.target_page_name();
                                if old_page != new_page {
                                    info!(
                                        "Page transition: {} -> {} ({}/{})",
                                        old_page,
                                        new_page,
                                        page_manager.target_page_index() + 1,
                                        page_manager.page_count()
                                    );
                                } else {
                                    debug!("Already at edge, no page switch");
                                }
                                needs_redraw = true;
                            }
                            Gesture::LongPress { x, y } => {
                                // Long press: Could be used for secondary actions
                                debug!("Long press at ({:.1}, {:.1})", x, y);
                                // TODO: Handle long press (e.g., context menu)
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Update backlight based on legacy config (could be enhanced later)
        // backlight.update_backlight(&legacy_cfg);
    }
}

/// Extract UID from a runtime directory path like /run/user/1000
fn extract_uid_from_runtime_dir(runtime_dir: &str) -> Option<u32> {
    runtime_dir
        .strip_prefix("/run/user/")
        .and_then(|s| s.parse::<u32>().ok())
}

/// Find Hyprland socket instance signature and runtime directory
fn find_hyprland_socket() -> Option<HyprlandSocketInfo> {
    // Check environment first
    if let Ok(sig) = std::env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        let uid = unsafe { libc::getuid() };
        let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
            .unwrap_or_else(|_| format!("/run/user/{}", uid));
        return Some(HyprlandSocketInfo {
            signature: sig,
            runtime_dir,
            uid,
        });
    }

    // Look for Hyprland sockets in various locations
    // Tuple of (hypr_path, runtime_dir, uid)
    let current_uid = unsafe { libc::getuid() };
    let search_paths: [(String, String, u32); 3] = [
        // /tmp/hypr is typically root-owned, use current UID as fallback
        ("/tmp/hypr".to_string(), "/tmp".to_string(), current_uid),
        // Common UID 1000 (first regular user)
        ("/run/user/1000/hypr".to_string(), "/run/user/1000".to_string(), 1000),
        // Current user's runtime dir
        (
            format!("/run/user/{}/hypr", current_uid),
            format!("/run/user/{}", current_uid),
            current_uid,
        ),
    ];

    for (hypr_path, runtime_dir, uid) in &search_paths {
        let hypr_dir = std::path::Path::new(hypr_path);
        if let Ok(entries) = std::fs::read_dir(hypr_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // Check if this looks like a Hyprland instance
                    let socket_path = path.join(".socket.sock");
                    if socket_path.exists() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            debug!("Found Hyprland socket: {} in {} (uid {})", name, runtime_dir, uid);
                            return Some(HyprlandSocketInfo {
                                signature: name.to_string(),
                                runtime_dir: runtime_dir.clone(),
                                uid: *uid,
                            });
                        }
                    }
                }
            }
        }
    }

    None
}

/// Handle module actions
fn handle_action<F: AsRawFd>(uinput: &mut UInputHandle<F>, action: Action, hyprland_socket: Option<&HyprlandSocketInfo>) {
    match action {
        Action::KeyPress(key) => {
            emit(uinput, EventKind::Key, key as u16, 1);
            emit(uinput, EventKind::Synchronize, SynchronizeKind::Report as u16, 0);
            emit(uinput, EventKind::Key, key as u16, 0);
            emit(uinput, EventKind::Synchronize, SynchronizeKind::Report as u16, 0);
        }
        Action::Command(cmd) => {
            debug!("Executing command: {}", cmd);
            if let Err(e) = std::process::Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .spawn()
            {
                warn!("Failed to execute command: {}", e);
            }
        }
        Action::Workspace(id) => {
            debug!("Switching to workspace {}", id);
            // Hyprland workspace switch via hyprctl
            // Must set both signature AND runtime_dir since we're running as nobody after privdrop
            if let Some(socket_info) = hyprland_socket {
                debug!("Using Hyprland socket: {} with runtime_dir: {}",
                       socket_info.signature, socket_info.runtime_dir);
                let _ = std::process::Command::new("hyprctl")
                    .env("HYPRLAND_INSTANCE_SIGNATURE", &socket_info.signature)
                    .env("XDG_RUNTIME_DIR", &socket_info.runtime_dir)
                    .args(["dispatch", "workspace", &id.to_string()])
                    .spawn();
            } else {
                warn!("Cannot switch workspace: Hyprland socket not found");
            }
        }
        Action::ToggleMute => {
            debug!("Toggling mute");
            let _ = std::process::Command::new("pactl")
                .args(["set-sink-mute", "@DEFAULT_SINK@", "toggle"])
                .spawn();
        }
        Action::SwitchToPage(page_name) => {
            debug!("SwitchToPage action received: {}", page_name);
            // Note: This action is handled by returning it to the caller
            // The PageManager will handle the actual page switch
        }
        Action::None => {}
    }
}
