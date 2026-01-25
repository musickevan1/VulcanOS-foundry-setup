use gtk::prelude::*;
use relm4::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use crate::models::Theme;

/// Input messages for PreviewPanel
#[derive(Debug)]
pub enum PreviewPanelInput {
    SetTheme(Option<Theme>),
}

/// Mock desktop preview showing theme colors
pub struct PreviewPanelModel {
    theme: Option<Theme>,
    drawing_area: Rc<RefCell<Option<gtk::DrawingArea>>>,
}

#[relm4::component(pub)]
impl SimpleComponent for PreviewPanelModel {
    type Init = ();
    type Input = PreviewPanelInput;
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 8,
            set_margin_all: 12,

            gtk::Label {
                set_markup: "<b>Preview</b>",
                set_halign: gtk::Align::Start,
            },

            // Mock desktop frame
            gtk::Frame {
                add_css_class: "preview-frame",

                #[name = "preview_area"]
                gtk::DrawingArea {
                    set_content_width: 300,
                    set_content_height: 200,
                    set_hexpand: true,
                    set_vexpand: true,
                },
            },

            // Theme info
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 4,

                gtk::Label {
                    #[watch]
                    set_markup: &format!("<b>{}</b>",
                        model.theme.as_ref().map(|t| t.theme_name.as_str()).unwrap_or("No theme selected")),
                    set_halign: gtk::Align::Start,
                },

                gtk::Label {
                    #[watch]
                    set_text: model.theme.as_ref().map(|t| t.theme_id.as_str()).unwrap_or(""),
                    add_css_class: "dim-label",
                    set_halign: gtk::Align::Start,
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let drawing_area: Rc<RefCell<Option<gtk::DrawingArea>>> = Rc::new(RefCell::new(None));

        let model = PreviewPanelModel {
            theme: None,
            drawing_area: drawing_area.clone(),
        };

        let widgets = view_output!();

        // Store reference to drawing area
        *drawing_area.borrow_mut() = Some(widgets.preview_area.clone());

        // Set initial draw function (empty state)
        set_empty_preview(&widgets.preview_area);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            PreviewPanelInput::SetTheme(theme) => {
                self.theme = theme;

                // Update drawing area
                if let Some(ref area) = *self.drawing_area.borrow() {
                    if let Some(ref theme) = self.theme {
                        set_theme_preview(area, theme);
                    } else {
                        set_empty_preview(area);
                    }
                    area.queue_draw();
                }
            }
        }
    }
}

/// Set the drawing function for empty preview
fn set_empty_preview(area: &gtk::DrawingArea) {
    area.set_draw_func(|_, cr, width, height| {
        // Default dark background
        cr.set_source_rgb(0.11, 0.09, 0.09);
        cr.rectangle(0.0, 0.0, width as f64, height as f64);
        let _ = cr.fill();

        // "No theme" text
        cr.set_source_rgb(0.5, 0.5, 0.5);
        cr.select_font_face("sans-serif", gtk::cairo::FontSlant::Normal, gtk::cairo::FontWeight::Normal);
        cr.set_font_size(14.0);
        let text = "Select a theme to preview";
        if let Ok(extents) = cr.text_extents(text) {
            cr.move_to(
                (width as f64 - extents.width()) / 2.0,
                (height as f64 + extents.height()) / 2.0,
            );
            let _ = cr.show_text(text);
        }
    });
}

