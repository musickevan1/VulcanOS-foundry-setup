use std::path::{Path, PathBuf};
use std::fs;
use image::{ImageReader, GenericImageView};
use anyhow::{Result, Context};

const THUMBNAIL_SIZE: u32 = 200;

/// Get the cache directory for thumbnails
pub fn cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("vulcan-wallpaper")
}

/// Generate a unique cache filename for a source image
fn cache_path(source: &Path) -> PathBuf {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    source.to_string_lossy().hash(&mut hasher);

    // Include file modification time in hash
    if let Ok(metadata) = fs::metadata(source) {
        if let Ok(modified) = metadata.modified() {
            modified.hash(&mut hasher);
        }
    }

    let hash = hasher.finish();
    cache_dir().join(format!("{:016x}.png", hash))
}

/// Get cached thumbnail path if it exists
pub fn get_cached_thumbnail(source: &Path) -> Option<PathBuf> {
    let cache = cache_path(source);
    if cache.exists() {
        Some(cache)
    } else {
        None
    }
}

/// Generate thumbnail for an image, caching the result
pub fn generate_thumbnail(source: &Path) -> Result<PathBuf> {
    let cache = cache_path(source);

    // Return cached version if exists
    if cache.exists() {
        return Ok(cache);
    }

    // Ensure cache directory exists
    let cache_parent = cache.parent().unwrap();
    fs::create_dir_all(cache_parent)
        .context("Failed to create cache directory")?;

    // Load and resize image
    let img = ImageReader::open(source)
        .context("Failed to open image")?
        .decode()
        .context("Failed to decode image")?;

    // Calculate thumbnail size maintaining aspect ratio
    let (width, height) = img.dimensions();
    let ratio = (THUMBNAIL_SIZE as f32 / width as f32)
        .min(THUMBNAIL_SIZE as f32 / height as f32);
    let thumb_width = (width as f32 * ratio) as u32;
    let thumb_height = (height as f32 * ratio) as u32;

    // Resize using Lanczos3 for quality
    let thumbnail = img.resize(
        thumb_width,
        thumb_height,
        image::imageops::FilterType::Lanczos3,
    );

    // Save thumbnail
    thumbnail.save(&cache)
        .context("Failed to save thumbnail")?;

    Ok(cache)
}

/// Scan a directory for image files
pub fn scan_wallpaper_directory(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut images = Vec::new();

    if !dir.exists() {
        return Ok(images);
    }

    for entry in fs::read_dir(dir).context("Failed to read directory")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "webp" | "bmp") {
                    images.push(path);
                }
            }
        }
    }

    images.sort();
    Ok(images)
}

/// Get the default wallpaper directory
pub fn default_wallpaper_dir() -> PathBuf {
    dirs::picture_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join("Pictures"))
        .join("Wallpapers")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_scan_wallpaper_directory() {
        let dir = default_wallpaper_dir();
        let result = scan_wallpaper_directory(&dir);
        assert!(result.is_ok());
        // Just verify it doesn't crash - may or may not find files
    }

    #[test]
    fn test_cache_dir_creation() {
        let cache = cache_dir();
        fs::create_dir_all(&cache).unwrap();
        assert!(cache.exists());
    }
}
