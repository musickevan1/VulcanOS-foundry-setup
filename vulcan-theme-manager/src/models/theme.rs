use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Complete theme definition with all color/config fields
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Theme {
    // Metadata (3)
    pub theme_name: String,
    pub theme_id: String,
    pub theme_description: String,

    // Backgrounds (4)
    pub bg_primary: String,
    pub bg_secondary: String,
    pub bg_tertiary: String,
    pub bg_surface: String,

    // Foregrounds (3)
    pub fg_primary: String,
    pub fg_secondary: String,
    pub fg_muted: String,

    // Accents (2)
    pub accent: String,
    pub accent_alt: String,

    // ANSI colors (8)
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub purple: String,
    pub cyan: String,
    pub orange: String,
    pub pink: String,

    // Bright ANSI (6)
    pub bright_red: String,
    pub bright_green: String,
    pub bright_yellow: String,
    pub bright_blue: String,
    pub bright_purple: String,
    pub bright_cyan: String,

    // UI colors (4)
    pub border_active: String,
    pub border_inactive: String,
    pub selection: String,
    pub cursor: String,

    // Gradients (2)
    pub gradient_start: String,
    pub gradient_end: String,

    // System themes (4)
    pub gtk_theme: String,
    pub icon_theme: String,
    pub cursor_theme: String,
    pub kvantum_theme: String,

    // Editor (1)
    pub nvim_colorscheme: String,

    // Wallpaper (1, optional)
    pub theme_wallpaper: Option<String>,

    // Source file path (for editing)
    #[serde(skip)]
    pub source_path: Option<PathBuf>,

    // Is this a built-in theme?
    #[serde(skip)]
    pub is_builtin: bool,
}

impl Theme {
    /// Create a new theme with default values (dark base)
    pub fn new(name: &str, id: &str) -> Self {
        Self {
            theme_name: name.to_string(),
            theme_id: id.to_string(),
            theme_description: format!("Custom theme: {}", name),
            // Dark defaults
            bg_primary: "#1c1917".to_string(),
            bg_secondary: "#292524".to_string(),
            bg_tertiary: "#44403c".to_string(),
            bg_surface: "#57534e".to_string(),
            fg_primary: "#fafaf9".to_string(),
            fg_secondary: "#a8a29e".to_string(),
            fg_muted: "#78716c".to_string(),
            accent: "#f97316".to_string(),
            accent_alt: "#fbbf24".to_string(),
            red: "#ef4444".to_string(),
            green: "#22c55e".to_string(),
            yellow: "#fbbf24".to_string(),
            blue: "#3b82f6".to_string(),
            purple: "#a855f7".to_string(),
            cyan: "#06b6d4".to_string(),
            orange: "#f97316".to_string(),
            pink: "#ec4899".to_string(),
            bright_red: "#f87171".to_string(),
            bright_green: "#4ade80".to_string(),
            bright_yellow: "#fcd34d".to_string(),
            bright_blue: "#60a5fa".to_string(),
            bright_purple: "#c084fc".to_string(),
            bright_cyan: "#22d3ee".to_string(),
            border_active: "#f97316".to_string(),
            border_inactive: "#44403c".to_string(),
            selection: "#44403c".to_string(),
            cursor: "#f97316".to_string(),
            gradient_start: "#f97316".to_string(),
            gradient_end: "#fbbf24".to_string(),
            gtk_theme: "Adwaita-dark".to_string(),
            icon_theme: "Papirus-Dark".to_string(),
            cursor_theme: "Adwaita".to_string(),
            kvantum_theme: "KvArcDark".to_string(),
            nvim_colorscheme: "tokyonight-night".to_string(),
            theme_wallpaper: None,
            source_path: None,
            is_builtin: false,
        }
    }

    /// Get the primary colors for preview display (first 8 representative colors)
    pub fn preview_colors(&self) -> Vec<&str> {
        vec![
            &self.bg_primary,
            &self.bg_secondary,
            &self.accent,
            &self.accent_alt,
            &self.red,
            &self.green,
            &self.blue,
            &self.purple,
        ]
    }

    /// Get ANSI terminal colors for preview
    pub fn ansi_colors(&self) -> Vec<&str> {
        vec![
            &self.red,
            &self.green,
            &self.yellow,
            &self.blue,
            &self.purple,
            &self.cyan,
            &self.orange,
            &self.pink,
        ]
    }
}
