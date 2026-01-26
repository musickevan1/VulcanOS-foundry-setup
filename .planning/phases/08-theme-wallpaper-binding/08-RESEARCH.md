# Phase 8: Theme-Wallpaper Binding - Research

**Researched:** 2026-01-25
**Domain:** GTK4/Relm4 UI components, data serialization, file path handling
**Confidence:** HIGH

## Summary

This phase introduces theme-wallpaper binding where themes suggest wallpapers and unified profiles save coordinated appearance (theme + wallpaper + binding state) as a single unit. The research focused on GTK4 UI patterns for previews, badges, and dialogs; TOML serialization for complex nested data; and file path resolution for theme-bundled wallpapers.

Key findings:
1. **GTK4 Picture widget** with `content-fit` property handles thumbnail previews with aspect ratio preservation
2. **GTK4 Overlay widget** with halign/valign provides corner badge positioning for override indicators
3. **Custom modal dialogs** with transient parent and mixed content (colors + images) require manual Window construction
4. **TOML serde** handles nested structs and HashMaps naturally with `#[serde(default)]` for optional fields
5. **Path resolution** for theme-bundled wallpapers requires parent() + join() for relative paths, with Path::exists() validation

**Primary recommendation:** Use Picture widget for wallpaper thumbnails, Overlay for badges, custom Window for preview dialog, extend existing WallpaperProfile struct to include theme binding data, and resolve wallpaper paths relative to theme source directory.

## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| gtk4 | 0.9+ | UI widgets (Picture, Overlay, Window) | Official GTK4 Rust bindings, already in use |
| relm4 | 0.9+ | Reactive UI framework | Project standard, message-driven architecture |
| serde | 1.0 | Serialization with derive macros | Rust ecosystem standard |
| toml | 0.8+ | TOML format support | Already used for profiles, simple human-readable |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| anyhow | 1.0 | Error handling | Project standard for Result types |
| std::path | stdlib | Path manipulation (parent, join, exists) | Built-in, sufficient for path operations |
| dirs | 5.0+ | Config directory location | Already in use for profile_dir() |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| TOML | JSON/RON | TOML more human-editable, already established |
| Custom Window | AlertDialog | AlertDialog lacks custom content support in GTK4.10+ |
| Picture | Image | Picture better for full-size content with scaling |

**Installation:**
```bash
# Already in Cargo.toml - no new dependencies needed
```

## Architecture Patterns

### Recommended Project Structure
```
vulcan-appearance-manager/
├── src/
│   ├── models/
│   │   ├── profile.rs        # Extend WallpaperProfile → UnifiedProfile
│   │   └── binding.rs        # NEW: BindingMode enum
│   ├── services/
│   │   └── profile_storage.rs # Update for unified profiles
│   ├── components/
│   │   ├── theme_card.rs     # Add wallpaper thumbnail overlay
│   │   ├── profile_card.rs   # NEW: Unified profile display
│   │   ├── binding_dialog.rs # NEW: "Apply wallpaper?" confirmation
│   │   └── profile_view.rs   # NEW: Third tab for profiles
│   └── app.rs                # Add profile view to ViewStack
```

### Pattern 1: Overlay Badge on Theme Card
**What:** Show visual indicator when theme's wallpaper has been overridden
**When to use:** User has manually changed wallpaper, breaking binding with theme
**Example:**
```rust
// Source: GTK4 Overlay documentation + existing theme_card.rs pattern
use gtk::prelude::*;

gtk::Overlay {
    // Main card content (existing color preview)
    #[wrap(Some)]
    set_child = &gtk::Box {
        // ... existing theme card content ...
    },

    // Badge indicator (top-right corner)
    add_overlay = &gtk::Image {
        set_icon_name: Some("emblem-default-symbolic"),
        set_halign: gtk::Align::End,
        set_valign: gtk::Align::Start,
        set_margin_top: 4,
        set_margin_end: 4,
        add_css_class: "override-badge",
    }
}
```

### Pattern 2: Picture Widget for Wallpaper Thumbnails
**What:** Display wallpaper preview with preserved aspect ratio
**When to use:** Theme card corner preview, dialog preview, profile card
**Example:**
```rust
// Source: https://docs.gtk.org/gtk4/class.Picture.html
use gtk::prelude::*;

gtk::Picture {
    set_file: Some(&gio::File::for_path(&wallpaper_path)),
    set_content_fit: gtk::ContentFit::Cover,  // Fill area, crop as needed
    set_can_shrink: true,
    set_width_request: 64,   // Small for card corner
    set_height_request: 48,
    add_css_class: "wallpaper-thumbnail",
}
```

