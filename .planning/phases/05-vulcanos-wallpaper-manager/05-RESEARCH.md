# Phase 5: VulcanOS Wallpaper Manager - Research

**Researched:** 2026-01-23
**Domain:** GTK4/Libadwaita GUI application development with Rust for Wayland multi-monitor wallpaper management
**Confidence:** HIGH

## Summary

This phase requires building a native GTK4/Libadwaita GUI application in Rust for managing multi-monitor wallpapers on Hyprland. The application integrates with existing VulcanOS infrastructure (hyprpaper, hyprmon-desc profiles) and provides visual monitor layout representation, per-monitor wallpaper assignment, profile management, and adaptive wallpaper generation.

The standard stack is GTK4 with Libadwaita via the gtk4-rs Rust bindings (minimum Rust 1.83). For reactive GUI architecture, Relm4 provides an Elm-inspired framework that simplifies state management and UI updates. Hyprpaper's IPC protocol via hyprctl enables dynamic wallpaper changes without daemon restarts. Monitor layout information comes from hyprctl monitors JSON output.

Key implementation requirements include: using AdwApplicationWindow (not GtkWindow), implementing weak references to prevent memory cycles, using Rust's image crate or magick-rust for thumbnail generation, and syncing wallpaper profiles to match hyprmon-desc monitor profiles.

**Primary recommendation:** Use Relm4 framework with libadwaita components, hyprctl IPC for wallpaper control, and the image crate for thumbnail generation. Structure as Model-Component-View with async support for background operations.

## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| gtk4 | 1:4.20.3 | GUI toolkit | Official GTK4 in Arch repos, Wayland-native, modern widgets |
| libadwaita | 1.6+ | GNOME HIG widgets | VulcanOS design language, adaptive layouts, dark mode support |
| gtk4-rs | 0.9+ | Rust GTK4 bindings | Official gtk-rs project, safe bindings, actively maintained |
| relm4 | 1.92.0+ | Reactive GUI framework | Elm-inspired state management, reduces boilerplate, async support |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| image | 0.25+ | Image processing/thumbnails | Pure Rust, fast, no external deps |
| magick-rust | 1.4+ | ImageMagick bindings | Complex image ops (panoramic splitting) |
| serde/serde_json | latest | Config serialization | Profile storage, hyprctl JSON parsing |
| tokio | 1.0+ | Async runtime | Background tasks (image loading, IPC calls) |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Relm4 | Raw gtk4-rs | More control but significantly more boilerplate and manual state management |
| image crate | imagemagick CLI | CLI simpler but slower, no in-process control |
| Libadwaita | Plain GTK4 | More flexibility but loses GNOME integration and adaptive features |

**Installation:**
```bash
# System dependencies
sudo pacman -S gtk4 libadwaita imagemagick

# Rust crates (in Cargo.toml)
cargo add gtk4 --rename gtk --features v4_20
cargo add libadwaita --rename adw --features v1_8
cargo add relm4 --features libadwaita
cargo add image
cargo add serde --features derive
cargo add serde_json
cargo add tokio --features full
```

## Architecture Patterns

### Recommended Project Structure
```
vulcan-wallpaper-manager/
├── src/
│   ├── main.rs              # Application entry, Relm4 initialization
│   ├── app.rs               # Main application model/component
│   ├── components/          # Relm4 UI components
│   │   ├── monitor_layout.rs    # Visual monitor arrangement widget
│   │   ├── wallpaper_picker.rs  # File selection and preview
│   │   ├── profile_manager.rs   # Profile save/load UI
│   │   └── image_preview.rs     # Thumbnail display widget
│   ├── models/              # Data structures
│   │   ├── monitor.rs           # Monitor info from hyprctl
│   │   ├── wallpaper.rs         # Wallpaper metadata
│   │   └── profile.rs           # Profile configuration
│   ├── services/            # Business logic
│   │   ├── hyprctl.rs           # hyprctl IPC wrapper
│   │   ├── hyprpaper.rs         # hyprpaper IPC commands
│   │   ├── image_processor.rs   # Thumbnail generation, splitting
│   │   └── profile_storage.rs   # Profile persistence
│   └── ui/                  # Relm4 view templates (optional)
├── resources/               # Desktop entry, icons
└── Cargo.toml
```

