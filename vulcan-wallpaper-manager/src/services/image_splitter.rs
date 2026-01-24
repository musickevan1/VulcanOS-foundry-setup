use std::path::{Path, PathBuf};
use std::fs;
use image::{GenericImageView, ImageReader, DynamicImage, imageops};
use anyhow::{Result, Context};

use crate::models::Monitor;

/// Result of splitting a panoramic image
#[derive(Debug)]
pub struct SplitResult {
    /// Map of monitor name -> generated wallpaper path
    pub wallpapers: Vec<(String, PathBuf)>,
    /// Output directory containing the wallpapers
    pub output_dir: PathBuf,
}

/// Calculate the bounding box for all monitors
fn calculate_canvas_bounds(monitors: &[Monitor]) -> (i32, i32, i32, i32) {
    if monitors.is_empty() {
        return (0, 0, 1920, 1080);
    }

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for mon in monitors {
        let (lw, lh) = mon.logical_size();
        let (w, h) = if mon.is_vertical() {
            (lh as i32, lw as i32)
        } else {
            (lw as i32, lh as i32)
        };

        min_x = min_x.min(mon.x);
        min_y = min_y.min(mon.y);
        max_x = max_x.max(mon.x + w);
        max_y = max_y.max(mon.y + h);
    }

    (min_x, min_y, max_x - min_x, max_y - min_y)
}

/// Split a panoramic image into per-monitor wallpapers
pub fn split_panoramic(
    source: &Path,
    monitors: &[Monitor],
    output_dir: &Path,
    prefix: &str,
) -> Result<SplitResult> {
    // Load source image
    let img = ImageReader::open(source)
        .context("Failed to open source image")?
        .decode()
        .context("Failed to decode source image")?;

    let (img_width, img_height) = img.dimensions();
    println!("Source image: {}x{}", img_width, img_height);

    // Calculate canvas dimensions from monitor layout
    let (min_x, min_y, canvas_width, canvas_height) = calculate_canvas_bounds(monitors);
    println!("Canvas bounds: {}x{} (offset: {}, {})", canvas_width, canvas_height, min_x, min_y);

    // Scale source to fit canvas, then crop to exact size
    let scale = (canvas_width as f32 / img_width as f32)
        .max(canvas_height as f32 / img_height as f32);

    let scaled_width = (img_width as f32 * scale) as u32;
    let scaled_height = (img_height as f32 * scale) as u32;

    println!("Scaling to: {}x{}", scaled_width, scaled_height);

    // Resize using Lanczos for quality
    let scaled = img.resize_exact(
        scaled_width,
        scaled_height,
        imageops::FilterType::Lanczos3,
    );

    // Crop to canvas size (center crop)
    let crop_x = (scaled_width.saturating_sub(canvas_width as u32)) / 2;
    let crop_y = (scaled_height.saturating_sub(canvas_height as u32)) / 2;

    let canvas = imageops::crop_imm(
        &scaled,
        crop_x,
        crop_y,
        canvas_width as u32,
        canvas_height as u32,
    ).to_image();

    println!("Canvas cropped to: {}x{}", canvas.width(), canvas.height());

    // Ensure output directory exists
    fs::create_dir_all(output_dir)
        .context("Failed to create output directory")?;

    // Extract each monitor's portion
    let mut wallpapers = Vec::new();

    for mon in monitors {
        let (lw, lh) = mon.logical_size();
        let (w, h) = if mon.is_vertical() {
            (lh as u32, lw as u32)
        } else {
            (lw as u32, lh as u32)
        };

        // Calculate crop position relative to canvas origin
        let x = (mon.x - min_x) as u32;
        let y = (mon.y - min_y) as u32;

        println!("  {} @ {}x{} from ({}, {})", mon.name, w, h, x, y);

        // Ensure we don't exceed canvas bounds
        let safe_w = w.min(canvas.width().saturating_sub(x));
        let safe_h = h.min(canvas.height().saturating_sub(y));

        // Crop monitor region
        let monitor_img = imageops::crop_imm(&canvas, x, y, safe_w, safe_h).to_image();

        // If the crop is smaller than expected, resize to fill
        let final_img = if monitor_img.width() < w || monitor_img.height() < h {
            DynamicImage::ImageRgba8(monitor_img).resize_exact(
                w,
                h,
                imageops::FilterType::Lanczos3,
            )
        } else {
            DynamicImage::ImageRgba8(monitor_img)
        };

        // Save to output directory
        let filename = format!("{}-{}.png", prefix, mon.name);
        let output_path = output_dir.join(&filename);

        final_img.save(&output_path)
            .context(format!("Failed to save {}", filename))?;

        wallpapers.push((mon.name.clone(), output_path));
    }

    Ok(SplitResult {
        wallpapers,
        output_dir: output_dir.to_path_buf(),
    })
}

/// Get default output directory for split wallpapers
pub fn default_split_output_dir() -> PathBuf {
    dirs::picture_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join("Pictures"))
        .join("Wallpapers")
        .join("spanning")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_bounds_single_monitor() {
        let monitors = vec![
            Monitor {
                id: 0,
                name: "eDP-1".to_string(),
                description: "".to_string(),
                make: "".to_string(),
                model: "".to_string(),
                width: 1920,
                height: 1200,
                x: 0,
                y: 0,
                scale: 1.0,
                transform: 0,
                focused: true,
            }
        ];

        let (min_x, min_y, w, h) = calculate_canvas_bounds(&monitors);
        assert_eq!((min_x, min_y), (0, 0));
        assert_eq!((w, h), (1920, 1200));
    }

    #[test]
    fn test_canvas_bounds_multi_monitor() {
        let monitors = vec![
            Monitor {
                id: 0,
                name: "DP-5".to_string(),
                description: "".to_string(),
                make: "".to_string(),
                model: "".to_string(),
                width: 1920,
                height: 1080,
                x: 0,
                y: 0,
                scale: 1.0,
                transform: 0,
                focused: false,
            },
            Monitor {
                id: 1,
                name: "eDP-1".to_string(),
                description: "".to_string(),
                make: "".to_string(),
                model: "".to_string(),
                width: 1920,
                height: 1200,
                x: 1920,
                y: 0,
                scale: 1.0,
                transform: 0,
                focused: true,
            },
        ];

        let (min_x, min_y, w, h) = calculate_canvas_bounds(&monitors);
        assert_eq!((min_x, min_y), (0, 0));
        assert_eq!(w, 3840); // 1920 + 1920
        assert_eq!(h, 1200); // max height
    }
}
