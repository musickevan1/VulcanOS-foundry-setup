use gtk::prelude::*;
use relm4::prelude::*;
use adw::prelude::*;

use crate::models::{Theme, resolve_theme_wallpaper};
use crate::services::{theme_applier, theme_storage};
use crate::state::{AppState, PreviewSnapshot};
use super::theme_browser::{ThemeBrowserModel, ThemeBrowserInput, ThemeBrowserOutput};
use super::preview_panel::{PreviewPanelModel, PreviewPanelInput};
use super::theme_editor::{ThemeEditorModel, ThemeEditorOutput};
use super::binding_dialog::{BindingDialogModel, BindingDialogOutput, BindingDialogInit};
use super::discovery_section::DiscoverySection;

#[derive(Debug)]
pub enum ThemeViewMsg {
    // Theme selection
    ThemeSelected(Theme),
    // Actions
    PreviewTheme,
    ApplyTheme,
    ApplyThemeById(String),  // Apply theme by ID (for profile loading)
    CancelPreview,
    NewTheme,
    EditTheme,
    // Editor callbacks
    ThemeSaved(Theme),
    EditorCancelled,
    // Binding dialog
    BindingChoice(BindingDialogOutput),
    // General
    Refresh,
    Import,
}

#[derive(Debug)]
pub enum ThemeViewOutput {
    ShowToast(String),
    ThemeApplied(String), // theme_id
    ApplyWallpaper(std::path::PathBuf),  // Request wallpaper application
    BindingModeChanged(crate::models::BindingMode),  // Notify binding state change
}

pub struct ThemeViewModel {
    theme_browser: Controller<ThemeBrowserModel>,
    preview_panel: Controller<PreviewPanelModel>,
    discovery: Controller<DiscoverySection>,
    editor_dialog: Option<Controller<ThemeEditorModel>>,
    editor_window: Option<gtk::Window>,
    binding_dialog: Option<Controller<BindingDialogModel>>,
    binding_window: Option<gtk::Window>,
    selected_theme: Option<Theme>,
    original_theme_id: String,
    app_state: AppState,
    preview_snapshot: Option<PreviewSnapshot>,
    previewing_theme_id: Option<String>,
}

#[relm4::component(pub)]
impl SimpleComponent for ThemeViewModel {
    type Init = ();
    type Input = ThemeViewMsg;
    type Output = ThemeViewOutput;

    view! {
        gtk::Paned {
            set_orientation: gtk::Orientation::Horizontal,
            set_shrink_start_child: false,
            set_shrink_end_child: false,
            set_position: 550,

            // Left: Theme browser
            #[wrap(Some)]
            set_start_child = &gtk::Frame {
                set_margin_all: 12,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 8,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 8,

                        gtk::Label {
                            set_markup: "<b>Available Themes</b>",
                            set_halign: gtk::Align::Start,
                            set_hexpand: true,
                        },

                        gtk::Button {
                            set_icon_name: "list-add-symbolic",
                            set_tooltip_text: Some("Create new theme"),
                            connect_clicked => ThemeViewMsg::NewTheme,
                        },

                        gtk::Button {
                            set_icon_name: "document-open-symbolic",
                            set_tooltip_text: Some("Import theme file"),
                            connect_clicked => ThemeViewMsg::Import,
                        },
                    },

                    model.theme_browser.widget() {},

                    // Discovery section in an expander
                    gtk::Expander {
                        set_label: Some("Third-Party Apps"),
                        set_margin_top: 12,

                        model.discovery.widget() {},
                    },
                },
            },

            // Right: Preview + actions
            #[wrap(Some)]
            set_end_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,
                set_margin_all: 12,

                model.preview_panel.widget() {},

                // Action buttons
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,
                    set_halign: gtk::Align::End,

                    gtk::Button {
                        set_label: "Edit",
                        set_tooltip_text: Some("Edit selected theme"),
                        #[watch]
                        set_sensitive: model.selected_theme.as_ref().map(|t| !t.is_builtin).unwrap_or(false),
                        connect_clicked => ThemeViewMsg::EditTheme,
                    },

