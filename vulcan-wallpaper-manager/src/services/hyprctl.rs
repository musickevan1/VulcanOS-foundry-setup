use std::process::Command;
use anyhow::{Result, Context};
use crate::models::Monitor;

/// Get list of connected monitors from hyprctl
pub fn get_monitors() -> Result<Vec<Monitor>> {
    let output = Command::new("hyprctl")
        .args(["monitors", "-j"])
        .output()
        .context("Failed to execute hyprctl")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("hyprctl failed: {}", stderr);
    }

    let monitors: Vec<Monitor> = serde_json::from_slice(&output.stdout)
        .context("Failed to parse monitor JSON")?;

    Ok(monitors)
}

/// Preload wallpaper into hyprpaper
pub fn preload_wallpaper(path: &str) -> Result<()> {
    let output = Command::new("hyprctl")
        .args(["hyprpaper", "preload", path])
        .output()
        .context("Failed to execute hyprctl hyprpaper preload")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("hyprpaper preload failed: {}", stderr);
    }
    Ok(())
}

/// Set wallpaper for a specific monitor
pub fn set_wallpaper(monitor: &str, path: &str) -> Result<()> {
    let arg = format!("{},{}", monitor, path);
    let output = Command::new("hyprctl")
        .args(["hyprpaper", "wallpaper", &arg])
        .output()
        .context("Failed to execute hyprctl hyprpaper wallpaper")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("hyprpaper set wallpaper failed: {}", stderr);
    }
    Ok(())
}

/// Unload wallpaper from hyprpaper to free memory
pub fn unload_wallpaper(path: &str) -> Result<()> {
    let output = Command::new("hyprctl")
        .args(["hyprpaper", "unload", path])
        .output()
        .context("Failed to execute hyprctl hyprpaper unload")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("hyprpaper unload failed: {}", stderr);
    }
    Ok(())
}

/// List currently active wallpapers
pub fn list_active_wallpapers() -> Result<Vec<(String, String)>> {
    let output = Command::new("hyprctl")
        .args(["hyprpaper", "listactive"])
        .output()
        .context("Failed to execute hyprctl hyprpaper listactive")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let wallpapers: Vec<(String, String)> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(" = ").collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].to_string()))
            } else {
                None
            }
        })
        .collect();

    Ok(wallpapers)
}
