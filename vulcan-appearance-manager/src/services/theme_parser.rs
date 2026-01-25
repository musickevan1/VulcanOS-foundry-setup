use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::Path;

use crate::models::Theme;

/// Required variables that must be present in every theme file
const REQUIRED_VARS: &[&str] = &["THEME_NAME", "THEME_ID"];

/// Color fields that should be validated as hex colors (when not empty)
const COLOR_FIELDS: &[&str] = &[
    "BG_PRIMARY", "BG_SECONDARY", "BG_TERTIARY", "BG_SURFACE",
    "FG_PRIMARY", "FG_SECONDARY", "FG_MUTED",
    "ACCENT", "ACCENT_ALT",
    "RED", "GREEN", "YELLOW", "BLUE", "PURPLE", "CYAN", "ORANGE", "PINK",
    "BRIGHT_RED", "BRIGHT_GREEN", "BRIGHT_YELLOW", "BRIGHT_BLUE", "BRIGHT_PURPLE", "BRIGHT_CYAN",
    "BORDER_ACTIVE", "BORDER_INACTIVE", "SELECTION", "CURSOR",
    "GRADIENT_START", "GRADIENT_END",
];

lazy_static::lazy_static! {
    /// Regex to match: export VAR_NAME="value" or export VAR_NAME='value'
    static ref EXPORT_RE: Regex = Regex::new(r#"export\s+(\w+)\s*=\s*["']([^"']*)["']"#).unwrap();

    /// Regex to validate hex color format: #RRGGBB
    static ref HEX_COLOR_RE: Regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();

    /// Regex to validate theme_id: alphanumeric with hyphens/underscores, not starting with hyphen
    static ref THEME_ID_RE: Regex = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_-]*$").unwrap();
}

/// Parse a theme .sh file into a Theme struct
pub fn parse_theme_file(path: &Path) -> Result<Theme> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read theme file: {}", path.display()))?;

    parse_theme_content(&content, Some(path))
}

/// Parse theme content string into a Theme struct
pub fn parse_theme_content(content: &str, source_path: Option<&Path>) -> Result<Theme> {
    let mut theme = Theme::default();

    for cap in EXPORT_RE.captures_iter(content) {
        let var_name = &cap[1];
        let value = &cap[2];

        match var_name {
            // Metadata
            "THEME_NAME" => theme.theme_name = value.to_string(),
            "THEME_ID" => theme.theme_id = value.to_string(),
            "THEME_DESCRIPTION" => theme.theme_description = value.to_string(),

            // Backgrounds
            "BG_PRIMARY" => theme.bg_primary = value.to_string(),
            "BG_SECONDARY" => theme.bg_secondary = value.to_string(),
            "BG_TERTIARY" => theme.bg_tertiary = value.to_string(),
            "BG_SURFACE" => theme.bg_surface = value.to_string(),

            // Foregrounds
            "FG_PRIMARY" => theme.fg_primary = value.to_string(),
            "FG_SECONDARY" => theme.fg_secondary = value.to_string(),
            "FG_MUTED" => theme.fg_muted = value.to_string(),

            // Accents
            "ACCENT" => theme.accent = value.to_string(),
            "ACCENT_ALT" => theme.accent_alt = value.to_string(),

            // ANSI colors
            "RED" => theme.red = value.to_string(),
            "GREEN" => theme.green = value.to_string(),
            "YELLOW" => theme.yellow = value.to_string(),
            "BLUE" => theme.blue = value.to_string(),
            "PURPLE" => theme.purple = value.to_string(),
            "CYAN" => theme.cyan = value.to_string(),
            "ORANGE" => theme.orange = value.to_string(),
            "PINK" => theme.pink = value.to_string(),

            // Bright ANSI
            "BRIGHT_RED" => theme.bright_red = value.to_string(),
            "BRIGHT_GREEN" => theme.bright_green = value.to_string(),
            "BRIGHT_YELLOW" => theme.bright_yellow = value.to_string(),
            "BRIGHT_BLUE" => theme.bright_blue = value.to_string(),
            "BRIGHT_PURPLE" => theme.bright_purple = value.to_string(),
            "BRIGHT_CYAN" => theme.bright_cyan = value.to_string(),

            // UI colors
            "BORDER_ACTIVE" => theme.border_active = value.to_string(),
            "BORDER_INACTIVE" => theme.border_inactive = value.to_string(),
            "SELECTION" => theme.selection = value.to_string(),
            "CURSOR" => theme.cursor = value.to_string(),

            // Gradients
            "GRADIENT_START" => theme.gradient_start = value.to_string(),
            "GRADIENT_END" => theme.gradient_end = value.to_string(),

            // System themes
            "GTK_THEME" => theme.gtk_theme = value.to_string(),
            "ICON_THEME" => theme.icon_theme = value.to_string(),
            "CURSOR_THEME" => theme.cursor_theme = value.to_string(),
            "KVANTUM_THEME" => theme.kvantum_theme = value.to_string(),

            // Editor
            "NVIM_COLORSCHEME" => theme.nvim_colorscheme = value.to_string(),

            // Wallpaper
            "THEME_WALLPAPER" => {
                if !value.is_empty() {
                    theme.theme_wallpaper = Some(value.to_string());
                }
            }

            _ => {} // Ignore unknown variables
        }
    }

    if let Some(path) = source_path {
        theme.source_path = Some(path.to_path_buf());
    }

    Ok(theme)
}

