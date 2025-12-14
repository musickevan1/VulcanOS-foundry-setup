//! Media Module
//!
//! MPRIS-based media player control with D-Bus integration.
//! Displays current track info and provides playback controls.

use anyhow::Result;
use crossbeam_channel::Sender;
use dbus::blocking::Connection;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::arg::RefArg;
use std::collections::HashMap;
use std::time::Duration;

use super::base::{draw_centered_text, draw_rounded_rect, get_background_color, CORNER_RADIUS};
use super::{Action, Module, ModuleEvent, RenderContext, TouchEvent};

/// Media module configuration
#[derive(Debug, Clone)]
pub struct MediaConfig {
    /// Update interval in milliseconds
    pub interval: u64,
    /// Format string for playing state
    pub format_playing: String,
    /// Format string for paused state
    pub format_paused: String,
    /// Format string for stopped/no player
    pub format_stopped: String,
    /// Maximum title length before truncation
    pub truncate_length: usize,
    /// Command to run on tap (default: toggle play/pause)
    pub on_click: Option<String>,
}

impl Default for MediaConfig {
    fn default() -> Self {
        Self {
            interval: 1000,
            format_playing: "󰐊 {title}".to_string(),
            format_paused: "󰏤 {title}".to_string(),
            format_stopped: "󰝛 No Media".to_string(),
            truncate_length: 30,
            on_click: None,
        }
    }
}

/// Playback status from MPRIS
#[derive(Debug, Clone, PartialEq)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

/// Media player state
#[derive(Debug, Clone, PartialEq)]
pub struct MediaState {
    /// Current playback status
    pub status: PlaybackStatus,
    /// Track title
    pub title: String,
    /// Artist name
    pub artist: String,
    /// Album name
    pub album: String,
    /// Active player name (e.g., "spotify", "firefox")
    pub player: Option<String>,
}

impl Default for MediaState {
    fn default() -> Self {
        Self {
            status: PlaybackStatus::Stopped,
            title: String::new(),
            artist: String::new(),
            album: String::new(),
            player: None,
        }
    }
}

/// Media module for MPRIS control
pub struct MediaModule {
    name: String,
    config: MediaConfig,
    state: MediaState,
    active: bool,
    /// Cached D-Bus session bus address for user session
    dbus_address: Option<String>,
    /// Whether D-Bus connection is available (don't retry if failed)
    dbus_available: bool,
}

impl MediaModule {
    pub fn new() -> Self {
        Self::with_config(MediaConfig::default())
    }

    pub fn with_config(config: MediaConfig) -> Self {
        // Try to get the user's D-Bus session address
        let dbus_address = Self::find_user_dbus_address();

        // Test D-Bus connection once at startup - don't block later if it fails
        let dbus_available = if dbus_address.is_some() {
            // Quick test with very short timeout
            std::env::set_var("DBUS_SESSION_BUS_ADDRESS", dbus_address.as_ref().unwrap());
            Connection::new_session().is_ok()
        } else {
            false
        };

        if !dbus_available {
            tracing::debug!("D-Bus session not available - media module will show placeholder");
        }

        Self {
            name: "media".to_string(),
            config,
            state: MediaState::default(),
            active: false,
            dbus_address,
            dbus_available,
        }
        // Note: Don't call update() here to avoid blocking on D-Bus at startup
    }

    pub fn from_config(config: &toml::Value) -> Result<Box<dyn Module>> {
        let mut media_config = MediaConfig::default();

        if let Some(table) = config.as_table() {
            if let Some(v) = table.get("interval").and_then(|v| v.as_integer()) {
                media_config.interval = v as u64;
            }
            if let Some(v) = table.get("format-playing").and_then(|v| v.as_str()) {
                media_config.format_playing = v.to_string();
            }
            if let Some(v) = table.get("format-paused").and_then(|v| v.as_str()) {
                media_config.format_paused = v.to_string();
            }
            if let Some(v) = table.get("format-stopped").and_then(|v| v.as_str()) {
                media_config.format_stopped = v.to_string();
            }
            if let Some(v) = table.get("truncate-length").and_then(|v| v.as_integer()) {
                media_config.truncate_length = v as usize;
            }
            if let Some(v) = table.get("on-click").and_then(|v| v.as_str()) {
                media_config.on_click = Some(v.to_string());
            }
        }

        Ok(Box::new(Self::with_config(media_config)))
    }

