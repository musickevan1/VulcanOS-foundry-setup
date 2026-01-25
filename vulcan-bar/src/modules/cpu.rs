//! CPU Module
//!
//! Displays CPU usage percentage with optional tap-to-expand to system page.

use anyhow::Result;
use crossbeam_channel::Sender;
use std::fs;
use std::time::Duration;

use super::base::{draw_centered_text, draw_rounded_rect, get_background_color, CORNER_RADIUS};
use super::{Action, Module, ModuleEvent, RenderContext, TouchEvent};

/// CPU statistics from /proc/stat
#[derive(Default, Clone)]
struct CpuStats {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
}

impl CpuStats {
    fn total(&self) -> u64 {
        self.user + self.nice + self.system + self.idle + self.iowait + self.irq + self.softirq + self.steal
    }

    fn active(&self) -> u64 {
        self.total() - self.idle - self.iowait
    }
}

/// CPU module configuration
#[derive(Debug, Clone)]
pub struct CpuConfig {
    /// Update interval in seconds
    pub interval: u64,
    /// Format string (use {usage} for percentage)
    pub format: String,
    /// Icon to display
    pub icon: String,
    /// Page to switch to on tap (None = no action)
    pub on_click_page: Option<String>,
}

impl Default for CpuConfig {
    fn default() -> Self {
        Self {
            interval: 2,
            format: "{icon} {usage}%".to_string(),
            icon: "󰻠".to_string(),
            on_click_page: Some("system".to_string()),
        }
    }
}

/// CPU module displaying usage percentage
pub struct CpuModule {
    name: String,
    config: CpuConfig,
    /// Current CPU usage percentage
    usage: f64,
    /// Previous CPU stats for delta calculation
    prev_stats: CpuStats,
    /// Whether currently being touched
    active: bool,
}

impl CpuModule {
    /// Create a new CPU module
    pub fn new() -> Self {
        Self::with_config(CpuConfig::default())
    }

    /// Create with specific configuration
    pub fn with_config(config: CpuConfig) -> Self {
        let mut module = Self {
            name: "cpu".to_string(),
            config,
            usage: 0.0,
            prev_stats: CpuStats::default(),
            active: false,
        };
        // Initialize stats
        module.prev_stats = module.read_cpu_stats().unwrap_or_default();
        module
    }

    /// Create from TOML configuration
    pub fn from_config(config: &toml::Value) -> Result<Box<dyn Module>> {
        let mut cpu_config = CpuConfig::default();

        if let Some(table) = config.as_table() {
            if let Some(v) = table.get("interval").and_then(|v| v.as_integer()) {
                cpu_config.interval = v as u64;
            }
            if let Some(v) = table.get("format").and_then(|v| v.as_str()) {
                cpu_config.format = v.to_string();
            }
            if let Some(v) = table.get("icon").and_then(|v| v.as_str()) {
                cpu_config.icon = v.to_string();
            }
            if let Some(v) = table.get("on-click-page").and_then(|v| v.as_str()) {
                cpu_config.on_click_page = Some(v.to_string());
            }
        }

        Ok(Box::new(Self::with_config(cpu_config)))
    }

    /// Read CPU stats from /proc/stat
    fn read_cpu_stats(&self) -> Result<CpuStats> {
        let content = fs::read_to_string("/proc/stat")?;
        let line = content.lines().next().ok_or_else(|| anyhow::anyhow!("Empty /proc/stat"))?;

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 9 || parts[0] != "cpu" {
            return Err(anyhow::anyhow!("Invalid /proc/stat format"));
        }

        Ok(CpuStats {
            user: parts[1].parse().unwrap_or(0),
            nice: parts[2].parse().unwrap_or(0),
            system: parts[3].parse().unwrap_or(0),
            idle: parts[4].parse().unwrap_or(0),
            iowait: parts[5].parse().unwrap_or(0),
            irq: parts[6].parse().unwrap_or(0),
            softirq: parts[7].parse().unwrap_or(0),
            steal: parts[8].parse().unwrap_or(0),
        })
    }

    /// Calculate usage percentage from stats delta
    fn calculate_usage(&mut self) -> f64 {
        if let Ok(current) = self.read_cpu_stats() {
            let total_delta = current.total().saturating_sub(self.prev_stats.total());
            let active_delta = current.active().saturating_sub(self.prev_stats.active());

            self.prev_stats = current;

            if total_delta > 0 {
                (active_delta as f64 / total_delta as f64) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Format the display string
    fn format_display(&self) -> String {
        self.config.format
            .replace("{icon}", &self.config.icon)
            .replace("{usage}", &format!("{:.0}", self.usage))
    }
}

impl Module for CpuModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn width(&self) -> i32 {
        // Icon + space + percentage (e.g., "󰻠 45%")
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
            // On release, switch to system page if configured
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
        let old_usage = self.usage as u32;
        self.usage = self.calculate_usage();
        Ok(old_usage != self.usage as u32)
    }

    fn start_listener(&mut self, _tx: Sender<ModuleEvent>) -> Result<Option<i32>> {
        Ok(None)
    }
}

impl Default for CpuModule {
    fn default() -> Self {
        Self::new()
    }
}
