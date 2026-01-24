use gtk::prelude::*;
use relm4::prelude::*;
use adw::prelude::*;

use crate::models::Monitor;
use crate::services::hyprctl;
use crate::components::monitor_layout::MonitorLayoutModel;

#[derive(Debug)]
pub enum AppMsg {
    MonitorSelected(String),
    RefreshMonitors,
}

pub struct App {
    monitors: Vec<Monitor>,
    selected_monitor: Option<String>,
    monitor_layout: Controller<MonitorLayoutModel>,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("VulcanOS Wallpaper Manager"),
            set_default_size: (900, 600),

            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "Wallpaper Manager",
                        set_subtitle: "Configure per-monitor wallpapers",
                    },
                },

                #[wrap(Some)]
                set_content = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 12,
                    set_margin_all: 12,

                    // Monitor layout visualization
                    gtk::Frame {
                        set_label: Some("Monitor Layout"),
                        set_vexpand: true,

                        model.monitor_layout.widget(),
                    },

                    // Bottom toolbar with actions
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 12,
                        set_halign: gtk::Align::End,

                        gtk::Button {
                            set_label: "Refresh",
                            set_icon_name: "view-refresh-symbolic",
                            connect_clicked => AppMsg::RefreshMonitors,
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

        // Create monitor layout component
        let monitor_layout = MonitorLayoutModel::builder()
            .launch(monitors.clone())
            .forward(sender.input_sender(), |msg| {
                match msg {
                    crate::components::monitor_layout::MonitorLayoutOutput::Selected(name) => {
                        AppMsg::MonitorSelected(name)
                    }
                }
            });

        let model = App {
            monitors,
            selected_monitor: None,
            monitor_layout,
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
            AppMsg::RefreshMonitors => {
                if let Ok(monitors) = hyprctl::get_monitors() {
                    self.monitors = monitors.clone();
                    self.monitor_layout.emit(
                        crate::components::monitor_layout::MonitorLayoutInput::UpdateMonitors(monitors)
                    );
                }
            }
        }
    }
}
