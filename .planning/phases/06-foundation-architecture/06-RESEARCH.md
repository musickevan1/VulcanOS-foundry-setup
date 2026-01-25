# Phase 6: Foundation Architecture - Research

**Researched:** 2026-01-24
**Domain:** Rust/GTK4/Relm4 crate merging, state machines, wallpaper backend abstraction, bash parsing, shared CSS
**Confidence:** HIGH (most findings verified against codebase and official docs)

## Summary

This phase merges vulcan-wallpaper-manager and vulcan-theme-manager into a single vulcan-appearance-manager crate, introduces an explicit state machine, abstracts the wallpaper backend behind a trait, hardens the theme bash parser, and creates a shared CSS module for brand colors.

The existing codebase is well-structured and the merge is straightforward: both crates use identical dependency versions (gtk4 0.9, libadwaita 0.7, relm4 0.9, anyhow 1.0, dirs 5, serde 1.0) and follow the same Relm4 SimpleComponent pattern. The wallpaper-manager is the more mature codebase (8 plans shipped) and should be the base. Theme-manager code is usable but needs hardening -- the parser lacks validation, and services call external commands without good error paths.

Both crates already contain duplicated brand CSS as inline `const VULCAN_CSS: &str` strings in their respective `main.rs` files. These are nearly identical and should be unified into a shared module that loads from the `branding/vulcan-palette.css` file or generates GTK CSS from it.

**Primary recommendation:** Rename vulcan-wallpaper-manager to vulcan-appearance-manager in Phase 6 (not Phase 7), because all subsequent phases build on this crate and renaming later causes more churn. Move theme-manager services (parser, storage, applier) and models into the renamed crate. Delete the old theme-manager crate.

## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| gtk4 | 0.9 (features: v4_16) | GTK4 bindings | Already used by both crates, locked decision |
| libadwaita | 0.7 (features: v1_6) | Adwaita widgets | Already used by both crates, locked decision |
| relm4 | 0.9 (features: libadwaita) | Reactive UI framework | Already used, locked decision |
| anyhow | 1.0 | Error handling | Already used, locked decision |
| serde | 1.0 (features: derive) | Serialization | Already used by both crates |
| toml | 0.8 | TOML serialization | Already used for profiles |
| regex | 1 | Bash script parsing | Already used by theme-manager parser |
| dirs | 5 | XDG directory paths | Already used by both crates |
| tokio | 1 (features: rt, process) | Async process execution | Already used by wallpaper-manager |
| image | 0.25 | Image handling | Already used by wallpaper-manager |
| serde_json | 1.0 | JSON (hyprctl output) | Already used by both crates |
| lazy_static | 1.4 | Lazy regex compilation | Already used by both crates |

### No New Dependencies Required

The merge does not require any new crate dependencies. All functionality can be implemented with the existing dependency set. Specifically:

- **State machine**: Use plain Rust enum + match. No external crate needed.
- **Bash parsing**: The existing `regex` crate is sufficient. Do NOT use conch-parser (archived since 2022) or yash-syntax (GPL-3.0 license, overkill for extracting `export VAR="value"` lines).
- **CSS loading**: Use `relm4::set_global_css()` which is already in both main.rs files.

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Plain enum state machine | `statig` crate (hierarchical FSM) | Overkill for 3-4 states; adds dependency for no benefit |
| Plain enum state machine | Typestate pattern (generic types) | Compile-time safety but incompatible with Relm4's model pattern (model must be a single type) |
| `regex` for bash parsing | `yash-syntax` (full POSIX parser) | GPL-3.0 license conflict; massive overkill for extracting `export VAR="value"` |
| `regex` for bash parsing | `conch-parser` (shell AST) | Archived since 2022, unmaintained, version 0.1 |
| Inline CSS strings | External `.css` file loaded at runtime | Could fail if file missing; `include_str!` at compile time is safer |

**Installation (no changes needed):**
```bash
# Cargo.toml already has all needed deps from both crates
# Just merge the two Cargo.toml dependency sections
```

## Architecture Patterns

### Recommended Project Structure After Merge

