use relm4::prelude::*;
use adw::prelude::*;

use crate::models::Theme;
use crate::services::{theme_applier, theme_storage};
use crate::components::theme_browser::{ThemeBrowserModel, ThemeBrowserInput, ThemeBrowserOutput};
use crate::components::preview_panel::{PreviewPanelModel, PreviewPanelInput};
use crate::components::theme_editor::{ThemeEditorModel, ThemeEditorOutput};

#[derive(Debug)]
pub enum AppMsg {
    // Theme selection
    ThemeSelected(Theme),
    // Preview actions
    PreviewTheme,
    ApplyTheme,
    CancelPreview,
    // Editor actions
    NewTheme,
    EditTheme,
    // Editor callbacks
    ThemeSaved(Theme),
    EditorCancelled,
    // General
    Refresh,
    Import,
    // Toast messages
    ShowToast(String),
}

pub struct App {
    theme_browser: Controller<ThemeBrowserModel>,
    preview_panel: Controller<PreviewPanelModel>,
    editor_dialog: Option<Controller<ThemeEditorModel>>,
    editor_window: Option<gtk::Window>,
    selected_theme: Option<Theme>,
    original_theme_id: String, // Theme before preview started
    toast_overlay: adw::ToastOverlay,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("VulcanOS Theme Manager"),
            set_default_size: (900, 650),

            #[name = "toast_overlay"]
            adw::ToastOverlay {
                adw::ToolbarView {
                    add_top_bar = &adw::HeaderBar {
                        #[wrap(Some)]
                        set_title_widget = &adw::WindowTitle {
                            set_title: "Theme Manager",
                            #[watch]
                            set_subtitle: &format!(
                                "Current: {}",
                                model.original_theme_id
                            ),
                        },

                        pack_start = &gtk::Button {
                            set_icon_name: "list-add-symbolic",
                            set_tooltip_text: Some("Create new theme"),
                            connect_clicked => AppMsg::NewTheme,
                        },

                        pack_start = &gtk::Button {
                            set_icon_name: "document-open-symbolic",
                            set_tooltip_text: Some("Import theme file"),
                            connect_clicked => AppMsg::Import,
                        },

                        pack_end = &gtk::Button {
                            set_icon_name: "view-refresh-symbolic",
                            set_tooltip_text: Some("Refresh themes"),
                            connect_clicked => AppMsg::Refresh,
                        },
                    },

                    #[wrap(Some)]
                    set_content = &gtk::Paned {
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

                                gtk::Label {
                                    set_markup: "<b>Available Themes</b>",
                                    set_halign: gtk::Align::Start,
                                    set_margin_start: 8,
                                },

                                model.theme_browser.widget() {},
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
                                    set_sensitive: model.selected_theme.is_some()
                                        && model.selected_theme.as_ref().map(|t| !t.is_builtin).unwrap_or(false),
                                    connect_clicked => AppMsg::EditTheme,
                                },

                                gtk::Button {
                                    set_label: "Preview",
                                    set_tooltip_text: Some("Preview theme (temporary)"),
                                    #[watch]
                                    set_sensitive: model.selected_theme.is_some(),
                                    connect_clicked => AppMsg::PreviewTheme,
                                },

                                gtk::Button {
                                    set_label: "Cancel",
                                    set_tooltip_text: Some("Revert to original theme"),
                                    connect_clicked => AppMsg::CancelPreview,
                                },

                                gtk::Button {
                                    set_label: "Apply",
                                    add_css_class: "suggested-action",
                                    set_tooltip_text: Some("Apply theme permanently"),
                                    #[watch]
                                    set_sensitive: model.selected_theme.is_some(),
                                    connect_clicked => AppMsg::ApplyTheme,
                                },
                            },
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
        // Get current theme
        let original_theme_id = theme_applier::get_current_theme()
            .unwrap_or_else(|_| "tokyonight".to_string());

        // Create theme browser
        let theme_browser = ThemeBrowserModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| {
                match msg {
                    ThemeBrowserOutput::ThemeSelected(theme) => AppMsg::ThemeSelected(theme),
                }
            });

        // Create preview panel
        let preview_panel = PreviewPanelModel::builder()
            .launch(())
            .detach();

