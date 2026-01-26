use gtk::prelude::*;
use gtk::gio;
use relm4::prelude::*;
use std::path::PathBuf;

use crate::models::Theme;

/// Output messages from binding dialog
#[derive(Debug)]
pub enum BindingDialogOutput {
    /// User chose to apply theme only (no wallpaper change)
    ApplyThemeOnly,
    /// User chose to apply both theme and its suggested wallpaper
    ApplyBoth(PathBuf),  // wallpaper path to apply
    /// User cancelled the dialog
    Cancelled,
}

/// Input messages for binding dialog
#[derive(Debug)]
pub enum BindingDialogInput {
    ThemeOnly,
    ApplyBoth,
    Cancel,
}

/// Init data for binding dialog
pub struct BindingDialogInit {
    pub theme: Theme,
    pub wallpaper_path: PathBuf,
}

pub struct BindingDialogModel {
    theme: Theme,
    wallpaper_path: PathBuf,
}

#[relm4::component(pub)]
impl SimpleComponent for BindingDialogModel {
    type Init = BindingDialogInit;
    type Input = BindingDialogInput;
    type Output = BindingDialogOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 16,
            set_margin_all: 24,

            // Title
            gtk::Label {
                set_markup: "<b>Apply Theme Wallpaper?</b>",
                set_halign: gtk::Align::Start,
            },

            // Description
            gtk::Label {
                #[watch]
                set_label: &format!(
                    "Theme '{}' suggests a wallpaper. Apply it too?",
                    model.theme.theme_name
                ),
                set_wrap: true,
                set_halign: gtk::Align::Start,
            },

            // Preview area (horizontal split)
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 16,
                set_hexpand: true,
                set_vexpand: true,
                set_height_request: 200,

                // Theme colors preview (left)
                gtk::Frame {
                    set_hexpand: true,
                    set_label: Some("Theme Colors"),

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 4,
                        set_margin_all: 8,

                        // Color swatches (simplified - 2 rows of 4)
                        #[name = "color_preview"]
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 2,
                            set_vexpand: true,
                        },
                    },
                },

                // Wallpaper preview (right)
                gtk::Frame {
                    set_hexpand: true,
                    set_label: Some("Suggested Wallpaper"),

                    #[name = "wallpaper_picture"]
                    gtk::Picture {
                        set_content_fit: gtk::ContentFit::Contain,
                        set_can_shrink: true,
                        set_hexpand: true,
                        set_vexpand: true,
                    },
                },
            },

            // Action buttons
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 8,
                set_halign: gtk::Align::End,
                set_margin_top: 8,

                gtk::Button {
                    set_label: "Cancel",
                    connect_clicked => BindingDialogInput::Cancel,
                },

                gtk::Button {
                    set_label: "Theme Only",
                    set_tooltip_text: Some("Apply theme colors, keep current wallpaper"),
                    connect_clicked => BindingDialogInput::ThemeOnly,
                },

                gtk::Button {
                    set_label: "Apply Both",
                    add_css_class: "suggested-action",
                    set_tooltip_text: Some("Apply theme colors and wallpaper"),
                    connect_clicked => BindingDialogInput::ApplyBoth,
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = BindingDialogModel {
            theme: init.theme,
            wallpaper_path: init.wallpaper_path,
        };

        let widgets = view_output!();

        // Set wallpaper picture
        widgets.wallpaper_picture.set_file(Some(&gio::File::for_path(&model.wallpaper_path)));

        // Build color preview (simplified for dialog)
        build_color_preview(&widgets.color_preview, &model.theme);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            BindingDialogInput::ThemeOnly => {
                sender.output(BindingDialogOutput::ApplyThemeOnly).ok();
            }
            BindingDialogInput::ApplyBoth => {
                sender.output(BindingDialogOutput::ApplyBoth(self.wallpaper_path.clone())).ok();
            }
            BindingDialogInput::Cancel => {
                sender.output(BindingDialogOutput::Cancelled).ok();
            }
        }
    }
}

fn build_color_preview(container: &gtk::Box, theme: &Theme) {
    let colors = theme.preview_colors();

    // Create two rows of 4 colors each
    for row_start in [0, 4] {
        let row = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        row.set_homogeneous(true);

        for i in row_start..std::cmp::min(row_start + 4, colors.len()) {
            let area = gtk::DrawingArea::new();
            area.set_content_height(40);
            area.set_content_width(50);

            let color = parse_hex_color(colors[i]);
            area.set_draw_func(move |_, cr, width, height| {
                cr.set_source_rgb(color.0, color.1, color.2);
                cr.rectangle(0.0, 0.0, width as f64, height as f64);
                let _ = cr.fill();
            });

            row.append(&area);
        }

        container.append(&row);
    }
}

fn parse_hex_color(hex: &str) -> (f64, f64, f64) {
    let hex = hex.trim_start_matches('#');
    if hex.len() >= 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f64 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f64 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f64 / 255.0;
        (r, g, b)
    } else {
        (0.5, 0.5, 0.5)
    }
}