### Pattern 3: Unified Profile Model
**What:** Extend WallpaperProfile to include theme and binding state
**When to use:** Saving/loading profiles that coordinate theme + wallpaper
**Example:**
```rust
// Source: Existing profile.rs + serde documentation
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BindingMode {
    /// Theme's suggested wallpaper is active
    ThemeBound,
    /// User has overridden theme's suggestion
    CustomOverride,
    /// Theme has no wallpaper suggestion
    Unbound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedProfile {
    pub name: String,
    #[serde(default)]
    pub description: String,

    // Theme binding
    pub theme_id: Option<String>,

    // Wallpaper configuration (per-monitor)
    pub monitor_wallpapers: HashMap<String, PathBuf>,

    // Binding state
    #[serde(default)]
    pub binding_mode: BindingMode,

    // Backend settings
    #[serde(default)]
    pub backend_config: BackendConfig,
}

impl Default for BindingMode {
    fn default() -> Self {
        BindingMode::Unbound
    }
}
```

### Pattern 4: Theme-Relative Wallpaper Path Resolution
**What:** Resolve THEME_WALLPAPER relative to theme file's directory
**When to use:** Loading suggested wallpaper from theme file
**Example:**
```rust
// Source: https://doc.rust-lang.org/std/path/struct.Path.html
use std::path::{Path, PathBuf};

pub fn resolve_theme_wallpaper(theme: &Theme) -> Option<PathBuf> {
    let wallpaper_rel = theme.theme_wallpaper.as_ref()?;
    let theme_dir = theme.source_path.as_ref()?.parent()?;

    // Resolve relative to theme directory
    let wallpaper_path = theme_dir.join(wallpaper_rel);

    // Validate existence before returning
    if wallpaper_path.exists() {
        Some(wallpaper_path)
    } else {
        eprintln!("Warning: Theme '{}' suggests wallpaper '{}' but file not found",
                  theme.theme_name, wallpaper_path.display());
        None
    }
}
```

### Pattern 5: Custom Modal Dialog with Preview
**What:** Dialog showing theme colors + wallpaper preview side-by-side
**When to use:** Confirming "Apply theme's suggested wallpaper?" action
**Example:**
```rust
// Source: https://docs.gtk.org/gtk4/method.Window.set_modal.html
// and existing theme_editor.rs dialog pattern
use relm4::prelude::*;

#[relm4::component(pub)]
impl SimpleComponent for BindingDialog {
    // ...

    view! {
        #[root]
        gtk::Window {
            set_modal: true,
            set_transient_for: Some(parent_window),
            set_title: Some("Apply Theme Wallpaper?"),
            set_default_width: 600,
            set_default_height: 400,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,
                set_margin_all: 24,

                // Preview content (horizontal split)
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 24,
                    set_hexpand: true,
                    set_vexpand: true,

                    // Theme colors preview
                    gtk::Frame {
                        set_hexpand: true,
                        // ... color swatches ...
                    },

                    // Wallpaper preview
                    gtk::Frame {
                        set_hexpand: true,

                        gtk::Picture {
                            set_file: Some(&gio::File::for_path(&wallpaper_path)),
                            set_content_fit: gtk::ContentFit::Contain,
                        }
                    },
                },

                // Action buttons
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,
                    set_halign: gtk::Align::End,

                    gtk::Button {
                        set_label: "Theme Only",
                        connect_clicked => BindingDialogInput::ApplyThemeOnly,
                    },
                    gtk::Button {
                        set_label: "Apply Both",
                        add_css_class: "suggested-action",
                        connect_clicked => BindingDialogInput::ApplyBoth,
                    },
                },
            },
        }
    }
}
```

### Anti-Patterns to Avoid
- **Don't load images synchronously on UI thread:** Use async loading for Picture widgets to avoid blocking
- **Don't use MessageDialog for custom content:** Deprecated in GTK4.10+, limited customization
- **Don't store absolute wallpaper paths in profiles:** Store relative paths when possible for portability
- **Don't validate paths only at save time:** Validate on load too (files may be deleted)

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Aspect ratio thumbnail scaling | Custom image cropping logic | `gtk::Picture` with `set_content_fit` | Handles aspect ratios, DPI scaling, format support automatically |
| Path canonicalization | String manipulation for .. resolution | `Path::parent()` + `Path::join()` + `Path::exists()` | Handles symlinks, platform differences, edge cases |
| Optional field serialization | Manual Option<T> TOML writing | `#[serde(default)]` + `#[serde(skip_serializing_if = "Option::is_none")]` | Automatic None handling, backward compatibility |
| Corner badge positioning | Manual pixel positioning | `gtk::Overlay` with `halign`/`valign` | Handles RTL layouts, DPI scaling, container resizing |
| Modal dialog parent tracking | Global dialog state | `set_transient_for(parent)` | Window manager handles stacking, focus, minimizing together |

