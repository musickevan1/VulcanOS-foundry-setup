use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::models::Theme;
use crate::services::{theme_parser, theme_applier};

/// Get home directory reliably
fn get_home() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
        .or_else(|| dirs::home_dir())
}

/// Get the themes directory (user's config)
pub fn themes_dir() -> PathBuf {
    get_home()
        .map(|h| h.join(".config"))
        .unwrap_or_else(|| PathBuf::from("."))
        .join("themes")
        .join("colors")
}

/// Get the custom themes directory for user-created themes
pub fn custom_themes_dir() -> PathBuf {
    themes_dir().join("custom")
}

/// List all available theme IDs and names from vulcan-theme CLI
pub fn list_theme_ids() -> Result<Vec<(String, String)>> {
    let vulcan_theme = theme_applier::find_vulcan_theme()?;

    let output = std::process::Command::new(&vulcan_theme)
        .arg("list-ids")
        .output()
        .context("Failed to run vulcan-theme list-ids")?;

    if !output.status.success() {
        anyhow::bail!("vulcan-theme list-ids failed");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut themes = Vec::new();

    for line in stdout.lines() {
        if let Some((id, name)) = line.split_once(':') {
            themes.push((id.to_string(), name.to_string()));
        }
    }

    Ok(themes)
}

/// Load all themes from the themes directory
pub fn load_all_themes() -> Result<Vec<Theme>> {
    let themes_path = themes_dir();
    let mut themes = Vec::new();

    if !themes_path.exists() {
        return Ok(themes);
    }

    // Read all .sh files in the colors directory
    for entry in fs::read_dir(&themes_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "sh") {
            match theme_parser::parse_and_validate(&path) {
                Ok(mut theme) => {
                    // Mark as builtin (in the main colors directory)
                    theme.is_builtin = true;
                    themes.push(theme);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to parse theme {}: {}", path.display(), e);
                }
            }
        }
    }

    // Also load custom themes if directory exists
    let custom_path = custom_themes_dir();
    if custom_path.exists() {
        for entry in fs::read_dir(&custom_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "sh") {
                match theme_parser::parse_and_validate(&path) {
                    Ok(mut theme) => {
                        theme.is_builtin = false;
                        themes.push(theme);
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to parse custom theme {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    // Sort by name
    themes.sort_by(|a, b| a.theme_name.cmp(&b.theme_name));

    Ok(themes)
}

/// Load a single theme by ID
pub fn load_theme(theme_id: &str) -> Result<Theme> {
    let themes_path = themes_dir();
    let theme_file = themes_path.join(format!("{}.sh", theme_id));

    if theme_file.exists() {
        let mut theme = theme_parser::parse_and_validate(&theme_file)?;
        theme.is_builtin = true;
        return Ok(theme);
    }

    // Check custom themes
    let custom_file = custom_themes_dir().join(format!("{}.sh", theme_id));
    if custom_file.exists() {
        let mut theme = theme_parser::parse_and_validate(&custom_file)?;
        theme.is_builtin = false;
        return Ok(theme);
    }

    anyhow::bail!("Theme not found: {}", theme_id)
}

/// Save a theme to the custom themes directory
pub fn save_theme(theme: &Theme) -> Result<PathBuf> {
    let custom_dir = custom_themes_dir();
    fs::create_dir_all(&custom_dir)?;

    let filename = format!("{}.sh", theme.theme_id);
    let path = custom_dir.join(&filename);

    let content = theme_parser::serialize_theme(theme);
    fs::write(&path, content)?;

    // Make it executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms)?;
    }

    Ok(path)
}

/// Delete a custom theme (cannot delete builtin themes)
pub fn delete_theme(theme_id: &str) -> Result<()> {
    let custom_file = custom_themes_dir().join(format!("{}.sh", theme_id));

    if custom_file.exists() {
        fs::remove_file(&custom_file)?;
        Ok(())
    } else {
        anyhow::bail!("Cannot delete builtin theme or theme not found: {}", theme_id)
    }
}

/// Import a theme from a file path
pub fn import_theme(source_path: &Path) -> Result<Theme> {
    let mut theme = theme_parser::parse_and_validate(source_path)?;
    theme.is_builtin = false;

    // Save to custom themes directory
    save_theme(&theme)?;

    Ok(theme)
}

/// Export a theme to a specified path
pub fn export_theme(theme: &Theme, dest_path: &Path) -> Result<()> {
    let content = theme_parser::serialize_theme(theme);
    fs::write(dest_path, content)?;
    Ok(())
}
