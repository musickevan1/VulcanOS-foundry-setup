use relm4::prelude::*;
use adw::prelude::*;

use crate::models::{Theme, ColorGroup};

/// Input messages for ThemeEditor
#[derive(Debug)]
pub enum ThemeEditorInput {
    /// Color changed for a field
    ColorChanged { field: String, color: String },
    /// Text field changed (for system theme names)
    TextChanged { field: String, value: String },
    /// Save the theme
    Save,
    /// Cancel editing
    Cancel,
}

/// Output messages from ThemeEditor
#[derive(Debug)]
pub enum ThemeEditorOutput {
    Saved(Theme),
    Cancelled,
}

/// Theme editor dialog for creating/editing themes
pub struct ThemeEditorModel {
    theme: Theme,
    is_new: bool,
}

#[relm4::component(pub)]
impl SimpleComponent for ThemeEditorModel {
    type Init = (Option<Theme>, bool); // (theme to edit or None for new, is_new)
    type Input = ThemeEditorInput;
    type Output = ThemeEditorOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 12,
            set_margin_all: 16,
            set_width_request: 500,

            // Header with name/id
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 12,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 4,
                    set_hexpand: true,

                    gtk::Label {
                        set_text: "Theme Name",
                        set_halign: gtk::Align::Start,
                        add_css_class: "dim-label",
                    },

                    #[name = "name_entry"]
                    gtk::Entry {
                        set_text: &model.theme.theme_name,
                        set_placeholder_text: Some("My Custom Theme"),
                        connect_changed[sender] => move |entry| {
                            sender.input(ThemeEditorInput::TextChanged {
                                field: "theme_name".to_string(),
                                value: entry.text().to_string(),
                            });
                        },
                    },
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 4,
                    set_hexpand: true,

                    gtk::Label {
                        set_text: "Theme ID",
                        set_halign: gtk::Align::Start,
                        add_css_class: "dim-label",
                    },

                    #[name = "id_entry"]
                    gtk::Entry {
                        set_text: &model.theme.theme_id,
                        set_placeholder_text: Some("my-custom-theme"),
                        #[watch]
                        set_sensitive: model.is_new,
                        connect_changed[sender] => move |entry| {
                            sender.input(ThemeEditorInput::TextChanged {
                                field: "theme_id".to_string(),
                                value: entry.text().to_string(),
                            });
                        },
                    },
                },
            },

            gtk::Separator {},

            // Color groups in a scrollable area
            gtk::ScrolledWindow {
                set_vexpand: true,
                set_min_content_height: 400,
                set_policy: (gtk::PolicyType::Never, gtk::PolicyType::Automatic),

                #[name = "groups_box"]
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 16,
                },
            },

            gtk::Separator {},

            // Action buttons
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 12,
                set_halign: gtk::Align::End,

                gtk::Button {
                    set_label: "Cancel",
                    connect_clicked => ThemeEditorInput::Cancel,
                },

                gtk::Button {
                    set_label: "Save Theme",
                    add_css_class: "suggested-action",
                    connect_clicked => ThemeEditorInput::Save,
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (theme_opt, is_new) = init;
        let theme = theme_opt.unwrap_or_else(|| Theme::new("New Theme", "new-theme"));

        let model = ThemeEditorModel { theme, is_new };
        let widgets = view_output!();

        // Build color group UI
        for group in ColorGroup::all_groups() {
            let expander = adw::ExpanderRow::builder()
                .title(group.name)
                .expanded(group.name == "Accents" || group.name == "Backgrounds")
                .build();

            for field in &group.fields {
                let row = build_color_row(&model.theme, field, sender.clone());
                expander.add_row(&row);
            }

            // Wrap in a ListBox for proper styling
            let list_box = gtk::ListBox::new();
            list_box.add_css_class("boxed-list");
            list_box.append(&expander);
            widgets.groups_box.append(&list_box);
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            ThemeEditorInput::ColorChanged { field, color } => {
                update_theme_color(&mut self.theme, &field, &color);
            }

            ThemeEditorInput::TextChanged { field, value } => {
                match field.as_str() {
                    "theme_name" => self.theme.theme_name = value,
                    "theme_id" => self.theme.theme_id = value,
                    "gtk_theme" => self.theme.gtk_theme = value,
                    "icon_theme" => self.theme.icon_theme = value,
                    "cursor_theme" => self.theme.cursor_theme = value,
                    "kvantum_theme" => self.theme.kvantum_theme = value,
                    "nvim_colorscheme" => self.theme.nvim_colorscheme = value,
                    "theme_wallpaper" => {
                        self.theme.theme_wallpaper = if value.is_empty() {
                            None
                        } else {
                            Some(value)
                        };
                    }
                    _ => {}
                }
            }

            ThemeEditorInput::Save => {
                sender.output(ThemeEditorOutput::Saved(self.theme.clone())).ok();
            }

            ThemeEditorInput::Cancel => {
                sender.output(ThemeEditorOutput::Cancelled).ok();
            }
        }
    }
}

