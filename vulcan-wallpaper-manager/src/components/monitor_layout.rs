use gtk::prelude::*;
use relm4::prelude::*;
use relm4::gtk::glib::clone;
use std::cell::RefCell;
use std::rc::Rc;

use crate::models::Monitor;

#[derive(Debug)]
pub enum MonitorLayoutInput {
    UpdateMonitors(Vec<Monitor>),
    Click(f64, f64),
}

#[derive(Debug)]
pub enum MonitorLayoutOutput {
    Selected(String),
}

pub struct MonitorLayoutModel {
    monitors: Rc<RefCell<Vec<Monitor>>>,
    selected: Option<String>,
}

#[relm4::component(pub)]
impl SimpleComponent for MonitorLayoutModel {
    type Init = Vec<Monitor>;
    type Input = MonitorLayoutInput;
    type Output = MonitorLayoutOutput;

    view! {
        #[name = "drawing_area"]
        gtk::DrawingArea {
            set_content_width: 600,
            set_content_height: 300,
            set_hexpand: true,
            set_vexpand: true,

            #[watch]
            set_draw_func: {
                let monitors = model.monitors.clone();
                let selected = model.selected.clone();
                move |_area, cr, width, height| {
                    draw_monitors(cr, width, height, &monitors.borrow(), selected.as_deref());
                }
            },
        }
    }

    fn init(
        monitors: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = MonitorLayoutModel {
            monitors: Rc::new(RefCell::new(monitors)),
            selected: None,
        };

        let widgets = view_output!();

        // Add click gesture controller
        let gesture = gtk::GestureClick::new();
        gesture.connect_pressed(clone!(
            #[strong]
            sender,
            move |gesture, _n, x, y| {
                gesture.set_state(gtk::EventSequenceState::Claimed);
                sender.input(MonitorLayoutInput::Click(x, y));
            }
        ));
        widgets.drawing_area.add_controller(gesture);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            MonitorLayoutInput::UpdateMonitors(monitors) => {
                *self.monitors.borrow_mut() = monitors;
            }
            MonitorLayoutInput::Click(x, y) => {
                // Find which monitor was clicked
                if let Some(name) = find_monitor_at(&self.monitors.borrow(), x, y, 600.0, 300.0) {
                    self.selected = Some(name.clone());
                    let _ = sender.output(MonitorLayoutOutput::Selected(name));
                }
            }
        }
    }
}

/// Calculate scale factor to fit all monitors in the drawing area
fn calculate_scale(monitors: &[Monitor], width: f64, height: f64) -> (f64, f64, f64) {
    if monitors.is_empty() {
        return (1.0, 0.0, 0.0);
    }

    // Find bounding box of all monitors
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for mon in monitors {
        let (lw, lh) = mon.logical_size();
        let w = if mon.is_vertical() { lh } else { lw };
        let h = if mon.is_vertical() { lw } else { lh };

        min_x = min_x.min(mon.x);
        min_y = min_y.min(mon.y);
        max_x = max_x.max(mon.x + w as i32);
        max_y = max_y.max(mon.y + h as i32);
    }

    let total_width = (max_x - min_x) as f64;
    let total_height = (max_y - min_y) as f64;

    // Calculate scale to fit with padding
    let padding = 40.0;
    let available_width = width - padding * 2.0;
    let available_height = height - padding * 2.0;

    let scale_x = available_width / total_width;
    let scale_y = available_height / total_height;
    let scale = scale_x.min(scale_y);

    // Calculate offset to center the layout
    let offset_x = (width - total_width * scale) / 2.0 - min_x as f64 * scale;
    let offset_y = (height - total_height * scale) / 2.0 - min_y as f64 * scale;

    (scale, offset_x, offset_y)
}

/// Draw all monitors on the cairo context
fn draw_monitors(
    cr: &gtk::cairo::Context,
    width: i32,
    height: i32,
    monitors: &[Monitor],
    selected: Option<&str>,
) {
    let width = width as f64;
    let height = height as f64;

    // Clear background
    cr.set_source_rgb(0.15, 0.15, 0.18);
    cr.paint().ok();

    let (scale, offset_x, offset_y) = calculate_scale(monitors, width, height);

    for mon in monitors {
        let (lw, lh) = mon.logical_size();
        let (w, h) = if mon.is_vertical() { (lh, lw) } else { (lw, lh) };

        let x = mon.x as f64 * scale + offset_x;
        let y = mon.y as f64 * scale + offset_y;
        let w = w * scale;
        let h = h * scale;

        // Draw monitor rectangle
        cr.rectangle(x, y, w, h);

        // Fill color based on selection
        if selected == Some(&mon.name) {
            cr.set_source_rgb(0.4, 0.6, 0.9); // Selected: blue
        } else {
            cr.set_source_rgb(0.25, 0.28, 0.35); // Normal: dark gray
        }
        cr.fill_preserve().ok();

        // Border
        cr.set_source_rgb(0.5, 0.55, 0.65);
        cr.set_line_width(2.0);
        cr.stroke().ok();

        // Monitor name label
        cr.set_source_rgb(0.9, 0.9, 0.9);
        cr.set_font_size(12.0);

        let text = &mon.name;
        let extents = cr.text_extents(text).unwrap();
        let text_x = x + (w - extents.width()) / 2.0;
        let text_y = y + (h + extents.height()) / 2.0;

        cr.move_to(text_x, text_y);
        cr.show_text(text).ok();

        // Resolution label below name
        let res_text = format!("{}x{}", mon.width, mon.height);
        cr.set_font_size(10.0);
        let res_extents = cr.text_extents(&res_text).unwrap();
        let res_x = x + (w - res_extents.width()) / 2.0;
        let res_y = text_y + 14.0;

        cr.move_to(res_x, res_y);
        cr.show_text(&res_text).ok();
    }
}

/// Find which monitor contains the given point
fn find_monitor_at(
    monitors: &[Monitor],
    click_x: f64,
    click_y: f64,
    width: f64,
    height: f64,
) -> Option<String> {
    let (scale, offset_x, offset_y) = calculate_scale(monitors, width, height);

    for mon in monitors {
        let (lw, lh) = mon.logical_size();
        let (w, h) = if mon.is_vertical() { (lh, lw) } else { (lw, lh) };

        let x = mon.x as f64 * scale + offset_x;
        let y = mon.y as f64 * scale + offset_y;
        let w = w * scale;
        let h = h * scale;

        if click_x >= x && click_x <= x + w && click_y >= y && click_y <= y + h {
            return Some(mon.name.clone());
        }
    }

    None
}