```
vulcan-appearance-manager/
├── Cargo.toml              # Merged deps from both crates
├── src/
│   ├── main.rs             # Single entry point, loads shared CSS
│   ├── app.rs              # Top-level Relm4 App (initially wallpaper-focused)
│   ├── brand_css.rs         # NEW: Shared brand CSS module
│   ├── state.rs             # NEW: Explicit state machine types
│   ├── models/
│   │   ├── mod.rs
│   │   ├── monitor.rs       # From wallpaper-manager (unchanged)
│   │   ├── wallpaper.rs     # From wallpaper-manager (unchanged)
│   │   ├── profile.rs       # From wallpaper-manager (unchanged)
│   │   ├── theme.rs         # From theme-manager (with validation added)
│   │   └── color_group.rs   # From theme-manager (unchanged)
│   ├── services/
│   │   ├── mod.rs
│   │   ├── hyprctl.rs       # From wallpaper-manager (unchanged)
│   │   ├── wallpaper_backend.rs  # NEW: Trait + swww/hyprpaper impls
│   │   ├── profile_storage.rs   # From wallpaper-manager (unchanged)
│   │   ├── thumbnail.rs     # From wallpaper-manager (unchanged)
│   │   ├── image_splitter.rs    # From wallpaper-manager (unchanged)
│   │   ├── theme_parser.rs  # From theme-manager (hardened)
│   │   ├── theme_storage.rs # From theme-manager (unchanged)
│   │   └── theme_applier.rs # From theme-manager (unchanged)
│   └── components/
│       ├── mod.rs
│       ├── monitor_layout.rs    # From wallpaper-manager
│       ├── wallpaper_picker.rs  # From wallpaper-manager
│       ├── profile_manager.rs   # From wallpaper-manager
│       ├── split_dialog.rs      # From wallpaper-manager
│       ├── theme_browser.rs     # From theme-manager
│       ├── theme_card.rs        # From theme-manager
│       ├── theme_editor.rs      # From theme-manager
│       └── preview_panel.rs     # From theme-manager
```

### Pattern 1: Enum State Machine in Relm4 Model

**What:** Use a plain Rust enum to represent application states, stored in the Relm4 model struct. State transitions happen in the `update()` method with explicit match arms that return `Result`.

**When to use:** When the app has distinct operational modes (Idle, Previewing, Applying) and invalid transitions should be caught.

**Why not typestate:** Relm4's `SimpleComponent` requires a single model type. The typestate pattern (where each state is a different generic type) is incompatible with this -- you cannot change the model's type at runtime. Use enum-based state machine with runtime validation instead.

**Example:**
```rust
// Source: Hoverbear "Pretty State Machine Patterns in Rust" + Relm4 model pattern

/// Application state machine with explicit transitions
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    /// No preview active, showing live system state
    Idle,
    /// User is previewing a change (wallpaper or theme) but hasn't applied
    Previewing {
        /// What was active before preview started (for revert)
        previous: PreviewSnapshot,
    },
    /// Currently applying changes to live system
    Applying,
    /// An error occurred during apply/preview
    Error {
        message: String,
        /// State to return to after acknowledging error
        recovery: Box<AppState>,
    },
}

/// Snapshot of state before preview, for revert
#[derive(Debug, Clone, PartialEq)]
pub struct PreviewSnapshot {
    pub wallpapers: HashMap<String, PathBuf>,
    pub theme_id: Option<String>,
}

impl AppState {
    /// Transition to Previewing state. Only valid from Idle.
    pub fn start_preview(&self, snapshot: PreviewSnapshot) -> Result<AppState> {
        match self {
            AppState::Idle => Ok(AppState::Previewing { previous: snapshot }),
            other => anyhow::bail!(
                "Cannot start preview from state: {:?}", other
            ),
        }
    }

    /// Transition to Applying state. Valid from Idle or Previewing.
    pub fn start_apply(&self) -> Result<AppState> {
        match self {
            AppState::Idle | AppState::Previewing { .. } => Ok(AppState::Applying),
            other => anyhow::bail!(
                "Cannot start apply from state: {:?}", other
            ),
        }
    }

    /// Transition back to Idle. Valid from Previewing or Applying.
    pub fn finish(&self) -> Result<AppState> {
        match self {
            AppState::Previewing { .. } | AppState::Applying => Ok(AppState::Idle),
            other => anyhow::bail!(
                "Cannot finish from state: {:?}", other
            ),
        }
    }

    /// Transition to Error state. Valid from any state.
    pub fn fail(&self, message: String) -> AppState {
        AppState::Error {
            message,
            recovery: Box::new(AppState::Idle),
        }
    }

    pub fn is_idle(&self) -> bool {
        matches!(self, AppState::Idle)
    }

    pub fn is_previewing(&self) -> bool {
        matches!(self, AppState::Previewing { .. })
    }
}
```

