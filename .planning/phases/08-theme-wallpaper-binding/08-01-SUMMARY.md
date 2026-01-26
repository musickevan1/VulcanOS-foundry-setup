---
phase: 08-theme-wallpaper-binding
plan: 01
subsystem: vulcan-appearance-manager
status: complete
tags: [rust, data-models, theme-parser, binding-mode, unified-profile]

dependency-graph:
  requires:
    - 07-05  # Final integration - app structure complete
  provides:
    - BindingMode enum (ThemeBound, CustomOverride, Unbound)
    - UnifiedProfile struct (theme + wallpaper unified config)
    - resolve_theme_wallpaper helper function
    - THEME_WALLPAPER validation (relative paths only)
  affects:
    - 08-02  # Profile storage (will persist UnifiedProfile)
    - 08-03  # UI integration (will display BindingMode)
    - 08-04  # Theme application (will use resolve_theme_wallpaper)

tech-stack:
  added: []
  patterns:
    - "BindingMode enum with Default trait for state representation"
    - "Security validation for relative paths (reject /.., prevent traversal)"
    - "Option<PathBuf> resolution pattern for theme-relative paths"

key-files:
  created:
    - vulcan-appearance-manager/src/models/binding.rs
  modified:
    - vulcan-appearance-manager/src/models/mod.rs
    - vulcan-appearance-manager/src/models/profile.rs
    - vulcan-appearance-manager/src/services/theme_parser.rs

decisions:
  - id: binding-mode-states
    decision: "BindingMode has three states: ThemeBound (theme wallpaper active), CustomOverride (user overrode), Unbound (no theme wallpaper)"
    rationale: "Explicit state tracking allows UI to show binding status and enables automatic re-binding when theme changes"
    context: "Phase 8 unified profile design"

  - id: wallpaper-path-security
    decision: "THEME_WALLPAPER must be relative path, reject absolute paths and .. traversal"
    rationale: "Security: prevent themes from referencing arbitrary system files or escaping theme directory"
    context: "Theme parser validation"

  - id: resolve-theme-wallpaper-helper
    decision: "Helper function resolves theme_wallpaper relative to theme source_path, returns None if missing"
    rationale: "Centralized resolution logic, graceful degradation if file doesn't exist"
    context: "Theme wallpaper resolution pattern"

metrics:
  duration: 2m 58s
  completed: 2026-01-26
---

# Phase 8 Plan 1: Foundation - Data Models Summary

**One-liner:** Created BindingMode enum, UnifiedProfile struct, and secured THEME_WALLPAPER parser validation for theme-wallpaper binding foundation

## What Was Built

### 1. BindingMode Enum (models/binding.rs)

**Core type for tracking theme-wallpaper relationship:**

```rust
pub enum BindingMode {
    ThemeBound,       // Theme's suggested wallpaper is active
    CustomOverride,   // User has overridden theme's suggestion
    Unbound,          // Theme has no wallpaper suggestion (default)
}
```

**Features:**
- Default trait returns `Unbound`
- `display_name()` method for UI labels
- Serde derives for persistence

**Why this matters:** Explicit state tracking enables UI to show whether wallpaper matches theme, and automatic re-binding when theme changes.

### 2. UnifiedProfile Struct (models/binding.rs)

**Combines theme and wallpaper configuration:**

```rust
pub struct UnifiedProfile {
    name: String,
    description: String,
    theme_id: Option<String>,              // Bound theme
    monitor_wallpapers: HashMap<String, PathBuf>,  // Per-monitor wallpapers
    binding_mode: BindingMode,              // Relationship state
}
```

**Pattern:** Extends WallpaperProfile pattern from services/profile_storage.rs with theme binding.

### 3. resolve_theme_wallpaper Helper (models/binding.rs)

**Resolves theme's wallpaper path to absolute filesystem path:**

```rust
pub fn resolve_theme_wallpaper(theme: &Theme) -> Option<PathBuf>
```

**Logic:**
1. Extract `theme.theme_wallpaper` (relative path like "wallpapers/dark.png")
2. Get theme's source directory from `theme.source_path`
3. Join relative path to theme directory
4. Return `Some(abs_path)` only if file exists, else `None` with warning

**Why this matters:** Centralized resolution with graceful degradation - theme can suggest wallpaper but won't break if file missing.

### 4. THEME_WALLPAPER Validation (services/theme_parser.rs)

**Security validation added to `validate_theme()`:**

- ✅ Accept: `"wallpapers/dark.png"`, `"bg.jpg"` (relative paths)
- ❌ Reject: `"/etc/passwd"` (absolute paths)
- ❌ Reject: `"../../../etc/passwd"` (directory traversal)

