use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

/// Get home directory reliably (works in GUI environment)
fn get_home() -> Option<PathBuf> {
    // Try $HOME first (most reliable in GUI)
    if let Ok(home) = std::env::var("HOME") {
        return Some(PathBuf::from(home));
    }
    // Fallback to dirs crate
    dirs::home_dir()
}

/// Find the vulcan-theme executable (pub for use by theme_storage)
pub fn find_vulcan_theme() -> Result<PathBuf> {
    let home = get_home();

    // Check common locations
    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Some(ref h) = home {
        // User's local bin (most likely via stow symlink)
        candidates.push(h.join(".local/bin/vulcan-theme"));
        // Direct path in VulcanOS repo
        candidates.push(h.join("VulcanOS/dotfiles/scripts/.local/bin/vulcan-theme"));
    }
    // System path
    candidates.push(PathBuf::from("/usr/local/bin/vulcan-theme"));

    for candidate in &candidates {
        if candidate.exists() {
            return Ok(candidate.clone());
        }
    }

    // Fallback: try PATH lookup (works if PATH is set correctly)
    if let Ok(output) = Command::new("which").arg("vulcan-theme").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Ok(PathBuf::from(path));
            }
        }
    }

    // Include debug info in error
    let checked = candidates.iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    anyhow::bail!("vulcan-theme not found. Checked: {}", checked)
}

/// Get the current theme ID from vulcan-theme CLI
pub fn get_current_theme() -> Result<String> {
    let vulcan_theme = find_vulcan_theme()?;

    let output = Command::new(&vulcan_theme)
        .arg("current")
        .output()
        .context("Failed to run vulcan-theme current")?;

    if !output.status.success() {
        anyhow::bail!("vulcan-theme current failed");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse "Current theme: Name (id)" format
    if let Some(start) = stdout.rfind('(') {
        if let Some(end) = stdout.rfind(')') {
            return Ok(stdout[start + 1..end].to_string());
        }
    }

    // Fallback: return trimmed output
    Ok(stdout.trim().to_string())
}

/// Preview a theme (temporary, won't be saved)
pub fn preview_theme(theme_id: &str) -> Result<()> {
    let vulcan_theme = find_vulcan_theme()?;

    let status = Command::new(&vulcan_theme)
        .arg("preview")
        .arg(theme_id)
        .status()
        .context("Failed to run vulcan-theme preview")?;

    if !status.success() {
        anyhow::bail!("Failed to preview theme: {}", theme_id);
    }

    Ok(())
}

/// Apply a theme permanently
pub fn apply_theme(theme_id: &str) -> Result<()> {
    let vulcan_theme = find_vulcan_theme()?;

    let status = Command::new(&vulcan_theme)
        .arg("set")
        .arg(theme_id)
        .status()
        .context("Failed to run vulcan-theme set")?;

    if !status.success() {
        anyhow::bail!("Failed to apply theme: {}", theme_id);
    }

    Ok(())
}

/// Revert to the saved current theme (after cancelling preview)
pub fn revert_theme() -> Result<()> {
    let current = get_current_theme()?;
    apply_theme(&current)
}
