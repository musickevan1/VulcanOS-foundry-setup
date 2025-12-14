//! Configuration loader for VulcanBar
//!
//! Handles loading and hot-reloading of TOML configuration files.

use anyhow::{Context, Result};
use nix::{
    errno::Errno,
    sys::inotify::{AddWatchFlags, InitFlags, Inotify, WatchDescriptor},
};
use std::fs;
use std::os::fd::AsFd;
use std::path::{Path, PathBuf};

use super::schema::VulcanBarConfig;

/// System-wide configuration path
const SYSTEM_CONFIG_PATH: &str = "/usr/share/vulcanbar/config.toml";
/// User configuration path
const USER_CONFIG_PATH: &str = "/etc/vulcanbar/config.toml";
/// XDG config home fallback
const XDG_CONFIG_PATH: &str = ".config/vulcanbar/config.toml";

/// Configuration loader with hot-reload support
pub struct ConfigLoader {
    /// Loaded configuration
    config: VulcanBarConfig,
    /// Inotify instance for watching config changes
    inotify: Inotify,
    /// Watch descriptor for user config
    watch_desc: Option<WatchDescriptor>,
    /// Path being watched
    watched_path: PathBuf,
}

impl ConfigLoader {
    /// Create a new config loader and load initial configuration
    pub fn new() -> Result<Self> {
        let inotify = Inotify::init(InitFlags::IN_NONBLOCK)
            .context("Failed to initialize inotify")?;

        let mut loader = Self {
            config: VulcanBarConfig::default(),
            inotify,
            watch_desc: None,
            watched_path: PathBuf::new(),
        };

        loader.load_config()?;
        loader.setup_watch();

        Ok(loader)
    }

    /// Load configuration from system and user paths
    fn load_config(&mut self) -> Result<()> {
        // Start with defaults
        let mut config = VulcanBarConfig::default();

        // Load system config if exists
        if Path::new(SYSTEM_CONFIG_PATH).exists() {
            tracing::debug!("Loading system config from: {}", SYSTEM_CONFIG_PATH);
            match load_config_file(SYSTEM_CONFIG_PATH) {
                Ok(system_config) => {
                    tracing::debug!("System config loaded - pages: {:?}, layout.center: {:?}",
                        system_config.pages.as_ref().map(|p| p.list.len()),
                        system_config.layout.center);
                    config = system_config;
                }
                Err(e) => tracing::warn!("Failed to load system config: {}", e),
            }
        } else {
            tracing::debug!("System config not found at: {}", SYSTEM_CONFIG_PATH);
        }

        // Determine user config path
        let user_path = get_user_config_path();
        self.watched_path = user_path.clone();
        tracing::debug!("User config path: {:?}", user_path);

        // Load and merge user config if exists
        if user_path.exists() {
            tracing::debug!("Loading user config from: {:?}", user_path);
            match load_config_file(&user_path) {
                Ok(user_config) => {
                    tracing::debug!("User config loaded - pages: {:?}, layout.center: {:?}",
                        user_config.pages.as_ref().map(|p| p.list.len()),
                        user_config.layout.center);
                    config.merge(user_config);
                    tracing::info!("After merge - pages: {:?}, layout.center: {:?}",
                        config.pages.as_ref().map(|p| p.list.len()),
                        config.layout.center);
                }
                Err(e) => tracing::warn!("Failed to load user config: {}", e),
            }
        } else {
            tracing::debug!("User config not found at: {:?}", user_path);
        }

        self.config = config;
        Ok(())
    }

    /// Set up inotify watch on user config file
    fn setup_watch(&mut self) {
        self.watch_desc = arm_inotify(&self.inotify, &self.watched_path);
    }

    /// Get current configuration
    pub fn config(&self) -> &VulcanBarConfig {
        &self.config
    }

    /// Check for config changes and reload if needed
    /// Returns true if config was reloaded
    pub fn check_reload(&mut self) -> bool {
        // Re-arm watch if not set
        if self.watch_desc.is_none() {
            self.watch_desc = arm_inotify(&self.inotify, &self.watched_path);
            return false;
        }

        // Check for events
        match self.inotify.read_events() {
            Err(Errno::EAGAIN) => false,
            Ok(events) => {
                let mut reloaded = false;
                for event in events {
                    if Some(event.wd) == self.watch_desc {
                        // Config file changed, reload
                        if self.load_config().is_ok() {
                            reloaded = true;
                        }
                        // Re-arm watch
                        self.watch_desc = arm_inotify(&self.inotify, &self.watched_path);
                    }
                }
                reloaded
            }
            Err(_) => false,
        }
    }

    /// Get file descriptor for epoll integration
    pub fn fd(&self) -> impl AsFd + '_ {
        &self.inotify
    }
}

/// Load a config file from the given path
fn load_config_file<P: AsRef<Path>>(path: P) -> Result<VulcanBarConfig> {
    let content = fs::read_to_string(path.as_ref())
        .with_context(|| format!("Failed to read config file: {:?}", path.as_ref()))?;

    let config: VulcanBarConfig = toml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {:?}", path.as_ref()))?;

    Ok(config)
}

/// Get the user configuration path
fn get_user_config_path() -> PathBuf {
    // Check /etc first (system-wide user override)
    let etc_path = PathBuf::from(USER_CONFIG_PATH);
    if etc_path.exists() {
        return etc_path;
    }

    // Check XDG_CONFIG_HOME
    if let Ok(xdg_home) = std::env::var("XDG_CONFIG_HOME") {
        let xdg_path = PathBuf::from(xdg_home).join("vulcanbar/config.toml");
        if xdg_path.exists() {
            return xdg_path;
        }
    }

    // Check ~/.config
    if let Ok(home) = std::env::var("HOME") {
        let home_path = PathBuf::from(home).join(XDG_CONFIG_PATH);
        if home_path.exists() {
            return home_path;
        }
    }

    // Default to /etc path even if doesn't exist (for watching)
    etc_path
}

/// Set up inotify watch on a file
fn arm_inotify(inotify: &Inotify, path: &Path) -> Option<WatchDescriptor> {
    let flags = AddWatchFlags::IN_MOVED_TO | AddWatchFlags::IN_CLOSE | AddWatchFlags::IN_ONESHOT;
    match inotify.add_watch(path, flags) {
        Ok(wd) => Some(wd),
        Err(Errno::ENOENT) => None, // File doesn't exist yet
        Err(e) => {
            eprintln!("Warning: Failed to watch config file: {}", e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = VulcanBarConfig::default();
        assert_eq!(config.general.font, ":bold");
        assert_eq!(config.layout.center, vec!["fkeys"]);
    }

    #[test]
    fn test_parse_minimal_config() {
        let toml = r#"
[general]
font = "JetBrains Mono"

[layout]
center = ["clock"]
"#;
        let config: VulcanBarConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.general.font, "JetBrains Mono");
        assert_eq!(config.layout.center, vec!["clock"]);
    }
}