**Why this matters:** Security - prevents themes from referencing arbitrary system files or escaping theme directory boundaries.

### 5. WallpaperProfile Consistency (models/profile.rs)

**Added description field with `#[serde(default)]`:**

Ensures `models/profile.rs` matches `services/profile_storage.rs` structure, preparing for UnifiedProfile to extend the pattern.

## Tasks Completed

| Task | Description | Commit | Files |
|------|-------------|--------|-------|
| 1 | Create BindingMode enum and UnifiedProfile struct | c9910d6 | binding.rs, mod.rs |
| 2 | Extend theme parser with THEME_WALLPAPER validation | bf0f7b8 | theme_parser.rs |
| 3 | Update WallpaperProfile with description field | 98778ca | profile.rs |

## Testing

**Theme parser tests (23 tests passing):**
- ✅ Accepts relative wallpaper paths
- ✅ Rejects absolute paths (/)
- ✅ Rejects directory traversal (..)
- ✅ Accepts None wallpaper
- ✅ Parses THEME_WALLPAPER from .sh files
- ✅ All existing security/validation tests still pass

**Compilation:**
- ✅ vulcan-appearance-manager compiles without errors
- ✅ BindingMode and UnifiedProfile exported from models/mod.rs
- ✅ resolve_theme_wallpaper helper compiles

## Code Quality

**Security validation:**
- Path traversal protection (reject ..)
- Absolute path rejection (reject /)
- Maintains existing dangerous pattern detection (command substitution, eval, pipes)

**Error handling:**
- Graceful degradation: resolve_theme_wallpaper returns None if file missing
- Warning logged when theme wallpaper not found (helps debugging)

**Type safety:**
- BindingMode enum prevents invalid states
- Option<String> for theme_id (explicit None when no theme)
- Option<PathBuf> for resolved wallpaper paths

## Next Phase Readiness

**For Plan 08-02 (Profile Storage):**
- ✅ UnifiedProfile struct ready for TOML serialization
- ✅ BindingMode derives Serialize/Deserialize
- ✅ Fields use #[serde(default)] for backwards compatibility

**For Plan 08-03 (UI Integration):**
- ✅ BindingMode::display_name() for UI labels
- ✅ UnifiedProfile has description field for display

**For Plan 08-04 (Theme Application):**
- ✅ resolve_theme_wallpaper() provides absolute paths for swww
- ✅ Security validation ensures safe paths only

## Deviations from Plan

None - plan executed exactly as written.

## Decisions Made

**1. BindingMode as enum (not boolean)**
- Chose three-state enum over simple bool
- Rationale: Distinguishes "no theme wallpaper" from "user override" from "theme-bound"
- Impact: UI can provide better feedback about binding status

**2. Security-first wallpaper path validation**
- Added comprehensive validation beyond basic parsing
- Rationale: Themes are user-editable shell scripts - prevent directory traversal attacks
- Impact: Themes cannot reference files outside their own directory

**3. Graceful degradation for missing wallpapers**
- resolve_theme_wallpaper returns None instead of erroring
- Rationale: Theme can suggest wallpaper but shouldn't break app if file missing
- Impact: More robust - handles user-deleted files, moved themes, etc.

## Performance Notes

**Build time:** ~3.3s for test compilation
**Test execution:** 23 tests in 0.02s
**Overall plan duration:** 2m 58s

## Lessons Learned

**Security validation should be comprehensive:**
- Not just parsing, but validating constraints (relative paths only)
- Reject patterns that could escape security boundaries
- Document security decisions in validation tests

**Type-driven design:**
- BindingMode enum makes invalid states unrepresentable
- Compiler enforces handling all binding modes
- Serde derives enable free persistence

**Foundation plans matter:**
- This plan is pure data modeling - no UI/logic complexity
- Solid data types make subsequent plans cleaner
- resolve_theme_wallpaper encapsulates tricky path resolution logic

## Documentation

**Module structure:**
```
models/
├── binding.rs          (NEW) BindingMode, UnifiedProfile, resolve_theme_wallpaper
├── mod.rs              (UPDATED) exports new types
├── profile.rs          (UPDATED) added description field
├── theme.rs            (existing) Theme struct with theme_wallpaper field
└── ...

services/
├── theme_parser.rs     (UPDATED) THEME_WALLPAPER validation
└── ...
```

**Public API exports:**
```rust
pub use models::{
    BindingMode,              // NEW
    UnifiedProfile,           // NEW
    resolve_theme_wallpaper,  // NEW
    // ... existing exports
};
```