### Pattern 1: Relm4 Component-Based Architecture
**What:** Decompose UI into Relm4 components with isolated state and message passing
**When to use:** Building complex UIs with multiple interactive sections
**Example:**
```rust
// Source: https://relm4.org/book/stable/ (Relm4 official docs)
use relm4::prelude::*;

#[derive(Debug)]
enum AppMsg {
    MonitorSelected(String),
    WallpaperChosen(PathBuf),
    ApplyWallpaper,
}

struct AppModel {
    monitors: Vec<Monitor>,
    selected_monitor: Option<String>,
    wallpaper_path: Option<PathBuf>,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("VulcanOS Wallpaper Manager"),
            set_default_size: (900, 600),

            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},

                #[wrap(Some)]
                set_content = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    // Monitor layout visualization component
                    MonitorLayoutComponent {},

                    // Wallpaper picker component
                    WallpaperPickerComponent {},
                }
            }
        }
    }

    fn update(&mut self, msg: AppMsg, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::MonitorSelected(name) => {
                self.selected_monitor = Some(name);
            }
            AppMsg::WallpaperChosen(path) => {
                self.wallpaper_path = Some(path);
            }
            AppMsg::ApplyWallpaper => {
                if let (Some(mon), Some(path)) = (&self.selected_monitor, &self.wallpaper_path) {
                    // Call hyprpaper IPC
                    apply_wallpaper_async(mon.clone(), path.clone());
                }
            }
        }
    }
}
```

### Pattern 2: Async Background Operations
**What:** Use Relm4's async support for non-blocking image processing and IPC
**When to use:** Loading images, generating thumbnails, calling external commands
**Example:**
```rust
// Source: https://relm4.org/book/stable/ (async chapter)
use relm4::prelude::*;

impl SimpleComponent for AppModel {
    fn update(&mut self, msg: AppMsg, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::LoadWallpapers => {
                sender.oneshot_command(async move {
                    // This runs in background thread
                    let wallpapers = scan_wallpaper_directory().await;
                    AppMsg::WallpapersLoaded(wallpapers)
                });
            }
            AppMsg::WallpapersLoaded(wallpapers) => {
                self.wallpapers = wallpapers;
            }
        }
    }
}
```

### Pattern 3: Monitor Layout Visualization with DrawingArea
**What:** Use GTK4 DrawingArea with Cairo to render monitor positions
**When to use:** Displaying spatial monitor arrangement with drag-and-drop
**Example:**
```rust
// Source: https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/
// (DrawingArea documentation)
use gtk::prelude::*;

fn create_monitor_layout(monitors: &[Monitor]) -> gtk::DrawingArea {
    let drawing_area = gtk::DrawingArea::new();
    let monitors_clone = monitors.to_vec();

    drawing_area.set_draw_func(move |_, cr, width, height| {
        // Scale monitor positions to fit widget
        let scale = calculate_scale(&monitors_clone, width, height);

        for monitor in &monitors_clone {
            let x = monitor.x as f64 * scale;
            let y = monitor.y as f64 * scale;
            let w = monitor.width as f64 * scale;
            let h = monitor.height as f64 * scale;

            // Draw monitor rectangle
            cr.rectangle(x, y, w, h);
            cr.set_source_rgb(0.2, 0.4, 0.8);
            cr.fill_preserve();
            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.stroke();

            // Draw monitor name
            cr.move_to(x + 5.0, y + 20.0);
            cr.show_text(&monitor.name);
        }
    });

    drawing_area
}
```

