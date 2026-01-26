use gtk::prelude::*;
use gtk::gio;
use relm4::prelude::*;
use relm4::factory::{FactoryComponent, DynamicIndex, FactorySender};

use crate::models::{UnifiedProfile, BindingMode};
use crate::services::theme_storage;

/// Output messages from ProfileCard
#[derive(Debug)]
pub enum ProfileCardOutput {
    Load(String),    // profile name to load
    Delete(String),  // profile name to delete
}

/// Factory item for profile display in FlowBox
#[derive(Debug)]
pub struct ProfileItem {
    pub profile: UnifiedProfile,
    pub is_active: bool,
}

#[relm4::factory(pub)]
impl FactoryComponent for ProfileItem {
    type Init = (UnifiedProfile, bool);
    type Input = ();
    type Output = ProfileCardOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::FlowBox;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 8,
            set_width_request: 200,
            set_margin_all: 8,
            add_css_class: "profile-card",

            // Preview area: theme colors (left) + wallpaper (right)
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 4,
                set_height_request: 100,

                // Theme color preview (if theme bound)
                #[name = "theme_preview"]
                gtk::Frame {
                    set_hexpand: true,

                    #[name = "color_box"]
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 2,
                        set_margin_all: 4,
                    },
                },

                // Wallpaper preview
                gtk::Frame {
                    set_hexpand: true,

                    #[name = "wallpaper_picture"]
                    gtk::Picture {
                        set_content_fit: gtk::ContentFit::Cover,
                        set_can_shrink: true,
                    },
                },
            },

            // Profile name
            gtk::Label {
                #[watch]
                set_label: &self.profile.name,
                add_css_class: "profile-name",
                set_halign: gtk::Align::Start,
                set_ellipsize: gtk::pango::EllipsizeMode::End,
            },

            // Info line: theme name + binding mode
            gtk::Label {
                #[watch]
                set_label: &format!(
                    "{} • {}",
                    self.profile.theme_id.as_deref().unwrap_or("No theme"),
                    self.profile.binding_mode.display_name()
                ),
                add_css_class: "dim-label",
                set_halign: gtk::Align::Start,
                set_ellipsize: gtk::pango::EllipsizeMode::End,
            },

            // Action buttons row
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 4,

                #[name = "load_button"]
                gtk::Button {
                    set_label: "Load",
                    set_hexpand: true,
                    connect_clicked[sender, name = self.profile.name.clone()] => move |_| {
                        sender.output(ProfileCardOutput::Load(name.clone())).ok();
                    },
                },

                gtk::Button {
                    set_icon_name: "user-trash-symbolic",
                    set_tooltip_text: Some("Delete profile"),
                    connect_clicked[sender, name = self.profile.name.clone()] => move |_| {
                        sender.output(ProfileCardOutput::Delete(name.clone())).ok();
                    },
                },
            },

            // Active badge
            gtk::Label {
                #[watch]
                set_visible: self.is_active,
                set_label: "Active",
                add_css_class: "current-badge",
                set_halign: gtk::Align::Center,
            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        ProfileItem {
            profile: init.0,
            is_active: init.1,
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

        // Set wallpaper preview (first monitor's wallpaper)
        if let Some(wallpaper_path) = self.profile.monitor_wallpapers.values().next() {
            if wallpaper_path.exists() {
                widgets.wallpaper_picture.set_file(Some(&gio::File::for_path(wallpaper_path)));
            }
        }

        // Set theme color preview if theme_id exists
        if let Some(ref theme_id) = self.profile.theme_id {
            if let Ok(themes) = theme_storage::load_all_themes() {
                if let Some(theme) = themes.iter().find(|t| &t.theme_id == theme_id) {
                    build_mini_color_preview(&widgets.color_box, theme);
                }
            }
        }

        // Add suggested-action class if active
        if self.is_active {
            widgets.load_button.add_css_class("suggested-action");
        }

        widgets
    }
}

/// Build a small color preview (4 colors in a row)
fn build_mini_color_preview(container: &gtk::Box, theme: &crate::models::Theme) {
    let colors = theme.preview_colors();
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    row.set_homogeneous(true);

    for color_str in colors.iter().take(4) {
        let area = gtk::DrawingArea::new();
        area.set_content_height(20);
        area.set_content_width(20);

        let color = parse_hex_color(color_str);
        area.set_draw_func(move |_, cr, width, height| {
            cr.set_source_rgb(color.0, color.1, color.2);
            cr.rectangle(0.0, 0.0, width as f64, height as f64);
            let _ = cr.fill();
        });

        row.append(&area);
    }

    container.append(&row);
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