**Key insight:** GTK4 provides high-level abstractions for complex UI patterns. Use them instead of reimplementing with lower-level primitives.

## Common Pitfalls

### Pitfall 1: Picture Widget Not Displaying Images
**What goes wrong:** Picture widget shows empty space instead of image
**Why it happens:** File path doesn't exist, format unsupported, or Picture not allocated size
**How to avoid:**
- Always validate path with `path.exists()` before creating Picture
- Set explicit size requests: `set_width_request()` and `set_height_request()`
- Use `set_can_shrink: true` to allow Picture to fit container
- Check console for GdkPixbuf warnings about unsupported formats
**Warning signs:** Console warnings about "Failed to load texture from file"

### Pitfall 2: TOML HashMap Ordering Breaking Tests
**What goes wrong:** Serialized TOML has random key order, test assertions fail
**Why it happens:** HashMap doesn't preserve insertion order, toml crate uses HashMap iteration
**How to avoid:**
- Don't test exact TOML string output, test deserialized struct equality
- For display purposes, sort keys manually before presenting to user
- Use BTreeMap instead of HashMap if ordering matters for file diffs
**Warning signs:** Intermittent test failures with "expected X but got Y" with same data

### Pitfall 3: Relative Path Resolution from Wrong Base
**What goes wrong:** Theme wallpaper path resolves to wrong location
**Why it happens:** Resolving relative to current working directory instead of theme file location
**How to avoid:**
- Always use `theme.source_path.parent()` as base for resolution
- Store source_path in Theme struct when parsing (already done in codebase)
- Validate resolved path exists before using: `resolved.exists()`
**Warning signs:** Wallpaper works for user who created theme, fails for others

### Pitfall 4: Modal Dialog Not Blocking Parent
**What goes wrong:** User can interact with parent window while dialog is open
**Why it happens:** Missing `set_modal(true)` or `set_transient_for(parent)`
**How to avoid:**
- Always set both modal AND transient_for: `set_modal: true` + `set_transient_for: Some(parent)`
- Get parent window with `widget.root().and_downcast::<gtk::Window>()`
- Verify modal behavior in testing
**Warning signs:** Multiple dialogs can be opened, parent window steals focus

### Pitfall 5: Overlay Badge Not Visible
**What goes wrong:** Badge widget exists but doesn't appear on screen
**Why it happens:** Z-order wrong, no size allocation, or CSS hiding it
**How to avoid:**
- Add overlay AFTER setting main child: first `set_child`, then `add_overlay`
- Set explicit size for badge: `set_width_request()` or use icon with fixed size
- Use halign/valign to position: `set_halign: gtk::Align::End`, `set_valign: gtk::Align::Start`
- Add margins to prevent edge clipping: `set_margin_top: 4`, `set_margin_end: 4`
**Warning signs:** Badge appears in wrong position or partially clipped

## Code Examples

Verified patterns from official sources:

### Loading Profile with Fallback Migration
```rust
// Source: Existing profile_storage.rs + serde default pattern
use anyhow::{Context, Result};

pub fn load_profile(name: &str) -> Result<UnifiedProfile> {
    let path = profile_dir().join(format!("{}.toml", name));
    let contents = fs::read_to_string(&path)
        .context("Failed to read profile file")?;

    // Try new format first
    if let Ok(profile) = toml::from_str::<UnifiedProfile>(&contents) {
        return Ok(profile);
    }

    // Fallback: migrate old WallpaperProfile format
    let old_profile: WallpaperProfile = toml::from_str(&contents)
        .context("Failed to parse profile")?;

    Ok(UnifiedProfile {
        name: old_profile.name,
        description: old_profile.description,
        theme_id: None,  // No theme in old format
        monitor_wallpapers: old_profile.monitor_wallpapers,
        binding_mode: BindingMode::Unbound,
        backend_config: BackendConfig::default(),
    })
}
```