### Pattern 4: Hyprpaper IPC Integration
**What:** Execute hyprctl commands via std::process::Command and parse JSON
**When to use:** All wallpaper operations (preload, set, unload)
**Example:**
```rust
// Source: https://wiki.hypr.land/Hypr-Ecosystem/hyprpaper/ (IPC section)
use std::process::Command;

fn preload_wallpaper(path: &str) -> Result<(), String> {
    let output = Command::new("hyprctl")
        .args(["hyprpaper", "preload", path])
        .output()
        .map_err(|e| format!("Failed to execute hyprctl: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(())
}

fn set_wallpaper(monitor: &str, path: &str) -> Result<(), String> {
    let arg = format!("{},{}", monitor, path);
    let output = Command::new("hyprctl")
        .args(["hyprpaper", "wallpaper", &arg])
        .output()
        .map_err(|e| format!("Failed to execute hyprctl: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(())
}

fn list_active_wallpapers() -> Result<Vec<(String, String)>, String> {
    let output = Command::new("hyprctl")
        .args(["hyprpaper", "listactive"])
        .output()
        .map_err(|e| format!("Failed to execute hyprctl: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Parse output: "monitor = path" format
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
```

### Anti-Patterns to Avoid
- **Using GtkWindow directly:** Always use AdwApplicationWindow for libadwaita integration. Using GtkWindow:titlebar or GtkWindow:child will crash.
- **Strong reference cycles:** GTK widgets in closures create reference cycles. Always use `#[weak]` macro or manual weak references.
- **Blocking the UI thread:** Never run image processing or IPC calls synchronously in event handlers. Use Relm4's async commands.
- **Manual child property:** Don't set AdwWindow:child directly. Use AdwWindow:content property instead.
- **Ignoring HiDPI scaling:** Monitor dimensions from hyprctl include scaling. Account for this when visualizing layout.

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Reactive UI updates | Manual widget mutation in callbacks | Relm4 message/update system | Reference cycles, state inconsistency, boilerplate explosion |
| Image thumbnail generation | Custom image loading/scaling | `image` crate's thumbnail() | Proper aspect ratio, format support, optimized resizing |
| File picker dialogs | GtkFileChooserDialog manually | AdwFileDialog (libadwaita 1.6+) | Adaptive, async-first API, better mobile support |
| Monitor layout parsing | String parsing of hyprctl output | Parse JSON with serde_json | Fragile regex vs structured data, type safety |
| Async operations | Manual thread spawning | Relm4 oneshot_command/command | Proper GTK main loop integration, message passing |
| Profile storage | Custom config format | TOML with serde | Human-readable, ecosystem standard, validation |

**Key insight:** GTK4 + Rust requires careful memory management patterns that are non-obvious. Relm4 abstracts these patterns (weak refs, message passing, async integration) preventing common bugs. Image processing has subtle edge cases (formats, color spaces, metadata) that battle-tested libraries handle.

## Common Pitfalls

### Pitfall 1: Reference Counting Cycles with Widgets
**What goes wrong:** Closures capture strong references to widgets, widgets hold closures, creating cycles that prevent deallocation and cause memory leaks.
**Why it happens:** Signal handlers require 'static lifetime, tempting developers to clone everything into the closure.
**How to avoid:** Use Relm4's `#[weak]` macro or manual `glib::clone!` macro with weak references. The pattern: one strong reference (in component state), weak references everywhere else.
**Warning signs:** Memory usage growing over time, widgets not being destroyed, closures still executing after widget "destruction".

### Pitfall 2: Thread Safety with GTK
**What goes wrong:** Calling GTK methods from background threads panics with "GTK may only be used from the main thread".
**Why it happens:** GTK is not thread-safe (no Send/Sync traits). Background tasks try to update UI directly.
**How to avoid:** Use Relm4's command system or glib::spawn_future for async operations. Send messages back to main thread for UI updates.
**Warning signs:** Random panics during async operations, "send on a destroyed channel" errors.