**Integration with Relm4 model:**
```rust
pub struct App {
    state: AppState,  // Replaces ad-hoc fields like selected_monitor, selected_wallpaper
    monitors: Vec<Monitor>,
    // ... components
}

fn update(&mut self, msg: AppMsg, sender: ComponentSender<Self>) {
    match msg {
        AppMsg::PreviewWallpaper(monitor, path) => {
            let snapshot = PreviewSnapshot {
                wallpapers: self.current_wallpapers(),
                theme_id: None,
            };
            match self.state.start_preview(snapshot) {
                Ok(new_state) => {
                    self.state = new_state;
                    // Apply preview to live system...
                }
                Err(e) => {
                    eprintln!("Invalid state transition: {}", e);
                }
            }
        }
        // ...
    }
}
```

### Pattern 2: Wallpaper Backend Trait

**What:** Abstract wallpaper operations (set, query, preload) behind a trait so the app can work with either swww or hyprpaper.

**Example:**
```rust
// Source: Existing hyprpaper.rs + swww command analysis

use std::path::Path;
use std::collections::HashMap;
use anyhow::Result;

/// Wallpaper backend abstraction
pub trait WallpaperBackend {
    /// Apply a wallpaper to a specific monitor
    fn apply(&self, monitor: &str, path: &Path) -> Result<()>;

    /// Query active wallpapers: monitor name -> image path
    fn query_active(&self) -> Result<HashMap<String, String>>;

    /// Backend name for display/logging
    fn name(&self) -> &str;
}

/// swww backend (current default)
pub struct SwwwBackend;

impl WallpaperBackend for SwwwBackend {
    fn apply(&self, monitor: &str, path: &Path) -> Result<()> {
        let output = std::process::Command::new("swww")
            .args(["img", &path.to_string_lossy(),
                   "--outputs", monitor,
                   "--transition-type", "fade",
                   "--transition-duration", "0.5"])
            .output()?;
        if !output.status.success() {
            anyhow::bail!("swww failed: {}", String::from_utf8_lossy(&output.stderr));
        }
        Ok(())
    }

    fn query_active(&self) -> Result<HashMap<String, String>> {
        let output = std::process::Command::new("swww")
            .arg("query")
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut map = HashMap::new();
        for line in stdout.lines() {
            // Format: "MONITOR_NAME: WxH, scale: S, currently displaying: image: /path"
            let line = line.trim_start_matches(": ");
            if let Some((monitor, rest)) = line.split_once(": ") {
                if let Some(img_start) = rest.find("image: ") {
                    map.insert(monitor.to_string(), rest[img_start + 7..].to_string());
                }
            }
        }
        Ok(map)
    }

    fn name(&self) -> &str { "swww" }
}

/// hyprpaper backend (alternative)
pub struct HyprpaperBackend;

impl WallpaperBackend for HyprpaperBackend {
    fn apply(&self, monitor: &str, path: &Path) -> Result<()> {
        // hyprpaper requires preload then set
        let path_str = path.to_string_lossy();
        std::process::Command::new("hyprctl")
            .args(["hyprpaper", "preload", &path_str])
            .output()?;
        let arg = format!("{},{}", monitor, path_str);
        let output = std::process::Command::new("hyprctl")
            .args(["hyprpaper", "wallpaper", &arg])
            .output()?;
        if !output.status.success() {
            anyhow::bail!("hyprpaper failed: {}", String::from_utf8_lossy(&output.stderr));
        }
        Ok(())
    }

    fn query_active(&self) -> Result<HashMap<String, String>> {
        let output = std::process::Command::new("hyprctl")
            .args(["hyprpaper", "listactive"])
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut map = HashMap::new();
        for line in stdout.lines() {
            // Format: "MONITOR = /path/to/image"
            if let Some((monitor, path)) = line.split_once(" = ") {
                map.insert(monitor.trim().to_string(), path.trim().to_string());
            }
        }
        Ok(map)
    }

    fn name(&self) -> &str { "hyprpaper" }
}

/// Detect which backend is available
pub fn detect_backend() -> Box<dyn WallpaperBackend> {
    // Check if swww daemon is running (preferred)
    if std::process::Command::new("swww").arg("query").output()
        .map(|o| o.status.success()).unwrap_or(false)
    {
        return Box::new(SwwwBackend);
    }
    // Fallback to hyprpaper
    Box::new(HyprpaperBackend)
}
```

### Pattern 3: Shared Brand CSS Module

**What:** A Rust module that generates GTK4 CSS from brand color definitions, loaded once via `relm4::set_global_css()`.

