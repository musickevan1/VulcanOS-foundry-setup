//! Battery Module
//!
//! Displays battery status with percentage and/or icon.

use anyhow::Result;
use crossbeam_channel::Sender;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use super::base::{
    draw_centered_text, draw_rounded_rect, get_colored_background, ColorType, CORNER_RADIUS,
};
use super::{Action, Module, ModuleEvent, RenderContext, TouchEvent};
use crate::config::BatteryConfig;

/// Battery charging state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryState {
    Charging,
    Discharging,
    Full,
    NotCharging,
    Unknown,
}

/// Battery module that displays battery status
pub struct BatteryModule {
    name: String,
    config: BatteryConfig,
    /// Path to battery device
    battery_path: Option<PathBuf>,
    /// Current battery percentage
    percentage: u32,
    /// Current charging state
    state: BatteryState,
    /// Whether currently being touched
    active: bool,
}

impl BatteryModule {
    /// Create a new battery module with default settings
    pub fn new() -> Self {
        Self::with_config(BatteryConfig::default())
    }

    /// Create a battery module with specific configuration
    pub fn with_config(config: BatteryConfig) -> Self {
        let battery_path = find_battery_device();

        let mut module = Self {
            name: "battery".to_string(),
            config,
            battery_path,
            percentage: 100,
            state: BatteryState::Unknown,
            active: false,
        };

        // Initialize battery state
        let _ = module.update();
        module
    }

    /// Create from TOML configuration
    pub fn from_config(config: &toml::Value) -> Result<Box<dyn Module>> {
        let battery_config: BatteryConfig = config.clone().try_into().unwrap_or_default();
        Ok(Box::new(Self::with_config(battery_config)))
    }

    /// Read battery percentage
    fn read_percentage(&self) -> Option<u32> {
        let path = self.battery_path.as_ref()?;

        // Prefer charge_now / charge_full for accurate current level
        // (capacity file shows % of design capacity, not current max)
        let charge_now = fs::read_to_string(path.join("charge_now"))
            .ok()
            .and_then(|s| s.trim().parse::<f64>().ok());
        let charge_full = fs::read_to_string(path.join("charge_full"))
            .ok()
            .and_then(|s| s.trim().parse::<f64>().ok());

        if let (Some(now), Some(full)) = (charge_now, charge_full) {
            if full > 0.0 {
                return Some(((now / full) * 100.0).round().min(100.0) as u32);
            }
        }

        // Fallback to capacity file
        if let Ok(content) = fs::read_to_string(path.join("capacity")) {
            if let Ok(capacity) = content.trim().parse::<u32>() {
                return Some(capacity.min(100));
            }
        }

        None
    }

    /// Read battery charging state
    fn read_state(&self) -> BatteryState {
        let Some(path) = &self.battery_path else {
            return BatteryState::Unknown;
        };

        let status = fs::read_to_string(path.join("status"))
            .unwrap_or_else(|_| "Unknown".to_string());

        match status.trim() {
            "Charging" => BatteryState::Charging,
            "Discharging" => BatteryState::Discharging,
            "Full" => BatteryState::Full,
            "Not charging" => BatteryState::NotCharging,
            _ => BatteryState::Unknown,
        }
    }

    /// Get color type based on battery state
    fn get_color_type(&self) -> ColorType {
        match self.state {
            BatteryState::Charging | BatteryState::Full => ColorType::Green,
            BatteryState::Discharging | BatteryState::NotCharging | BatteryState::Unknown => {
                if self.percentage <= self.config.critical_threshold {
                    ColorType::Red
                } else if self.percentage <= self.config.low_threshold {
                    ColorType::Yellow
                } else {
                    ColorType::Normal
                }
            }
        }
    }

    /// Get display text based on config
    fn get_display_text(&self) -> String {
        match self.config.display.as_str() {
            "icon" => self.get_battery_icon().to_string(),
            "percentage" => format!("{}%", self.percentage),
            "both" | _ => format!("{} {}%", self.get_battery_icon(), self.percentage),
        }
    }

    /// Get battery icon based on percentage and state
    fn get_battery_icon(&self) -> &'static str {
        let charging = matches!(self.state, BatteryState::Charging);

        if charging {
            match self.percentage {
                0..=20 => "󰢜",
                21..=40 => "󰂆",
                41..=60 => "󰂈",
                61..=80 => "󰂉",
                81..=99 => "󰂋",
                _ => "󰂅",
            }
        } else {
            match self.percentage {
                0..=10 => "󰂎",
                11..=20 => "󰁺",
                21..=30 => "󰁻",
                31..=40 => "󰁼",
                41..=50 => "󰁽",
                51..=60 => "󰁾",
                61..=70 => "󰁿",
                71..=80 => "󰂀",
                81..=90 => "󰂁",
                91..=99 => "󰂂",
                _ => "󰁹",
            }
        }
    }
}

impl Module for BatteryModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn width(&self) -> i32 {
        match self.config.display.as_str() {
            "icon" => 60,
            "percentage" => 80,
            _ => 120,
        }
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        // Draw background with color based on state
        let color = get_colored_background(self.active, ctx.show_outlines, self.get_color_type());
        draw_rounded_rect(
            ctx.cairo,
            ctx.x_offset,
            0.0,
            ctx.width as f64,
            ctx.height as f64,
            CORNER_RADIUS,
            color,
        );

        // Draw battery text/icon
        ctx.cairo.set_source_rgb(1.0, 1.0, 1.0);
        let text = self.get_display_text();
        draw_centered_text(
            ctx.cairo,
            &text,
            ctx.x_offset,
            0.0,
            ctx.width as f64,
            ctx.height as f64,
            ctx.y_offset,
        )?;

        Ok(())
    }

    fn on_touch(&mut self, event: TouchEvent) -> Option<Action> {
        self.active = event.pressed;
        None // Battery has no touch action
    }

    fn update_interval(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.config.interval))
    }

    fn update(&mut self) -> Result<bool> {
        let old_percentage = self.percentage;
        let old_state = self.state;

        self.percentage = self.read_percentage().unwrap_or(100);
        self.state = self.read_state();

        Ok(old_percentage != self.percentage || old_state != self.state)
    }
}

impl Default for BatteryModule {
    fn default() -> Self {
        Self::new()
    }
}

/// Find the battery device path
fn find_battery_device() -> Option<PathBuf> {
    let power_supply_path = "/sys/class/power_supply";

    if let Ok(entries) = fs::read_dir(power_supply_path) {
        for entry in entries.flatten() {
            let dev_path = entry.path();
            let type_path = dev_path.join("type");

            if let Ok(typ) = fs::read_to_string(&type_path) {
                if typ.trim() == "Battery" {
                    return Some(dev_path);
                }
            }
        }
    }
    None
}
