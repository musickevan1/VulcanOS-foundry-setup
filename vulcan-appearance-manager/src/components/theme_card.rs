use gtk::prelude::*;
use gtk::gio;
use relm4::prelude::*;
use relm4::factory::{FactoryComponent, DynamicIndex, FactorySender};

use crate::models::{Theme, resolve_theme_wallpaper};

/// Output messages from ThemeCard
#[derive(Debug)]
pub enum ThemeCardOutput {
    Selected(Theme),
}

/// Input messages to ThemeCard
#[derive(Debug)]
pub enum ThemeItemInput {
    SetOverride(bool),
}

/// Factory item for theme display in FlowBox
#[derive(Debug)]
pub struct ThemeItem {
    pub theme: Theme,
    pub is_current: bool,
    pub is_override: bool,
}

#[relm4::factory(pub)]
impl FactoryComponent for ThemeItem {
    type Init = (Theme, bool);
    type Input = ThemeItemInput;
    type Output = ThemeCardOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::FlowBox;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 8,
            set_width_request: 180,
            set_margin_all: 8,
            add_css_class: "theme-card",

            // Color palette preview with wallpaper overlay
            gtk::Overlay {
                // Main content: color preview frame
                #[wrap(Some)]
                set_child = &gtk::Frame {
                    add_css_class: "color-preview-frame",

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 2,
                        set_margin_all: 4,

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

                // Wallpaper thumbnail (bottom-right corner)
                #[name = "wallpaper_thumb"]
                add_overlay = &gtk::Picture {
                    set_halign: gtk::Align::End,
                    set_valign: gtk::Align::End,
                    set_margin_end: 4,
                    set_margin_bottom: 4,
                    set_width_request: 60,
                    set_height_request: 40,
                    set_content_fit: gtk::ContentFit::Cover,
                    set_can_shrink: true,
                    add_css_class: "wallpaper-corner-preview",
                },

                // Override badge (top-right corner)
                #[name = "override_badge"]
                add_overlay = &gtk::Image {
                    set_icon_name: Some("emblem-default-symbolic"),
                    set_pixel_size: 16,
                    set_halign: gtk::Align::End,
                    set_valign: gtk::Align::Start,
                    set_margin_top: 4,
                    set_margin_end: 4,
                    add_css_class: "override-badge",
                },
            },

            // Theme name
            gtk::Label {
                set_label: &self.theme.theme_name,
                add_css_class: "theme-name",
                set_ellipsize: gtk::pango::EllipsizeMode::End,
            },

            // Badges row
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 4,
                set_halign: gtk::Align::Center,

                gtk::Label {
                    #[watch]
                    set_visible: self.is_current,
                    set_label: "Current",
                    add_css_class: "current-badge",
                },

                gtk::Label {
                    set_visible: self.theme.is_builtin,
                    set_label: "Built-in",
                    add_css_class: "builtin-badge",
                },
            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        ThemeItem {
            theme: init.0,
            is_current: init.1,
            is_override: false,
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            ThemeItemInput::SetOverride(is_override) => {
                self.is_override = is_override;
            }
        }
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();

        // Set wallpaper thumbnail if theme has one
        if let Some(wallpaper_path) = resolve_theme_wallpaper(&self.theme) {
            widgets.wallpaper_thumb.set_file(Some(&gio::File::for_path(&wallpaper_path)));
            widgets.wallpaper_thumb.set_visible(true);
        } else {
            widgets.wallpaper_thumb.set_visible(false);
        }

        // Show override badge only if is_override is true
        widgets.override_badge.set_visible(self.is_override);

        // Set up color preview drawing areas
        let colors = self.theme.preview_colors();
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

        // Make clickable
        let gesture = gtk::GestureClick::new();
        let theme = self.theme.clone();
        gesture.connect_released(move |_, _, _, _| {
            sender.output(ThemeCardOutput::Selected(theme.clone())).ok();
        });
        root.add_controller(gesture);

        widgets
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
        (0.5, 0.5, 0.5)
    }
}
