//! System Tray Module
//!
//! Displays a tray icon with notification badge count.
//! Tapping expands to show tray items.

use anyhow::Result;
use crossbeam_channel::Sender;
use std::time::Duration;

use super::base::{draw_centered_text, draw_rounded_rect, get_background_color, CORNER_RADIUS};
use super::{Action, Module, ModuleEvent, RenderContext, TouchEvent};

/// Tray module configuration
#[derive(Debug, Clone)]
pub struct TrayConfig {
    /// Update interval in seconds
    pub interval: u64,
    /// Icon to display
    pub icon: String,
    /// Page to switch to on tap
    pub on_click_page: Option<String>,
}

impl Default for TrayConfig {
    fn default() -> Self {
        Self {
            interval: 5,
            icon: "󰏗".to_string(),  // System tray icon
            on_click_page: None,    // No page switch by default (tray page not implemented)
        }
    }
}

/// Tray module
pub struct TrayModule {
    name: String,
    config: TrayConfig,
    /// Number of notification badges / tray items
    badge_count: usize,
    active: bool,
}

impl TrayModule {
    pub fn new() -> Self {
        Self::with_config(TrayConfig::default())
    }

    pub fn with_config(config: TrayConfig) -> Self {
        Self {
            name: "tray".to_string(),
            config,
            badge_count: 0,
            active: false,
        }
    }

    pub fn from_config(config: &toml::Value) -> Result<Box<dyn Module>> {
        let mut tray_config = TrayConfig::default();

        if let Some(table) = config.as_table() {
            if let Some(v) = table.get("interval").and_then(|v| v.as_integer()) {
                tray_config.interval = v as u64;
            }
            if let Some(v) = table.get("icon").and_then(|v| v.as_str()) {
                tray_config.icon = v.to_string();
            }
            if let Some(v) = table.get("on-click-page").and_then(|v| v.as_str()) {
                tray_config.on_click_page = Some(v.to_string());
            }
        }

        Ok(Box::new(Self::with_config(tray_config)))
    }

    fn format_display(&self) -> String {
        if self.badge_count > 0 {
            format!("{} {}", self.config.icon, self.badge_count)
        } else {
            self.config.icon.clone()
        }
    }
}

impl Module for TrayModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn width(&self) -> i32 {
        if self.badge_count > 0 {
            70
        } else {
            50
        }
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        let color = get_background_color(self.active, ctx.show_outlines);
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
            &self.format_display(),
            ctx.x_offset,
            0.0,
            ctx.width as f64,
            ctx.height as f64,
            ctx.y_offset,
        )?;

        // Draw badge dot if there are items
        if self.badge_count > 0 {
            ctx.cairo.set_source_rgb(1.0, 0.3, 0.3); // Red badge
            let badge_x = ctx.x_offset + ctx.width as f64 - 12.0;
            let badge_y = 10.0;
            ctx.cairo.arc(badge_x, badge_y, 5.0, 0.0, 2.0 * std::f64::consts::PI);
            ctx.cairo.fill()?;
        }

        Ok(())
    }

    fn on_touch(&mut self, event: TouchEvent) -> Option<Action> {
        self.active = event.pressed;

        if !event.pressed {
            if let Some(ref page) = self.config.on_click_page {
                return Some(Action::SwitchToPage(page.clone()));
            }
        }
        None
    }

    fn update_interval(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.config.interval))
    }

    fn update(&mut self) -> Result<bool> {
        // TODO: Implement actual tray item counting via D-Bus StatusNotifierWatcher
        // For now, this is a placeholder
        Ok(false)
    }

    fn start_listener(&mut self, _tx: Sender<ModuleEvent>) -> Result<Option<i32>> {
        Ok(None)
    }
}

impl Default for TrayModule {
    fn default() -> Self {
        Self::new()
    }
}
