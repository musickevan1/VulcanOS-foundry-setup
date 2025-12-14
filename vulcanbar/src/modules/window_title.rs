//! Window Title Module
//!
//! Displays the active window title from Hyprland.

use anyhow::Result;
use crossbeam_channel::Sender;
use std::time::Duration;

use super::base::{draw_rounded_rect, get_background_color, CORNER_RADIUS};
use super::{Action, Module, ModuleEvent, RenderContext, TouchEvent};

/// Window title module configuration
#[derive(Debug, Clone)]
pub struct WindowTitleConfig {
    /// Maximum length of title
    pub max_length: usize,
    /// Update interval in seconds
    pub interval: u64,
    /// Icon mappings for specific apps
    pub rewrite_rules: Vec<(String, String)>,
}

impl Default for WindowTitleConfig {
    fn default() -> Self {
        Self {
            max_length: 50,
            interval: 1,
            rewrite_rules: vec![
                ("Mozilla Firefox".to_string(), "󰈹 ".to_string()),
                ("Visual Studio Code".to_string(), "󰨞 ".to_string()),
                ("Alacritty".to_string(), " ".to_string()),
                ("kitty".to_string(), " ".to_string()),
                ("Thunar".to_string(), "󰉋 ".to_string()),
                ("Discord".to_string(), "󰙯 ".to_string()),
                ("Spotify".to_string(), "󰓇 ".to_string()),
            ],
        }
    }
}

/// Window title module
pub struct WindowTitleModule {
    name: String,
    config: WindowTitleConfig,
    /// Current window title
    title: String,
    /// Whether currently being touched
    active: bool,
}

impl WindowTitleModule {
    pub fn new() -> Self {
        Self::with_config(WindowTitleConfig::default())
    }

    pub fn with_config(config: WindowTitleConfig) -> Self {
        let mut module = Self {
            name: "window-title".to_string(),
            config,
            title: String::new(),
            active: false,
        };
        let _ = module.update();
        module
    }

    pub fn from_config(config: &toml::Value) -> Result<Box<dyn Module>> {
        let mut wt_config = WindowTitleConfig::default();

        if let Some(table) = config.as_table() {
            if let Some(v) = table.get("max-length").and_then(|v| v.as_integer()) {
                wt_config.max_length = v as usize;
            }
            if let Some(v) = table.get("interval").and_then(|v| v.as_integer()) {
                wt_config.interval = v as u64;
            }
        }

        Ok(Box::new(Self::with_config(wt_config)))
    }

    /// Get active window title from Hyprland
    fn get_active_window(&self) -> String {
        // Use hyprctl to get active window title
        let output = std::process::Command::new("hyprctl")
            .args(["activewindow"])
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);

                // Debug logging
                if !stderr.is_empty() {
                    eprintln!("[window-title] hyprctl stderr: {}", stderr.trim());
                }

                // Parse plain text output - look for "title:" line
                for line in stdout.lines() {
                    if let Some(title) = line.strip_prefix("title: ") {
                        let result = self.apply_rewrites(title.trim());
                        eprintln!("[window-title] Got title: {}", result);
                        return result;
                    }
                }
                eprintln!("[window-title] No title found in output");
                String::new()
            }
            Err(e) => {
                eprintln!("[window-title] hyprctl error: {}", e);
                String::new()
            }
        }
    }

    /// Apply rewrite rules to add icons
    fn apply_rewrites(&self, title: &str) -> String {
        for (pattern, prefix) in &self.config.rewrite_rules {
            if title.contains(pattern) {
                // Remove the pattern from title and add icon prefix
                let cleaned = title.replace(&format!(" — {}", pattern), "")
                    .replace(&format!(" - {}", pattern), "");
                return format!("{}{}", prefix, cleaned);
            }
        }
        title.to_string()
    }

    /// Truncate title to max length
    fn truncate(&self, title: &str) -> String {
        if title.chars().count() > self.config.max_length {
            format!("{}…", title.chars().take(self.config.max_length - 1).collect::<String>())
        } else {
            title.to_string()
        }
    }
}

impl Module for WindowTitleModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn width(&self) -> i32 {
        // Variable width based on title, but capped
        let char_count = self.title.chars().count();
        ((char_count as i32 * 14) + 20).min(400).max(60)
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

        if !self.title.is_empty() {
            ctx.cairo.set_source_rgb(0.8, 0.8, 0.8); // Slightly dimmer than other modules

            // Left-align the text with some padding
            let extents = ctx.cairo.text_extents(&self.title)?;
            let x = ctx.x_offset + 10.0;
            let y = ctx.y_offset + (ctx.height as f64 / 2.0 + extents.height() / 2.0);

            ctx.cairo.move_to(x, y);
            ctx.cairo.show_text(&self.title)?;
        }

        Ok(())
    }

    fn on_touch(&mut self, event: TouchEvent) -> Option<Action> {
        self.active = event.pressed;
        // Window title doesn't have a click action
        None
    }

    fn update_interval(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.config.interval))
    }

    fn update(&mut self) -> Result<bool> {
        let old_title = self.title.clone();
        let new_title = self.get_active_window();
        self.title = self.truncate(&new_title);
        Ok(old_title != self.title)
    }

    fn start_listener(&mut self, _tx: Sender<ModuleEvent>) -> Result<Option<i32>> {
        // TODO: Could use Hyprland IPC events for real-time updates
        Ok(None)
    }
}

impl Default for WindowTitleModule {
    fn default() -> Self {
        Self::new()
    }
}
