mod models;
mod services;

use anyhow::Result;

fn main() -> Result<()> {
    println!("VulcanOS Wallpaper Manager");

    // Test monitor detection
    let monitors = services::hyprctl::get_monitors()?;
    println!("Detected {} monitors:", monitors.len());
    for mon in &monitors {
        let (lw, lh) = mon.logical_size();
        println!("  {} @ {}x{} (logical: {:.0}x{:.0}, scale: {})",
            mon.name, mon.width, mon.height, lw, lh, mon.scale);
    }

    Ok(())
}
