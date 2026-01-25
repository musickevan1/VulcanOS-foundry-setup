use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

/// Known hyprmon-desc profile names
pub const KNOWN_PROFILES: &[&str] = &["desktop", "console", "campus", "laptop", "presentation"];

/// Wallpaper profile configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperProfile {
    pub name: String,
    /// Map of monitor name -> wallpaper path
    pub monitor_wallpapers: HashMap<String, PathBuf>,
    /// Optional description
    #[serde(default)]
    pub description: String,
}

impl WallpaperProfile {
    pub fn new(name: String) -> Self {
        Self {
            name,
            monitor_wallpapers: HashMap::new(),
            description: String::new(),
        }
    }

    pub fn with_wallpapers(name: String, wallpapers: HashMap<String, PathBuf>) -> Self {
        Self {
            name,
            monitor_wallpapers: wallpapers,
            description: String::new(),
        }
    }
}

/// Get the profile configuration directory
pub fn profile_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("vulcan-wallpaper")
        .join("profiles")
}

/// Ensure the profile directory exists
pub fn ensure_profile_dir() -> Result<PathBuf> {
    let dir = profile_dir();
    fs::create_dir_all(&dir).context("Failed to create profile directory")?;
    Ok(dir)
}

/// Save a profile to disk
pub fn save_profile(profile: &WallpaperProfile) -> Result<PathBuf> {
    let dir = ensure_profile_dir()?;
    let path = dir.join(format!("{}.toml", profile.name));

    let toml = toml::to_string_pretty(profile)
        .context("Failed to serialize profile")?;

    fs::write(&path, toml)
        .context("Failed to write profile file")?;

    Ok(path)
}

/// Load a profile from disk
pub fn load_profile(name: &str) -> Result<WallpaperProfile> {
    let dir = profile_dir();
    let path = dir.join(format!("{}.toml", name));

    let contents = fs::read_to_string(&path)
        .context("Failed to read profile file")?;

    let profile: WallpaperProfile = toml::from_str(&contents)
        .context("Failed to parse profile TOML")?;

    Ok(profile)
}

/// List all available profiles
pub fn list_profiles() -> Result<Vec<String>> {
    let dir = profile_dir();

    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut profiles = Vec::new();

    for entry in fs::read_dir(&dir).context("Failed to read profile directory")? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                profiles.push(name.to_string());
            }
        }
    }

    profiles.sort();
    Ok(profiles)
}

/// Delete a profile
pub fn delete_profile(name: &str) -> Result<()> {
    let dir = profile_dir();
    let path = dir.join(format!("{}.toml", name));

    if path.exists() {
        fs::remove_file(&path).context("Failed to delete profile")?;
    }

    Ok(())
}

/// Ensure all known profile files exist (creates empty profiles if missing)
pub fn ensure_known_profiles() {
    if let Ok(dir) = ensure_profile_dir() {
        for name in KNOWN_PROFILES {
            let path = dir.join(format!("{}.toml", name));
            if !path.exists() {
                // Create empty profile
                let profile = WallpaperProfile {
                    name: name.to_string(),
                    monitor_wallpapers: HashMap::new(),
                    description: format!("VulcanOS {} profile", name),
                };
                let _ = save_profile(&profile);
            }
        }
    }
}

/// Get the current profile name based on monitor count
/// This matches the logic in vulcan-wallpaper-menu
pub fn detect_current_profile() -> Option<String> {
    // Read from cache file if exists
    let cache_file = dirs::cache_dir()
        .map(|d| d.join("vulcan-current-profile"));

    if let Some(path) = cache_file {
        if let Ok(name) = fs::read_to_string(&path) {
            let name = name.trim();
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
    }

    // Fallback: detect based on monitor count
    if let Ok(output) = std::process::Command::new("hyprctl")
        .args(["monitors", "-j"])
        .output()
    {
        if let Ok(monitors) = serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout) {
            let count = monitors.len();
            return Some(match count {
                5 => "desktop",
                4 => "console",
                2 => "campus",
                1 => "laptop",
                _ => "desktop",
            }.to_string());
        }
    }

    None
}

/// Save the current profile name to cache
pub fn set_current_profile(name: &str) -> Result<()> {
    if let Some(cache_dir) = dirs::cache_dir() {
        let path = cache_dir.join("vulcan-current-profile");
        fs::write(&path, name).context("Failed to save current profile")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_profile_roundtrip() {
        // Create test profile
        let mut wallpapers = HashMap::new();
        wallpapers.insert("eDP-1".to_string(), PathBuf::from("/home/test/wall.png"));
        wallpapers.insert("DP-5".to_string(), PathBuf::from("/home/test/wall2.png"));

        let profile = WallpaperProfile::with_wallpapers(
            "test-roundtrip".to_string(),
            wallpapers.clone(),
        );

        // Save
        let result = save_profile(&profile);
        assert!(result.is_ok());

        // Load
        let loaded = load_profile("test-roundtrip");
        assert!(loaded.is_ok());

        let loaded = loaded.unwrap();
        assert_eq!(loaded.name, "test-roundtrip");
        assert_eq!(loaded.monitor_wallpapers, wallpapers);

        // Cleanup
        let _ = delete_profile("test-roundtrip");
    }

    #[test]
    fn test_list_profiles() {
        let result = list_profiles();
        assert!(result.is_ok());
        // Just verify it doesn't crash
    }

    #[test]
    fn test_known_profiles() {
        assert!(KNOWN_PROFILES.contains(&"desktop"));
        assert!(KNOWN_PROFILES.contains(&"laptop"));
        assert!(KNOWN_PROFILES.contains(&"campus"));
    }
}
