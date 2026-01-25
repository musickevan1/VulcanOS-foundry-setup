//! Audio Module
//!
//! Displays volume level with tap-to-expand to media page.

use anyhow::Result;
use crossbeam_channel::Sender;
use std::process::Command;
use std::time::Duration;

use super::base::{draw_centered_text, draw_rounded_rect, get_background_color, CORNER_RADIUS};
use super::{Action, Module, ModuleEvent, RenderContext, TouchEvent};

/// Audio module configuration
#[derive(Debug, Clone)]
pub struct AudioConfig {
    /// Update interval in seconds
    pub interval: u64,
    /// Format string
    pub format: String,
    /// Format when muted
    pub format_muted: String,
    /// Page to switch to on tap
    pub on_click_page: Option<String>,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            interval: 1,
            format: "{icon} {volume}%".to_string(),
            format_muted: "󰝟 Muted".to_string(),
            on_click_page: Some("media".to_string()),
        }
    }
}

/// Audio state
#[derive(Debug, Clone, PartialEq)]
struct AudioState {
    volume: u32,
    muted: bool,
}

impl Default for AudioState {
    fn default() -> Self {
        Self { volume: 0, muted: false }
    }
}

/// Audio module
pub struct AudioModule {
    name: String,
    config: AudioConfig,
    state: AudioState,
    active: bool,
}

impl AudioModule {
    pub fn new() -> Self {
        Self::with_config(AudioConfig::default())
    }

    pub fn with_config(config: AudioConfig) -> Self {
        let mut module = Self {
            name: "audio".to_string(),
            config,
            state: AudioState::default(),
            active: false,
        };
        let _ = module.update();
        module
    }

    pub fn from_config(config: &toml::Value) -> Result<Box<dyn Module>> {
        let mut audio_config = AudioConfig::default();

        if let Some(table) = config.as_table() {
            if let Some(v) = table.get("interval").and_then(|v| v.as_integer()) {
                audio_config.interval = v as u64;
            }
            if let Some(v) = table.get("format").and_then(|v| v.as_str()) {
                audio_config.format = v.to_string();
            }
            if let Some(v) = table.get("format-muted").and_then(|v| v.as_str()) {
                audio_config.format_muted = v.to_string();
            }
            if let Some(v) = table.get("on-click-page").and_then(|v| v.as_str()) {
                audio_config.on_click_page = Some(v.to_string());
            }
        }

        Ok(Box::new(Self::with_config(audio_config)))
    }

    /// Get volume from wpctl or pactl
    fn get_audio_state(&self) -> AudioState {
        // Try wpctl first (PipeWire)
        if let Ok(output) = Command::new("wpctl")
            .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Output format: "Volume: 0.75" or "Volume: 0.75 [MUTED]"
            if stdout.starts_with("Volume:") {
                let parts: Vec<&str> = stdout.trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    let volume = parts[1].parse::<f64>().unwrap_or(0.0);
                    let muted = stdout.contains("[MUTED]");
                    return AudioState {
                        volume: (volume * 100.0).round() as u32,
                        muted,
                    };
                }
            }
        }

        // Fallback to pactl
        if let Ok(output) = Command::new("pactl")
            .args(["get-sink-volume", "@DEFAULT_SINK@"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Look for percentage
            for part in stdout.split_whitespace() {
                if part.ends_with('%') {
                    if let Ok(vol) = part.trim_end_matches('%').parse::<u32>() {
                        // Check mute status
                        let muted = Command::new("pactl")
                            .args(["get-sink-mute", "@DEFAULT_SINK@"])
                            .output()
                            .map(|o| String::from_utf8_lossy(&o.stdout).contains("yes"))
                            .unwrap_or(false);

                        return AudioState { volume: vol, muted };
                    }
                }
            }
        }

        AudioState::default()
    }

    /// Get volume icon based on level
    fn volume_icon(&self) -> &'static str {
        if self.state.muted {
            return "󰝟";
        }
        match self.state.volume {
            0 => "󰕿",
            1..=33 => "󰕿",
            34..=66 => "󰖀",
            _ => "󰕾",
        }
    }

    fn format_display(&self) -> String {
        if self.state.muted {
            return self.config.format_muted.clone();
        }

        self.config.format
            .replace("{icon}", self.volume_icon())
            .replace("{volume}", &self.state.volume.to_string())
    }
}

impl Module for AudioModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn width(&self) -> i32 {
        if self.state.muted {
            100
        } else {
            90
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
        let old_state = self.state.clone();
        self.state = self.get_audio_state();
        Ok(old_state != self.state)
    }

    fn start_listener(&mut self, _tx: Sender<ModuleEvent>) -> Result<Option<i32>> {
        Ok(None)
    }
}

impl Default for AudioModule {
    fn default() -> Self {
        Self::new()
    }
}