### Theme Card with Conditional Wallpaper Badge
```rust
// Source: Existing theme_card.rs + GTK4 Overlay pattern
#[relm4::factory(pub)]
impl FactoryComponent for ThemeItem {
    // ...

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 8,

            // Overlay for badge + color preview
            gtk::Overlay {
                #[wrap(Some)]
                set_child = &gtk::Frame {
                    add_css_class: "color-preview-frame",
                    // ... existing color preview rows ...
                },

                // Wallpaper thumbnail (bottom-right corner)
                #[name = "wallpaper_preview"]
                add_overlay = &gtk::Picture {
                    #[watch]
                    set_visible: self.theme.theme_wallpaper.is_some(),
                    set_file: self.theme.theme_wallpaper.as_ref()
                        .and_then(|_| resolve_theme_wallpaper(&self.theme))
                        .map(|p| gio::File::for_path(&p)),
                    set_content_fit: gtk::ContentFit::Cover,
                    set_width_request: 60,
                    set_height_request: 40,
                    set_halign: gtk::Align::End,
                    set_valign: gtk::Align::End,
                    set_margin_end: 4,
                    set_margin_bottom: 4,
                    add_css_class: "wallpaper-corner-preview",
                },

                // Override badge (top-right corner) - shown when binding broken
                #[name = "override_badge"]
                add_overlay = &gtk::Image {
                    #[watch]
                    set_visible: self.is_override,
                    set_icon_name: Some("emblem-default-symbolic"),
                    set_pixel_size: 16,
                    set_halign: gtk::Align::End,
                    set_valign: gtk::Align::Start,
                    set_margin_top: 4,
                    set_margin_end: 4,
                    add_css_class: "override-badge",
                }
            },

            // Theme name/description (existing)
            // ...
        }
    }
}
```