        let model = App {
            theme_browser,
            preview_panel,
            editor_dialog: None,
            editor_window: None,
            selected_theme: None,
            original_theme_id,
            toast_overlay: adw::ToastOverlay::new(),
        };

        let widgets = view_output!();

        // Store toast overlay reference
        // model.toast_overlay = widgets.toast_overlay.clone();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::ThemeSelected(theme) => {
                println!("Selected theme: {}", theme.theme_name);
                self.preview_panel.emit(PreviewPanelInput::SetTheme(Some(theme.clone())));
                self.selected_theme = Some(theme);
            }

            AppMsg::PreviewTheme => {
                if let Some(ref theme) = self.selected_theme {
                    if let Err(e) = theme_applier::preview_theme(&theme.theme_id) {
                        eprintln!("Preview failed: {}", e);
                        sender.input(AppMsg::ShowToast(format!("Preview failed: {}", e)));
                    } else {
                        sender.input(AppMsg::ShowToast(format!("Previewing: {}", theme.theme_name)));
                    }
                }
            }

            AppMsg::ApplyTheme => {
                if let Some(ref theme) = self.selected_theme {
                    if let Err(e) = theme_applier::apply_theme(&theme.theme_id) {
                        eprintln!("Apply failed: {}", e);
                        sender.input(AppMsg::ShowToast(format!("Apply failed: {}", e)));
                    } else {
                        self.original_theme_id = theme.theme_id.clone();
                        self.theme_browser.emit(ThemeBrowserInput::SetCurrentTheme(theme.theme_id.clone()));
                        sender.input(AppMsg::ShowToast(format!("Applied: {}", theme.theme_name)));
                    }
                }
            }

            AppMsg::CancelPreview => {
                if let Err(e) = theme_applier::revert_theme() {
                    eprintln!("Revert failed: {}", e);
                } else {
                    sender.input(AppMsg::ShowToast("Reverted to original theme".to_string()));
                }
            }

            AppMsg::NewTheme => {
                self.open_editor(None, true, sender.clone());
            }

            AppMsg::EditTheme => {
                if let Some(ref theme) = self.selected_theme {
                    if !theme.is_builtin {
                        self.open_editor(Some(theme.clone()), false, sender.clone());
                    }
                }
            }

            AppMsg::ThemeSaved(theme) => {
                // Save the theme
                match theme_storage::save_theme(&theme) {
                    Ok(_) => {
                        sender.input(AppMsg::ShowToast(format!("Saved: {}", theme.theme_name)));
                        // Refresh the browser
                        self.theme_browser.emit(ThemeBrowserInput::Refresh);
                    }
                    Err(e) => {
                        sender.input(AppMsg::ShowToast(format!("Save failed: {}", e)));
                    }
                }

                // Close editor window
                if let Some(window) = self.editor_window.take() {
                    window.close();
                }
                self.editor_dialog = None;
            }

            AppMsg::EditorCancelled => {
                if let Some(window) = self.editor_window.take() {
                    window.close();
                }
                self.editor_dialog = None;
            }

            AppMsg::Refresh => {
                self.theme_browser.emit(ThemeBrowserInput::Refresh);
                sender.input(AppMsg::ShowToast("Themes refreshed".to_string()));
            }

            AppMsg::Import => {
                self.import_theme_dialog(sender.clone());
            }

            AppMsg::ShowToast(message) => {
                println!("Toast: {}", message);
                // Note: In a full implementation, we'd show the toast via the overlay
            }
        }
    }
}

impl App {
    fn open_editor(&mut self, theme: Option<Theme>, is_new: bool, sender: ComponentSender<Self>) {
        let editor = ThemeEditorModel::builder()
            .launch((theme, is_new))
            .forward(sender.input_sender(), |msg| {
                match msg {
                    ThemeEditorOutput::Saved(theme) => AppMsg::ThemeSaved(theme),
                    ThemeEditorOutput::Cancelled => AppMsg::EditorCancelled,
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
                            sender.input(AppMsg::ShowToast(format!("Imported: {}", theme.theme_name)));
                            sender.input(AppMsg::Refresh);
                        }
                        Err(e) => {
                            sender.input(AppMsg::ShowToast(format!("Import failed: {}", e)));
                        }
                    }
                }
            }
        });
    }
}