### Pitfall 3: AdwWindow Content Property Misuse
**What goes wrong:** Application crashes on startup with cryptic assertion failure when using GtkWindow APIs.
**Why it happens:** AdwWindow requires `content` property, not `child` or `titlebar`. Libadwaita docs explicitly state using these will crash.
**How to avoid:** Always use `AdwApplicationWindow::set_content()` and `AdwToolbarView` for header areas.
**Warning signs:** Immediate crash on window creation, assertion failures mentioning titlebar.

### Pitfall 4: Monitor Resolution vs Logical Size Confusion
**What goes wrong:** Wallpaper appears wrong size or aspect ratio distorted on HiDPI displays.
**Why it happens:** hyprctl reports both physical resolution (3072x1920) and logical size after scaling (1920x1200 @ 1.6 scale). Mixing these breaks layout calculations.
**How to avoid:** Use logical dimensions for UI visualization, physical dimensions for wallpaper generation. Parse both `width`/`height` and `scale` from hyprctl JSON.
**Warning signs:** MacBook display wallpapers stretched, monitor layout visualization not matching reality.

### Pitfall 5: Hyprpaper Preload Memory Management
**What goes wrong:** Application memory usage grows unbounded as wallpapers are previewed.
**Why it happens:** hyprpaper preloads images into memory. Forgetting to unload unused images leaks memory in hyprpaper daemon.
**How to avoid:** Call `hyprctl hyprpaper unload <path>` for previewed but not-applied wallpapers. Use `listloaded` to audit preloaded images.
**Warning signs:** hyprpaper process memory growing, system slowdown after browsing many wallpapers.

### Pitfall 6: Async Image Loading Without Cancellation
**What goes wrong:** Rapidly switching wallpaper previews loads all images even though user moved on, causing lag.
**Why it happens:** Spawned async tasks continue even when user navigates away. No cancellation mechanism.
**How to avoid:** Use Relm4's command handles to track in-flight operations. Cancel previous load when new selection made.
**Warning signs:** UI freezing when quickly clicking through wallpapers, thumbnails appearing out of order.

## Code Examples

Verified patterns from official sources:

### Monitor Information from hyprctl
```rust
// Source: https://wiki.hypr.land/Hypr-Ecosystem/hyprpaper/
// hyprctl monitors -j output structure
use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize, Clone)]
pub struct Monitor {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub make: String,
    pub model: String,
    pub width: u32,      // Physical resolution
    pub height: u32,     // Physical resolution
    pub x: i32,          // Position in layout
    pub y: i32,          // Position in layout
    pub scale: f64,      // HiDPI scaling factor
    pub transform: u32,  // Rotation: 0=normal, 1=90°, 2=180°, 3=270°
    pub focused: bool,
}

pub fn get_monitors() -> Result<Vec<Monitor>, String> {
    let output = Command::new("hyprctl")
        .args(["monitors", "-j"])
        .output()
        .map_err(|e| format!("Failed to execute hyprctl: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let monitors: Vec<Monitor> = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(monitors)
}

// Calculate logical dimensions for UI display
pub fn logical_size(monitor: &Monitor) -> (f64, f64) {
    let logical_width = monitor.width as f64 / monitor.scale;
    let logical_height = monitor.height as f64 / monitor.scale;
    (logical_width, logical_height)
}
```

### Thumbnail Generation with image Crate
```rust
// Source: https://docs.rs/image/ (official image crate docs)
use image::ImageReader;
use std::path::Path;

pub fn generate_thumbnail(
    source_path: &Path,
    output_path: &Path,
    max_width: u32,
    max_height: u32,
) -> Result<(), String> {
    // Load image
    let img = ImageReader::open(source_path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    // Calculate thumbnail size maintaining aspect ratio
    let (width, height) = img.dimensions();
    let ratio = (max_width as f32 / width as f32)
        .min(max_height as f32 / height as f32);
    let thumb_width = (width as f32 * ratio) as u32;
    let thumb_height = (height as f32 * ratio) as u32;

    // Resize using Lanczos3 (high quality)
    let thumbnail = img.resize(
        thumb_width,
        thumb_height,
        image::imageops::FilterType::Lanczos3,
    );

    // Save thumbnail
    thumbnail.save(output_path)
        .map_err(|e| format!("Failed to save thumbnail: {}", e))?;

    Ok(())
}
```

