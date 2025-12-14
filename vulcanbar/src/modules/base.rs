//! Base module utilities and helpers
//!
//! Provides common rendering functions and utilities that modules can use.

use anyhow::Result;
use cairo::{Context, FontFace};
use librsvg_rebind::{prelude::HandleExt, Handle, Rectangle};

/// Default button styling constants
pub const BUTTON_COLOR_INACTIVE: f64 = 0.200;
pub const BUTTON_COLOR_ACTIVE: f64 = 0.400;
pub const ICON_SIZE: i32 = 48;
pub const CORNER_RADIUS: f64 = 8.0;
pub const VERTICAL_PADDING: f64 = 0.15; // 15% from top/bottom

/// Draw a rounded rectangle background for a module
pub fn draw_rounded_rect(
    ctx: &Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    radius: f64,
    color: (f64, f64, f64),
) {
    let (r, g, b) = color;
    ctx.set_source_rgb(r, g, b);

    let top = y + height * VERTICAL_PADDING;
    let bottom = y + height * (1.0 - VERTICAL_PADDING);
    let left = x + radius;
    let right = x + width - radius;

    ctx.new_sub_path();
    ctx.arc(
        right,
        top + radius,
        radius,
        (-90.0f64).to_radians(),
        (0.0f64).to_radians(),
    );
    ctx.arc(
        right,
        bottom - radius,
        radius,
        (0.0f64).to_radians(),
        (90.0f64).to_radians(),
    );
    ctx.arc(
        left,
        bottom - radius,
        radius,
        (90.0f64).to_radians(),
        (180.0f64).to_radians(),
    );
    ctx.arc(
        left,
        top + radius,
        radius,
        (180.0f64).to_radians(),
        (270.0f64).to_radians(),
    );
    ctx.close_path();
    ctx.fill().unwrap();
}

/// Get the background color based on module state
pub fn get_background_color(is_active: bool, show_outlines: bool) -> (f64, f64, f64) {
    if is_active {
        (BUTTON_COLOR_ACTIVE, BUTTON_COLOR_ACTIVE, BUTTON_COLOR_ACTIVE)
    } else if show_outlines {
        (BUTTON_COLOR_INACTIVE, BUTTON_COLOR_INACTIVE, BUTTON_COLOR_INACTIVE)
    } else {
        (0.0, 0.0, 0.0)
    }
}

/// Get a colored background (for battery states, etc.)
pub fn get_colored_background(
    is_active: bool,
    show_outlines: bool,
    color_type: ColorType,
) -> (f64, f64, f64) {
    let base = if is_active {
        BUTTON_COLOR_ACTIVE
    } else if show_outlines {
        BUTTON_COLOR_INACTIVE
    } else {
        0.0
    };

    match color_type {
        ColorType::Normal => (base, base, base),
        ColorType::Green => (0.0, base, 0.0),
        ColorType::Red => (base, 0.0, 0.0),
        ColorType::Yellow => (base, base, 0.0),
    }
}

/// Color types for module backgrounds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
    Normal,
    Green,
    Red,
    Yellow,
}

/// Draw centered text in a module
pub fn draw_centered_text(
    ctx: &Context,
    text: &str,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    y_shift: f64,
) -> Result<()> {
    let extents = ctx.text_extents(text)?;
    ctx.move_to(
        x + (width / 2.0 - extents.width() / 2.0).round(),
        y_shift + (height / 2.0 + extents.height() / 2.0).round(),
    );
    ctx.show_text(text)?;
    Ok(())
}

/// Draw a centered SVG icon in a module
pub fn draw_centered_svg(
    ctx: &Context,
    svg: &Handle,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    y_shift: f64,
) -> Result<()> {
    let icon_x = x + (width / 2.0 - (ICON_SIZE / 2) as f64).round();
    let icon_y = y_shift + ((height - ICON_SIZE as f64) / 2.0).round();

    svg.render_document(
        ctx,
        &Rectangle::new(icon_x, icon_y, ICON_SIZE as f64, ICON_SIZE as f64),
    )?;
    Ok(())
}

/// Set up the cairo context with the given font
pub fn setup_font(ctx: &Context, font: &FontFace, size: f64) {
    ctx.set_font_face(font);
    ctx.set_font_size(size);
}

/// Clear a rectangular region to black
pub fn clear_region(ctx: &Context, x: f64, y: f64, width: f64, height: f64) {
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.rectangle(x, y, width, height);
    ctx.fill().unwrap();
}

/// Check if a point is within the touchable area of a module
pub fn point_in_touchable_area(
    x: f64,
    y: f64,
    module_x: f64,
    module_width: f64,
    module_height: f64,
) -> bool {
    let touchable_top = module_height * 0.1;
    let touchable_bottom = module_height * 0.9;

    x >= module_x
        && x <= module_x + module_width
        && y >= touchable_top
        && y <= touchable_bottom
}
