use gtk::prelude::*;
use relm4::prelude::*;
use adw::prelude::*;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::components::profile_manager::{ProfileManagerModel, ProfileManagerInput, ProfileManagerOutput};

#[derive(Debug)]
pub enum AppMsg {
    Refresh,
    ProfileApply(HashMap<String, PathBuf>),
    ProfileSaved(String),
    ProfileError(String),
    ShowToast(String),
}

pub struct App {
    view_stack: adw::ViewStack,
    profile_manager: Controller<ProfileManagerModel>,
    toast_overlay: adw::ToastOverlay,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("VulcanOS Appearance Manager"),
            set_default_size: (1000, 700),

            #[local_ref]
            toast_overlay -> adw::ToastOverlay {
                adw::ToolbarView {
                    add_top_bar = &adw::HeaderBar {
                        #[wrap(Some)]
                        set_title_widget = &adw::ViewSwitcher {
                            #[watch]
                            set_stack: Some(&model.view_stack),
                            set_policy: adw::ViewSwitcherPolicy::Wide,
                        },

                        // Profile manager shared across both tabs
                        pack_start = model.profile_manager.widget() {},

                        // Refresh button
                        pack_end = &gtk::Button {
                            set_icon_name: "view-refresh-symbolic",
                            set_tooltip_text: Some("Refresh"),
                            connect_clicked => AppMsg::Refresh,
                        },
                    },

                    #[wrap(Some)]
                    set_content = &model.view_stack.clone(),
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Create ViewStack for tab navigation
        let view_stack = adw::ViewStack::new();

        // Placeholder for Themes (will be replaced in Plan 3)
        let themes_placeholder = gtk::Label::builder()
            .label("Themes View (loading...)")
            .build();
        view_stack.add_titled_with_icon(
            &themes_placeholder,
            Some("themes"),
            "Themes",
            "preferences-color-symbolic"
        );

        // Placeholder for Wallpapers (will be replaced in Plan 4)
        let wallpapers_placeholder = gtk::Label::builder()
            .label("Wallpapers View (loading...)")
            .build();
        view_stack.add_titled_with_icon(
            &wallpapers_placeholder,
            Some("wallpapers"),
            "Wallpapers",
            "preferences-desktop-wallpaper-symbolic"
        );

        // Create profile manager component (shared across both tabs)
        let profile_manager = ProfileManagerModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| {
                match msg {
                    ProfileManagerOutput::ApplyProfile(wallpapers) => AppMsg::ProfileApply(wallpapers),
                    ProfileManagerOutput::ProfileSaved(name) => AppMsg::ProfileSaved(name),
                    ProfileManagerOutput::Error(e) => AppMsg::ProfileError(e),
                }
            });

        let toast_overlay = adw::ToastOverlay::new();

        let model = App {
            view_stack,
            profile_manager,
            toast_overlay: toast_overlay.clone(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Refresh => {
                // Refresh current view (will be forwarded to active view in later plans)
                println!("Refresh requested");
            }

            AppMsg::ProfileApply(wallpapers) => {
                // Apply profile (will be forwarded to wallpaper view in Plan 4)
                println!("Applying profile with {} wallpapers", wallpapers.len());
            }

            AppMsg::ProfileSaved(name) => {
                let toast = adw::Toast::new(&format!("Profile '{}' saved", name));
                self.toast_overlay.add_toast(toast);
            }

            AppMsg::ProfileError(error) => {
                let toast = adw::Toast::new(&format!("Error: {}", error));
                self.toast_overlay.add_toast(toast);
            }

            AppMsg::ShowToast(message) => {
                let toast = adw::Toast::new(&message);
                self.toast_overlay.add_toast(toast);
            }
        }
    }
}
