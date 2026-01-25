use gtk::prelude::*;
use relm4::prelude::*;

use crate::models::Theme;

/// Input messages for ThemeCard
#[derive(Debug)]
pub enum ThemeCardInput {
    SetCurrent(bool),
}

/// Output messages from ThemeCard
#[derive(Debug)]
pub enum ThemeCardOutput {
    Selected(String), // theme_id
}

/// A single theme card displaying color preview and metadata
pub struct ThemeCardModel {
    theme: Theme,
    is_current: bool,
}

#[relm4::component(pub)]
impl SimpleComponent for ThemeCardModel {
    type Init = (Theme, bool);
    type Input = ThemeCardInput;
    type Output = ThemeCardOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 8,
            set_width_request: 180,
            add_css_class: "theme-card",

            // Color palette preview (8 color squares in 2 rows)
            gtk::Frame {
                add_css_class: "color-preview-frame",
                set_margin_all: 4,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 2,

                    // Row 1: backgrounds + accents
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 2,
                        set_homogeneous: true,

                        #[name = "color0"]
                        gtk::DrawingArea {
                            set_content_height: 24,
                            set_content_width: 40,
                        },
                        #[name = "color1"]
                        gtk::DrawingArea {
                            set_content_height: 24,
                            set_content_width: 40,
                        },
                        #[name = "color2"]
                        gtk::DrawingArea {
                            set_content_height: 24,
                            set_content_width: 40,
                        },
                        #[name = "color3"]
                        gtk::DrawingArea {
                            set_content_height: 24,
                            set_content_width: 40,
                        },
                    },

                    // Row 2: ANSI colors
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 2,
                        set_homogeneous: true,

                        #[name = "color4"]
                        gtk::DrawingArea {
                            set_content_height: 24,
                            set_content_width: 40,
                        },
                        #[name = "color5"]
                        gtk::DrawingArea {
                            set_content_height: 24,
                            set_content_width: 40,
                        },
                        #[name = "color6"]
                        gtk::DrawingArea {
                            set_content_height: 24,
                            set_content_width: 40,
                        },
                        #[name = "color7"]
                        gtk::DrawingArea {
                            set_content_height: 24,
                            set_content_width: 40,
                        },
                    },
                },
            },

            // Theme name
            gtk::Label {
                set_label: &model.theme.theme_name,
                add_css_class: "theme-name",
                set_ellipsize: gtk::pango::EllipsizeMode::End,
            },

            // Current indicator
            gtk::Label {
                #[watch]
                set_visible: model.is_current,
                set_label: "Current",
                add_css_class: "current-badge",
            },

            // Built-in badge
            gtk::Label {
                set_visible: model.theme.is_builtin,
                set_label: "Built-in",
                add_css_class: "builtin-badge",
            },
        }
    }

    fn init(
        (theme, is_current): Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ThemeCardModel { theme, is_current };
        let widgets = view_output!();

        // Set up color preview drawing areas
        let colors = model.theme.preview_colors();
        let color_widgets = [
            &widgets.color0,
            &widgets.color1,
            &widgets.color2,
            &widgets.color3,
            &widgets.color4,
            &widgets.color5,
            &widgets.color6,
            &widgets.color7,
        ];

        for (i, widget) in color_widgets.iter().enumerate() {
            if let Some(color_str) = colors.get(i) {
                let color = parse_hex_color(color_str);
                widget.set_draw_func(move |_, cr, width, height| {
                    cr.set_source_rgb(color.0, color.1, color.2);
                    cr.rectangle(0.0, 0.0, width as f64, height as f64);
                    let _ = cr.fill();
                });
            }
        }

        // Make the card clickable
        let gesture = gtk::GestureClick::new();
        let theme_id = model.theme.theme_id.clone();
        gesture.connect_released(move |_, _, _, _| {
            sender.output(ThemeCardOutput::Selected(theme_id.clone())).ok();
        });
        root.add_controller(gesture);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ThemeCardInput::SetCurrent(is_current) => {
                self.is_current = is_current;
            }
        }
    }
}

/// Parse a hex color string (#RRGGBB) into RGB floats (0.0-1.0)
fn parse_hex_color(hex: &str) -> (f64, f64, f64) {
    let hex = hex.trim_start_matches('#');
    if hex.len() >= 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f64 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f64 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f64 / 255.0;
        (r, g, b)
    } else {
        (0.5, 0.5, 0.5) // Default gray
    }
}
