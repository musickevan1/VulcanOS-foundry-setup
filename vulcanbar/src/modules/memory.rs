//! Memory Module
//!
//! Displays RAM usage percentage with optional tap-to-expand to system page.

use anyhow::Result;
use crossbeam_channel::Sender;
use std::fs;
use std::time::Duration;

use super::base::{draw_centered_text, draw_rounded_rect, get_background_color, CORNER_RADIUS};
use super::{Action, Module, ModuleEvent, RenderContext, TouchEvent};

/// Memory module configuration
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    /// Update interval in seconds
    pub interval: u64,
    /// Format string
    pub format: String,
    /// Icon to display
    pub icon: String,
    /// Page to switch to on tap
    pub on_click_page: Option<String>,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            interval: 2,
            format: "{icon} {percentage}%".to_string(),
            icon: "󰍛".to_string(),
            on_click_page: Some("system".to_string()),
        }
    }
}

/// Memory statistics
#[derive(Default)]
struct MemInfo {
    total: u64,
    available: u64,
    used: u64,
}

/// Memory module displaying RAM usage
pub struct MemoryModule {
    name: String,
    config: MemoryConfig,
    /// Memory info
    mem_info: MemInfo,
    /// Whether currently being touched
    active: bool,
}

impl MemoryModule {
    pub fn new() -> Self {
        Self::with_config(MemoryConfig::default())
    }

    pub fn with_config(config: MemoryConfig) -> Self {
        let mut module = Self {
            name: "memory".to_string(),
            config,
            mem_info: MemInfo::default(),
            active: false,
        };
        let _ = module.update();
        module
    }

    pub fn from_config(config: &toml::Value) -> Result<Box<dyn Module>> {
        let mut mem_config = MemoryConfig::default();

        if let Some(table) = config.as_table() {
            if let Some(v) = table.get("interval").and_then(|v| v.as_integer()) {
                mem_config.interval = v as u64;
            }
            if let Some(v) = table.get("format").and_then(|v| v.as_str()) {
                mem_config.format = v.to_string();
            }
            if let Some(v) = table.get("icon").and_then(|v| v.as_str()) {
                mem_config.icon = v.to_string();
            }
            if let Some(v) = table.get("on-click-page").and_then(|v| v.as_str()) {
                mem_config.on_click_page = Some(v.to_string());
            }
        }

        Ok(Box::new(Self::with_config(mem_config)))
    }

    /// Read memory info from /proc/meminfo
    fn read_meminfo(&self) -> Result<MemInfo> {
        let content = fs::read_to_string("/proc/meminfo")?;
        let mut info = MemInfo::default();

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let value: u64 = parts[1].parse().unwrap_or(0);
                match parts[0] {
                    "MemTotal:" => info.total = value,
                    "MemAvailable:" => info.available = value,
                    _ => {}
                }
            }
        }

        info.used = info.total.saturating_sub(info.available);
        Ok(info)
    }

    fn percentage(&self) -> f64 {
        if self.mem_info.total > 0 {
            (self.mem_info.used as f64 / self.mem_info.total as f64) * 100.0
        } else {
            0.0
        }
    }

    fn format_display(&self) -> String {
        let used_gb = self.mem_info.used as f64 / 1024.0 / 1024.0;
        let total_gb = self.mem_info.total as f64 / 1024.0 / 1024.0;

        self.config.format
            .replace("{icon}", &self.config.icon)
            .replace("{percentage}", &format!("{:.0}", self.percentage()))
            .replace("{used}", &format!("{:.1}", used_gb))
            .replace("{total}", &format!("{:.1}", total_gb))
    }
}

impl Module for MemoryModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn width(&self) -> i32 {
        100
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
        let old_percentage = self.percentage() as u32;
        self.mem_info = self.read_meminfo()?;
        Ok(old_percentage != self.percentage() as u32)
    }

    fn start_listener(&mut self, _tx: Sender<ModuleEvent>) -> Result<Option<i32>> {
        Ok(None)
    }
}

impl Default for MemoryModule {
    fn default() -> Self {
        Self::new()
    }
}