### Profile Card for Unified Profiles Tab
```rust
// Source: Pattern from theme_card.rs adapted for profiles
#[derive(Debug)]
pub struct ProfileItem {
    pub profile: UnifiedProfile,
    pub is_active: bool,
}

#[relm4::factory(pub)]
impl FactoryComponent for ProfileItem {
    type Init = (UnifiedProfile, bool);
    type Input = ();
    type Output = ProfileCardOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::FlowBox;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 8,
            set_width_request: 200,
            set_margin_all: 8,
            add_css_class: "profile-card",

            // Split preview: theme colors (left) + wallpaper (right)
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 4,
                set_height_request: 120,

                // Theme color preview (if theme bound)
                gtk::Frame {
                    set_hexpand: true,
                    #[watch]
                    set_visible: self.profile.theme_id.is_some(),
                    // ... render color swatches if theme exists ...
                },

                // Wallpaper preview
                gtk::Frame {
                    set_hexpand: true,

                    gtk::Picture {
                        // Show first monitor's wallpaper
                        set_file: self.profile.monitor_wallpapers.values()
                            .next()
                            .map(|p| gio::File::for_path(p)),
                        set_content_fit: gtk::ContentFit::Cover,
                    }
                },
            },

            // Profile info
            gtk::Label {
                set_label: &self.profile.name,
                add_css_class: "profile-name",
                set_halign: gtk::Align::Start,
            },

            gtk::Label {
                #[watch]
                set_label: &format!("{} • {}",
                    self.profile.theme_id.as_deref().unwrap_or("No theme"),
                    self.profile.binding_mode.display_name()
                ),
                add_css_class: "dim-label",
                set_halign: gtk::Align::Start,
            },

            // Action buttons
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 4,

                gtk::Button {
                    set_label: "Load",
                    set_hexpand: true,
                    connect_clicked => ProfileCardOutput::Load(self.profile.name.clone()),
                },

                gtk::Button {
                    set_icon_name: "user-trash-symbolic",
                    connect_clicked => ProfileCardOutput::Delete(self.profile.name.clone()),
                }
            }
        }
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| MessageDialog with custom content | AlertDialog (limited) or custom Window | GTK 4.10 (2023) | Must use custom Window for complex dialogs |
| Picture.keep-aspect-ratio | Picture.content-fit | GTK 4.8 (2022) | Better control over scaling modes |
| Manual symlink resolution | Path::canonicalize() | Rust 1.5+ (2015) | Safer, handles edge cases |
| HashMap for key-order preservation | BTreeMap or sort at display | Always | HashMap never guaranteed order |

**Deprecated/outdated:**
- `keep-aspect-ratio` property on Picture widget → use `content-fit` instead
- MessageDialog for custom layouts → use custom Window with modal + transient
- Storing profile paths as ~/.config/vulcan-wallpaper → migrate to ~/.config/vulcan-appearance-manager (but support reading old paths for migration)

## Open Questions

Things that couldn't be fully resolved:

1. **Should profiles auto-detect and migrate on first launch?**
   - What we know: Old WallpaperProfile format exists, new UnifiedProfile format extends it
   - What's unclear: Whether to auto-migrate all old profiles or migrate on-demand when loaded
   - Recommendation: Migrate on-demand (when loading profile), keep old files intact as backup
   - Confidence: MEDIUM (both approaches work, on-demand is safer)

2. **How to handle themes that reference multiple wallpapers?**
   - What we know: CONTEXT.md says "lean toward simplicity" for single vs multiple
   - What's unclear: Future extensibility vs current simplicity tradeoff
   - Recommendation: Single THEME_WALLPAPER field initially, can extend to THEME_WALLPAPER_N later if needed
   - Confidence: HIGH (matches CONTEXT.md guidance, simpler to implement)

3. **CSS styling for wallpaper-corner-preview sizing?**
   - What we know: Picture widget needs explicit size request for thumbnails
   - What's unclear: Whether to hardcode sizes or make them CSS-customizable
   - Recommendation: Use hardcoded size requests (60x40) but add CSS class for border/shadow styling
   - Confidence: HIGH (GTK size requests work better than CSS for widget sizing)

4. **Should BindingMode track per-monitor overrides?**
   - What we know: Multi-monitor setups may override wallpaper on some monitors only
   - What's unclear: Whether to track binding at profile level or per-monitor level
   - Recommendation: Profile-level binding for Phase 8, per-monitor tracking deferred to future if needed
   - Confidence: MEDIUM (simpler but may need refinement based on user feedback)

## Sources

### Primary (HIGH confidence)
- [GTK4 Picture class](https://docs.gtk.org/gtk4/class.Picture.html) - Picture widget, content-fit property
- [GTK4 Overlay class](https://docs.gtk.org/gtk4/class.Overlay.html) - Overlay positioning with halign/valign
- [GTK4 Window.set_modal](https://docs.gtk.org/gtk4/method.Window.set_modal.html) - Modal dialog behavior
- [GTK4 Window.set_transient_for](https://docs.gtk.org/gtk4/method.Window.set_transient_for.html) - Dialog parent relationship
- [Rust std::path::Path](https://doc.rust-lang.org/std/path/struct.Path.html) - Path manipulation (parent, join, exists)
- [Rust std::path::PathBuf](https://doc.rust-lang.org/std/path/struct.PathBuf.html) - Owned path type
- [Serde field attributes](https://serde.rs/field-attrs.html) - #[serde(default)] and skip_serializing_if
- [TOML Rust crate](https://docs.rs/toml/latest/toml/) - TOML serialization

### Secondary (MEDIUM confidence)
- [GTK4 Picture content-fit property](https://docs.gtk.org/gtk4/property.Picture.content-fit.html) - Content scaling modes
- [GTK4 Picture keep-aspect-ratio (deprecated)](https://docs.gtk.org/gtk4/property.Picture.keep-aspect-ratio.html) - Legacy API replaced by content-fit
- [Relm4 AsyncComponent](https://docs.rs/relm4/latest/relm4/component/trait.AsyncComponent.html) - Component trait for dialogs
- [GTK4 FlowBox](https://docs.gtk.org/gtk4/class.FlowBox.html) - Container for profile cards

### Tertiary (LOW confidence)
- Blog posts about GTK4 overlays - Implementation examples but not authoritative
- Forum discussions about TOML HashMap ordering - Describes problem but no official solution

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All libraries already in use, official documentation available
- Architecture: HIGH - Patterns adapted from existing codebase (theme_card, profile_storage)
- Pitfalls: HIGH - Verified through official GTK4 docs and Rust stdlib documentation
- Code examples: HIGH - Based on existing working code + official documentation

**Research date:** 2026-01-25
**Valid until:** 2026-03-25 (60 days - stable GTK4/Relm4 APIs, unlikely to change)

**Domain-specific notes:**
- GTK4 API stable since 4.0 (2020), most patterns well-established
- Relm4 patterns match existing Phase 7 implementation
- TOML serialization straightforward with serde derives
- Path handling uses stable Rust stdlib, no platform-specific concerns for Linux target