**Example:**
```rust
// brand_css.rs - Single source of truth for GTK CSS brand colors

/// Brand colors as Rust constants (parsed from branding/vulcan-palette.css conceptually)
pub mod colors {
    pub const EMBER: &str = "#f97316";
    pub const MOLTEN: &str = "#ea580c";
    pub const GOLD: &str = "#fbbf24";
    pub const FLAME: &str = "#dc2626";
    pub const OBSIDIAN: &str = "#1c1917";
    pub const CHARCOAL: &str = "#292524";
    pub const ASH: &str = "#44403c";
    pub const SMOKE: &str = "#57534e";
    pub const WHITE: &str = "#fafaf9";
    pub const STONE: &str = "#a8a29e";
    pub const GRAY: &str = "#78716c";
    pub const SUCCESS: &str = "#22c55e";
    pub const WARNING: &str = "#fbbf24";
    pub const ERROR: &str = "#ef4444";
    pub const INFO: &str = "#3b82f6";
}

/// GTK4 CSS string with @define-color declarations for brand colors.
/// Use with `relm4::set_global_css()` to make these available app-wide.
pub const BRAND_CSS: &str = r#"
/* VulcanOS Brand Colors - generated from branding/vulcan-palette.css */
@define-color vulcan_ember #f97316;
@define-color vulcan_molten #ea580c;
@define-color vulcan_gold #fbbf24;
@define-color vulcan_obsidian #1c1917;
@define-color vulcan_charcoal #292524;
@define-color vulcan_ash #44403c;
@define-color vulcan_white #fafaf9;
@define-color vulcan_stone #a8a29e;

/* Override Adwaita accent colors */
@define-color accent_bg_color @vulcan_ember;
@define-color accent_fg_color @vulcan_white;
@define-color accent_color @vulcan_ember;
"#;

/// Full application CSS: brand colors + widget styling
pub const APP_CSS: &str = include_str!("style.css");
// Alternatively, concatenate BRAND_CSS + widget CSS at compile time
```

### Anti-Patterns to Avoid

- **Do not use typestate pattern for Relm4 state:** Relm4 requires a single concrete model type. The typestate pattern encodes states as different types, which is incompatible. Use enum-based state machine instead.
- **Do not use a full shell parser for theme files:** The theme files are simple `export VAR="value"` lines. A regex-based parser is sufficient, correct, and avoids adding a GPL dependency.
- **Do not duplicate CSS across crates:** Both current main.rs files contain nearly identical 150-line CSS strings. Extract to a shared module.
- **Do not call `eval` or `source` on theme files:** Security requirement. Parse with regex only.
- **Do not create a new crate from scratch:** Extend the existing wallpaper-manager. Decision is locked.

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Wallpaper backend detection | Manual path/env checks | `Command::new("swww").arg("query").output()` | swww query exits non-zero if daemon not running |
| Color hex validation | Custom parsing | `regex` with pattern `^#[0-9a-fA-F]{6}$` | Edge cases: 3-digit hex, alpha channels, named colors |
| XDG directory paths | `$HOME/.config` manual concat | `dirs` crate (already a dependency) | Handles $XDG_CONFIG_HOME, $XDG_CACHE_HOME correctly |
| TOML profile serialization | Custom format | `toml` crate with `serde` derive (already used) | Profile roundtrip already works |
| GTK CSS loading | Custom CssProvider setup | `relm4::set_global_css()` | Handles provider creation, display registration |
| Process output parsing | Custom string splitting | Typed parsers per-backend behind trait | swww and hyprpaper have different output formats |

**Key insight:** The existing codebase already has correct implementations for most operations. The work is restructuring (moving files, renaming modules, adding trait abstraction), not reimplementing.

## Common Pitfalls

### Pitfall 1: Relm4 Model Type Constraints
**What goes wrong:** Attempting to use the typestate pattern (different types per state) in a Relm4 SimpleComponent model.
**Why it happens:** SimpleComponent requires `type Init`, `type Input`, `type Output` -- the model is a fixed struct. You cannot change its type at runtime.
**How to avoid:** Use an enum field inside the model struct. State transitions modify the enum variant, not the struct type.
**Warning signs:** Compile errors like "expected App, found App<Previewing>".

### Pitfall 2: Module Name Collisions After Merge
**What goes wrong:** Both crates have `models/mod.rs`, `services/mod.rs`, `components/mod.rs`, and `app.rs`. Naive copy creates conflicts.
**Why it happens:** Rust module system requires unique module names at each level.
**How to avoid:** Plan the merge order: (1) rename wallpaper-manager crate in Cargo.toml, (2) move theme-manager models into existing models/ module, (3) move theme-manager services into existing services/ module, (4) move theme-manager components into existing components/ module. Each step adds `pub mod` declarations.
**Warning signs:** "file not found for module" or "duplicate module" compiler errors.

