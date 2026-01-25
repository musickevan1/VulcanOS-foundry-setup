//! Bluetooth Module
//!
//! Displays Bluetooth status and connected device count.

use anyhow::Result;
use crossbeam_channel::Sender;
use std::process::Command;
use std::time::Duration;

use super::base::{draw_centered_text, draw_rounded_rect, get_background_color, CORNER_RADIUS};
use super::{Action, Module, ModuleEvent, RenderContext, TouchEvent};

/// Bluetooth module configuration
#[derive(Debug, Clone)]
pub struct BluetoothConfig {
    /// Update interval in seconds
    pub interval: u64,
    /// Format when enabled but not connected
    pub format: String,
    /// Format when disabled
    pub format_disabled: String,
    /// Format when connected
    pub format_connected: String,
    /// Page to switch to on tap
    pub on_click_page: Option<String>,
}

impl Default for BluetoothConfig {
    fn default() -> Self {
        Self {
            interval: 5,
            format: "󰂯".to_string(),
            format_disabled: "󰂲".to_string(),
            format_connected: "󰂱 {num_connections}".to_string(),
            on_click_page: Some("connectivity".to_string()),
        }
    }
}

/// Bluetooth state
#[derive(Debug, Clone, PartialEq)]
enum BluetoothState {
    Disabled,
    Enabled,
    Connected { count: usize },
}

/// Bluetooth module
pub struct BluetoothModule {
    name: String,
    config: BluetoothConfig,
    state: BluetoothState,
    active: bool,
}

impl BluetoothModule {
    pub fn new() -> Self {
        Self::with_config(BluetoothConfig::default())
    }

    pub fn with_config(config: BluetoothConfig) -> Self {
        let mut module = Self {
            name: "bluetooth".to_string(),
            config,
            state: BluetoothState::Disabled,
            active: false,
        };
        let _ = module.update();
        module
    }

    pub fn from_config(config: &toml::Value) -> Result<Box<dyn Module>> {
        let mut bt_config = BluetoothConfig::default();

        if let Some(table) = config.as_table() {
            if let Some(v) = table.get("interval").and_then(|v| v.as_integer()) {
                bt_config.interval = v as u64;
            }
            if let Some(v) = table.get("format").and_then(|v| v.as_str()) {
                bt_config.format = v.to_string();
            }
            if let Some(v) = table.get("format-disabled").and_then(|v| v.as_str()) {
                bt_config.format_disabled = v.to_string();
            }
            if let Some(v) = table.get("format-connected").and_then(|v| v.as_str()) {
                bt_config.format_connected = v.to_string();
            }
            if let Some(v) = table.get("on-click-page").and_then(|v| v.as_str()) {
                bt_config.on_click_page = Some(v.to_string());
            }
        }

        Ok(Box::new(Self::with_config(bt_config)))
    }

    /// Query bluetooth status using bluetoothctl
    fn get_bluetooth_state(&self) -> BluetoothState {
        // Check if bluetooth is powered on
        let power_output = Command::new("bluetoothctl")
            .args(["show"])
            .output();

        let powered = power_output
            .map(|o| String::from_utf8_lossy(&o.stdout).contains("Powered: yes"))
            .unwrap_or(false);

        if !powered {
            return BluetoothState::Disabled;
        }

        // Count connected devices
        let devices_output = Command::new("bluetoothctl")
            .args(["devices", "Connected"])
            .output();

        let count = devices_output
            .map(|o| {
                String::from_utf8_lossy(&o.stdout)
                    .lines()
                    .filter(|l| l.starts_with("Device"))
                    .count()
            })
            .unwrap_or(0);

        if count > 0 {
            BluetoothState::Connected { count }
        } else {
            BluetoothState::Enabled
        }
    }

    fn format_display(&self) -> String {
        match &self.state {
            BluetoothState::Disabled => self.config.format_disabled.clone(),
            BluetoothState::Enabled => self.config.format.clone(),
            BluetoothState::Connected { count } => {
                self.config.format_connected
                    .replace("{num_connections}", &count.to_string())
            }
        }
    }
}

impl Module for BluetoothModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn width(&self) -> i32 {
        match &self.state {
            BluetoothState::Connected { .. } => 80,
            _ => 50,
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
        self.state = self.get_bluetooth_state();
        Ok(old_state != self.state)
    }

    fn start_listener(&mut self, _tx: Sender<ModuleEvent>) -> Result<Option<i32>> {
        Ok(None)
    }
}

impl Default for BluetoothModule {
    fn default() -> Self {
        Self::new()
    }
}
