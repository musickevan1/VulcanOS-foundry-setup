//! Network Module
//!
//! Displays WiFi/Ethernet status with signal strength.

use anyhow::Result;
use crossbeam_channel::Sender;
use std::fs;
use std::process::Command;
use std::time::Duration;

use super::base::{draw_centered_text, draw_rounded_rect, get_background_color, CORNER_RADIUS};
use super::{Action, Module, ModuleEvent, RenderContext, TouchEvent};

/// Network module configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Update interval in seconds
    pub interval: u64,
    /// Interface to monitor (None = auto-detect)
    pub interface: Option<String>,
    /// Format for WiFi connections
    pub format_wifi: String,
    /// Format for Ethernet connections
    pub format_ethernet: String,
    /// Format when disconnected
    pub format_disconnected: String,
    /// Page to switch to on tap
    pub on_click_page: Option<String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            interval: 5,
            interface: None,
            format_wifi: "󰤨 {essid}".to_string(),
            format_ethernet: "󰈀 {ipaddr}".to_string(),
            format_disconnected: "󰤭".to_string(),
            on_click_page: Some("connectivity".to_string()),
        }
    }
}

/// Network connection state
#[derive(Debug, Clone)]
enum NetworkState {
    Disconnected,
    Wifi { essid: String, signal: i32 },
    Ethernet { interface: String },
}

/// Network module
pub struct NetworkModule {
    name: String,
    config: NetworkConfig,
    state: NetworkState,
    active: bool,
}

impl NetworkModule {
    pub fn new() -> Self {
        Self::with_config(NetworkConfig::default())
    }

    pub fn with_config(config: NetworkConfig) -> Self {
        let mut module = Self {
            name: "network".to_string(),
            config,
            state: NetworkState::Disconnected,
            active: false,
        };
        let _ = module.update();
        module
    }

    pub fn from_config(config: &toml::Value) -> Result<Box<dyn Module>> {
        let mut net_config = NetworkConfig::default();

        if let Some(table) = config.as_table() {
            if let Some(v) = table.get("interval").and_then(|v| v.as_integer()) {
                net_config.interval = v as u64;
            }
            if let Some(v) = table.get("interface").and_then(|v| v.as_str()) {
                net_config.interface = Some(v.to_string());
            }
            if let Some(v) = table.get("format-wifi").and_then(|v| v.as_str()) {
                net_config.format_wifi = v.to_string();
            }
            if let Some(v) = table.get("format-ethernet").and_then(|v| v.as_str()) {
                net_config.format_ethernet = v.to_string();
            }
            if let Some(v) = table.get("format-disconnected").and_then(|v| v.as_str()) {
                net_config.format_disconnected = v.to_string();
            }
            if let Some(v) = table.get("on-click-page").and_then(|v| v.as_str()) {
                net_config.on_click_page = Some(v.to_string());
            }
        }

        Ok(Box::new(Self::with_config(net_config)))
    }

    /// Get WiFi info using iwctl or /proc/net/wireless
    fn get_wifi_info(&self) -> Option<(String, i32)> {
        // Try to get ESSID from iwctl
        if let Ok(output) = Command::new("iwctl")
            .args(["station", "wlan0", "show"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut essid = None;
            let mut signal = -100i32;

            for line in stdout.lines() {
                let line = line.trim();
                if line.starts_with("Connected network") {
                    essid = line.split_whitespace().last().map(|s| s.to_string());
                }
                if line.starts_with("RSSI") {
                    if let Some(dbm) = line.split_whitespace().nth(1) {
                        signal = dbm.trim_end_matches(" dBm").parse().unwrap_or(-100);
                    }
                }
            }

            if let Some(name) = essid {
                return Some((name, signal));
            }
        }

        // Fallback: try /proc/net/wireless for signal
        if let Ok(content) = fs::read_to_string("/proc/net/wireless") {
            for line in content.lines().skip(2) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let signal: i32 = parts[3].trim_end_matches('.').parse().unwrap_or(-100);
                    // Try to get ESSID from /sys
                    if let Ok(essid) = fs::read_to_string("/sys/class/net/wlan0/wireless/essid") {
                        return Some((essid.trim().to_string(), signal));
                    }
                    return Some(("WiFi".to_string(), signal));
                }
            }
        }

        None
    }

    /// Check if ethernet is connected
    fn is_ethernet_connected(&self) -> Option<String> {
        // Check common ethernet interface names
        for iface in &["eth0", "enp0s31f6", "eno1", "enp1s0"] {
            let operstate = format!("/sys/class/net/{}/operstate", iface);
            if let Ok(state) = fs::read_to_string(&operstate) {
                if state.trim() == "up" {
                    return Some(iface.to_string());
                }
            }
        }
        None
    }

    /// Get signal strength icon based on dBm
    fn signal_icon(signal: i32) -> &'static str {
        match signal {
            s if s >= -50 => "󰤨",  // Excellent
            s if s >= -60 => "󰤥",  // Good
            s if s >= -70 => "󰤢",  // Fair
            _ => "󰤟",              // Weak
        }
    }

    fn format_display(&self) -> String {
        match &self.state {
            NetworkState::Disconnected => self.config.format_disconnected.clone(),
            NetworkState::Wifi { essid, signal } => {
                self.config.format_wifi
                    .replace("{essid}", essid)
                    .replace("{signal}", &signal.to_string())
                    .replace("{icon}", Self::signal_icon(*signal))
            }
            NetworkState::Ethernet { interface } => {
                self.config.format_ethernet
                    .replace("{interface}", interface)
                    .replace("{icon}", "󰈀")
            }
        }
    }
}

impl Module for NetworkModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn width(&self) -> i32 {
        match &self.state {
            NetworkState::Disconnected => 50,
            NetworkState::Wifi { essid, .. } => {
                // Icon + essid (estimate)
                (50 + essid.len() as i32 * 12).min(200)
            }
            NetworkState::Ethernet { .. } => 50,
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
        let old_state = format!("{:?}", self.state);

        // Check WiFi first
        if let Some((essid, signal)) = self.get_wifi_info() {
            self.state = NetworkState::Wifi { essid, signal };
        } else if let Some(iface) = self.is_ethernet_connected() {
            self.state = NetworkState::Ethernet { interface: iface };
        } else {
            self.state = NetworkState::Disconnected;
        }

        Ok(old_state != format!("{:?}", self.state))
    }

    fn start_listener(&mut self, _tx: Sender<ModuleEvent>) -> Result<Option<i32>> {
        Ok(None)
    }
}

impl Default for NetworkModule {
    fn default() -> Self {
        Self::new()
    }
}
