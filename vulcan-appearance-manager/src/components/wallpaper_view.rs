use gtk::prelude::*;
use relm4::prelude::*;
use adw::prelude::*;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::models::Monitor;
use crate::services::{hyprctl, thumbnail};
use crate::services::wallpaper_backend::{WallpaperBackend, detect_backend};
use super::monitor_layout::{MonitorLayoutModel, MonitorLayoutInput, MonitorLayoutOutput};
use super::wallpaper_picker::{WallpaperPickerModel, WallpaperPickerInput, WallpaperPickerOutput};
use super::split_dialog::{SplitDialogModel, SplitDialogInput, SplitDialogOutput};

#[derive(Debug)]
pub enum WallpaperViewMsg {
    MonitorSelected(String),
    WallpaperSelected(PathBuf),
    ApplyWallpaper,
    RefreshMonitors,
    OpenDirectory,
    ShowSplitDialog,
    SplitGenerated(HashMap<String, PathBuf>),
    SplitCancelled,
    SplitError(String),
    ApplyProfile(HashMap<String, PathBuf>),
    SetWallpaperForAll(PathBuf),  // Apply wallpaper to all monitors
}

#[derive(Debug)]
pub enum WallpaperViewOutput {
    ShowToast(String),
    WallpapersChanged(HashMap<String, PathBuf>),
}

pub struct WallpaperViewModel {
    monitors: Vec<Monitor>,
    selected_monitor: Option<String>,
    selected_wallpaper: Option<PathBuf>,
    monitor_wallpapers: HashMap<String, PathBuf>,
    monitor_layout: Controller<MonitorLayoutModel>,
    wallpaper_picker: Controller<WallpaperPickerModel>,
    split_dialog: Option<Controller<SplitDialogModel>>,
    backend: Box<dyn WallpaperBackend>,
}

#[relm4::component(pub)]
impl SimpleComponent for WallpaperViewModel {
    type Init = ();
    type Input = WallpaperViewMsg;
    type Output = WallpaperViewOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 0,

            gtk::Paned {
                set_orientation: gtk::Orientation::Vertical,
                set_shrink_start_child: false,
                set_shrink_end_child: false,
                set_position: 350,
                set_vexpand: true,

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
                                set_icon_name: "folder-open-symbolic",
                                set_tooltip_text: Some("Open wallpaper folder"),
                                connect_clicked => WallpaperViewMsg::OpenDirectory,
                            },

                            gtk::Button {
                                set_icon_name: "insert-image-symbolic",
                                set_tooltip_text: Some("Import panoramic image"),
                                connect_clicked => WallpaperViewMsg::ShowSplitDialog,
                            },

