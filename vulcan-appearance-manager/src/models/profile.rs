use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use anyhow::Result;

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

    pub fn save(&self, profile_dir: &Path) -> Result<()> {
        let path = profile_dir.join(format!("{}.toml", self.name));
        let toml = toml::to_string_pretty(self)?;
        fs::write(&path, toml)?;
        Ok(())
    }

    pub fn load(profile_dir: &Path, name: &str) -> Result<Self> {
        let path = profile_dir.join(format!("{}.toml", name));
        let contents = fs::read_to_string(&path)?;
        let profile: WallpaperProfile = toml::from_str(&contents)?;
        Ok(profile)
    }

    pub fn list_profiles(profile_dir: &Path) -> Result<Vec<String>> {
        let mut profiles = Vec::new();
        if profile_dir.exists() {
            for entry in fs::read_dir(profile_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                    if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                        profiles.push(name.to_string());
                    }
                }
            }
        }
        Ok(profiles)
    }
}
