//! Configuration schema definitions for VulcanBar
//!
//! Defines the structure for the Waybar-style TOML configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root configuration structure
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct VulcanBarConfig {
    /// General settings
    pub general: GeneralConfig,
    /// Layout configuration (for single-page mode)
    pub layout: LayoutConfig,
    /// Multi-page configuration (optional)
    pub pages: Option<PagesConfig>,
    /// Gesture configuration (optional)
    pub gestures: Option<GesturesConfig>,
    /// Per-module configurations
    #[serde(default)]
    pub modules: HashMap<String, toml::Value>,
}

impl Default for VulcanBarConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            layout: LayoutConfig::default(),
            pages: None,
            gestures: None,
            modules: HashMap::new(),
        }
    }
}

/// Multi-page configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct PagesConfig {
    /// Default page to show on startup
    pub default: String,
    /// Enable swipe gestures for page switching
    pub enable_swipe: bool,
    /// Transition animation duration in milliseconds
    pub transition_duration_ms: u64,
    /// List of page definitions
    pub list: Vec<PageDefinition>,
}

impl Default for PagesConfig {
    fn default() -> Self {
        Self {
            default: "main".to_string(),
            enable_swipe: true,
            transition_duration_ms: 200,
            list: vec![],
        }
    }
}

/// Single page definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageDefinition {
    /// Page name/identifier
    pub name: String,
    /// Layout for this page
    pub layout: LayoutConfig,
}

/// Gesture detection configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct GesturesConfig {
    /// Minimum horizontal distance (pixels) to trigger a swipe
    pub swipe_threshold_px: f64,
    /// Minimum velocity (pixels/second) to trigger a swipe
    pub swipe_velocity_threshold: f64,
    /// Duration (ms) to trigger a long press
    pub long_press_duration_ms: u64,
}

impl Default for GesturesConfig {
    fn default() -> Self {
        Self {
            swipe_threshold_px: 50.0,
            swipe_velocity_threshold: 100.0,
            long_press_duration_ms: 500,
        }
    }
}

/// General configuration options
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct GeneralConfig {
    /// Font family to use (fontconfig pattern)
    pub font: String,
    /// Font size in pixels
    pub font_size: f64,
    /// Enable pixel shifting for OLED longevity
    pub enable_pixel_shift: bool,
    /// Show button outlines
    pub show_button_outlines: bool,
    /// Enable adaptive brightness (follow display)
    pub adaptive_brightness: bool,
    /// Active brightness level (0-255)
    pub active_brightness: u32,
    /// Spacing between modules in pixels
    pub spacing: i32,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            font: ":bold".to_string(),
            font_size: 32.0,
            enable_pixel_shift: false,
            show_button_outlines: true,
            adaptive_brightness: true,
            active_brightness: 128,
            spacing: 16,
        }
    }
}

/// Layout configuration - which modules go where
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct LayoutConfig {
    /// Modules on the left side (fixed width)
    pub left: Vec<String>,
    /// Modules in the center (stretch to fill)
    pub center: Vec<String>,
    /// Modules on the right side (fixed width)
    pub right: Vec<String>,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            left: vec![],
            center: vec!["fkeys".to_string()],
            right: vec![],
        }
    }
}

/// Clock module configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct ClockConfig {
    /// Time format string (strftime)
    pub format: String,
    /// Alternate format for tap-to-toggle (typically date)
    pub format_alt: Option<String>,
    /// Update interval in seconds
    pub interval: u64,
    /// Locale for time formatting
    pub locale: Option<String>,
}

impl Default for ClockConfig {
    fn default() -> Self {
        Self {
            format: "%I:%M %p".to_string(),
            format_alt: Some("%A, %B %d, %Y".to_string()),
            interval: 1,
            locale: None,
        }
    }
}

/// Battery module configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BatteryConfig {
    /// Display mode: "icon", "percentage", or "both"
    pub display: String,
    /// Update interval in seconds
    pub interval: u64,
    /// Low battery threshold percentage
    pub low_threshold: u32,
    /// Critical battery threshold percentage
    pub critical_threshold: u32,
}

impl Default for BatteryConfig {
    fn default() -> Self {
        Self {
            display: "both".to_string(),
            interval: 30,
            low_threshold: 20,
            critical_threshold: 10,
        }
    }
}

/// Volume module configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct VolumeConfig {
    /// Display mode: "icon", "percentage", or "both"
    pub display: String,
    /// Action on tap
    pub on_click: String,
}

impl Default for VolumeConfig {
    fn default() -> Self {
        Self {
            display: "icon".to_string(),
            on_click: "toggle-mute".to_string(),
        }
    }
}