### Profile Persistence with TOML
```rust
// Source: VulcanOS existing profile patterns
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct WallpaperProfile {
    pub name: String,
    pub monitor_wallpapers: HashMap<String, PathBuf>,
}

impl WallpaperProfile {
    pub fn save(&self, profile_dir: &Path) -> Result<(), String> {
        let path = profile_dir.join(format!("{}.toml", self.name));
        let toml = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize: {}", e))?;

        fs::write(&path, toml)
            .map_err(|e| format!("Failed to write file: {}", e))?;

        Ok(())
    }

    pub fn load(profile_dir: &Path, name: &str) -> Result<Self, String> {
        let path = profile_dir.join(format!("{}.toml", name));
        let contents = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let profile: WallpaperProfile = toml::from_str(&contents)
            .map_err(|e| format!("Failed to parse TOML: {}", e))?;

        Ok(profile)
    }

    pub fn list_profiles(profile_dir: &Path) -> Result<Vec<String>, String> {
        let entries = fs::read_dir(profile_dir)
            .map_err(|e| format!("Failed to read directory: {}", e))?;

        let mut profiles = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    profiles.push(name.to_string());
                }
            }
        }

        Ok(profiles)
    }
}
```

### Drag and Drop File Handling
```rust
// Source: https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.DropTarget.html
use gtk::prelude::*;
use gtk::glib;

pub fn setup_wallpaper_drop_target(
    widget: &impl IsA<gtk::Widget>,
    on_drop: impl Fn(Vec<gio::File>) + 'static,
) {
    let drop_target = gtk::DropTarget::new(
        gio::File::static_type(),
        gdk::DragAction::COPY,
    );

    drop_target.connect_drop(move |_target, value, _x, _y| {
        if let Ok(file) = value.get::<gio::File>() {
            on_drop(vec![file]);
            true
        } else if let Ok(file_list) = value.get::<gdk::FileList>() {
            let files = file_list.files();
            on_drop(files);
            true
        } else {
            false
        }
    });

    widget.add_controller(drop_target);
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| GTK3 + C | GTK4 + Rust | GTK4 released 2020, gtk4-rs stable 2021 | Better type safety, memory safety, modern async |
| Manual widget creation | Relm4 declarative macros | Relm4 v0.4 released 2022 | Less boilerplate, reactive patterns |
| GtkFileChooserDialog | AdwFileDialog | libadwaita 1.6 (2024) | Async-first, adaptive for mobile |
| Imagemagick CLI | Rust image crate | image 0.24 stabilized 2021 | In-process, type-safe, no subprocess overhead |
| swww for wallpapers | hyprpaper standard | Hyprland 0.20+ standardized hyprpaper | Better IPC, memory efficient, maintained by hyprwm |

**Deprecated/outdated:**
- **GtkApplication with menubar:** libadwaita doesn't support it, use app menu in header bar instead
- **GtkFileChooserDialog:** Use AdwFileDialog for modern async API and better UX
- **Manual weak reference macros:** Relm4 provides `#[weak]` attribute, simpler than raw `glib::clone!`
- **String-based hyprctl parsing:** hyprctl added JSON output, use serde instead of regex

## Open Questions

Things that couldn't be fully resolved:

1. **Adaptive wallpaper generation algorithm details**
   - What we know: VulcanOS has `split-wallpaper.sh` using ImageMagick to crop panoramic images
   - What's unclear: Optimal seamless blending at monitor boundaries, handling monitors at different DPI scales
   - Recommendation: Start with simple crop-based approach from existing script, add intelligent scaling/blending as enhancement

