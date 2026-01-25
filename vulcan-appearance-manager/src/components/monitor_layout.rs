use gtk::prelude::*;
use relm4::prelude::*;
use relm4::gtk::glib::clone;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::models::Monitor;

#[derive(Debug)]
pub enum MonitorLayoutInput {
    UpdateMonitors(Vec<Monitor>),
    UpdateWallpapers(HashMap<String, PathBuf>),
    Click(f64, f64, f64, f64), // click_x, click_y, widget_width, widget_height
}

#[derive(Debug)]
pub enum MonitorLayoutOutput {
    Selected(String),
}

pub struct MonitorLayoutModel {
    monitors: Rc<RefCell<Vec<Monitor>>>,
    selected: Rc<RefCell<Option<String>>>,
    /// Map of monitor name -> assigned wallpaper path
    wallpapers: Rc<RefCell<HashMap<String, PathBuf>>>,
    /// Drawing area reference for manual redraws
    drawing_area: Rc<RefCell<Option<gtk::DrawingArea>>>,
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
        }
    }

    fn init(
        monitors: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let monitors_rc = Rc::new(RefCell::new(monitors));
        let selected_rc = Rc::new(RefCell::new(None::<String>));
        let wallpapers_rc = Rc::new(RefCell::new(HashMap::new()));
        let drawing_area_rc = Rc::new(RefCell::new(None::<gtk::DrawingArea>));

        let model = MonitorLayoutModel {
            monitors: monitors_rc.clone(),
            selected: selected_rc.clone(),
            wallpapers: wallpapers_rc.clone(),
            drawing_area: drawing_area_rc.clone(),
        };

        let widgets = view_output!();

        // Store drawing area reference
        *drawing_area_rc.borrow_mut() = Some(widgets.drawing_area.clone());

        // Set up draw function with cloned Rcs
        let monitors_for_draw = monitors_rc.clone();
        let selected_for_draw = selected_rc.clone();
        let wallpapers_for_draw = wallpapers_rc.clone();
        widgets.drawing_area.set_draw_func(move |_area, cr, width, height| {
            draw_monitors(
                cr, width, height,
                &monitors_for_draw.borrow(),
                selected_for_draw.borrow().as_deref(),
                &wallpapers_for_draw.borrow()
            );
        });

        // Add click gesture controller
        let gesture = gtk::GestureClick::new();
        let drawing_area_ref = widgets.drawing_area.clone();
        gesture.connect_pressed(clone!(
            #[strong]
            sender,
            #[strong]
            drawing_area_ref,
            move |gesture, _n, x, y| {
                gesture.set_state(gtk::EventSequenceState::Claimed);
                // Get actual widget dimensions for accurate hit testing
                let width = drawing_area_ref.width() as f64;
                let height = drawing_area_ref.height() as f64;
                sender.input(MonitorLayoutInput::Click(x, y, width, height));
            }
        ));
        widgets.drawing_area.add_controller(gesture);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            MonitorLayoutInput::UpdateMonitors(monitors) => {
                *self.monitors.borrow_mut() = monitors;
                self.queue_draw();
            }
            MonitorLayoutInput::UpdateWallpapers(wallpapers) => {
                *self.wallpapers.borrow_mut() = wallpapers;
                self.queue_draw();
            }
            MonitorLayoutInput::Click(x, y, width, height) => {
                // Find which monitor was clicked using actual widget dimensions
                if let Some(name) = find_monitor_at(&self.monitors.borrow(), x, y, width, height) {
                    *self.selected.borrow_mut() = Some(name.clone());
                    self.queue_draw();
                    let _ = sender.output(MonitorLayoutOutput::Selected(name));
                }
            }
        }
    }
}

impl MonitorLayoutModel {
    fn queue_draw(&self) {
        if let Some(ref area) = *self.drawing_area.borrow() {
            area.queue_draw();
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
    wallpapers: &HashMap<String, PathBuf>,
) {
    let width = width as f64;
    let height = height as f64;

    // Clear background - Vulcan obsidian
    cr.set_source_rgb(0.11, 0.098, 0.09); // #1c1917
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

        // Fill color based on selection and wallpaper assignment
        let has_wallpaper = wallpapers.contains_key(&mon.name);

        if selected == Some(&mon.name) {
            // Selected monitor - bright ember with border highlight
            cr.set_source_rgb(0.976, 0.451, 0.086); // Vulcan ember #f97316
        } else if has_wallpaper {
            // Has wallpaper assigned - dimmer ember
            cr.set_source_rgb(0.918, 0.345, 0.047); // Vulcan molten #ea580c
        } else {
            // No wallpaper - dark charcoal
            cr.set_source_rgb(0.16, 0.15, 0.14); // Vulcan charcoal #292524
        }
        cr.fill_preserve().ok();

        // Border - highlight selected with thicker ember border
        if selected == Some(&mon.name) {
            cr.set_source_rgb(0.984, 0.749, 0.141); // Vulcan gold #fbbf24
            cr.set_line_width(3.0);
        } else {
            cr.set_source_rgb(0.267, 0.251, 0.235); // Vulcan ash #44403c
            cr.set_line_width(2.0);
        }
        cr.stroke().ok();

        // Monitor name label
        cr.set_source_rgb(0.98, 0.98, 0.97); // Vulcan white #fafaf9
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
