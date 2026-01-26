use relm4::prelude::*;
use adw::prelude::*;
use gtk::prelude::*;
use gtk::{CssProvider, gdk::Display};
use std::path::PathBuf;
use std::collections::HashMap;

use crate::components::profile_manager::{ProfileManagerModel, ProfileManagerInput, ProfileManagerOutput};
use crate::components::theme_view::{ThemeViewModel, ThemeViewMsg, ThemeViewOutput};
use crate::components::wallpaper_view::{WallpaperViewModel, WallpaperViewMsg, WallpaperViewOutput};
use crate::components::profile_view::{ProfileViewModel, ProfileViewMsg, ProfileViewOutput};
use crate::models::{UnifiedProfile, BindingMode};
use crate::services::theme_css;

/// Load theme CSS at runtime via CssProvider
///
/// This supplements brand_css.rs which provides defaults.
/// Theme CSS overrides brand colors with active theme colors.
fn load_theme_css() {
    if let Some(css_content) = theme_css::get_theme_css() {
        let provider = CssProvider::new();
        provider.load_from_string(&css_content);

        if let Some(display) = Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_USER, // Higher priority than APPLICATION
            );
        }
    }
}

#[derive(Debug)]
pub enum AppMsg {
    Refresh,
    ProfileApply(HashMap<String, PathBuf>),
    ProfileSaved(String),
    ProfileError(String),
    ShowToast(String),
    ThemeApplied(String),
    WallpapersChanged(HashMap<String, PathBuf>),
    ProfileLoad(UnifiedProfile),
    ProfileDeleted(String),
    BindingModeChanged(BindingMode),
    ApplyThemeWallpaper(PathBuf),
}

pub struct App {
    view_stack: adw::ViewStack,
    theme_view: Controller<ThemeViewModel>,
    wallpaper_view: Controller<WallpaperViewModel>,
    profile_view: Controller<ProfileViewModel>,
    profile_manager: Controller<ProfileManagerModel>,
    toast_overlay: adw::ToastOverlay,
    // Current appearance state for profile saving
    current_binding_mode: BindingMode,
    current_theme_id: Option<String>,
    current_wallpapers: HashMap<String, PathBuf>,
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

        // Add keyboard shortcuts for tab navigation
        let shortcut_controller = gtk::ShortcutController::new();
        shortcut_controller.set_scope(gtk::ShortcutScope::Managed);

        // Clone view_stack for closures
        let view_stack_clone1 = view_stack.clone();
        let view_stack_clone2 = view_stack.clone();
        let view_stack_clone3 = view_stack.clone();

        // Ctrl+1 for Themes
        let themes_action = gtk::CallbackAction::new(move |_, _| {
            view_stack_clone1.set_visible_child_name("themes");
            gtk::glib::Propagation::Stop
        });
        let themes_shortcut = gtk::Shortcut::new(
            gtk::ShortcutTrigger::parse_string("<Control>1"),
            Some(themes_action),
        );

        // Ctrl+2 for Wallpapers
        let wallpapers_action = gtk::CallbackAction::new(move |_, _| {
            view_stack_clone2.set_visible_child_name("wallpapers");
            gtk::glib::Propagation::Stop
        });
        let wallpapers_shortcut = gtk::Shortcut::new(
            gtk::ShortcutTrigger::parse_string("<Control>2"),
            Some(wallpapers_action),
        );

        // Ctrl+3 for Profiles
        let profiles_action = gtk::CallbackAction::new(move |_, _| {
            view_stack_clone3.set_visible_child_name("profiles");
            gtk::glib::Propagation::Stop
        });
        let profiles_shortcut = gtk::Shortcut::new(
            gtk::ShortcutTrigger::parse_string("<Control>3"),
            Some(profiles_action),
        );

        shortcut_controller.add_shortcut(themes_shortcut);
        shortcut_controller.add_shortcut(wallpapers_shortcut);
        shortcut_controller.add_shortcut(profiles_shortcut);
        root.add_controller(shortcut_controller);