                    gtk::Button {
                        set_label: "Preview",
                        set_tooltip_text: Some("Preview theme (temporary)"),
                        #[watch]
                        set_sensitive: model.selected_theme.is_some(),
                        connect_clicked => ThemeViewMsg::PreviewTheme,
                    },

                    gtk::Button {
                        set_label: "Cancel",
                        set_tooltip_text: Some("Revert to original theme"),
                        connect_clicked => ThemeViewMsg::CancelPreview,
                    },

                    gtk::Button {
                        set_label: "Apply",
                        add_css_class: "suggested-action",
                        set_tooltip_text: Some("Apply theme permanently"),
                        #[watch]
                        set_sensitive: model.selected_theme.is_some(),
                        connect_clicked => ThemeViewMsg::ApplyTheme,
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
        // Get current theme
        let original_theme_id = theme_applier::get_current_theme()
            .unwrap_or_else(|_| "tokyonight".to_string());

        // Create theme browser
        let theme_browser = ThemeBrowserModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| {
                match msg {
                    ThemeBrowserOutput::ThemeSelected(theme) => ThemeViewMsg::ThemeSelected(theme),
                }
            });

        // Create preview panel
        let preview_panel = PreviewPanelModel::builder()
            .launch(())
            .detach();

        // Create discovery section
        let discovery = DiscoverySection::builder()
            .launch(())
            .detach();