/// Build a color picker row for a color field
fn build_color_row(
    theme: &Theme,
    field: &crate::models::ColorField,
    sender: ComponentSender<ThemeEditorModel>,
) -> adw::ActionRow {
    let row = adw::ActionRow::builder()
        .title(field.label)
        .subtitle(field.description)
        .build();

    // Check if this is a text field (system themes) or color field
    let is_text_field = matches!(
        field.field,
        "gtk_theme" | "icon_theme" | "cursor_theme" | "kvantum_theme" | "nvim_colorscheme" | "theme_wallpaper"
    );

    if is_text_field {
        let entry = gtk::Entry::new();
        entry.set_valign(gtk::Align::Center);
        entry.set_width_chars(20);
        entry.set_text(&get_theme_text_value(theme, field.field));

        let field_name = field.field.to_string();
        entry.connect_changed(move |e| {
            sender.input(ThemeEditorInput::TextChanged {
                field: field_name.clone(),
                value: e.text().to_string(),
            });
        });

        row.add_suffix(&entry);
    } else {
        // Color button (deprecated but works)
        let color_str = get_theme_color(theme, field.field);
        let rgba = parse_color_to_rgba(&color_str);

        let color_button = gtk::ColorButton::with_rgba(&rgba);
        color_button.set_valign(gtk::Align::Center);
        color_button.set_use_alpha(false);

        let field_name = field.field.to_string();
        color_button.connect_color_set(move |btn| {
            let rgba = btn.rgba();
            let hex = rgba_to_hex(&rgba);
            sender.input(ThemeEditorInput::ColorChanged {
                field: field_name.clone(),
                color: hex,
            });
        });

        row.add_suffix(&color_button);
    }

    row
}

/// Get color value from theme by field name
fn get_theme_color(theme: &Theme, field: &str) -> String {
    match field {
        "bg_primary" => theme.bg_primary.clone(),
        "bg_secondary" => theme.bg_secondary.clone(),
        "bg_tertiary" => theme.bg_tertiary.clone(),
        "bg_surface" => theme.bg_surface.clone(),
        "fg_primary" => theme.fg_primary.clone(),
        "fg_secondary" => theme.fg_secondary.clone(),
        "fg_muted" => theme.fg_muted.clone(),
        "accent" => theme.accent.clone(),
        "accent_alt" => theme.accent_alt.clone(),
        "red" => theme.red.clone(),
        "green" => theme.green.clone(),
        "yellow" => theme.yellow.clone(),
        "blue" => theme.blue.clone(),
        "purple" => theme.purple.clone(),
        "cyan" => theme.cyan.clone(),
        "orange" => theme.orange.clone(),
        "pink" => theme.pink.clone(),
        "bright_red" => theme.bright_red.clone(),
        "bright_green" => theme.bright_green.clone(),
        "bright_yellow" => theme.bright_yellow.clone(),
        "bright_blue" => theme.bright_blue.clone(),
        "bright_purple" => theme.bright_purple.clone(),
        "bright_cyan" => theme.bright_cyan.clone(),
        "border_active" => theme.border_active.clone(),
        "border_inactive" => theme.border_inactive.clone(),
        "selection" => theme.selection.clone(),
        "cursor" => theme.cursor.clone(),
        "gradient_start" => theme.gradient_start.clone(),
        "gradient_end" => theme.gradient_end.clone(),
        _ => "#888888".to_string(),
    }
}

/// Get text value from theme by field name
fn get_theme_text_value(theme: &Theme, field: &str) -> String {
    match field {
        "gtk_theme" => theme.gtk_theme.clone(),
        "icon_theme" => theme.icon_theme.clone(),
        "cursor_theme" => theme.cursor_theme.clone(),
        "kvantum_theme" => theme.kvantum_theme.clone(),
        "nvim_colorscheme" => theme.nvim_colorscheme.clone(),
        "theme_wallpaper" => theme.theme_wallpaper.clone().unwrap_or_default(),
        _ => String::new(),
    }
}

/// Update theme color by field name
fn update_theme_color(theme: &mut Theme, field: &str, color: &str) {
    let color = color.to_string();
    match field {
        "bg_primary" => theme.bg_primary = color,
        "bg_secondary" => theme.bg_secondary = color,
        "bg_tertiary" => theme.bg_tertiary = color,
        "bg_surface" => theme.bg_surface = color,
        "fg_primary" => theme.fg_primary = color,
        "fg_secondary" => theme.fg_secondary = color,
        "fg_muted" => theme.fg_muted = color,
        "accent" => theme.accent = color,
        "accent_alt" => theme.accent_alt = color,
        "red" => theme.red = color,
        "green" => theme.green = color,
        "yellow" => theme.yellow = color,
        "blue" => theme.blue = color,
        "purple" => theme.purple = color,
        "cyan" => theme.cyan = color,
        "orange" => theme.orange = color,
        "pink" => theme.pink = color,
        "bright_red" => theme.bright_red = color,
        "bright_green" => theme.bright_green = color,
        "bright_yellow" => theme.bright_yellow = color,
        "bright_blue" => theme.bright_blue = color,
        "bright_purple" => theme.bright_purple = color,
        "bright_cyan" => theme.bright_cyan = color,
        "border_active" => theme.border_active = color,
        "border_inactive" => theme.border_inactive = color,
        "selection" => theme.selection = color,
        "cursor" => theme.cursor = color,
        "gradient_start" => theme.gradient_start = color,
        "gradient_end" => theme.gradient_end = color,
        _ => {}
    }
}

/// Parse hex color to GTK RGBA
fn parse_color_to_rgba(hex: &str) -> gtk::gdk::RGBA {
    let hex = hex.trim_start_matches('#');
    if hex.len() >= 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(128) as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(128) as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(128) as f32 / 255.0;
        gtk::gdk::RGBA::new(r, g, b, 1.0)
    } else {
        gtk::gdk::RGBA::new(0.5, 0.5, 0.5, 1.0)
    }
}

/// Convert GTK RGBA to hex string
fn rgba_to_hex(rgba: &gtk::gdk::RGBA) -> String {
    format!(
        "#{:02x}{:02x}{:02x}",
        (rgba.red() * 255.0) as u8,
        (rgba.green() * 255.0) as u8,
        (rgba.blue() * 255.0) as u8
    )
}
