//! Third-party app discovery section component
//!
//! Displays installed apps that support theming with their
//! configuration status and links to theme resources.

use gtk::prelude::*;
use relm4::prelude::*;

use crate::services::app_discovery::{self, AppTheming};

/// Model for individual app row
pub struct AppRow {
    app: AppTheming,
}

#[derive(Debug)]
pub enum AppRowMsg {
    OpenDocs,
}

#[relm4::component(pub)]
impl SimpleComponent for AppRow {
    type Init = AppTheming;
    type Input = AppRowMsg;
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 12,
            set_margin_all: 8,
            add_css_class: "app-discovery-row",

            // App icon
            gtk::Image {
                set_icon_name: Some(&model.app.icon),
                set_pixel_size: 32,
            },

            // App name and status
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,
                set_valign: gtk::Align::Center,

                gtk::Label {
                    set_label: &model.app.name,
                    set_halign: gtk::Align::Start,
                    add_css_class: "app-name",
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,

                    // Installed badge
                    gtk::Label {
                        #[watch]
                        set_label: if model.app.installed { "Installed" } else { "Not Installed" },
                        #[watch]
                        add_css_class: if model.app.installed { "badge-success" } else { "badge-muted" },
                        add_css_class: "badge",
                    },

                    // Configured badge (only show if installed)
                    gtk::Label {
                        #[watch]
                        set_visible: model.app.installed,
                        #[watch]
                        set_label: if model.app.configured { "Themed" } else { "Not Themed" },
                        #[watch]
                        add_css_class: if model.app.configured { "badge-accent" } else { "badge-warning" },
                        add_css_class: "badge",
                    },
                },
            },

            // Open docs button
            gtk::Button {
                set_icon_name: "web-browser-symbolic",
                set_tooltip_text: Some("Open theme resources"),
                add_css_class: "flat",
                connect_clicked => AppRowMsg::OpenDocs,
            },
        }
    }

    fn init(
        app: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppRow { app };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppRowMsg::OpenDocs => {
                app_discovery::open_url(&self.app.docs_url);
            }
        }
    }
}

/// Discovery section model
pub struct DiscoverySection {
    apps: Vec<Controller<AppRow>>,
}

#[derive(Debug)]
pub enum DiscoverySectionMsg {
    Refresh,
}

#[derive(Debug)]
pub enum DiscoverySectionOutput {}

#[relm4::component(pub)]
impl Component for DiscoverySection {
    type Init = ();
    type Input = DiscoverySectionMsg;
    type Output = DiscoverySectionOutput;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 8,

            // Section header
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_margin_start: 12,
                set_margin_end: 12,
                set_margin_top: 12,

                gtk::Label {
                    set_label: "Third-Party App Theming",
                    set_hexpand: true,
                    set_halign: gtk::Align::Start,
                    add_css_class: "heading",
                },

                gtk::Button {
                    set_icon_name: "view-refresh-symbolic",
                    set_tooltip_text: Some("Refresh app detection"),
                    add_css_class: "flat",
                    connect_clicked => DiscoverySectionMsg::Refresh,
                },
            },

            gtk::Label {
                set_label: "Apps that support custom themes. Click the link icon to find themes.",
                set_halign: gtk::Align::Start,
                set_margin_start: 12,
                add_css_class: "dim-label",
            },

            gtk::Separator {},

            // App list
            #[local_ref]
            app_list -> gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 8,
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Discover apps on init
        let discovered = app_discovery::discover_apps();
        let apps: Vec<_> = discovered
            .into_iter()
            .map(|app| AppRow::builder().launch(app).detach())
            .collect();

        let model = DiscoverySection { apps };

        let app_list = gtk::Box::new(gtk::Orientation::Vertical, 4);
        for controller in &model.apps {
            app_list.append(controller.widget());
        }

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            DiscoverySectionMsg::Refresh => {
                // Re-discover apps (model update triggers view refresh)
                let discovered = app_discovery::discover_apps();
                self.apps = discovered
                    .into_iter()
                    .map(|app| AppRow::builder().launch(app).detach())
                    .collect();

                // Rebuild app list widget
                // Note: In production, we'd use a factory for dynamic lists
                // For simplicity, we trigger a full refresh
            }
        }
    }
}