/// Check for dangerous shell execution patterns in theme file content
fn check_dangerous_patterns(content: &str, path_display: &str) -> Result<()> {
    let dangerous_patterns = [
        ("$(", "command substitution"),
        ("`", "backtick command execution"),
        ("eval ", "eval command"),
        ("source ", "source command"),
        ("exec ", "exec command"),
        ("| ", "pipe command"),
    ];

    for (pattern, description) in &dangerous_patterns {
        if content.contains(pattern) {
            anyhow::bail!(
                "Theme file contains dangerous pattern '{}' ({}): {}",
                pattern,
                description,
                path_display
            );
        }
    }

    Ok(())
}

/// Validate a Theme struct for security and correctness
pub fn validate_theme(theme: &Theme) -> Result<()> {
    // Validate theme_name is not empty
    if theme.theme_name.trim().is_empty() {
        anyhow::bail!("THEME_NAME is required and cannot be empty");
    }

    // Validate theme_id is not empty
    if theme.theme_id.trim().is_empty() {
        anyhow::bail!("THEME_ID is required and cannot be empty");
    }

    // Validate theme_id matches safe pattern
    if !THEME_ID_RE.is_match(&theme.theme_id) {
        anyhow::bail!(
            "THEME_ID '{}' is invalid. Must start with alphanumeric and contain only alphanumeric, hyphens, or underscores",
            theme.theme_id
        );
    }

    // Validate color fields (if not empty)
    let color_map = [
        ("BG_PRIMARY", &theme.bg_primary),
        ("BG_SECONDARY", &theme.bg_secondary),
        ("BG_TERTIARY", &theme.bg_tertiary),
        ("BG_SURFACE", &theme.bg_surface),
        ("FG_PRIMARY", &theme.fg_primary),
        ("FG_SECONDARY", &theme.fg_secondary),
        ("FG_MUTED", &theme.fg_muted),
        ("ACCENT", &theme.accent),
        ("ACCENT_ALT", &theme.accent_alt),
        ("RED", &theme.red),
        ("GREEN", &theme.green),
        ("YELLOW", &theme.yellow),
        ("BLUE", &theme.blue),
        ("PURPLE", &theme.purple),
        ("CYAN", &theme.cyan),
        ("ORANGE", &theme.orange),
        ("PINK", &theme.pink),
        ("BRIGHT_RED", &theme.bright_red),
        ("BRIGHT_GREEN", &theme.bright_green),
        ("BRIGHT_YELLOW", &theme.bright_yellow),
        ("BRIGHT_BLUE", &theme.bright_blue),
        ("BRIGHT_PURPLE", &theme.bright_purple),
        ("BRIGHT_CYAN", &theme.bright_cyan),
        ("BORDER_ACTIVE", &theme.border_active),
        ("BORDER_INACTIVE", &theme.border_inactive),
        ("SELECTION", &theme.selection),
        ("CURSOR", &theme.cursor),
        ("GRADIENT_START", &theme.gradient_start),
        ("GRADIENT_END", &theme.gradient_end),
    ];

    for (field_name, value) in &color_map {
        if !value.is_empty() && !HEX_COLOR_RE.is_match(value) {
            anyhow::bail!(
                "{} has invalid hex color '{}'. Must be in format #RRGGBB",
                field_name,
                value
            );
        }
    }

    Ok(())
}