### Pitfall 3: Cargo.toml Dependency Deduplication
**What goes wrong:** Merging both Cargo.toml files creates duplicate dependency entries with slightly different features.
**Why it happens:** Both crates declare the same deps but theme-manager has `regex` while wallpaper-manager has `image` and `tokio`.
**How to avoid:** Union of all dependencies. Both crates use identical versions for shared deps (gtk4 0.9, libadwaita 0.7, relm4 0.9, etc.), so no version conflicts. Just add `regex = "1"` to the merged Cargo.toml (from theme-manager) and keep all existing deps from wallpaper-manager.
**Warning signs:** Compilation errors about missing features or version mismatches.

### Pitfall 4: hyprpaper Module Name Confusion
**What goes wrong:** The wallpaper-manager has a file called `hyprpaper.rs` but it actually implements the swww backend (not hyprpaper). The `hyprctl.rs` file contains the hyprpaper IPC commands.
**Why it happens:** Historical naming -- the module was originally for hyprpaper, then swww replaced it but the filename was reused.
**How to avoid:** During the merge, rename `hyprpaper.rs` to `wallpaper_backend.rs` and make it implement the `WallpaperBackend` trait. Move hyprpaper-specific code from `hyprctl.rs` into a `HyprpaperBackend` impl.
**Warning signs:** Confusion about which backend is actually being used.

### Pitfall 5: CSS @define-color vs CSS Custom Properties
**What goes wrong:** Using CSS custom properties (`--vulcan-ember: #f97316`) in GTK4 when you should use `@define-color`.
**Why it happens:** Web CSS uses `--custom-property` syntax. GTK4 has its own `@define-color` syntax which is different from CSS custom properties. As of GTK 4.16/libadwaita 1.6, CSS variables are being introduced but `@define-color` is still the primary mechanism.
**How to avoid:** Use `@define-color vulcan_ember #f97316;` (GTK4 syntax) in the GTK CSS, not `--vulcan-ember: #f97316` (web CSS syntax). The existing code already does this correctly.
**Warning signs:** Colors not being applied; GTK CSS parsing warnings.

### Pitfall 6: Theme Parser Security -- Regex Bypass
**What goes wrong:** A crafted theme file could contain shell injection if the parsed values are later passed to `Command::new()`.
**Why it happens:** The regex `export\s+(\w+)\s*=\s*["']([^"']*)["']` extracts values between quotes, but those values are later used in commands like `vulcan-theme set <theme_id>`.
**How to avoid:** Validate extracted values: theme_id must match `^[a-zA-Z0-9_-]+$`, color values must match `^#[0-9a-fA-F]{6}$`, file paths must not contain shell metacharacters. Add a validation step after parsing and before any use.
**Warning signs:** Theme files with unusual characters in values, unexpected command behavior.

