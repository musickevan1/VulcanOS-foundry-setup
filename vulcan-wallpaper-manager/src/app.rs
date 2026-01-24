use gtk::prelude::*;
use relm4::prelude::*;
use adw::prelude::*;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::models::Monitor;
use crate::services::{hyprctl, hyprpaper, thumbnail};
use crate::components::monitor_layout::{MonitorLayoutModel, MonitorLayoutInput, MonitorLayoutOutput};
use crate::components::wallpaper_picker::{WallpaperPickerModel, WallpaperPickerInput, WallpaperPickerOutput};
use crate::components::profile_manager::{ProfileManagerModel, ProfileManagerInput, ProfileManagerOutput};

#[derive(Debug)]
pub enum AppMsg {
    MonitorSelected(String),
    WallpaperSelected(PathBuf),
    ApplyWallpaper,
    RefreshMonitors,
    OpenDirectory,
    // Profile management messages
    ProfileApply(HashMap<String, PathBuf>),
    ProfileSaved(String),
    ProfileError(String),
}

pub struct App {
    monitors: Vec<Monitor>,
    selected_monitor: Option<String>,
    selected_wallpaper: Option<PathBuf>,
    monitor_wallpapers: HashMap<String, PathBuf>,
    monitor_layout: Controller<MonitorLayoutModel>,
    wallpaper_picker: Controller<WallpaperPickerModel>,
    profile_manager: Controller<ProfileManagerModel>,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("VulcanOS Wallpaper Manager"),
            set_default_size: (1000, 700),

            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "Wallpaper Manager",
                        #[watch]
                        set_subtitle: &format!(
                            "{}",
                            model.selected_monitor.as_deref().unwrap_or("Select a monitor")
                        ),
                    },

                    pack_start = &gtk::Button {
                        set_icon_name: "folder-open-symbolic",
                        set_tooltip_text: Some("Open wallpaper folder"),
                        connect_clicked => AppMsg::OpenDirectory,
                    },

                    pack_start = &gtk::Separator {
                        set_orientation: gtk::Orientation::Vertical,
                    },

                    // Profile manager in header
                    pack_start = model.profile_manager.widget() {},

                    pack_end = &gtk::Button {
                        set_icon_name: "view-refresh-symbolic",
                        set_tooltip_text: Some("Refresh"),
                        connect_clicked => AppMsg::RefreshMonitors,
                    },
                },

                #[wrap(Some)]
                set_content = &gtk::Paned {
                    set_orientation: gtk::Orientation::Vertical,
                    set_shrink_start_child: false,
                    set_shrink_end_child: false,
                    set_position: 350,

                    // Top: Monitor layout
                    #[wrap(Some)]
                    set_start_child = &gtk::Frame {
                        set_margin_all: 12,

                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 8,

                            gtk::Label {
                                set_markup: "<b>Monitor Layout</b>",
                                set_halign: gtk::Align::Start,
                            },

                            model.monitor_layout.widget() {},
                        },
                    },

                    // Bottom: Wallpaper picker
                    #[wrap(Some)]
                    set_end_child = &gtk::Frame {
                        set_margin_all: 12,

                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 8,

                            gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 12,

                                gtk::Label {
                                    set_markup: "<b>Wallpapers</b>",
                                    set_halign: gtk::Align::Start,
                                    set_hexpand: true,
                                },

                                gtk::Button {
                                    set_label: "Apply",
                                    #[watch]
                                    set_sensitive: model.selected_monitor.is_some() && model.selected_wallpaper.is_some(),
                                    add_css_class: "suggested-action",
                                    connect_clicked => AppMsg::ApplyWallpaper,
                                },
                            },

                            model.wallpaper_picker.widget() {},
                        },
                    },
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Load monitors on startup
        let monitors = hyprctl::get_monitors().unwrap_or_default();

        // Load current wallpaper assignments
        let mut monitor_wallpapers = HashMap::new();
        if let Ok(active) = hyprpaper::list_active() {
            for (mon, path) in active {
                monitor_wallpapers.insert(mon, PathBuf::from(path));
            }
        }

        // Create monitor layout component
        let monitor_layout = MonitorLayoutModel::builder()
            .launch(monitors.clone())
            .forward(sender.input_sender(), |msg| {
                match msg {
                    MonitorLayoutOutput::Selected(name) => AppMsg::MonitorSelected(name),
                }
            });

        // Create wallpaper picker component
        let wallpaper_dir = thumbnail::default_wallpaper_dir();
        let wallpaper_picker = WallpaperPickerModel::builder()
            .launch(wallpaper_dir)
            .forward(sender.input_sender(), |msg| {
                match msg {
                    WallpaperPickerOutput::Selected(path) => AppMsg::WallpaperSelected(path),
                }
            });

        // Create profile manager component
        let profile_manager = ProfileManagerModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| {
                match msg {
                    ProfileManagerOutput::ApplyProfile(wallpapers) => AppMsg::ProfileApply(wallpapers),
                    ProfileManagerOutput::ProfileSaved(name) => AppMsg::ProfileSaved(name),
                    ProfileManagerOutput::Error(e) => AppMsg::ProfileError(e),
                }
            });

        let model = App {
            monitors,
            selected_monitor: None,
            selected_wallpaper: None,
            monitor_wallpapers,
            monitor_layout,
            wallpaper_picker,
            profile_manager,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::MonitorSelected(name) => {
                self.selected_monitor = Some(name.clone());
                println!("Selected monitor: {}", name);
            }

            AppMsg::WallpaperSelected(path) => {
                self.selected_wallpaper = Some(path.clone());
                println!("Selected wallpaper: {}", path.display());
            }

            AppMsg::ApplyWallpaper => {
                if let (Some(monitor), Some(path)) = (&self.selected_monitor, &self.selected_wallpaper) {
                    match hyprpaper::apply_wallpaper(monitor, path) {
                        Ok(()) => {
                            println!("Applied {} to {}", path.display(), monitor);
                            self.monitor_wallpapers.insert(monitor.clone(), path.clone());
                            // Notify profile manager of current wallpaper state
                            self.profile_manager.emit(ProfileManagerInput::UpdateWallpapers(
                                self.monitor_wallpapers.clone()
                            ));
                        }
                        Err(e) => {
                            eprintln!("Failed to apply wallpaper: {}", e);
                        }
                    }
                }
            }

            AppMsg::RefreshMonitors => {
                if let Ok(monitors) = hyprctl::get_monitors() {
                    self.monitors = monitors.clone();
                    self.monitor_layout.emit(MonitorLayoutInput::UpdateMonitors(monitors));
                }
                self.wallpaper_picker.emit(WallpaperPickerInput::Refresh);
            }

            AppMsg::OpenDirectory => {
                let dir = thumbnail::default_wallpaper_dir();
                if let Err(e) = std::process::Command::new("xdg-open")
                    .arg(&dir)
                    .spawn()
                {
                    eprintln!("Failed to open directory: {}", e);
                }
            }

            AppMsg::ProfileApply(wallpapers) => {
                println!("Applying profile with {} wallpapers", wallpapers.len());
                for (monitor, path) in &wallpapers {
                    if let Err(e) = hyprpaper::apply_wallpaper(monitor, path) {
                        eprintln!("Failed to apply to {}: {}", monitor, e);
                    }
                }
                self.monitor_wallpapers.extend(wallpapers);
            }

            AppMsg::ProfileSaved(name) => {
                println!("Profile saved: {}", name);
            }

            AppMsg::ProfileError(error) => {
                eprintln!("Profile error: {}", error);
            }
        }
    }
}
