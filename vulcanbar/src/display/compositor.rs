//! Compositor for Touch Bar module layout
//!
//! Handles positioning modules in left/center/right zones and
//! mapping touch coordinates to the appropriate module.

use anyhow::Result;
use cairo::{Context, FontFace, ImageSurface};
use drm::control::ClipRect;
use std::collections::HashMap;

use crate::config::{GeneralConfig, LayoutConfig, VulcanBarConfig};
use crate::display::load_font_face;
use crate::modules::{Module, ModuleRegistry, RenderContext, TouchEvent, Action};

/// Position of a module on the Touch Bar
#[derive(Debug, Clone)]
struct ModulePosition {
    /// Module name/id
    name: String,
    /// X offset from left edge
    x: i32,
    /// Width allocated to this module
    width: i32,
}

/// Compositor manages module layout and rendering
pub struct Compositor {
    /// Display width in pixels
    width: i32,
    /// Display height in pixels
    height: i32,
    /// Spacing between modules
    spacing: i32,
    /// Left-aligned modules (fixed width)
    left_modules: Vec<Box<dyn Module>>,
    /// Center modules (stretch to fill)
    center_modules: Vec<Box<dyn Module>>,
    /// Right-aligned modules (fixed width)
    right_modules: Vec<Box<dyn Module>>,
    /// Calculated positions for all modules
    positions: Vec<ModulePosition>,
    /// Whether layout needs recalculation
    layout_dirty: bool,
    /// Font face for rendering text (loaded from config)
    font_face: Option<FontFace>,
}

impl Compositor {
    /// Create a new compositor with the given display dimensions
    pub fn new(width: i32, height: i32, spacing: i32) -> Self {
        Self {
            width,
            height,
            spacing,
            left_modules: Vec::new(),
            center_modules: Vec::new(),
            right_modules: Vec::new(),
            positions: Vec::new(),
            layout_dirty: true,
            font_face: None,
        }
    }

    /// Create a new compositor with a specific font
    pub fn with_font(width: i32, height: i32, spacing: i32, font_pattern: &str) -> Self {
        let font_face = load_font_face(font_pattern)
            .map_err(|e| {
                tracing::warn!("Failed to load font '{}': {:?}, using default", font_pattern, e);
            })
            .ok();

        Self {
            width,
            height,
            spacing,
            left_modules: Vec::new(),
            center_modules: Vec::new(),
            right_modules: Vec::new(),
            positions: Vec::new(),
            layout_dirty: true,
            font_face,
        }
    }

    /// Create a compositor from configuration
    pub fn from_config(
        config: &VulcanBarConfig,
        registry: &ModuleRegistry,
        width: i32,
        height: i32,
    ) -> Result<Self> {
        let mut compositor = Self::with_font(width, height, config.general.spacing, &config.general.font);

        // Create left modules
        for name in &config.layout.left {
            let module_config = config.modules.get(name)
                .cloned()
                .unwrap_or(toml::Value::Table(toml::map::Map::new()));
            if let Ok(module) = registry.create(name, &module_config) {
                compositor.left_modules.push(module);
            }
        }

        // Create center modules
        for name in &config.layout.center {
            let module_config = config.modules.get(name)
                .cloned()
                .unwrap_or(toml::Value::Table(toml::map::Map::new()));
            if let Ok(module) = registry.create(name, &module_config) {
                compositor.center_modules.push(module);
            }
        }

        // Create right modules
        for name in &config.layout.right {
            let module_config = config.modules.get(name)
                .cloned()
                .unwrap_or(toml::Value::Table(toml::map::Map::new()));
            if let Ok(module) = registry.create(name, &module_config) {
                compositor.right_modules.push(module);
            }
        }

        compositor.calculate_layout();
        Ok(compositor)
    }

    /// Create a compositor from a layout config (for multi-page mode)
    pub fn from_layout_config(
        layout: &LayoutConfig,
        general: &GeneralConfig,
        modules: &HashMap<String, toml::Value>,
        registry: &ModuleRegistry,
        width: i32,
        height: i32,
    ) -> Result<Self> {
        let mut compositor = Self::with_font(width, height, general.spacing, &general.font);

        // Create left modules
        for name in &layout.left {
            let module_config = modules.get(name)
                .cloned()
                .unwrap_or(toml::Value::Table(toml::map::Map::new()));
            if let Ok(module) = registry.create(name, &module_config) {
                compositor.left_modules.push(module);
            }
        }

        // Create center modules
        for name in &layout.center {
            let module_config = modules.get(name)
                .cloned()
                .unwrap_or(toml::Value::Table(toml::map::Map::new()));
            if let Ok(module) = registry.create(name, &module_config) {
                compositor.center_modules.push(module);
            }
        }

        // Create right modules
        for name in &layout.right {
            let module_config = modules.get(name)
                .cloned()
                .unwrap_or(toml::Value::Table(toml::map::Map::new()));
            if let Ok(module) = registry.create(name, &module_config) {
                compositor.right_modules.push(module);
            }
        }

        compositor.calculate_layout();
        Ok(compositor)
    }