        let model = ThemeViewModel {
            theme_browser,
            preview_panel,
            discovery,
            editor_dialog: None,
            editor_window: None,
            binding_dialog: None,
            binding_window: None,
            selected_theme: None,
            original_theme_id,
            app_state: AppState::Idle,
            preview_snapshot: None,
            previewing_theme_id: None,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            ThemeViewMsg::ThemeSelected(theme) => {
                println!("Selected theme: {}", theme.theme_name);
                self.preview_panel.emit(PreviewPanelInput::SetTheme(Some(theme.clone())));
                self.selected_theme = Some(theme.clone());

                // STATE MACHINE: Handle preview based on current state
                if self.app_state.is_idle() {
                    // First click: Idle -> Previewing
                    let snapshot = self.create_preview_snapshot();
                    match self.app_state.clone().start_preview(snapshot.clone()) {
                        Ok(new_state) => {
                            self.app_state = new_state;
                            self.preview_snapshot = Some(snapshot);
                            self.previewing_theme_id = Some(theme.theme_id.clone());

                            // Apply preview immediately
                            if let Err(e) = theme_applier::preview_theme(&theme.theme_id) {
                                eprintln!("Preview failed: {}", e);
                                sender.output(ThemeViewOutput::ShowToast(format!("Preview failed: {}", e))).ok();
                                // Revert state on failure
                                self.app_state = AppState::Idle;
                                self.preview_snapshot = None;
                                self.previewing_theme_id = None;
                            } else {
                                sender.output(ThemeViewOutput::ShowToast(format!("Previewing: {}", theme.theme_name))).ok();
                            }
                        }
                        Err(e) => {
                            eprintln!("Invalid state transition: {}", e);
                        }
                    }
                } else if self.app_state.is_previewing() {
                    // Subsequent click: Switch preview, keep ORIGINAL snapshot
                    self.previewing_theme_id = Some(theme.theme_id.clone());
                    if let Err(e) = theme_applier::preview_theme(&theme.theme_id) {
                        eprintln!("Preview switch failed: {}", e);
                        sender.output(ThemeViewOutput::ShowToast(format!("Preview failed: {}", e))).ok();
                    } else {
                        sender.output(ThemeViewOutput::ShowToast(format!("Previewing: {}", theme.theme_name))).ok();
                    }
                }
                // If in Applying or Error state, ignore click (handled by button sensitivity)
            }

            ThemeViewMsg::PreviewTheme => {
                if let Some(ref theme) = self.selected_theme {
                    if let Err(e) = theme_applier::preview_theme(&theme.theme_id) {
                        eprintln!("Preview failed: {}", e);
                        sender.output(ThemeViewOutput::ShowToast(format!("Preview failed: {}", e))).ok();
                    } else {
                        sender.output(ThemeViewOutput::ShowToast(format!("Previewing: {}", theme.theme_name))).ok();
                    }
                }
            }

            ThemeViewMsg::ApplyTheme => {
                if let Some(ref theme) = self.selected_theme {
                    // Check if theme has a suggested wallpaper
                    if let Some(wallpaper_path) = resolve_theme_wallpaper(theme) {
                        // Show binding dialog
                        self.show_binding_dialog(theme.clone(), wallpaper_path, sender.clone());
                    } else {
                        // No wallpaper - apply theme only
                        let theme_id = theme.theme_id.clone();
                        self.apply_theme_only(&theme_id, sender.clone());
                    }
                }
            }

            ThemeViewMsg::CancelPreview => {
                if let Err(e) = theme_applier::revert_theme() {
                    eprintln!("Revert failed: {}", e);
                    sender.output(ThemeViewOutput::ShowToast(format!("Revert failed: {}", e))).ok();
                } else {
                    sender.output(ThemeViewOutput::ShowToast("Reverted to original theme".to_string())).ok();
                }
            }

            ThemeViewMsg::NewTheme => {
                self.open_editor(None, true, sender.clone());
            }

            ThemeViewMsg::EditTheme => {
                if let Some(ref theme) = self.selected_theme {
                    if !theme.is_builtin {
                        self.open_editor(Some(theme.clone()), false, sender.clone());
                    }
                }
            }

            ThemeViewMsg::ThemeSaved(theme) => {
                // Save the theme
                match theme_storage::save_theme(&theme) {
                    Ok(_) => {
                        sender.output(ThemeViewOutput::ShowToast(format!("Saved: {}", theme.theme_name))).ok();
                        // Refresh the browser
                        self.theme_browser.emit(ThemeBrowserInput::Refresh);
                    }
                    Err(e) => {
                        sender.output(ThemeViewOutput::ShowToast(format!("Save failed: {}", e))).ok();
                    }
                }

                // Close editor window
                if let Some(window) = self.editor_window.take() {
                    window.close();
                }
                self.editor_dialog = None;
            }

            ThemeViewMsg::EditorCancelled => {
                if let Some(window) = self.editor_window.take() {
                    window.close();
                }
                self.editor_dialog = None;
            }

            ThemeViewMsg::Refresh => {
                self.theme_browser.emit(ThemeBrowserInput::Refresh);
                sender.output(ThemeViewOutput::ShowToast("Themes refreshed".to_string())).ok();
            }

            ThemeViewMsg::Import => {
                self.import_theme_dialog(sender.clone());
            }

            ThemeViewMsg::ApplyThemeById(theme_id) => {
                // Apply theme by ID (used when loading profiles)
                if let Err(e) = theme_applier::apply_theme(&theme_id) {
                    eprintln!("Apply failed: {}", e);
                    sender.output(ThemeViewOutput::ShowToast(format!("Apply failed: {}", e))).ok();
                } else {
                    self.original_theme_id = theme_id.clone();
                    self.theme_browser.emit(ThemeBrowserInput::SetCurrentTheme(theme_id.clone()));
                    sender.output(ThemeViewOutput::ThemeApplied(theme_id)).ok();
                }
            }

            ThemeViewMsg::BindingChoice(choice) => {
                // Close binding dialog
                if let Some(window) = self.binding_window.take() {
                    window.close();
                }
                self.binding_dialog = None;

                match choice {
                    BindingDialogOutput::ApplyThemeOnly => {
                        // Apply theme only, keep current wallpaper
                        if let Some(ref theme) = self.selected_theme {
                            let theme_id = theme.theme_id.clone();
                            self.apply_theme_only(&theme_id, sender.clone());
                            sender.output(ThemeViewOutput::BindingModeChanged(
                                crate::models::BindingMode::Unbound
                            )).ok();
                        }
                    }
                    BindingDialogOutput::ApplyBoth(wallpaper_path) => {
                        // Apply theme AND wallpaper
                        if let Some(ref theme) = self.selected_theme {
                            let theme_id = theme.theme_id.clone();
                            self.apply_theme_only(&theme_id, sender.clone());
                            sender.output(ThemeViewOutput::ApplyWallpaper(wallpaper_path)).ok();
                            sender.output(ThemeViewOutput::BindingModeChanged(
                                crate::models::BindingMode::ThemeBound
                            )).ok();
                        }
                    }
                    BindingDialogOutput::Cancelled => {
                        // User cancelled - do nothing
                    }
                }
            }
        }
    }
}

