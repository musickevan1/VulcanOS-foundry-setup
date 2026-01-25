//! Wallpaper backend abstraction for VulcanOS
//!
//! Provides a unified interface for wallpaper management that works with both:
//! - swww (preferred): Smooth transitions, better animation support
//! - hyprpaper: Built-in Hyprland wallpaper daemon
//!
//! The abstraction allows runtime detection and seamless switching between backends.

use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use anyhow::{Result, Context, bail};

/// Wallpaper backend abstraction.
/// Allows the app to work with either swww or hyprpaper.
pub trait WallpaperBackend {
    /// Apply a wallpaper to a specific monitor
    fn apply(&self, monitor: &str, path: &Path) -> Result<()>;

    /// Query active wallpapers: monitor name -> image path
    fn query_active(&self) -> Result<HashMap<String, String>>;

    /// Backend name for display/logging
    fn name(&self) -> &str;
}

/// swww backend - provides smooth wallpaper transitions
pub struct SwwwBackend;

impl WallpaperBackend for SwwwBackend {
    fn apply(&self, monitor: &str, path: &Path) -> Result<()> {
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
            bail!("swww set wallpaper failed: {}", stderr);
        }

        Ok(())
    }

    fn query_active(&self) -> Result<HashMap<String, String>> {
        let output = Command::new("swww")
            .arg("query")
            .output()
            .context("Failed to execute swww query")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(parse_swww_query(&stdout))
    }

    fn name(&self) -> &str {
        "swww"
    }
}

/// hyprpaper backend - Hyprland's built-in wallpaper daemon
pub struct HyprpaperBackend;

impl WallpaperBackend for HyprpaperBackend {
    fn apply(&self, monitor: &str, path: &Path) -> Result<()> {
        let path_str = path.to_string_lossy().to_string();

        // Step 1: Preload the image
        let preload_output = Command::new("hyprctl")
            .args(["hyprpaper", "preload", &path_str])
            .output()
            .context("Failed to execute hyprctl hyprpaper preload")?;

        if !preload_output.status.success() {
            let stderr = String::from_utf8_lossy(&preload_output.stderr);
            bail!("hyprpaper preload failed: {}", stderr);
        }

        // Step 2: Set wallpaper for monitor
        let wallpaper_arg = format!("{},{}", monitor, path_str);
        let wallpaper_output = Command::new("hyprctl")
            .args(["hyprpaper", "wallpaper", &wallpaper_arg])
            .output()
            .context("Failed to execute hyprctl hyprpaper wallpaper")?;

        if !wallpaper_output.status.success() {
            let stderr = String::from_utf8_lossy(&wallpaper_output.stderr);
            bail!("hyprpaper set wallpaper failed: {}", stderr);
        }

        Ok(())
    }

    fn query_active(&self) -> Result<HashMap<String, String>> {
        let output = Command::new("hyprctl")
            .args(["hyprpaper", "listactive"])
            .output()
            .context("Failed to execute hyprctl hyprpaper listactive")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(parse_hyprpaper_query(&stdout))
    }

    fn name(&self) -> &str {
        "hyprpaper"
    }
}

/// Parse swww query stdout into monitor->path map
///
/// Format: "DP-10: 1920x1080, scale: 1, currently displaying: image: /path/to/image.png"
fn parse_swww_query(stdout: &str) -> HashMap<String, String> {
    stdout
        .lines()
        .filter_map(|line| {
            // Remove leading ": " if present
            let line = line.trim_start_matches(": ");

            // Split on first ": " to separate monitor from the rest
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() == 2 {
                let monitor = parts[0].to_string();

                // Extract path from "1920x1080, scale: 1, currently displaying: image: /path"
                if let Some(img_start) = parts[1].find("image: ") {
                    let path = parts[1][img_start + 7..].trim().to_string();
                    return Some((monitor, path));
                }
            }
            None
        })
        .collect()
}

/// Parse hyprpaper listactive stdout into monitor->path map
///
/// Format: "DP-10 = /path/to/image.png"
fn parse_hyprpaper_query(stdout: &str) -> HashMap<String, String> {
    stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(" = ").collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else {
                None
            }
        })
        .collect()
}

/// Detect which wallpaper backend is available and running.
/// Prefers swww (smoother transitions). Falls back to hyprpaper.
/// Returns Err if neither is available.
pub fn detect_backend() -> Result<Box<dyn WallpaperBackend>> {
    // Try swww first (preferred for better transitions)
    if let Ok(output) = Command::new("swww").arg("query").output() {
        if output.status.success() {
            return Ok(Box::new(SwwwBackend));
        }
    }

    // Fall back to hyprpaper
    if let Ok(output) = Command::new("hyprctl").args(["hyprpaper", "listactive"]).output() {
        if output.status.success() {
            return Ok(Box::new(HyprpaperBackend));
        }
    }

    bail!("No wallpaper backend found. Install swww (recommended) or start hyprpaper daemon.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swww_query_parsing() {
        let sample_output = r#": DP-10: 1920x1080, scale: 1.000000, currently displaying: image: /home/user/wallpaper1.png
: eDP-1: 2560x1600, scale: 1.000000, currently displaying: image: /home/user/wallpaper2.jpg"#;

        let result = parse_swww_query(sample_output);

        assert_eq!(result.len(), 2);
        assert_eq!(result.get("DP-10"), Some(&"/home/user/wallpaper1.png".to_string()));
        assert_eq!(result.get("eDP-1"), Some(&"/home/user/wallpaper2.jpg".to_string()));
    }

    #[test]
    fn test_hyprpaper_query_parsing() {
        let sample_output = r#"DP-10 = /home/user/wallpaper1.png
eDP-1 = /home/user/wallpaper2.jpg"#;

        let result = parse_hyprpaper_query(sample_output);

        assert_eq!(result.len(), 2);
        assert_eq!(result.get("DP-10"), Some(&"/home/user/wallpaper1.png".to_string()));
        assert_eq!(result.get("eDP-1"), Some(&"/home/user/wallpaper2.jpg".to_string()));
    }

    #[test]
    fn test_swww_query_empty() {
        let empty_output = "";
        let result = parse_swww_query(empty_output);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_hyprpaper_query_empty() {
        let empty_output = "";
        let result = parse_hyprpaper_query(empty_output);
        assert_eq!(result.len(), 0);
    }
}
