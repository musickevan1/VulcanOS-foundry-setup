//! Brightness Module
//!
//! Displays screen brightness level.

use anyhow::Result;
use crossbeam_channel::Sender;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use super::base::{draw_centered_text, draw_rounded_rect, get_background_color, CORNER_RADIUS};
use super::{Action, Module, ModuleEvent, RenderContext, TouchEvent};
use crate::config::BrightnessConfig;

/// Brightness module that displays screen brightness
pub struct BrightnessModule {
    name: String,
    config: BrightnessConfig,
    /// Path to backlight device
    backlight_path: Option<PathBuf>,
    /// Maximum brightness value
    max_brightness: u32,
    /// Current brightness percentage
    percentage: u32,
    /// Whether currently being touched
    active: bool,
}

impl BrightnessModule {
    /// Create a new brightness module with default settings
    pub fn new() -> Self {
        Self::with_config(BrightnessConfig::default())
    }

    /// Create a brightness module with specific configuration
    pub fn with_config(config: BrightnessConfig) -> Self {
        let backlight_path = find_backlight_device();
        let max_brightness = backlight_path
            .as_ref()
            .and_then(|p| read_brightness_value(&p.join("max_brightness")))
            .unwrap_or(100);

        let mut module = Self {
            name: "brightness".to_string(),
            config,
            backlight_path,
            max_brightness,
            percentage: 100,
            active: false,
        };

        // Initialize brightness
        let _ = module.update();
        module
    }

    /// Create from TOML configuration
    pub fn from_config(config: &toml::Value) -> Result<Box<dyn Module>> {
        let brightness_config: BrightnessConfig = config.clone().try_into().unwrap_or_default();
        Ok(Box::new(Self::with_config(brightness_config)))
    }

    /// Read current brightness percentage
    fn read_percentage(&self) -> Option<u32> {
        let path = self.backlight_path.as_ref()?;
        let current = read_brightness_value(&path.join("brightness"))?;

        if self.max_brightness > 0 {
            Some(((current as f64 / self.max_brightness as f64) * 100.0).round() as u32)
        } else {
            Some(100)
        }
    }

    /// Get brightness icon based on percentage
    fn get_brightness_icon(&self) -> &'static str {
        match self.percentage {
            0..=25 => "󰃞",
            26..=50 => "󰃟",
            51..=75 => "󰃠",
            _ => "󰃡",
        }
    }

    /// Get display text based on config
    fn get_display_text(&self) -> String {
        match self.config.display.as_str() {
            "icon" => self.get_brightness_icon().to_string(),
            "percentage" => format!("{}%", self.percentage),
            "both" | _ => format!("{} {}%", self.get_brightness_icon(), self.percentage),
        }
    }
}

impl Module for BrightnessModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn width(&self) -> i32 {
        match self.config.display.as_str() {
            "icon" => 60,
            "percentage" => 80,
            _ => 120,
        }
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

        // Draw brightness text/icon
        ctx.cairo.set_source_rgb(1.0, 1.0, 1.0);
        let text = self.get_display_text();
        draw_centered_text(
            ctx.cairo,
            &text,
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
        None // Brightness has no touch action (could add adjustment later)
    }

    fn update_interval(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.config.interval))
    }

    fn update(&mut self) -> Result<bool> {
        let old_percentage = self.percentage;
        self.percentage = self.read_percentage().unwrap_or(100);
        Ok(old_percentage != self.percentage)
    }
}

impl Default for BrightnessModule {
    fn default() -> Self {
        Self::new()
    }
}

/// Find the display backlight device path
fn find_backlight_device() -> Option<PathBuf> {
    let backlight_path = "/sys/class/backlight";

    // Priority list for different Mac types
    let priorities = [
        "apple-panel-bl",    // Apple Silicon
        "gmux_backlight",    // T2 Mac with discrete GPU
        "intel_backlight",   // Intel integrated
        "acpi_video0",       // Generic fallback
    ];

    // Try priority devices first
    for &device in &priorities {
        let path = PathBuf::from(backlight_path).join(device);
        if path.exists() {
            return Some(path);
        }
    }

    // Fall back to first available device
    if let Ok(entries) = fs::read_dir(backlight_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.join("brightness").exists() {
                return Some(path);
            }
        }
    }

    None
}

/// Read a brightness value from a sysfs file
fn read_brightness_value(path: &PathBuf) -> Option<u32> {
    fs::read_to_string(path)
        .ok()?
        .trim()
        .parse::<u32>()
        .ok()
}
