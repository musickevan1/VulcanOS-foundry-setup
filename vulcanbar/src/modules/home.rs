//! Home Button Module
//!
//! A simple navigation button that returns to the main page.

use anyhow::Result;
use crossbeam_channel::Sender;
use std::time::Duration;

use super::base::{draw_centered_text, draw_rounded_rect, get_background_color, CORNER_RADIUS};
use super::{Action, Module, ModuleEvent, RenderContext, TouchEvent};

/// Home button configuration
#[derive(Debug, Clone)]
pub struct HomeConfig {
    /// Icon to display
    pub icon: String,
    /// Target page (default: "main")
    pub target_page: String,
}

impl Default for HomeConfig {
    fn default() -> Self {
        Self {
            icon: "󰋜".to_string(),
            target_page: "main".to_string(),
        }
    }
}

/// Home button module - navigates back to main page
pub struct HomeModule {
    name: String,
    config: HomeConfig,
    active: bool,
}

impl HomeModule {
    pub fn new() -> Self {
        Self::with_config(HomeConfig::default())
    }

    pub fn with_config(config: HomeConfig) -> Self {
        Self {
            name: "home".to_string(),
            config,
            active: false,
        }
    }

    pub fn from_config(config: &toml::Value) -> Result<Box<dyn Module>> {
        let mut home_config = HomeConfig::default();

        if let Some(table) = config.as_table() {
            if let Some(v) = table.get("icon").and_then(|v| v.as_str()) {
                home_config.icon = v.to_string();
            }
            if let Some(v) = table.get("target-page").and_then(|v| v.as_str()) {
                home_config.target_page = v.to_string();
            }
        }

        Ok(Box::new(Self::with_config(home_config)))
    }
}

impl Module for HomeModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn width(&self) -> i32 {
        60  // Slightly wider for icon
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        // Use a brighter color when active to show press feedback
        let color = if self.active {
            (0.4, 0.4, 0.4)  // Bright when pressed
        } else {
            get_background_color(false, ctx.show_outlines)
        };

        draw_rounded_rect(
            ctx.cairo,
            ctx.x_offset,
            0.0,
            ctx.width as f64,
            ctx.height as f64,
            CORNER_RADIUS,
            color,
        );

        ctx.cairo.set_source_rgb(1.0, 1.0, 1.0);
        draw_centered_text(
            ctx.cairo,
            &self.config.icon,
            ctx.x_offset,
            0.0,
            ctx.width as f64,
            ctx.height as f64,
            ctx.y_offset,
        )?;

        Ok(())
    }

    fn on_touch(&mut self, event: TouchEvent) -> Option<Action> {
        self.active = event.pressed;

        if !event.pressed {
            // On release, go to target page
            return Some(Action::SwitchToPage(self.config.target_page.clone()));
        }
        None
    }

    fn update_interval(&self) -> Option<Duration> {
        None  // Static module, no updates needed
    }

    fn update(&mut self) -> Result<bool> {
        Ok(false)  // Never changes
    }

    fn start_listener(&mut self, _tx: Sender<ModuleEvent>) -> Result<Option<i32>> {
        Ok(None)
    }
}

impl Default for HomeModule {
    fn default() -> Self {
        Self::new()
    }
}