    /// Find the user's D-Bus session bus address
    /// Since we run as root, we need to find the user's session
    fn find_user_dbus_address() -> Option<String> {
        // First check environment
        if let Ok(addr) = std::env::var("DBUS_SESSION_BUS_ADDRESS") {
            return Some(addr);
        }

        // Try to find from /run/user/1000 (common UID for first user)
        for uid in [1000u32, 1001, 500] {
            let path = format!("/run/user/{}/bus", uid);
            if std::path::Path::new(&path).exists() {
                return Some(format!("unix:path={}", path));
            }
        }

        None
    }

    /// Connect to the user's D-Bus session
    fn connect_dbus(&self) -> Option<Connection> {
        // Skip if D-Bus was already determined to be unavailable
        if !self.dbus_available {
            return None;
        }

        if let Some(ref addr) = self.dbus_address {
            // Set the address temporarily and connect
            std::env::set_var("DBUS_SESSION_BUS_ADDRESS", addr);
            match Connection::new_session() {
                Ok(conn) => return Some(conn),
                Err(e) => {
                    tracing::debug!("Failed to connect to D-Bus session: {}", e);
                }
            }
        }

        // Try default session
        Connection::new_session().ok()
    }

    /// Find active MPRIS players
    fn find_mpris_players(&self, conn: &Connection) -> Vec<String> {
        let proxy = conn.with_proxy(
            "org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            Duration::from_millis(100),  // Short timeout to avoid blocking
        );

        let names: Result<(Vec<String>,), _> = proxy.method_call(
            "org.freedesktop.DBus",
            "ListNames",
            (),
        );

        match names {
            Ok((names,)) => {
                names.into_iter()
                    .filter(|n| n.starts_with("org.mpris.MediaPlayer2."))
                    .collect()
            }
            Err(_) => Vec::new(),
        }
    }

    /// Get playback status from an MPRIS player
    fn get_player_state(&self, conn: &Connection, player: &str) -> Option<MediaState> {
        let proxy = conn.with_proxy(
            player,
            "/org/mpris/MediaPlayer2",
            Duration::from_millis(100),  // Short timeout to avoid blocking
        );

        // Get PlaybackStatus
        let status: Result<dbus::arg::Variant<String>, _> = proxy.get(
            "org.mpris.MediaPlayer2.Player",
            "PlaybackStatus",
        );

        let playback_status = match status {
            Ok(s) => match s.0.as_str() {
                "Playing" => PlaybackStatus::Playing,
                "Paused" => PlaybackStatus::Paused,
                _ => PlaybackStatus::Stopped,
            },
            Err(_) => return None,
        };

        // Get Metadata
        let metadata: Result<dbus::arg::Variant<HashMap<String, dbus::arg::Variant<Box<dyn RefArg + 'static>>>>, _> = proxy.get(
            "org.mpris.MediaPlayer2.Player",
            "Metadata",
        );

        let (title, artist, album) = match metadata {
            Ok(m) => {
                let map = &m.0;
                let title = Self::extract_string(map, "xesam:title")
                    .unwrap_or_default();
                let artist = Self::extract_string_array(map, "xesam:artist")
                    .unwrap_or_default();
                let album = Self::extract_string(map, "xesam:album")
                    .unwrap_or_default();
                (title, artist, album)
            }
            Err(_) => (String::new(), String::new(), String::new()),
        };

        // Extract player name from bus name
        let player_name = player
            .strip_prefix("org.mpris.MediaPlayer2.")
            .map(|s| s.to_string());

