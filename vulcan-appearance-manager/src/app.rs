use gtk::prelude::*;
use relm4::prelude::*;
use adw::prelude::*;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::components::profile_manager::{ProfileManagerModel, ProfileManagerInput, ProfileManagerOutput};
use crate::components::theme_view::{ThemeViewModel, ThemeViewMsg, ThemeViewOutput};
use crate::components::wallpaper_view::{WallpaperViewModel, WallpaperViewMsg, WallpaperViewOutput};

#[derive(Debug)]
pub enum AppMsg {
    Refresh,
    ProfileApply(HashMap<String, PathBuf>),
    ProfileSaved(String),
    ProfileError(String),
    ShowToast(String),
    ThemeApplied(String),
    WallpapersChanged(HashMap<String, PathBuf>),
}

pub struct App {
    view_stack: adw::ViewStack,
    theme_view: Controller<ThemeViewModel>,
    wallpaper_view: Controller<WallpaperViewModel>,
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

        // Create theme view component
        let theme_view = ThemeViewModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| {
                match msg {
                    ThemeViewOutput::ShowToast(text) => AppMsg::ShowToast(text),
                    ThemeViewOutput::ThemeApplied(id) => AppMsg::ThemeApplied(id),
                }
            });

        // Add Themes tab to ViewStack
        view_stack.add_titled_with_icon(
            theme_view.widget(),
            Some("themes"),
            "Themes",
            "preferences-color-symbolic"
        );

        // Create wallpaper view component
        let wallpaper_view = WallpaperViewModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| {
                match msg {
                    WallpaperViewOutput::ShowToast(text) => AppMsg::ShowToast(text),
                    WallpaperViewOutput::WallpapersChanged(wps) => AppMsg::WallpapersChanged(wps),
                }
            });

        // Add Wallpapers tab to ViewStack
        view_stack.add_titled_with_icon(
            wallpaper_view.widget(),
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
            theme_view,
            wallpaper_view,
            profile_manager,
            toast_overlay: toast_overlay.clone(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Refresh => {
                // Forward refresh to wallpaper view
                self.wallpaper_view.emit(WallpaperViewMsg::RefreshMonitors);
            }

            AppMsg::ProfileApply(wallpapers) => {
                // Forward profile application to wallpaper view
                self.wallpaper_view.emit(WallpaperViewMsg::ApplyProfile(wallpapers));
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

            AppMsg::ThemeApplied(theme_id) => {
                let toast = adw::Toast::new(&format!("Applied theme: {}", theme_id));
                self.toast_overlay.add_toast(toast);
            }

            AppMsg::WallpapersChanged(wallpapers) => {
                // Notify profile manager of wallpaper changes
                self.profile_manager.emit(ProfileManagerInput::UpdateWallpapers(wallpapers));
            }
        }
    }
}