                            gtk::Button {
                                set_label: "Apply",
                                #[watch]
                                set_sensitive: model.selected_monitor.is_some() && model.selected_wallpaper.is_some(),
                                add_css_class: "suggested-action",
                                connect_clicked => WallpaperViewMsg::ApplyWallpaper,
                            },
                        },

                        model.wallpaper_picker.widget() {},
                    },
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Detect wallpaper backend
        let backend = detect_backend()
            .unwrap_or_else(|_| {
                eprintln!("Warning: No wallpaper backend detected. Using dummy backend.");
                Box::new(DummyBackend) as Box<dyn WallpaperBackend>
            });

        // Load monitors
        let monitors = hyprctl::get_monitors().unwrap_or_else(|e| {
            eprintln!("Failed to load monitors: {}", e);
            Vec::new()
        });

        // Load current wallpapers
        let monitor_wallpapers: HashMap<String, PathBuf> = backend
            .query_active()
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| (k, PathBuf::from(v)))
            .collect();

        // Create MonitorLayoutModel
        let monitor_layout = MonitorLayoutModel::builder()
            .launch(monitors.clone())
            .forward(sender.input_sender(), |msg| {
                match msg {
                    MonitorLayoutOutput::Selected(name) => WallpaperViewMsg::MonitorSelected(name),
                }
            });

        // Update monitor layout with wallpapers
        monitor_layout.emit(MonitorLayoutInput::UpdateWallpapers(monitor_wallpapers.clone()));

        // Create WallpaperPickerModel
        let wallpaper_dir = thumbnail::default_wallpaper_dir();
        let wallpaper_picker = WallpaperPickerModel::builder()
            .launch(wallpaper_dir)
            .forward(sender.input_sender(), |msg| {
                match msg {
                    WallpaperPickerOutput::Selected(path) => WallpaperViewMsg::WallpaperSelected(path),
                }
            });

        let model = WallpaperViewModel {
            monitors,
            selected_monitor: None,
            selected_wallpaper: None,
            monitor_wallpapers,
            monitor_layout,
            wallpaper_picker,
            split_dialog: None,
            backend,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            WallpaperViewMsg::MonitorSelected(name) => {
                self.selected_monitor = Some(name);
            }

            WallpaperViewMsg::WallpaperSelected(path) => {
                self.selected_wallpaper = Some(path);
            }

            WallpaperViewMsg::ApplyWallpaper => {
                if let (Some(ref monitor), Some(ref wallpaper)) =
                    (&self.selected_monitor, &self.selected_wallpaper)
                {
                    match self.backend.apply(monitor, wallpaper) {
                        Ok(_) => {
                            // Update internal state
                            self.monitor_wallpapers.insert(monitor.clone(), wallpaper.clone());

                            // Update monitor layout visualization
                            self.monitor_layout.emit(
                                MonitorLayoutInput::UpdateWallpapers(self.monitor_wallpapers.clone())
                            );

                            // Notify parent
                            let _ = sender.output(WallpaperViewOutput::ShowToast(
                                format!("Applied wallpaper to {}", monitor)
                            ));
                            let _ = sender.output(WallpaperViewOutput::WallpapersChanged(
                                self.monitor_wallpapers.clone()
                            ));
                        }
                        Err(e) => {
                            let _ = sender.output(WallpaperViewOutput::ShowToast(
                                format!("Failed to apply wallpaper: {}", e)
                            ));
                        }
                    }
                }
            }

            WallpaperViewMsg::RefreshMonitors => {
                // Reload monitors
                if let Ok(monitors) = hyprctl::get_monitors() {
                    self.monitors = monitors.clone();
                    self.monitor_layout.emit(MonitorLayoutInput::UpdateMonitors(monitors));
                }

                // Reload wallpapers
                if let Ok(wallpapers) = self.backend.query_active() {
                    self.monitor_wallpapers = wallpapers
                        .into_iter()
                        .map(|(k, v)| (k, PathBuf::from(v)))
                        .collect();
                    self.monitor_layout.emit(
                        MonitorLayoutInput::UpdateWallpapers(self.monitor_wallpapers.clone())
                    );
                }

                // Reload wallpaper picker
                self.wallpaper_picker.emit(WallpaperPickerInput::Refresh);
            }

            WallpaperViewMsg::OpenDirectory => {
                let wallpaper_dir = thumbnail::default_wallpaper_dir();
                if let Err(e) = std::process::Command::new("xdg-open")
                    .arg(&wallpaper_dir)
                    .spawn()
                {
                    let _ = sender.output(WallpaperViewOutput::ShowToast(
                        format!("Failed to open folder: {}", e)
                    ));
                }
            }

            WallpaperViewMsg::ShowSplitDialog => {
                // Create modal dialog window
                let window = gtk::Window::builder()
                    .title("Import Panoramic Wallpaper")
                    .modal(true)
                    .resizable(false)
                    .build();

                // Create split dialog controller
                let split_dialog = SplitDialogModel::builder()
                    .launch(self.monitors.clone())
                    .forward(sender.input_sender(), |msg| {
                        match msg {
                            SplitDialogOutput::Generated(wallpapers) =>
                                WallpaperViewMsg::SplitGenerated(wallpapers),
                            SplitDialogOutput::Cancelled =>
                                WallpaperViewMsg::SplitCancelled,
                            SplitDialogOutput::Error(e) =>
                                WallpaperViewMsg::SplitError(e),
                        }
                    });

                window.set_child(Some(split_dialog.widget()));
                window.present();

                self.split_dialog = Some(split_dialog);
            }

            WallpaperViewMsg::SplitGenerated(wallpapers) => {
                // Apply all generated wallpapers
                for (monitor, path) in &wallpapers {
                    if let Err(e) = self.backend.apply(monitor, path) {
                        let _ = sender.output(WallpaperViewOutput::ShowToast(
                            format!("Failed to apply wallpaper to {}: {}", monitor, e)
                        ));
                    }
                }

                // Update internal state
                self.monitor_wallpapers.extend(wallpapers.clone());
                self.monitor_layout.emit(
                    MonitorLayoutInput::UpdateWallpapers(self.monitor_wallpapers.clone())
                );

                // Refresh picker to show new wallpapers
                self.wallpaper_picker.emit(WallpaperPickerInput::Refresh);

                // Close dialog
                self.split_dialog = None;

                // Notify parent
                let _ = sender.output(WallpaperViewOutput::ShowToast(
                    format!("Applied wallpapers to {} monitors", wallpapers.len())
                ));
                let _ = sender.output(WallpaperViewOutput::WallpapersChanged(
                    self.monitor_wallpapers.clone()
                ));
            }

            WallpaperViewMsg::SplitCancelled => {
                // Close dialog
                self.split_dialog = None;
            }

            WallpaperViewMsg::SplitError(error) => {
                // Close dialog and show error
                self.split_dialog = None;
                let _ = sender.output(WallpaperViewOutput::ShowToast(
                    format!("Split failed: {}", error)
                ));
            }

            WallpaperViewMsg::ApplyProfile(wallpapers) => {
                // Apply wallpapers from profile
                let mut success_count = 0;
                for (monitor, path) in &wallpapers {
                    if let Err(e) = self.backend.apply(monitor, path) {
                        let _ = sender.output(WallpaperViewOutput::ShowToast(
                            format!("Failed to apply wallpaper to {}: {}", monitor, e)
                        ));
                    } else {
                        success_count += 1;
                    }
                }

                // Update internal state
                self.monitor_wallpapers.extend(wallpapers.clone());
                self.monitor_layout.emit(
                    MonitorLayoutInput::UpdateWallpapers(self.monitor_wallpapers.clone())
                );

                // Notify parent
                if success_count > 0 {
                    let _ = sender.output(WallpaperViewOutput::ShowToast(
                        format!("Applied profile to {} monitors", success_count)
                    ));
                    let _ = sender.output(WallpaperViewOutput::WallpapersChanged(
                        self.monitor_wallpapers.clone()
                    ));
                }
            }

            WallpaperViewMsg::SetWallpaperForAll(wallpaper_path) => {
                // Apply the same wallpaper to all monitors
                let mut success_count = 0;
                for monitor in &self.monitors {
                    if let Err(e) = self.backend.apply(&monitor.name, &wallpaper_path) {
                        let _ = sender.output(WallpaperViewOutput::ShowToast(
                            format!("Failed to apply wallpaper to {}: {}", monitor.name, e)
                        ));
                    } else {
                        self.monitor_wallpapers.insert(monitor.name.clone(), wallpaper_path.clone());
                        success_count += 1;
                    }
                }

                // Update monitor layout visualization
                self.monitor_layout.emit(
                    MonitorLayoutInput::UpdateWallpapers(self.monitor_wallpapers.clone())
                );

                // Notify parent
                if success_count > 0 {
                    let _ = sender.output(WallpaperViewOutput::ShowToast(
                        format!("Applied wallpaper to {} monitors", success_count)
                    ));
                    let _ = sender.output(WallpaperViewOutput::WallpapersChanged(
                        self.monitor_wallpapers.clone()
                    ));
                }
            }
        }
    }
}

/// Dummy backend for when no real backend is available (graceful degradation)
struct DummyBackend;

impl WallpaperBackend for DummyBackend {
    fn apply(&self, _monitor: &str, _path: &std::path::Path) -> anyhow::Result<()> {
        anyhow::bail!("No wallpaper backend available")
    }

    fn query_active(&self) -> anyhow::Result<HashMap<String, String>> {
        Ok(HashMap::new())
    }

    fn name(&self) -> &str {
        "dummy"
    }
}