### Pitfall 7: State Machine Recovery After Failed Apply
**What goes wrong:** Apply operation fails mid-way (e.g., swww crashes), leaving the system in a partially-applied state with the state machine stuck in `Applying`.
**Why it happens:** External process failures are unpredictable.
**How to avoid:** Wrap apply operations in a function that (1) captures pre-apply state, (2) attempts apply, (3) on failure transitions to `Error` state with recovery info. The Error state stores the recovery path. After user acknowledges, transition back to Idle (system may be partially changed -- that's acceptable, user can re-apply).
**Warning signs:** UI appears frozen or unresponsive after a wallpaper/theme apply fails.

## Code Examples

### Example 1: Hardened Theme Parser with Validation

```rust
// Source: Based on existing theme_parser.rs with validation added

use anyhow::{Context, Result, bail};
use regex::Regex;
use std::path::Path;

lazy_static::lazy_static! {
    /// Match: export VAR_NAME="value" or export VAR_NAME='value'
    static ref EXPORT_RE: Regex = Regex::new(
        r#"export\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*["']([^"']*)["']"#
    ).unwrap();

    /// Valid color hex pattern
    static ref HEX_COLOR_RE: Regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();

    /// Valid theme ID pattern (safe for use in file paths and commands)
    static ref THEME_ID_RE: Regex = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_-]*$").unwrap();
}

/// Required variables that every valid theme file must define
const REQUIRED_VARS: &[&str] = &["THEME_NAME", "THEME_ID", "BG_PRIMARY", "FG_PRIMARY", "ACCENT"];

/// Variables that must be valid hex colors
const COLOR_VARS: &[&str] = &[
    "BG_PRIMARY", "BG_SECONDARY", "BG_TERTIARY", "BG_SURFACE",
    "FG_PRIMARY", "FG_SECONDARY", "FG_MUTED",
    "ACCENT", "ACCENT_ALT",
    "RED", "GREEN", "YELLOW", "BLUE", "PURPLE", "CYAN", "ORANGE", "PINK",
    "BRIGHT_RED", "BRIGHT_GREEN", "BRIGHT_YELLOW", "BRIGHT_BLUE",
    "BRIGHT_PURPLE", "BRIGHT_CYAN",
    "BORDER_ACTIVE", "BORDER_INACTIVE", "SELECTION", "CURSOR",
    "GRADIENT_START", "GRADIENT_END",
];

/// Validate a parsed theme for completeness and correctness
pub fn validate_theme(theme: &Theme) -> Result<()> {
    // Check theme_id is safe for file paths and commands
    if !THEME_ID_RE.is_match(&theme.theme_id) {
        bail!("Invalid theme ID '{}': must be alphanumeric with hyphens/underscores",
              theme.theme_id);
    }

    // Check theme_name is not empty
    if theme.theme_name.trim().is_empty() {
        bail!("Theme name cannot be empty");
    }

    // Check required color fields are valid hex
    // (check each field that was set to non-default)
    if !theme.bg_primary.is_empty() && !HEX_COLOR_RE.is_match(&theme.bg_primary) {
        bail!("Invalid color for BG_PRIMARY: '{}'", theme.bg_primary);
    }
    // ... repeat for other color fields

    Ok(())
}

/// Parse and validate a theme file. Returns Err if file is malformed.
pub fn parse_and_validate(path: &Path) -> Result<Theme> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read theme file: {}", path.display()))?;

    // Check for dangerous patterns (defense in depth)
    if content.contains("$(") || content.contains("`") || content.contains("eval ") {
        bail!("Theme file contains shell execution patterns (not allowed): {}",
              path.display());
    }

    let theme = parse_theme_content(&content, Some(path))?;

    // Check required variables were found
    if theme.theme_name.is_empty() {
        bail!("Missing required variable THEME_NAME in {}", path.display());
    }
    if theme.theme_id.is_empty() {
        bail!("Missing required variable THEME_ID in {}", path.display());
    }

    validate_theme(&theme)?;

    Ok(theme)
}
```

### Example 2: Backend Detection and Usage

```rust
// Source: swww GitHub README + existing hyprctl.rs code patterns

/// Detect which wallpaper backend is available and running
pub fn detect_backend() -> Result<Box<dyn WallpaperBackend>> {
    // Try swww first (preferred, supports smooth transitions)
    match std::process::Command::new("swww").arg("query").output() {
        Ok(output) if output.status.success() => {
            return Ok(Box::new(SwwwBackend));
        }
        _ => {} // swww not available or daemon not running
    }

    // Try hyprpaper via hyprctl
    match std::process::Command::new("hyprctl")
        .args(["hyprpaper", "listactive"])
        .output()
    {
        Ok(output) if output.status.success() => {
            return Ok(Box::new(HyprpaperBackend));
        }
        _ => {} // hyprpaper not available
    }

    anyhow::bail!("No wallpaper backend found. Install swww or hyprpaper.")
}
```

### Example 3: Shared CSS Module Loading

```rust
// main.rs - Single entry point after merge

mod app;
mod brand_css;
mod components;
mod models;
mod services;
mod state;

use relm4::RelmApp;
use app::App;

fn main() {
    let app = RelmApp::new("com.vulcanos.appearance-manager");

    // Load shared brand CSS (colors only) + app-specific widget CSS
    relm4::set_global_css(brand_css::FULL_CSS);

    app.run::<App>(());
}
```

```rust
// brand_css.rs

/// Brand color definitions as GTK4 @define-color declarations.
/// These match branding/vulcan-palette.css but in GTK4 CSS syntax.
pub const BRAND_COLORS: &str = r#"
@define-color vulcan_ember #f97316;
@define-color vulcan_molten #ea580c;
@define-color vulcan_gold #fbbf24;
@define-color vulcan_flame #dc2626;
@define-color vulcan_obsidian #1c1917;
@define-color vulcan_charcoal #292524;
@define-color vulcan_ash #44403c;
@define-color vulcan_smoke #57534e;
@define-color vulcan_white #fafaf9;
@define-color vulcan_stone #a8a29e;
@define-color vulcan_gray #78716c;
@define-color vulcan_success #22c55e;
@define-color vulcan_warning #fbbf24;
@define-color vulcan_error #ef4444;
@define-color vulcan_info #3b82f6;

/* Adwaita overrides */
@define-color accent_bg_color @vulcan_ember;
@define-color accent_fg_color @vulcan_white;
@define-color accent_color @vulcan_ember;
"#;