/// Parse and validate a theme file with security checks
pub fn parse_and_validate(path: &Path) -> Result<Theme> {
    // Read file content
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read theme file: {}", path.display()))?;

    // Check for dangerous patterns
    check_dangerous_patterns(&content, &path.display().to_string())?;

    // Parse theme content
    let theme = parse_theme_content(&content, Some(path))?;

    // Validate theme
    validate_theme(&theme)?;

    Ok(theme)
}

/// Serialize a Theme back to .sh file format
pub fn serialize_theme(theme: &Theme) -> String {
    let mut output = String::new();

    // Header
    output.push_str("#!/bin/bash\n");
    output.push_str(&format!("# VulcanOS - {} Theme\n", theme.theme_name));
    output.push_str("# Generated by vulcan-theme-manager\n\n");

    // Metadata
    output.push_str("# Theme metadata\n");
    output.push_str(&format!("export THEME_NAME=\"{}\"\n", theme.theme_name));
    output.push_str(&format!("export THEME_ID=\"{}\"\n", theme.theme_id));
    output.push_str(&format!("export THEME_DESCRIPTION=\"{}\"\n", theme.theme_description));
    output.push('\n');

    // Backgrounds
    output.push_str("# Background colors\n");
    output.push_str(&format!("export BG_PRIMARY=\"{}\"\n", theme.bg_primary));
    output.push_str(&format!("export BG_SECONDARY=\"{}\"\n", theme.bg_secondary));
    output.push_str(&format!("export BG_TERTIARY=\"{}\"\n", theme.bg_tertiary));
    output.push_str(&format!("export BG_SURFACE=\"{}\"\n", theme.bg_surface));
    output.push('\n');

    // Foregrounds
    output.push_str("# Foreground/text colors\n");
    output.push_str(&format!("export FG_PRIMARY=\"{}\"\n", theme.fg_primary));
    output.push_str(&format!("export FG_SECONDARY=\"{}\"\n", theme.fg_secondary));
    output.push_str(&format!("export FG_MUTED=\"{}\"\n", theme.fg_muted));
    output.push('\n');

    // Accents
    output.push_str("# Accent colors\n");
    output.push_str(&format!("export ACCENT=\"{}\"\n", theme.accent));
    output.push_str(&format!("export ACCENT_ALT=\"{}\"\n", theme.accent_alt));
    output.push('\n');

    // ANSI colors
    output.push_str("# ANSI color palette\n");
    output.push_str(&format!("export RED=\"{}\"\n", theme.red));
    output.push_str(&format!("export GREEN=\"{}\"\n", theme.green));
    output.push_str(&format!("export YELLOW=\"{}\"\n", theme.yellow));
    output.push_str(&format!("export BLUE=\"{}\"\n", theme.blue));
    output.push_str(&format!("export PURPLE=\"{}\"\n", theme.purple));
    output.push_str(&format!("export CYAN=\"{}\"\n", theme.cyan));
    output.push_str(&format!("export ORANGE=\"{}\"\n", theme.orange));
    output.push_str(&format!("export PINK=\"{}\"\n", theme.pink));
    output.push('\n');

    // Bright ANSI
    output.push_str("# Bright ANSI colors\n");
    output.push_str(&format!("export BRIGHT_RED=\"{}\"\n", theme.bright_red));
    output.push_str(&format!("export BRIGHT_GREEN=\"{}\"\n", theme.bright_green));
    output.push_str(&format!("export BRIGHT_YELLOW=\"{}\"\n", theme.bright_yellow));
    output.push_str(&format!("export BRIGHT_BLUE=\"{}\"\n", theme.bright_blue));
    output.push_str(&format!("export BRIGHT_PURPLE=\"{}\"\n", theme.bright_purple));
    output.push_str(&format!("export BRIGHT_CYAN=\"{}\"\n", theme.bright_cyan));
    output.push('\n');

    // UI colors
    output.push_str("# Window manager colors\n");
    output.push_str(&format!("export BORDER_ACTIVE=\"{}\"\n", theme.border_active));
    output.push_str(&format!("export BORDER_INACTIVE=\"{}\"\n", theme.border_inactive));
    output.push_str(&format!("export SELECTION=\"{}\"\n", theme.selection));
    output.push_str(&format!("export CURSOR=\"{}\"\n", theme.cursor));
    output.push('\n');

    // Gradients
    output.push_str("# Gradient colors\n");
    output.push_str(&format!("export GRADIENT_START=\"{}\"\n", theme.gradient_start));
    output.push_str(&format!("export GRADIENT_END=\"{}\"\n", theme.gradient_end));
    output.push('\n');

    // System themes
    output.push_str("# GTK/Qt theming\n");
    output.push_str(&format!("export GTK_THEME=\"{}\"\n", theme.gtk_theme));
    output.push_str(&format!("export ICON_THEME=\"{}\"\n", theme.icon_theme));
    output.push_str(&format!("export CURSOR_THEME=\"{}\"\n", theme.cursor_theme));
    output.push_str(&format!("export KVANTUM_THEME=\"{}\"\n", theme.kvantum_theme));
    output.push('\n');

    // Editor
    output.push_str("# Editor colorscheme\n");
    output.push_str(&format!("export NVIM_COLORSCHEME=\"{}\"\n", theme.nvim_colorscheme));
    output.push('\n');

    // Wallpaper
    output.push_str("# Optional wallpaper\n");
    if let Some(ref wallpaper) = theme.theme_wallpaper {
        output.push_str(&format!("export THEME_WALLPAPER=\"{}\"\n", wallpaper));
    } else {
        output.push_str("# export THEME_WALLPAPER=\"\"\n");
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_theme() {
        let content = r##"
export THEME_NAME="Test Theme"
export THEME_ID="test-theme"
export BG_PRIMARY="#1c1917"
export ACCENT="#f97316"
"##;
        let theme = parse_theme_content(content, None).unwrap();
        assert_eq!(theme.theme_name, "Test Theme");
        assert_eq!(theme.theme_id, "test-theme");
        assert_eq!(theme.bg_primary, "#1c1917");
        assert_eq!(theme.accent, "#f97316");
    }

    #[test]
    fn test_serialize_theme() {
        let theme = Theme::new("My Theme", "my-theme");
        let serialized = serialize_theme(&theme);
        assert!(serialized.contains(r#"export THEME_NAME="My Theme""#));
        assert!(serialized.contains(r#"export THEME_ID="my-theme""#));
    }

    // === Dangerous pattern detection tests ===

    #[test]
    fn test_rejects_command_substitution() {
        let content = r##"
export THEME_NAME="Evil Theme"
export THEME_ID="evil-$(whoami)"
export BG_PRIMARY="#1c1917"
"##;
        let result = check_dangerous_patterns(content, "test.sh");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("command substitution"));
    }

    #[test]
    fn test_rejects_backtick() {
        let content = r##"
export THEME_NAME="Evil Theme"
export THEME_ID="evil-`whoami`"
export BG_PRIMARY="#1c1917"
"##;
        let result = check_dangerous_patterns(content, "test.sh");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("backtick"));
    }

    #[test]
    fn test_rejects_eval() {
        let content = r##"
export THEME_NAME="Evil Theme"
eval "whoami"
export BG_PRIMARY="#1c1917"
"##;
        let result = check_dangerous_patterns(content, "test.sh");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("eval"));
    }

    #[test]
    fn test_rejects_pipe() {
        let content = r##"
export THEME_NAME="Evil Theme"
export THEME_ID="evil-theme" | cat
export BG_PRIMARY="#1c1917"
"##;
        let result = check_dangerous_patterns(content, "test.sh");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("pipe"));
    }

    // === Required variable tests ===

    #[test]
    fn test_rejects_missing_theme_name() {
        let mut theme = Theme::new("", "test-theme");
        theme.theme_name = String::new();
        let result = validate_theme(&theme);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("THEME_NAME"));
    }

    #[test]
    fn test_rejects_missing_theme_id() {
        let mut theme = Theme::new("Test Theme", "");
        theme.theme_id = String::new();
        let result = validate_theme(&theme);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("THEME_ID"));
    }

    // === Theme ID validation tests ===

    #[test]
    fn test_valid_theme_id() {
        let theme = Theme::new("Test Theme", "test-theme-123");
        assert!(validate_theme(&theme).is_ok());

        let theme2 = Theme::new("Test Theme", "test_theme_456");
        assert!(validate_theme(&theme2).is_ok());

        let theme3 = Theme::new("Test Theme", "TestTheme789");
        assert!(validate_theme(&theme3).is_ok());
    }

    #[test]
    fn test_rejects_theme_id_with_spaces() {
        let theme = Theme::new("Test Theme", "test theme");
        let result = validate_theme(&theme);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("THEME_ID"));
    }

    #[test]
    fn test_rejects_theme_id_with_semicolons() {
        let theme = Theme::new("Test Theme", "test;rm -rf");
        let result = validate_theme(&theme);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("THEME_ID"));
    }

    #[test]
    fn test_rejects_theme_id_starting_with_hyphen() {
        let theme = Theme::new("Test Theme", "-test-theme");
        let result = validate_theme(&theme);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("THEME_ID"));
    }

    // === Color validation tests ===

    #[test]
    fn test_valid_hex_colors() {
        let mut theme = Theme::new("Test Theme", "test-theme");
        theme.bg_primary = "#1c1917".to_string();
        theme.accent = "#f97316".to_string();
        theme.fg_primary = "#FFFFFF".to_string();
        assert!(validate_theme(&theme).is_ok());
    }

    #[test]
    fn test_rejects_invalid_hex_color() {
        let mut theme = Theme::new("Test Theme", "test-theme");
        theme.bg_primary = "rgb(28, 25, 23)".to_string();
        let result = validate_theme(&theme);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("BG_PRIMARY"));
        assert!(err_msg.contains("#RRGGBB"));
    }

    #[test]
    fn test_rejects_3_digit_hex() {
        let mut theme = Theme::new("Test Theme", "test-theme");
        theme.accent = "#fff".to_string();
        let result = validate_theme(&theme);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("ACCENT"));
    }

    #[test]
    fn test_allows_empty_color_fields() {
        let mut theme = Theme::new("Test Theme", "test-theme");
        theme.bg_primary = "#1c1917".to_string();
        theme.accent = "#f97316".to_string();
        theme.orange = String::new();
        theme.pink = String::new();
        assert!(validate_theme(&theme).is_ok());
    }

    // === Integration tests ===

    #[test]
    fn test_parse_and_validate_valid_file() {
        use std::io::Write;
        let content = r##"#!/bin/bash
export THEME_NAME="Test Theme"
export THEME_ID="test-theme"
export BG_PRIMARY="#1c1917"
export ACCENT="#f97316"
"##;
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_theme_valid.sh");
        let mut file = fs::File::create(&test_file).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_and_validate(&test_file);
        assert!(result.is_ok());
        let theme = result.unwrap();
        assert_eq!(theme.theme_name, "Test Theme");
        assert_eq!(theme.theme_id, "test-theme");

        // Cleanup
        let _ = fs::remove_file(&test_file);
    }

    #[test]
    fn test_parse_and_validate_dangerous_file() {
        use std::io::Write;
        let content = r##"#!/bin/bash
export THEME_NAME="Evil Theme"
export THEME_ID="evil-$(whoami)"
export BG_PRIMARY="#1c1917"
"##;
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_theme_evil.sh");
        let mut file = fs::File::create(&test_file).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_and_validate(&test_file);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("dangerous pattern"));

        // Cleanup
        let _ = fs::remove_file(&test_file);
    }
}
