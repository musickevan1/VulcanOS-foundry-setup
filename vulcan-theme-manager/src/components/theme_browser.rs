use gtk::prelude::*;
use relm4::prelude::*;
use relm4::factory::FactoryVecDeque;

use crate::models::Theme;
use crate::services::theme_storage;

/// Input messages for ThemeBrowser
#[derive(Debug)]
pub enum ThemeBrowserInput {
    Refresh,
    SetCurrentTheme(String),
}

/// Output messages from ThemeBrowser
#[derive(Debug)]
pub enum ThemeBrowserOutput {
    ThemeSelected(Theme),
}

/// Factory item for theme display in FlowBox
#[derive(Debug)]
pub struct ThemeItem {
    theme: Theme,
    is_current: bool,
}

#[relm4::factory(pub)]
impl FactoryComponent for ThemeItem {
    type Init = (Theme, bool);
    type Input = ();
    type Output = ThemeBrowserOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::FlowBox;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 8,
            set_width_request: 180,
            set_margin_all: 8,
            add_css_class: "theme-card",

            // Color palette preview
            gtk::Frame {
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
            sender.output(ThemeBrowserOutput::ThemeSelected(theme.clone())).ok();
        });
        root.add_controller(gesture);

        widgets
    }
}

/// Theme browser component with FlowBox grid of themes
pub struct ThemeBrowserModel {
    themes: FactoryVecDeque<ThemeItem>,
    current_theme_id: String,
}

#[relm4::component(pub)]
impl SimpleComponent for ThemeBrowserModel {
    type Init = ();
    type Input = ThemeBrowserInput;
    type Output = ThemeBrowserOutput;

    view! {
        gtk::ScrolledWindow {
            set_hexpand: true,
            set_vexpand: true,
            set_policy: (gtk::PolicyType::Never, gtk::PolicyType::Automatic),

            #[local_ref]
            themes_box -> gtk::FlowBox {
                set_selection_mode: gtk::SelectionMode::Single,
                set_homogeneous: false,
                set_max_children_per_line: 6,
                set_min_children_per_line: 2,
                set_row_spacing: 12,
                set_column_spacing: 12,
                set_margin_all: 12,
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Get current theme
        let current_theme_id = crate::services::theme_applier::get_current_theme()
            .unwrap_or_else(|_| "tokyonight".to_string());

        // Create factory
        let mut themes = FactoryVecDeque::builder()
            .launch(gtk::FlowBox::default())
            .forward(sender.output_sender(), |msg| msg);

        // Load themes
        if let Ok(theme_list) = theme_storage::load_all_themes() {
            for theme in theme_list {
                let is_current = theme.theme_id == current_theme_id;
                themes.guard().push_back((theme, is_current));
            }
        }

        let model = ThemeBrowserModel {
            themes,
            current_theme_id,
        };

        let themes_box = model.themes.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ThemeBrowserInput::Refresh => {
                // Clear and reload themes
                self.themes.guard().clear();

                if let Ok(current) = crate::services::theme_applier::get_current_theme() {
                    self.current_theme_id = current;
                }

                if let Ok(theme_list) = theme_storage::load_all_themes() {
                    for theme in theme_list {
                        let is_current = theme.theme_id == self.current_theme_id;
                        self.themes.guard().push_back((theme, is_current));
                    }
                }
            }

            ThemeBrowserInput::SetCurrentTheme(theme_id) => {
                self.current_theme_id = theme_id.clone();

                // Update current indicators - need to rebuild
                let mut guard = self.themes.guard();
                let count = guard.len();
                for i in 0..count {
                    if let Some(item) = guard.get_mut(i) {
                        item.is_current = item.theme.theme_id == theme_id;
                    }
                }
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
        (0.5, 0.5, 0.5)
    }
}
