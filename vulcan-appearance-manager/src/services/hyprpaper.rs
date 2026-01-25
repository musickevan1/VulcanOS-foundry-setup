//! Wallpaper backend service for VulcanOS
//!
//! Uses swww (Solution to your Wayland Wallpaper Woes) for wallpaper management.
//! swww provides smooth transitions and per-monitor wallpaper support.

use std::path::Path;
use std::process::Command;
use anyhow::{Result, Context};

/// Apply wallpaper to a specific monitor using swww
///
/// swww handles everything in one command - no separate preload needed.
pub fn apply_wallpaper(monitor: &str, path: &Path) -> Result<()> {
    let path_str = path.to_string_lossy().to_string();

    let output = Command::new("swww")
        .args([
            "img",
            &path_str,
            "--outputs", monitor,
            "--transition-type", "fade",
            "--transition-duration", "0.5",
        ])
        .output()
        .context("Failed to execute swww")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("swww set wallpaper failed: {}", stderr);
    }

    Ok(())
}

/// Set wallpaper for a specific monitor (alias for apply_wallpaper)
pub fn set_wallpaper(monitor: &str, path: &Path) -> Result<()> {
    apply_wallpaper(monitor, path)
}

/// Preload is a no-op for swww (kept for API compatibility)
pub fn preload(_path: &Path) -> Result<()> {
    // swww doesn't have a separate preload step
    Ok(())
}

/// Unload is a no-op for swww (kept for API compatibility)
pub fn unload(_path: &Path) -> Result<()> {
    // swww manages memory automatically
    Ok(())
}

/// List active wallpapers (monitor -> path mapping) by querying swww
pub fn list_active() -> Result<Vec<(String, String)>> {
    let output = Command::new("swww")
        .arg("query")
        .output()
        .context("Failed to execute swww query")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let active: Vec<(String, String)> = stdout
        .lines()
        .filter_map(|line| {
            // Format: ": DP-10: 1920x1080, scale: 1, currently displaying: image: /path/to/image.png"
            // We need to extract monitor name and image path
            let line = line.trim_start_matches(": ");
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() == 2 {
                let monitor = parts[0].to_string();
                // Extract path from "1920x1080, scale: 1, currently displaying: image: /path"
                if let Some(img_start) = parts[1].find("image: ") {
                    let path = parts[1][img_start + 7..].to_string();
                    return Some((monitor, path));
                }
            }
            None
        })
        .collect();

    Ok(active)
}

/// List currently loaded wallpapers (returns active wallpaper paths)
pub fn list_loaded() -> Result<Vec<String>> {
    let active = list_active()?;
    Ok(active.into_iter().map(|(_, path)| path).collect())
}