        // Create theme view component
        let theme_view = ThemeViewModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| {
                match msg {
                    ThemeViewOutput::ShowToast(text) => AppMsg::ShowToast(text),
                    ThemeViewOutput::ThemeApplied(id) => AppMsg::ThemeApplied(id),
                    ThemeViewOutput::ApplyWallpaper(path) => AppMsg::ApplyThemeWallpaper(path),
                    ThemeViewOutput::BindingModeChanged(mode) => AppMsg::BindingModeChanged(mode),
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

        // Create profile view component
        let profile_view = ProfileViewModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| {
                match msg {
                    ProfileViewOutput::ShowToast(text) => AppMsg::ShowToast(text),
                    ProfileViewOutput::LoadProfile(profile) => AppMsg::ProfileLoad(profile),
                    ProfileViewOutput::ProfileDeleted(name) => AppMsg::ProfileDeleted(name),
                }
            });

        // Add Profiles tab to ViewStack
        view_stack.add_titled_with_icon(
            profile_view.widget(),
            Some("profiles"),
            "Profiles",
            "user-bookmarks-symbolic"
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
            profile_view,
            profile_manager,
            toast_overlay: toast_overlay.clone(),
            current_binding_mode: BindingMode::Unbound,
            current_theme_id: None,
            current_wallpapers: HashMap::new(),
        };

        // Load theme CSS for self-theming (if a theme has been applied previously)
        load_theme_css();

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Refresh => {
                // Determine active view and refresh appropriately
                if self.view_stack.visible_child_name().as_deref() == Some("themes") {
                    self.theme_view.emit(ThemeViewMsg::Refresh);
                } else {
                    self.wallpaper_view.emit(WallpaperViewMsg::RefreshMonitors);
                }
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
                toast.set_timeout(3);
                self.toast_overlay.add_toast(toast);
            }

            AppMsg::ThemeApplied(theme_id) => {
                // Track current theme
                self.current_theme_id = Some(theme_id.clone());

                // Reload theme CSS for self-theming (vulcan-theme set generates new CSS)
                load_theme_css();

                // Sync state to profile view for saving
                self.profile_view.emit(ProfileViewMsg::UpdateCurrentState {
                    theme_id: self.current_theme_id.clone(),
                    wallpapers: self.current_wallpapers.clone(),
                    binding_mode: self.current_binding_mode.clone(),
                });

                let toast = adw::Toast::new(&format!("Applied theme: {}", theme_id));
                self.toast_overlay.add_toast(toast);
            }

            AppMsg::WallpapersChanged(wallpapers) => {
                // Track current wallpapers
                self.current_wallpapers = wallpapers.clone();

                // Notify profile manager of wallpaper changes
                self.profile_manager.emit(ProfileManagerInput::UpdateWallpapers(wallpapers.clone()));

                // Sync state to profile view for saving
                self.profile_view.emit(ProfileViewMsg::UpdateCurrentState {
                    theme_id: self.current_theme_id.clone(),
                    wallpapers: self.current_wallpapers.clone(),
                    binding_mode: self.current_binding_mode.clone(),
                });
            }

            AppMsg::ProfileLoad(profile) => {
                // Apply theme if present
                if let Some(ref theme_id) = profile.theme_id {
                    self.theme_view.emit(ThemeViewMsg::ApplyThemeById(theme_id.clone()));
                }

                // Apply wallpapers
                self.wallpaper_view.emit(WallpaperViewMsg::ApplyProfile(profile.monitor_wallpapers.clone()));

                // Update binding mode
                self.current_binding_mode = profile.binding_mode;

                let toast = adw::Toast::new(&format!("Loaded profile: {}", profile.name));
                self.toast_overlay.add_toast(toast);
            }

            AppMsg::ProfileDeleted(name) => {
                let toast = adw::Toast::new(&format!("Deleted profile: {}", name));
                self.toast_overlay.add_toast(toast);
            }

            AppMsg::BindingModeChanged(mode) => {
                self.current_binding_mode = mode.clone();

                // Sync state to profile view for saving
                self.profile_view.emit(ProfileViewMsg::UpdateCurrentState {
                    theme_id: self.current_theme_id.clone(),
                    wallpapers: self.current_wallpapers.clone(),
                    binding_mode: mode,
                });
            }

            AppMsg::ApplyThemeWallpaper(wallpaper_path) => {
                // Apply wallpaper to all monitors
                // Get current monitor list from wallpaper view
                // For now, apply to all monitors
                self.wallpaper_view.emit(WallpaperViewMsg::SetWallpaperForAll(wallpaper_path));
            }
        }
    }
}
