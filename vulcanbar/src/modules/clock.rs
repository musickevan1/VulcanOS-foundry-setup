//! Clock Module
//!
//! Displays the current time with configurable format.

use anyhow::Result;
use chrono::{Local, Locale};
use crossbeam_channel::Sender;
use std::time::Duration;

use super::base::{draw_centered_text, draw_rounded_rect, get_background_color, CORNER_RADIUS};
use super::{Action, Module, ModuleEvent, RenderContext, TouchEvent};
use crate::config::ClockConfig;

/// Clock module that displays current time
pub struct ClockModule {
    name: String,
    config: ClockConfig,
    /// Cached formatted time string
    cached_time: String,
    /// Locale for formatting
    locale: Locale,
    /// Whether currently being touched
    active: bool,
    /// Whether showing alternate format (date instead of time)
    show_alt: bool,
}

impl ClockModule {
    /// Create a new clock module with default settings
    pub fn new() -> Self {
        Self::with_config(ClockConfig::default())
    }

    /// Create a clock module with specific configuration
    pub fn with_config(config: ClockConfig) -> Self {
        let locale = config
            .locale
            .as_ref()
            .and_then(|l| Locale::try_from(l.as_str()).ok())
            .unwrap_or(Locale::POSIX);

        let mut module = Self {
            name: "clock".to_string(),
            config,
            cached_time: String::new(),
            locale,
            active: false,
            show_alt: false,
        };

        // Initialize cached time
        module.update_time();
        module
    }

    /// Create from TOML configuration
    pub fn from_config(config: &toml::Value) -> Result<Box<dyn Module>> {
        let clock_config: ClockConfig = config.clone().try_into().unwrap_or_default();
        Ok(Box::new(Self::with_config(clock_config)))
    }

    /// Update the cached time string
    fn update_time(&mut self) {
        let now = Local::now();
        let format = if self.show_alt {
            // Use alternate format (typically date)
            self.config.format_alt.as_deref().unwrap_or("%A, %B %d, %Y")
        } else {
            &self.config.format
        };
        self.cached_time = now
            .format_localized(format, self.locale)
            .to_string();
    }
}

impl Module for ClockModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn width(&self) -> i32 {
        // Estimate width based on format length
        // Average character width ~20px at font size 32
        (self.cached_time.len() as i32 * 20).max(100)
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        // Draw background
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

        // Draw time text
        ctx.cairo.set_source_rgb(1.0, 1.0, 1.0);
        draw_centered_text(
            ctx.cairo,
            &self.cached_time,
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

        // Toggle between time and date on tap release
        if !event.pressed {
            self.show_alt = !self.show_alt;
            self.update_time();
        }

        None
    }

    fn update_interval(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.config.interval))
    }

    fn update(&mut self) -> Result<bool> {
        let old_time = self.cached_time.clone();
        self.update_time();
        Ok(old_time != self.cached_time)
    }
}

impl Default for ClockModule {
    fn default() -> Self {
        Self::new()
    }
}
