# Technology Stack

**Project:** VulcanOS Unified Appearance Manager
**Researched:** 2026-01-24
**Context:** Subsequent milestone - merging vulcan-theme-manager + vulcan-wallpaper-manager

## Executive Summary

This research focuses on **stack additions/changes** needed for a unified appearance manager. The existing GTK4/Relm4 foundation is validated and should be kept. Key additions are lightweight crates for file watching, CSS manipulation, and shared state patterns already available in Relm4.

**Philosophy:** Minimal additions, maximum reuse of proven existing stack.

## Existing Stack (Keep As-Is)

### Core GUI Framework
| Technology | Version | Purpose | Why Keep |
|------------|---------|---------|----------|
| gtk4 | 0.10.3 | GUI toolkit bindings | Already used, latest stable, excellent Wayland support |
| libadwaita | 0.8.1 | Modern GNOME styling | Already used, provides native appearance for integrated apps |
| relm4 | 0.10.1 | Elm-inspired reactive framework | Already used, handles component communication and state |

### Supporting Libraries (Existing)
| Library | Version | Purpose | Status |
|---------|---------|---------|--------|
| serde / serde_json | 1.0 | Data serialization | Keep - theme/profile storage |
| anyhow | 1.0 | Error handling | Keep - consistent error propagation |
| dirs | 5 | Directory paths | Keep - XDG directory discovery |
| lazy_static | 1.4 | Static initialization | Keep - singleton patterns |
| tokio | 1.x | Async runtime | Keep - for wallpaper backend (swww calls) |
| image | 0.25 | Image decoding | Keep - wallpaper thumbnails |
| regex | 1.x | Pattern matching | Keep - theme file parsing |

