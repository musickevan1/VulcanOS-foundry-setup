use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::models::Theme;

/// Binding mode for unified profiles - how wallpaper relates to theme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BindingMode {
    /// Theme's suggested wallpaper is active
    ThemeBound,
    /// User has overridden theme's suggestion with custom wallpaper
    CustomOverride,
    /// Theme has no wallpaper suggestion (default)
    Unbound,
}

impl Default for BindingMode {
    fn default() -> Self {
        BindingMode::Unbound
    }
}

impl BindingMode {
    /// Get display name for UI
    pub fn display_name(&self) -> &str {
        match self {
            BindingMode::ThemeBound => "Theme Wallpaper",
            BindingMode::CustomOverride => "Custom Override",
            BindingMode::Unbound => "No Theme Wallpaper",
        }
    }
}

/// Unified profile combining theme and wallpaper configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedProfile {
    /// Profile name
    pub name: String,
    /// Optional description
    #[serde(default)]
    pub description: String,
    /// Bound theme ID (None if no theme selected)
    pub theme_id: Option<String>,
    /// Per-monitor wallpapers
    pub monitor_wallpapers: HashMap<String, PathBuf>,
    /// Binding mode (how wallpaper relates to theme)
    #[serde(default)]
    pub binding_mode: BindingMode,
}

impl UnifiedProfile {
    /// Create a new unified profile
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            theme_id: None,
            monitor_wallpapers: HashMap::new(),
            binding_mode: BindingMode::Unbound,
        }
    }
}

/// Resolve theme's wallpaper path to absolute filesystem path
///
/// Extracts the theme_wallpaper field from the theme and resolves it
/// to the wallpapers directory structure. Wallpapers are stored at:
/// `dotfiles/wallpapers/{theme_id}/{filename}`
///
/// Returns None if:
/// - Theme has no wallpaper suggestion
/// - Theme has no source_path
/// - Resolved path doesn't exist
pub fn resolve_theme_wallpaper(theme: &Theme) -> Option<PathBuf> {
    // Extract theme_wallpaper field (just the filename, e.g., "catppuccin-mocha.png")
    let wallpaper_filename = theme.theme_wallpaper.as_ref()?;
    let theme_id = &theme.theme_id;

    // Get the theme source path to navigate to dotfiles directory
    // Theme files are at: dotfiles/themes/colors/{theme}.sh
    let theme_source = theme.source_path.as_ref()?;

    // Navigate up to dotfiles/ directory (3 levels: colors/ -> themes/ -> dotfiles/)
    let dotfiles_dir = theme_source.parent()?.parent()?.parent()?;

    // Build path to wallpapers directory: dotfiles/wallpapers/{theme_id}/{filename}
    let wallpaper_path = dotfiles_dir
        .join("wallpapers")
        .join(theme_id)
        .join(wallpaper_filename);

    // Return only if path exists
    if wallpaper_path.exists() {
        Some(wallpaper_path)
    } else {
        eprintln!(
            "Warning: Theme '{}' suggests wallpaper '{}' but file not found at {:?}",
            theme.theme_name, wallpaper_filename, wallpaper_path
        );
        None
    }
}