/// Set the drawing function to show a theme preview
fn set_theme_preview(area: &gtk::DrawingArea, theme: &Theme) {
    let bg_primary = parse_hex_color(&theme.bg_primary);
    let bg_secondary = parse_hex_color(&theme.bg_secondary);
    let bg_tertiary = parse_hex_color(&theme.bg_tertiary);
    let fg_primary = parse_hex_color(&theme.fg_primary);
    let fg_secondary = parse_hex_color(&theme.fg_secondary);
    let accent = parse_hex_color(&theme.accent);
    let accent_alt = parse_hex_color(&theme.accent_alt);
    let red = parse_hex_color(&theme.red);
    let green = parse_hex_color(&theme.green);
    let blue = parse_hex_color(&theme.blue);
    let purple = parse_hex_color(&theme.purple);

    area.set_draw_func(move |_, cr, width, height| {
        let w = width as f64;
        let h = height as f64;

        // Background
        cr.set_source_rgb(bg_primary.0, bg_primary.1, bg_primary.2);
        cr.rectangle(0.0, 0.0, w, h);
        let _ = cr.fill();

        // Top bar (waybar mock)
        cr.set_source_rgb(bg_secondary.0, bg_secondary.1, bg_secondary.2);
        cr.rectangle(0.0, 0.0, w, 28.0);
        let _ = cr.fill();

        // Accent highlight on bar
        cr.set_source_rgb(accent.0, accent.1, accent.2);
        cr.rectangle(10.0, 8.0, 60.0, 12.0);
        let _ = cr.fill();

        // Mock window
        let win_x = 20.0;
        let win_y = 45.0;
        let win_w = w - 40.0;
        let win_h = h - 65.0;

        // Window background
        cr.set_source_rgb(bg_secondary.0, bg_secondary.1, bg_secondary.2);
        cr.rectangle(win_x, win_y, win_w, win_h);
        let _ = cr.fill();

        // Window border (accent)
        cr.set_source_rgb(accent.0, accent.1, accent.2);
        cr.set_line_width(2.0);
        cr.rectangle(win_x, win_y, win_w, win_h);
        let _ = cr.stroke();

        // Window title bar
        cr.set_source_rgb(bg_tertiary.0, bg_tertiary.1, bg_tertiary.2);
        cr.rectangle(win_x, win_y, win_w, 24.0);
        let _ = cr.fill();

        // Title text
        cr.set_source_rgb(fg_primary.0, fg_primary.1, fg_primary.2);
        cr.select_font_face("sans-serif", gtk::cairo::FontSlant::Normal, gtk::cairo::FontWeight::Normal);
        cr.set_font_size(11.0);
        cr.move_to(win_x + 10.0, win_y + 16.0);
        let _ = cr.show_text("Terminal");

        // Terminal content mock
        let term_y = win_y + 30.0;

        // Prompt
        cr.set_source_rgb(green.0, green.1, green.2);
        cr.move_to(win_x + 10.0, term_y + 14.0);
        let _ = cr.show_text("user@vulcan");

        cr.set_source_rgb(fg_secondary.0, fg_secondary.1, fg_secondary.2);
        let _ = cr.show_text(" ~ $ ");

        cr.set_source_rgb(fg_primary.0, fg_primary.1, fg_primary.2);
        let _ = cr.show_text("ls -la");

        // Output lines
        cr.set_source_rgb(blue.0, blue.1, blue.2);
        cr.move_to(win_x + 10.0, term_y + 32.0);
        let _ = cr.show_text("drwxr-xr-x");

        cr.set_source_rgb(fg_primary.0, fg_primary.1, fg_primary.2);
        let _ = cr.show_text("  Documents/");

        cr.set_source_rgb(purple.0, purple.1, purple.2);
        cr.move_to(win_x + 10.0, term_y + 48.0);
        let _ = cr.show_text("-rw-r--r--");

        cr.set_source_rgb(fg_primary.0, fg_primary.1, fg_primary.2);
        let _ = cr.show_text("  config.toml");

        cr.set_source_rgb(red.0, red.1, red.2);
        cr.move_to(win_x + 10.0, term_y + 64.0);
        let _ = cr.show_text("-rwxr-xr-x");

        cr.set_source_rgb(accent_alt.0, accent_alt.1, accent_alt.2);
        let _ = cr.show_text("  script.sh");

        // Cursor
        cr.set_source_rgb(accent.0, accent.1, accent.2);
        cr.rectangle(win_x + 10.0, term_y + 80.0, 8.0, 14.0);
        let _ = cr.fill();
    });
}

/// Parse a hex color string into RGB floats
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