/// Widget styling CSS (shared across all views)
pub const WIDGET_CSS: &str = r#"
window, .background {
    background-color: @vulcan_obsidian;
}

headerbar {
    background: linear-gradient(to bottom, @vulcan_charcoal, shade(@vulcan_obsidian, 1.1));
    border-bottom: 1px solid @vulcan_ash;
}

/* ... rest of shared widget styles ... */
"#;

/// Full CSS string: brand colors + widget styles
/// Concatenated at compile time via const
pub const FULL_CSS: &str = concat!(
    // Brand colors
    "@define-color vulcan_ember #f97316;\n",
    "@define-color vulcan_molten #ea580c;\n",
    "@define-color vulcan_gold #fbbf24;\n",
    // ... (in practice, use include_str! for the full file)
    ""
);

// Alternative approach using include_str! (preferred):
// pub const FULL_CSS: &str = include_str!("style.css");
// Where style.css contains both @define-color declarations and widget styles
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| GTK4 `@define-color` only | CSS custom properties (variables) available in GTK 4.16+ | GTK 4.16 (2024) | Could use `--vulcan-ember` syntax in future, but `@define-color` still works and is better supported |
| hyprpaper for wallpapers | swww gaining popularity (smoother transitions) | 2023-2024 | Current codebase already uses swww despite file being named `hyprpaper.rs` |
| Separate theme/wallpaper apps | Unified appearance managers (like GNOME Settings) | Trend | This phase implements this merge |

**Not deprecated/still current:**
- `@define-color` in GTK4 CSS: Still works in GTK 4.16, will continue until GTK5
- `relm4::set_global_css()`: Still the recommended way to load app CSS
- `regex` crate: Stable at 1.x, no breaking changes expected

## Discretion Recommendations

Based on the research, here are recommendations for items left at Claude's discretion:

### 1. Crate Rename Timing: Phase 6 (NOW)
**Recommendation:** Rename in Phase 6, not Phase 7.
**Reasoning:** All subsequent phases (7, 8, etc.) will build on this crate. Renaming later means updating imports, paths, and references in code that was written for the old name. Renaming now means one clean rename before any new code is written. The rename itself is trivial: change `name` in Cargo.toml, update the `RelmApp::new()` app ID, rename the directory.

### 2. What to Absorb from vulcan-theme-manager
**Recommendation:** Absorb ALL service modules (theme_parser, theme_storage, theme_applier) and ALL model modules (theme.rs, color_group.rs). These are well-structured and functional. Do NOT absorb components yet (they have UI concerns that belong in Phase 7). Do NOT absorb app.rs (it will be replaced by the merged app).
**Reasoning:** The theme-manager services are clean, have tests, and follow the same patterns as wallpaper-manager services. The components are more tightly coupled to the old app's structure and will need reworking for the unified UI in Phase 7.

### 3. Live System Truth Detection
**Recommendation:** Use backend-specific query commands.
- **swww:** `swww query` returns per-monitor wallpaper paths. Parse the output format: `MONITOR: WxH, scale: S, currently displaying: image: /path/to/image`
- **hyprpaper:** `hyprctl hyprpaper listactive` returns `MONITOR = /path/to/image`
- **Theme:** `vulcan-theme current` returns current theme ID (already implemented in theme_applier.rs)
**Reasoning:** Both swww and hyprpaper provide IPC query mechanisms. The existing code already implements these queries correctly. The WallpaperBackend trait should formalize this via `query_active()`.

### 4. Error Recovery Strategy
**Recommendation:** Error-state-with-retry pattern.
- On failed apply: Transition to `Error { message, recovery: Box<AppState> }` state
- Error state shows the error message in UI (toast notification)
- User can retry (transition back to Applying) or dismiss (transition to Idle)
- Do NOT attempt automatic rollback (wallpaper state is volatile, partial apply is acceptable)
**Reasoning:** Automatic rollback is complex (what if rollback also fails?) and wallpaper state is inherently transient. The user can always manually re-apply. Theme apply is delegated to `vulcan-theme` CLI which handles its own atomicity.

### 5. Theme Variable Scope
**Recommendation:** Keep the existing scope from theme.rs -- it already defines a comprehensive set:
- 3 metadata fields (name, id, description)
- 4 backgrounds, 3 foregrounds, 2 accents
- 8 ANSI colors + 6 bright variants
- 4 UI colors (borders, selection, cursor)
- 2 gradient colors
- 4 system theme names (GTK, icon, cursor, Kvantum)
- 1 editor config (nvim colorscheme)
- 1 optional wallpaper
**Reasoning:** This is already defined and working. The scope is comprehensive for a desktop theme system. Adding more variables can happen in future phases.