/// Brightness module configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BrightnessConfig {
    /// Display mode: "icon", "percentage", or "both"
    pub display: String,
    /// Update interval in seconds
    pub interval: u64,
}

impl Default for BrightnessConfig {
    fn default() -> Self {
        Self {
            display: "icon".to_string(),
            interval: 5,
        }
    }
}

/// Workspaces module configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct WorkspacesConfig {
    /// Number of persistent workspace buttons to show
    pub persistent_workspaces: u32,
    /// Show only active workspaces
    pub active_only: bool,
    /// Format for workspace labels
    pub format: String,
}

impl Default for WorkspacesConfig {
    fn default() -> Self {
        Self {
            persistent_workspaces: 5,
            active_only: false,
            format: "{id}".to_string(),
        }
    }
}

/// F-Keys module configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct FKeysConfig {
    /// Whether to show media keys by default (vs F-keys)
    pub media_layer_default: bool,
    /// Custom key definitions
    pub keys: Option<Vec<FKeyDefinition>>,
    /// Custom media key definitions
    pub media_keys: Option<Vec<FKeyDefinition>>,
}

impl Default for FKeysConfig {
    fn default() -> Self {
        Self {
            media_layer_default: false,
            keys: None,
            media_keys: None,
        }
    }
}

/// Single F-key definition
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct FKeyDefinition {
    /// Display label
    pub label: Option<String>,
    /// Icon name (SVG)
    pub icon: Option<String>,
    /// Key action to emit
    pub action: String,
    /// Width multiplier
    #[serde(default = "default_stretch")]
    pub stretch: usize,
}

fn default_stretch() -> usize {
    1
}

impl VulcanBarConfig {
    /// Get module configuration as a specific type
    pub fn get_module_config<T: for<'de> Deserialize<'de> + Default>(
        &self,
        module_name: &str,
    ) -> T {
        self.modules
            .get(module_name)
            .and_then(|v| v.clone().try_into().ok())
            .unwrap_or_default()
    }

    /// Merge another config into this one (other takes precedence)
    pub fn merge(&mut self, other: VulcanBarConfig) {
        // Merge general settings (other overwrites)
        if other.general.font != GeneralConfig::default().font {
            self.general.font = other.general.font;
        }
        if other.general.font_size != GeneralConfig::default().font_size {
            self.general.font_size = other.general.font_size;
        }
        if other.general.enable_pixel_shift != GeneralConfig::default().enable_pixel_shift {
            self.general.enable_pixel_shift = other.general.enable_pixel_shift;
        }
        if other.general.show_button_outlines != GeneralConfig::default().show_button_outlines {
            self.general.show_button_outlines = other.general.show_button_outlines;
        }
        if other.general.adaptive_brightness != GeneralConfig::default().adaptive_brightness {
            self.general.adaptive_brightness = other.general.adaptive_brightness;
        }
        if other.general.active_brightness != GeneralConfig::default().active_brightness {
            self.general.active_brightness = other.general.active_brightness;
        }
        if other.general.spacing != GeneralConfig::default().spacing {
            self.general.spacing = other.general.spacing;
        }

        // Merge layout (if non-empty, replace entirely)
        if !other.layout.left.is_empty()
            || !other.layout.center.is_empty()
            || !other.layout.right.is_empty()
        {
            self.layout = other.layout;
        }

        // Merge pages config (other takes precedence if present)
        if other.pages.is_some() {
            self.pages = other.pages;
        }

        // Merge gestures config (other takes precedence if present)
        if other.gestures.is_some() {
            self.gestures = other.gestures;
        }

        // Merge module configs
        for (name, config) in other.modules {
            self.modules.insert(name, config);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multipage_config_parsing() {
        let toml_str = r#"
[general]
font = "Test Font"
font-size = 28.0

[pages]
default = "main"
enable-swipe = true
transition-duration-ms = 200

[[pages.list]]
name = "main"
[pages.list.layout]
left = ["workspaces"]
center = ["clock"]
right = ["battery"]

[[pages.list]]
name = "controls"
[pages.list.layout]
left = []
center = ["brightness", "volume"]
right = ["clock"]

[modules.clock]
format = "%H:%M"
"#;

        let config: VulcanBarConfig = toml::from_str(toml_str).unwrap();

        assert!(config.pages.is_some(), "pages should be Some");
        let pages = config.pages.unwrap();
        assert_eq!(pages.default, "main");
        assert!(pages.enable_swipe);
        assert_eq!(pages.list.len(), 2, "Should have 2 pages");
        assert_eq!(pages.list[0].name, "main");
        assert_eq!(pages.list[0].layout.left, vec!["workspaces"]);
        assert_eq!(pages.list[0].layout.center, vec!["clock"]);
        assert_eq!(pages.list[1].name, "controls");
    }
}
