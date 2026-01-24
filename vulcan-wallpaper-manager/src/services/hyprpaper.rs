use std::path::Path;
use std::process::Command;
use std::collections::HashSet;
use std::sync::Mutex;
use anyhow::{Result, Context};

lazy_static::lazy_static! {
    /// Track preloaded wallpapers to avoid redundant preloads
    static ref PRELOADED: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

/// Preload a wallpaper image into hyprpaper memory
pub fn preload(path: &Path) -> Result<()> {
    let path_str = path.to_string_lossy().to_string();

    // Check if already preloaded
    {
        let preloaded = PRELOADED.lock().unwrap();
        if preloaded.contains(&path_str) {
            return Ok(());
        }
    }

    let output = Command::new("hyprctl")
        .args(["hyprpaper", "preload", &path_str])
        .output()
        .context("Failed to execute hyprctl")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Don't fail if already preloaded
        if !stderr.contains("already") {
            anyhow::bail!("hyprpaper preload failed: {}", stderr);
        }
    }

    // Mark as preloaded
    {
        let mut preloaded = PRELOADED.lock().unwrap();
        preloaded.insert(path_str);
    }

    Ok(())
}

/// Set wallpaper for a specific monitor
pub fn set_wallpaper(monitor: &str, path: &Path) -> Result<()> {
    let path_str = path.to_string_lossy().to_string();
    let arg = format!("{},{}", monitor, path_str);

    let output = Command::new("hyprctl")
        .args(["hyprpaper", "wallpaper", &arg])
        .output()
        .context("Failed to execute hyprctl")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("hyprpaper set wallpaper failed: {}", stderr);
    }

    Ok(())
}

/// Apply wallpaper to a monitor (preload + set in one call)
pub fn apply_wallpaper(monitor: &str, path: &Path) -> Result<()> {
    preload(path)?;
    set_wallpaper(monitor, path)?;
    Ok(())
}

/// Unload a wallpaper from hyprpaper to free memory
pub fn unload(path: &Path) -> Result<()> {
    let path_str = path.to_string_lossy().to_string();

    let output = Command::new("hyprctl")
        .args(["hyprpaper", "unload", &path_str])
        .output()
        .context("Failed to execute hyprctl")?;

    if output.status.success() {
        // Remove from preloaded set
        let mut preloaded = PRELOADED.lock().unwrap();
        preloaded.remove(&path_str);
    }

    Ok(())
}

/// List currently loaded wallpapers
pub fn list_loaded() -> Result<Vec<String>> {
    let output = Command::new("hyprctl")
        .args(["hyprpaper", "listloaded"])
        .output()
        .context("Failed to execute hyprctl")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let loaded: Vec<String> = stdout
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    Ok(loaded)
}

/// List active wallpapers (monitor -> path mapping)
pub fn list_active() -> Result<Vec<(String, String)>> {
    let output = Command::new("hyprctl")
        .args(["hyprpaper", "listactive"])
        .output()
        .context("Failed to execute hyprctl")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let active: Vec<(String, String)> = stdout
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

    Ok(active)
}