### 6. Brand Color Consumption Layer
**Recommendation:** Both CSS and Rust constants.
- **GTK CSS:** Use `@define-color` declarations loaded via `relm4::set_global_css()` for widget styling
- **Rust constants:** A `colors` module with `pub const` strings for programmatic access (e.g., generating preview swatches, validating theme colors against brand palette)
**Reasoning:** CSS colors are needed for GTK widget styling. Rust constants are needed for code that generates or validates colors. Both are trivially cheap to maintain since they come from the same source file.

### 7. Brand Palette: Static
**Recommendation:** Static palette for Phase 6. The brand colors are fixed VulcanOS identity colors, not theme-driven.
**Reasoning:** If the brand palette were theme-driven, changing themes would change the brand identity, which defeats the purpose. Themes define their own colors; the brand palette provides the default/fallback and UI chrome colors. A future phase could make specific brand colors overridable by themes, but that's beyond Phase 6 scope.

## Open Questions

Things that couldn't be fully resolved:

1. **Exact swww query output format**
   - What we know: Format is approximately `MONITOR: WxH, scale: S, currently displaying: image: /path`. The existing code in `hyprpaper.rs` already parses this successfully.
   - What's unclear: The exact format may vary between swww versions. The existing parsing code uses `find("image: ")` which is robust.
   - Recommendation: Keep the existing parser, add a fallback/error message if format changes.

2. **Whether theme components (browser, card, editor, preview) compile cleanly in new crate**
   - What we know: They reference `crate::models::Theme` and `crate::services::*` which will exist after merge.
   - What's unclear: There may be import path issues or subtle coupling to the old app.rs structure.
   - Recommendation: Move component files but only add them to `mod.rs` after verifying they compile. Components are Phase 7 integration work but having the files present is useful.

3. **`concat!` macro for compile-time CSS string building**
   - What we know: Rust's `concat!` macro joins string literals at compile time. `include_str!` embeds file contents.
   - What's unclear: Whether `concat!(include_str!("colors.css"), include_str!("widgets.css"))` works (it should, both produce string literals).
   - Recommendation: Test this approach. If it fails, use a single `include_str!("style.css")` file that contains everything.

## Sources

### Primary (HIGH confidence)
- Existing codebase: vulcan-wallpaper-manager/src/ (all files read directly)
- Existing codebase: vulcan-theme-manager/src/ (all files read directly)
- branding/vulcan-palette.css (read directly)
- [GTK4 CssProvider API docs](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.CssProvider.html)
- [GTK4 CSS in gtk4-rs book](https://gtk-rs.org/gtk4-rs/stable/latest/book/css.html)
- [Relm4 Components Guide](https://relm4.org/book/next/components.html)

### Secondary (MEDIUM confidence)
- [swww GitHub README](https://github.com/LGFae/swww) - query output format description
- [hyprpaper Hyprland Wiki](https://wiki.hypr.land/Hypr-Ecosystem/hyprpaper/) - IPC commands
- [Hoverbear "Pretty State Machine Patterns in Rust"](https://hoverbear.org/blog/rust-state-machine-pattern/) - enum vs typestate comparison
- [GTK4 CSS Overview](https://docs.gtk.org/gtk4/css-overview.html) - @define-color documentation
- [CSS Happenings blog](https://blogs.gnome.org/alicem/2024/06/07/css-happenings/) - CSS variables vs @define-color evolution

### Tertiary (LOW confidence)
- [conch-parser](https://github.com/ipetkov/conch-parser) - Evaluated but rejected (archived 2022)
- [yash-syntax](https://lib.rs/crates/yash-syntax) - Evaluated but rejected (GPL-3.0, overkill)
- [statig crate](https://github.com/mdeloof/statig) - Evaluated but rejected (overkill for 3-4 states)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All libraries already in use, no new dependencies
- Architecture (merge strategy): HIGH - Both codebases read, structures are compatible
- Architecture (state machine): HIGH - Elm pattern well-documented, Relm4 constraints verified
- Architecture (backend trait): HIGH - Both swww and hyprpaper APIs verified
- Architecture (CSS module): HIGH - GTK4 CSS mechanism verified, existing code demonstrates pattern
- Theme parser hardening: MEDIUM - Regex approach is sound but exact validation rules need refinement during implementation
- Pitfalls: HIGH - Based on direct codebase analysis (e.g., hyprpaper.rs naming confusion is observable fact)

**Research date:** 2026-01-24
**Valid until:** 2026-03-24 (stable domain, no fast-moving dependencies)