    /// Calculate positions for all modules
    fn calculate_layout(&mut self) {
        self.positions.clear();

        // Calculate total fixed width for left and right
        let left_width: i32 = self.left_modules.iter()
            .map(|m| m.width().max(60)) // Minimum 60px per module
            .sum::<i32>()
            + (self.left_modules.len().saturating_sub(1) as i32 * self.spacing);

        let right_width: i32 = self.right_modules.iter()
            .map(|m| m.width().max(60))
            .sum::<i32>()
            + (self.right_modules.len().saturating_sub(1) as i32 * self.spacing);

        // Center gets remaining space
        let center_width = (self.width - left_width - right_width - self.spacing * 2).max(0);

        // Position left modules
        let mut x = 0;
        for module in &self.left_modules {
            let w = module.width().max(60);
            self.positions.push(ModulePosition {
                name: module.name().to_string(),
                x,
                width: w,
            });
            x += w + self.spacing;
        }

        // Position center modules (stretch to fill)
        let center_start = left_width + self.spacing;
        if !self.center_modules.is_empty() {
            let per_module = center_width / self.center_modules.len() as i32;
            let mut cx = center_start;
            for module in &self.center_modules {
                let w = if module.width() == 0 {
                    per_module // Stretch
                } else {
                    module.width().max(60)
                };
                self.positions.push(ModulePosition {
                    name: module.name().to_string(),
                    x: cx,
                    width: w,
                });
                cx += w + self.spacing;
            }
        }

        // Position right modules (right-aligned)
        let mut rx = self.width;
        for module in self.right_modules.iter().rev() {
            let w = module.width().max(60);
            rx -= w;
            self.positions.push(ModulePosition {
                name: module.name().to_string(),
                x: rx,
                width: w,
            });
            rx -= self.spacing;
        }

        self.layout_dirty = false;
    }

    /// Render all modules to the surface
    pub fn render(
        &self,
        surface: &ImageSurface,
        config: &GeneralConfig,
        pixel_shift: (f64, f64),
        complete_redraw: bool,
    ) -> Result<Vec<ClipRect>> {
        let ctx = Context::new(surface)?;
        let mut clips = Vec::new();

        // Rotate 90 degrees for Touch Bar orientation
        ctx.translate(self.height as f64, 0.0);
        ctx.rotate((90.0f64).to_radians());

        // Clear if complete redraw
        if complete_redraw {
            ctx.set_source_rgb(0.0, 0.0, 0.0);
            ctx.paint()?;
            clips.push(ClipRect::new(0, 0, self.height as u16, self.width as u16));
        }

        // Set up font - CRITICAL: must set font_face for Nerd Font icons to render
        if let Some(ref font_face) = self.font_face {
            ctx.set_font_face(font_face);
        }
        ctx.set_font_size(config.font_size);

        let (shift_x, shift_y) = pixel_shift;

        // Render each module
        let all_modules = self.left_modules.iter()
            .chain(self.center_modules.iter())
            .chain(self.right_modules.iter());

        for module in all_modules {
            if let Some(pos) = self.positions.iter().find(|p| p.name == module.name()) {
                let render_ctx = RenderContext {
                    cairo: &ctx,
                    width: pos.width,
                    height: self.height,
                    x_offset: pos.x as f64 + shift_x,
                    y_offset: shift_y,
                    show_outlines: config.show_button_outlines,
                    is_active: false,
                };

                if let Err(e) = module.render(&render_ctx) {
                    eprintln!("Error rendering module {}: {}", module.name(), e);
                }
            }
        }

        Ok(clips)
    }

    /// Handle touch event and route to appropriate module
    pub fn handle_touch(&mut self, x: f64, y: f64, pressed: bool, slot: i32) -> Option<Action> {
        // Find which module was touched
        for pos in &self.positions {
            if x >= pos.x as f64 && x < (pos.x + pos.width) as f64 {
                let event = TouchEvent {
                    x: x - pos.x as f64,
                    y,
                    pressed,
                    slot,
                };

                // Find and call the module
                for module in self.left_modules.iter_mut()
                    .chain(self.center_modules.iter_mut())
                    .chain(self.right_modules.iter_mut())
                {
                    if module.name() == pos.name {
                        return module.on_touch(event);
                    }
                }
            }
        }
        None
    }

    /// Update all modules that need polling
    pub fn update_modules(&mut self) -> Result<bool> {
        let mut any_changed = false;

        for module in self.left_modules.iter_mut()
            .chain(self.center_modules.iter_mut())
            .chain(self.right_modules.iter_mut())
        {
            if let Ok(changed) = module.update() {
                any_changed = any_changed || changed;
            }
        }

        Ok(any_changed)
    }

    /// Get the minimum update interval across all modules
    pub fn min_update_interval(&self) -> Option<std::time::Duration> {
        self.left_modules.iter()
            .chain(self.center_modules.iter())
            .chain(self.right_modules.iter())
            .filter_map(|m| m.update_interval())
            .min()
    }

    /// Get number of modules
    pub fn module_count(&self) -> usize {
        self.left_modules.len() + self.center_modules.len() + self.right_modules.len()
    }
}