**Confidence:** HIGH - All versions verified from [docs.rs](https://docs.rs/)

## New Stack Additions

### 1. Configuration Format: TOML

**Current state:** wallpaper-manager uses TOML, theme-manager uses shell scripts (.sh)

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| toml | 0.8 | Config serialization | Already in wallpaper-manager; TOML beats JSON/YAML for config files |

**Rationale:**
- **TOML for unified config:** Merge theme variables + wallpaper bindings + metadata into single format
- **Why not YAML:** YAML's flexibility is dangerous (indentation errors, security issues) per [DEV Community comparison](https://dev.to/jsontoall_tools/json-vs-yaml-vs-toml-which-configuration-format-should-you-use-in-2026-1hlb)
- **Why not keep .sh:** Shell scripts are powerful but harder to parse, edit programmatically, and validate
- **Rust ecosystem fit:** Cargo uses TOML, [consistent with Rust conventions](https://doc.rust-lang.org/cargo/reference/config.html)

**Migration path:**
```toml
[metadata]
name = "Vulcan Forge"
id = "vulcan-forge"
description = "Warm forge-inspired colors"

[colors.background]
primary = "#1c1917"
secondary = "#292524"
tertiary = "#44403c"

[colors.accent]
primary = "#f97316"
alt = "#fbbf24"

[wallpapers]
suggested = ["vulcan-gradient.png", "forge-abstract.png"]
default = "vulcan-gradient.png"

[targets]
waybar = true
wofi = true
swaync = true
hyprland = true
terminals = ["kitty", "alacritty"]
```

**Confidence:** HIGH - TOML is proven, well-supported in Rust

### 2. File System Watching (Optional)

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| notify | 6.x | File system events | Only if live theme reload needed |

**Rationale:**
- **Watch theme directories** for external changes (user editing .toml files)
- **Auto-reload themes** when files change on disk
- **Cross-platform:** Works on Linux/macOS/Windows via [notify-rs](https://github.com/notify-rs/notify)
- **Battle-tested:** Used by cargo-watch, rust-analyzer, mdBook

**Use case:** Developer creates custom theme in editor → appears in theme browser immediately

**Alternative:** Skip it for MVP, add in post-release phase if users request it

**Confidence:** MEDIUM - Feature is nice-to-have, not critical path

### 3. Directory Traversal

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| walkdir | 2.x | Recursive directory walk | Third-party app discovery |

**Rationale:**
- **Scan XDG directories** (`~/.local/share/applications`, `/usr/share/applications`) for .desktop files
- **Efficient traversal:** [Comparable to GNU find](https://docs.rs/walkdir/latest/walkdir/) in performance
- **Standard pattern:** Used throughout Rust ecosystem for file discovery

**Use case:** Discover installed apps with theming support (GTK apps, terminals, etc.)

**Discovery algorithm:**
1. Walk XDG application directories
2. Parse desktop entries ([ArchWiki reference](https://wiki.archlinux.org/title/Desktop_entries))
3. Check for known config paths (e.g., `~/.config/{app}/style.css`)
4. Flag apps that support theming

**Confidence:** HIGH - Standard Rust crate for this task

### 4. CSS Manipulation (Future Consideration)

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| cssparser | 0.x | CSS parsing/serialization | Only if programmatic CSS editing needed |

**Rationale:**
- **Browser-grade parser:** [Used in Firefox](https://github.com/servo/rust-cssparser), implements CSS Syntax Level 3
- **Not needed for MVP:** Current envsubst template approach works well
- **Consider for advanced features:** Live CSS editing, CSS variable extraction from existing themes

**When to add:** Post-MVP if users request visual CSS editor or theme import from arbitrary stylesheets

**Confidence:** LOW - May not be needed at all

## Shared State Patterns

### Relm4 Built-in Solutions

Relm4 provides [shared state modules](https://github.com/orgs/Relm4/discussions/552) for cross-component communication. No additional crates needed.

**Pattern for unified app:**
```rust
// Shared configuration state
pub struct AppConfig {
    pub current_theme_id: String,
    pub wallpaper_profiles: Vec<WallpaperProfile>,
    pub auto_apply: bool,
}

// Components send messages via Relm4 channels
enum AppMsg {
    ThemeChanged(String),
    WallpaperAssigned { monitor: String, path: PathBuf },
    ConfigUpdated,
}
```

**Communication:**
- Theme browser → App → Wallpaper panel (when theme changes, suggest wallpapers)
- Wallpaper panel → Theme applier (apply wallpaper + theme atomically)
- Both → Settings service (persist unified config)

**Resources:**
- [Relm4 Book - Introduction](https://relm4.org/book/stable/)
- [GitHub Discussion - Global State](https://github.com/orgs/Relm4/discussions/552)

**Confidence:** HIGH - Relm4's messaging system handles this pattern natively

## CSS Propagation Strategy

### Background: Current Architecture

**Theme application flow:**
1. User selects theme in GUI
2. `vulcan-theme` CLI loads theme .sh file (exports variables)
3. `envsubst` processes templates (`.tpl` files) with variables
4. Generated configs written to `~/.config/{app}/`
5. Apps reload configs (some require restart)

**Template files:**
- `waybar-style.css.tpl` → `~/.config/waybar/style.css`
- `wofi-style.css.tpl` → `~/.config/wofi/style.css`
- `swaync-style.css.tpl` → `~/.config/swaync/style.css`
- `hyprland-looknfeel.conf.tpl` → `~/.config/hypr/looknfeel.conf`
- `kitty.conf.tpl` → `~/.config/kitty/kitty.conf`
- `alacritty.toml.tpl` → `~/.config/alacritty/alacritty.toml`

**Variables used:** `${BG_PRIMARY}`, `${ACCENT}`, `${FG_PRIMARY}`, etc. (50+ variables)

### Propagation Requirements

**Table stakes:**
1. **Atomic application** - All configs update together, no partial states
2. **Template preservation** - Keep `.tpl` files as source of truth
3. **Variable consistency** - Same variable names across all targets
4. **Preview without apply** - Show theme without persisting changes

**Nice-to-have:**
1. **Live reload** - Signal apps to reload configs without restart
2. **Rollback** - Undo theme application if something breaks
3. **Per-app targeting** - Apply theme to subset of apps

### Recommended Approach: Keep Current System + Rust Wrapper

**DO NOT rewrite the templating system.** It works well. Instead:

1. **Rust theme applier calls vulcan-theme CLI:**
   ```rust
   use std::process::Command;

   pub fn apply_theme(theme_id: &str) -> Result<()> {
       Command::new("vulcan-theme")
           .args(["set", theme_id])
           .output()?;
       Ok(())
   }
   ```

2. **Preview mode uses temporary directory:**
   ```rust
   pub fn preview_theme(theme_id: &str) -> Result<()> {
       let temp_dir = create_temp_config_dir()?;
       // Apply theme to temp_dir, show previews from there
       // Original configs untouched
   }
   ```

3. **Unified app provides GUI for discovery:**
   - Load theme metadata from .toml files
   - Show color palette + suggested wallpapers
   - Apply button → calls vulcan-theme CLI
   - Wallpaper assignment → calls swww (already working)

**Why not rewrite in Rust:**
- envsubst is battle-tested, handles edge cases
- Shell script templating is well-understood
- No performance issues with current approach
- Risk of introducing bugs during rewrite

**What Rust adds:**
- GUI for theme discovery and browsing
- Preview without apply (temp directory trick)
- Unified theme + wallpaper experience
- Third-party app detection

**Confidence:** HIGH - Reuse existing tooling, add GUI layer

## Third-Party App Theming Discovery

### Discovery Algorithm

**Tier 1: Known configs (hardcoded list)**
```rust
const THEMED_APPS: &[(&str, &str)] = &[
    ("waybar", "~/.config/waybar/style.css"),
    ("wofi", "~/.config/wofi/style.css"),
    ("swaync", "~/.config/swaync/style.css"),
    ("kitty", "~/.config/kitty/kitty.conf"),
    ("alacritty", "~/.config/alacritty/alacritty.toml"),
    ("hyprland", "~/.config/hypr/looknfeel.conf"),
];
```

**Tier 2: GTK4/Adwaita apps (via .desktop files)**
1. Scan `/usr/share/applications/*.desktop` and `~/.local/share/applications/*.desktop`
2. Parse desktop entries ([ArchWiki reference](https://wiki.archlinux.org/title/Desktop_entries))
3. Check for GTK/GNOME in categories or keywords
4. Mark as "themed via GTK theme" (no per-app config needed)

**Tier 3: Custom config detection**
1. For each installed app, check common config patterns:
   - `~/.config/{app}/style.css`
   - `~/.config/{app}/theme.toml`
   - `~/.config/{app}/colors.conf`
2. If file exists, mark as "custom themeable" (requires user template creation)

**What NOT to do:**
- Don't try to parse arbitrary config formats
- Don't inject themes into non-themed apps
- Don't modify configs without templates

**Display in GUI:**
- ✅ Themed: Apps with active theme support
- ⚠️ GTK-themed: Apps that inherit GTK theme
- ℹ️ Themeable: Apps with config files, needs template
- ❌ Not themed: Apps without theming support

**Confidence:** MEDIUM - Tier 1 is solid, Tier 2/3 need testing with real apps

## Integration Points with Existing Stack

### With vulcan-theme CLI
- **Call via Command::new("vulcan-theme")** - Don't reimplement
- **Parse theme list from `vulcan-theme list`** - Avoid duplicating discovery logic
- **Use exit codes for error handling** - Rust sees success/failure

### With swww (wallpaper backend)
- **Already working in wallpaper-manager** - No changes needed
- **Atomic theme+wallpaper apply:**
  ```rust
  pub fn apply_appearance(theme_id: &str, wallpapers: &HashMap<String, PathBuf>) -> Result<()> {
      // 1. Apply theme (updates all CSS files)
      apply_theme(theme_id)?;

      // 2. Apply wallpapers (per-monitor)
      for (monitor, path) in wallpapers {
          set_wallpaper(monitor, path)?;
      }

      Ok(())
  }
  ```

### With Hyprland
- **Reload Hyprland config after theme change:**
  ```bash
  hyprctl reload
  ```
- **Or just restart relevant apps** - Hyprland itself picks up looknfeel.conf changes

**Confidence:** HIGH - Integration points are well-defined

## Alternatives Considered

### Alternative 1: CSS Variables with Live Injection

**Approach:** Use CSS custom properties (`--color-bg: #1c1917;`) in app stylesheets, inject via CssProvider at runtime.

**Why not:**
- Requires apps to support CSS variables (Waybar, wofi, swaync don't use them consistently)
- Can't theme non-GTK apps (terminals, Hyprland)
- More complex than template approach
- Loses existing 50+ theme variables

### Alternative 2: Rewrite Theme System in Pure Rust

**Approach:** Parse templates in Rust, replace variables, write configs.

**Why not:**
- Reinventing the wheel (envsubst exists)
- Risk of template parsing bugs
- No user-facing benefit
- Harder to maintain templates (two systems instead of one)

### Alternative 3: D-Bus Theming Service

**Approach:** Expose theme service via D-Bus, apps subscribe to theme changes.

**Why not:**
- Apps need modification to support D-Bus subscription
- Over-engineering for single-user desktop system
- Adds complexity with no MVP benefit
- Doesn't solve legacy app theming

**Confidence:** HIGH - Template approach is proven and correct

## What NOT to Add

### Do NOT add these crates:
- **gtk-rs CSS libraries** - Use vulcan-theme CLI instead
- **D-Bus crates** - Over-engineering for MVP
- **Custom config parsers** - TOML + serde handles everything
- **Advanced image processing** - `image` crate is sufficient
- **Web-based UI** - GTK4 is native and integrated

### Do NOT change these patterns:
- **Template-based theming** - Works well, don't rewrite
- **swww for wallpapers** - Already validated
- **GTK4/Relm4 foundation** - Proven architecture

## Installation

### Updated Cargo.toml for Unified App

```toml
[package]
name = "vulcan-appearance-manager"
version = "0.1.0"
edition = "2021"

[dependencies]
# GUI framework (existing)
gtk4 = { version = "0.10", package = "gtk4", features = ["v4_16"] }
libadwaita = { version = "0.8", package = "libadwaita", features = ["v1_6"] }
relm4 = { version = "0.10", features = ["libadwaita"] }

# Data handling (existing + TOML for unified config)
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# Error handling (existing)
anyhow = "1.0"

# Utilities (existing)
dirs = "5"
regex = "1"
lazy_static = "1.4"

# Image handling (existing)
image = "0.25"

# Async runtime (existing, for swww calls)
tokio = { version = "1", features = ["rt", "process"] }

# NEW: File system operations
walkdir = "2"

# OPTIONAL: File watching (defer to post-MVP)
# notify = "6"
```

## Version Policy

**Semantic versioning:**
- **0.10.x for GUI stack** - Match Relm4 version family
- **1.x for stable crates** - Use latest stable (serde, anyhow, regex, etc.)
- **0.x for evolving crates** - Pin minor versions to avoid breakage

**Update cadence:**
- Check for updates quarterly
- Pin exact versions in Cargo.lock
- Test before bumping minor versions

**Minimum Rust version:** 1.83 (required by gtk4)

## Sources

### Official Documentation
- [GTK4 Rust Bindings Documentation](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/)
- [Relm4 Book](https://relm4.org/book/stable/)
- [Cargo Configuration Reference](https://doc.rust-lang.org/cargo/reference/config.html)
- [CSS in GTK4](https://gtk-rs.org/gtk4-rs/git/book/css.html)
- [GTK4 CSS Properties](https://docs.gtk.org/gtk4/css-properties.html)

### Crate Documentation
- [gtk4 0.10.3 on docs.rs](https://docs.rs/gtk4/0.10.3/gtk4/)
- [relm4 0.10.1 on docs.rs](https://docs.rs/relm4/0.10.1/relm4/)
- [libadwaita 0.8.1 on docs.rs](https://docs.rs/libadwaita/0.8.1/libadwaita/)
- [walkdir on docs.rs](https://docs.rs/walkdir/latest/walkdir/)
- [notify-rs on GitHub](https://github.com/notify-rs/notify)
- [cssparser on docs.rs](https://docs.rs/cssparser/)

### Community Resources
- [GTK4 CSS Styling Tutorial](https://jamesbenner.hashnode.dev/how-to-style-your-gtk4-rust-app-with-css)
- [Rust GTK4 CSS Dynamic Theming Example](https://github.com/jbenner-radham/rust-gtk4-css-styling)
- [Relm4 Discussion - Global State](https://github.com/orgs/Relm4/discussions/552)
- [JSON vs YAML vs TOML Comparison](https://dev.to/jsontoall_tools/json-vs-yaml-vs-toml-which-configuration-format-should-you-use-in-2026-1hlb)

### System Integration
- [ArchWiki: Desktop Entries](https://wiki.archlinux.org/title/Desktop_entries)
- [Waybar Theming Examples on GitHub](https://github.com/topics/waybar-themes)

### Research Confidence
- **HIGH:** Core stack (GTK4/Relm4), TOML, walkdir, integration patterns
- **MEDIUM:** File watching (nice-to-have), third-party discovery (needs testing)
- **LOW:** CSS parsing (may not be needed)