impl ThemeViewModel {
    fn create_preview_snapshot(&self) -> PreviewSnapshot {
        // Query current wallpapers from system
        // Use wallpaper_backend::detect_backend() and query_active()
        use crate::services::wallpaper_backend::detect_backend;
        use std::path::PathBuf;

        let wallpapers = detect_backend()
            .and_then(|backend| backend.query_active())
            .map(|wps| {
                wps.into_iter()
                    .map(|(k, v)| (k, PathBuf::from(v)))
                    .collect()
            })
            .unwrap_or_default();

        PreviewSnapshot {
            wallpapers,
            theme_id: Some(self.original_theme_id.clone()),
        }
    }

    fn open_editor(&mut self, theme: Option<Theme>, is_new: bool, sender: ComponentSender<Self>) {
        let editor = ThemeEditorModel::builder()
            .launch((theme, is_new))
            .forward(sender.input_sender(), |msg| {
                match msg {
                    ThemeEditorOutput::Saved(theme) => ThemeViewMsg::ThemeSaved(theme),
                    ThemeEditorOutput::Cancelled => ThemeViewMsg::EditorCancelled,
                }
            });

        let title = if is_new { "New Theme" } else { "Edit Theme" };

        let window = gtk::Window::builder()
            .title(title)
            .modal(true)
            .default_width(550)
            .default_height(600)
            .child(editor.widget())
            .build();

        window.present();

        self.editor_dialog = Some(editor);
        self.editor_window = Some(window);
    }

    fn import_theme_dialog(&self, sender: ComponentSender<Self>) {
        let dialog = gtk::FileDialog::builder()
            .title("Import Theme")
            .accept_label("Import")
            .build();

        // Set up filter for .sh files
        let filter = gtk::FileFilter::new();
        filter.add_pattern("*.sh");
        filter.set_name(Some("Theme files (*.sh)"));

        let filters = gtk::gio::ListStore::new::<gtk::FileFilter>();
        filters.append(&filter);
        dialog.set_filters(Some(&filters));

        dialog.open(None::<&gtk::Window>, None::<&gtk::gio::Cancellable>, move |result| {
            if let Ok(file) = result {
                if let Some(path) = file.path() {
                    match theme_storage::import_theme(&path) {
                        Ok(theme) => {
                            sender.input(ThemeViewMsg::Refresh);
                            sender.output(ThemeViewOutput::ShowToast(format!("Imported: {}", theme.theme_name))).ok();
                        }
                        Err(e) => {
                            sender.output(ThemeViewOutput::ShowToast(format!("Import failed: {}", e))).ok();
                        }
                    }
                }
            }
        });
    }

    fn show_binding_dialog(&mut self, theme: Theme, wallpaper_path: std::path::PathBuf, sender: ComponentSender<Self>) {
        let (connector, window) = BindingDialogModel::show_dialog(
            None::<&gtk::Window>,
            theme,
            wallpaper_path,
        );

        // Forward binding dialog output to our input
        let controller = connector.forward(sender.input_sender(), |msg| {
            ThemeViewMsg::BindingChoice(msg)
        });

        self.binding_dialog = Some(controller);
        self.binding_window = Some(window);
    }

    fn apply_theme_only(&mut self, theme_id: &str, sender: ComponentSender<Self>) {
        if let Err(e) = theme_applier::apply_theme(theme_id) {
            eprintln!("Apply failed: {}", e);
            sender.output(ThemeViewOutput::ShowToast(format!("Apply failed: {}", e))).ok();
        } else {
            self.original_theme_id = theme_id.to_string();
            self.theme_browser.emit(ThemeBrowserInput::SetCurrentTheme(theme_id.to_string()));
            sender.output(ThemeViewOutput::ShowToast(format!("Applied theme: {}", theme_id))).ok();
            sender.output(ThemeViewOutput::ThemeApplied(theme_id.to_string())).ok();
        }
    }
}