2. **Profile sync mechanism to archiso skeleton**
   - What we know: Desktop profiles stored in `~/.config/hyprmon-desc/profiles/`, skeleton at `archiso/airootfs/etc/skel/`
   - What's unclear: Should profiles auto-sync, or manual export? How to handle user-specific wallpaper paths?
   - Recommendation: Implement export function that copies profiles with relative paths, document manual sync process

3. **Integration with existing vulcan-menu submenu structure**
   - What we know: vulcan-menu has wallpaper submenu calling vulcan-wallpaper-menu (bash wofi script)
   - What's unclear: Replace bash script entirely or keep as alternative? GUI vs TUI preference
   - Recommendation: GUI becomes primary, keep bash script for terminal users, update vulcan-menu to launch GUI first with fallback

4. **Real-time preview rendering performance**
   - What we know: Drawing wallpaper previews in monitor layout requires loading/scaling images
   - What's unclear: Will GTK4 Picture widgets handle 5+ monitors with live previews without lag?
   - Recommendation: Implement async thumbnail loading with placeholders, cache thumbnails in ~/.cache/vulcan-wallpaper/

## Sources

### Primary (HIGH confidence)
- [GTK4-rs Official Book](https://gtk-rs.org/gtk4-rs/stable/latest/book/) - Architecture patterns, memory management
- [Relm4 Official Documentation](https://relm4.org/book/stable/) - Component framework, async patterns
- [Hyprland Wiki - hyprpaper](https://wiki.hypr.land/Hypr-Ecosystem/hyprpaper/) - IPC protocol, commands
- [Libadwaita Official Docs - AdwApplicationWindow](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1.0/class.ApplicationWindow.html) - Window setup, content property
- [Arch Linux GTK Package Guidelines](https://wiki.archlinux.org/title/GNOME_package_guidelines) - Packaging standards

### Secondary (MEDIUM confidence)
- [MVPVM Architecture for GTK4 Rust](https://w-graj.net/posts/rust-gtk4-mvpvm/) - Application structure patterns
- [Best Practices with GTK + Rust](https://mmstick.keybase.pub/rust-gtk-practices/) - Common patterns and pitfalls
- [ripdrag - Drag and Drop Example](https://github.com/nik012003/ripdrag) - Complete GTK4 Rust drag-and-drop implementation
- [image crate documentation](https://docs.rs/image/) - Thumbnail generation, format support
- [magick-rust crate](https://github.com/nlfiedler/magick-rust) - ImageMagick bindings for complex operations

### Tertiary (LOW confidence)
- WebSearch: "GTK4 Adwaita best practices 2026" - General ecosystem status, no specific implementation details
- WebSearch: "hyprctl hyprpaper IPC commands" - Command syntax confirmed but examples limited

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Official packages in Arch repos, mature gtk4-rs/relm4 projects, hyprpaper standard for Hyprland
- Architecture: HIGH - Official gtk4-rs book patterns, established Relm4 framework, documented best practices
- Pitfalls: HIGH - Direct experience patterns from official docs, verified memory management issues, clear anti-patterns

**Research date:** 2026-01-23
**Valid until:** 30 days (GTK4/Rust ecosystem stable, hyprland slow-moving)

**Special considerations for VulcanOS:**
- Must integrate with existing hyprmon-desc profile structure (5 profiles: desktop, console, campus, laptop, presentation)
- Monitor layout visualization must account for rotated monitors (DP-9 vertical in desktop profile)
- HiDPI scaling critical (MacBook at 1.6 scale, external monitors at 1.0)
- Profile wallpapers should be stored in `~/Pictures/Wallpapers/profiles/<profile-name>/`
- Must sync to archiso skeleton for fresh installs (profiles included in ISO)