        Some(MediaState {
            status: playback_status,
            title,
            artist,
            album,
            player: player_name,
        })
    }

    /// Extract a string value from metadata
    fn extract_string(
        map: &HashMap<String, dbus::arg::Variant<Box<dyn RefArg + 'static>>>,
        key: &str,
    ) -> Option<String> {
        map.get(key).and_then(|v| {
            v.0.as_str().map(|s| s.to_string())
        })
    }

    /// Extract string array (e.g., artist list) from metadata
    fn extract_string_array(
        map: &HashMap<String, dbus::arg::Variant<Box<dyn RefArg + 'static>>>,
        key: &str,
    ) -> Option<String> {
        map.get(key).and_then(|v| {
            // Try to iterate as array
            if let Some(iter) = v.0.as_iter() {
                let artists: Vec<String> = iter
                    .filter_map(|item| item.as_str().map(|s| s.to_string()))
                    .collect();
                if !artists.is_empty() {
                    return Some(artists.join(", "));
                }
            }
            None
        })
    }

    /// Toggle play/pause on the active player
    fn toggle_play_pause(&self) {
        if let Some(conn) = self.connect_dbus() {
            if let Some(ref player_name) = self.state.player {
                let bus_name = format!("org.mpris.MediaPlayer2.{}", player_name);
                let proxy = conn.with_proxy(
                    &bus_name,
                    "/org/mpris/MediaPlayer2",
                    Duration::from_millis(100),  // Short timeout to avoid blocking
                );

                let _: Result<(), _> = proxy.method_call(
                    "org.mpris.MediaPlayer2.Player",
                    "PlayPause",
                    (),
                );
            }
        }
    }

    /// Truncate title to configured length
    fn truncate_title(&self, title: &str) -> String {
        if title.len() > self.config.truncate_length {
            format!("{}...", &title[..self.config.truncate_length - 3])
        } else {
            title.to_string()
        }
    }

    /// Format display string based on current state
    fn format_display(&self) -> String {
        if self.state.player.is_none() || self.state.title.is_empty() {
            return self.config.format_stopped.clone();
        }

        let format = match self.state.status {
            PlaybackStatus::Playing => &self.config.format_playing,
            PlaybackStatus::Paused => &self.config.format_paused,
            PlaybackStatus::Stopped => &self.config.format_stopped,
        };

        format
            .replace("{title}", &self.truncate_title(&self.state.title))
            .replace("{artist}", &self.state.artist)
            .replace("{album}", &self.state.album)
            .replace("{player}", self.state.player.as_deref().unwrap_or(""))
    }
}

impl Module for MediaModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn width(&self) -> i32 {
        // Dynamic width based on content, with reasonable bounds
        let text = self.format_display();
        let char_width = 12; // Approximate pixels per character
        let min_width = 100;
        let max_width = 300;

        let calculated = (text.chars().count() * char_width) as i32 + 20;
        calculated.clamp(min_width, max_width)
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
            // On release, toggle play/pause
            if let Some(ref cmd) = self.config.on_click {
                return Some(Action::Command(cmd.clone()));
            }
            // Default: toggle play/pause via MPRIS
            self.toggle_play_pause();
        }
        None
    }

    fn update_interval(&self) -> Option<Duration> {
        Some(Duration::from_millis(self.config.interval))
    }

    fn update(&mut self) -> Result<bool> {
        let old_state = self.state.clone();

        // Connect to D-Bus and find players
        if let Some(conn) = self.connect_dbus() {
            let players = self.find_mpris_players(&conn);

            // Find the first playing player, or first paused, or first available
            let mut best_state: Option<MediaState> = None;

            for player in &players {
                if let Some(state) = self.get_player_state(&conn, player) {
                    match (&best_state, &state.status) {
                        (None, _) => best_state = Some(state),
                        (Some(current), PlaybackStatus::Playing)
                            if current.status != PlaybackStatus::Playing => {
                            best_state = Some(state);
                        }
                        _ => {}
                    }
                }
            }

            self.state = best_state.unwrap_or_default();
        } else {
            self.state = MediaState::default();
        }

        Ok(old_state != self.state)
    }

    fn start_listener(&mut self, _tx: Sender<ModuleEvent>) -> Result<Option<i32>> {
        // Could implement D-Bus signal listener for real-time updates
        // For now, rely on polling
        Ok(None)
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Default for MediaModule {
    fn default() -> Self {
        Self::new()
    }
}
