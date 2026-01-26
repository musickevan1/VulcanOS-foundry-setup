---
phase: 09-theming-infrastructure
plan: 02
subsystem: theming
tags: [gtk, css, vulcan-theme, rust, services]

dependency_graph:
  requires: []
  provides: ["GTK CSS generation", "theme_css service"]
  affects: [09-03, 09-04]

tech_stack:
  added: []
  patterns: ["@define-color syntax", "heredoc CSS generation"]

key_files:
  created:
    - vulcan-appearance-manager/src/services/theme_css.rs
  modified:
    - dotfiles/scripts/.local/bin/vulcan-theme
    - vulcan-appearance-manager/src/services/mod.rs

decisions:
  - id: gtk-css-path
    choice: "~/.config/vulcan/current-theme.css"
    rationale: "Standard vulcan config directory, clear naming, single source of truth"
  - id: libadwaita-overrides
    choice: "Include window_bg_color, view_bg_color, headerbar_bg_color, etc."
    rationale: "Enables consistent theming across all GTK4/Libadwaita applications"

metrics:
  duration: "1m 28s"
  completed: "2026-01-26"
---

# Phase 09 Plan 02: Theme CSS Generation Summary

**One-liner:** vulcan-theme generates GTK @define-color CSS file, Rust service reads it for app theming

## What Was Done

### Task 1: GTK CSS generation in vulcan-theme
- Added `generate_gtk_css()` function that creates `~/.config/vulcan/current-theme.css`
- Uses heredoc syntax for clean CSS output
- Generates 27 @define-color statements:
  - Theme colors (theme_accent, theme_bg_*, theme_fg_*, semantic colors)
  - Libadwaita overrides (accent_bg_color, window_bg_color, view_bg_color, etc.)
- Called automatically during every `apply_theme()` execution

### Task 2: theme_css Rust service
- Created `services/theme_css.rs` with three public functions:
  - `get_theme_css_path()` - Returns PathBuf to CSS file
  - `get_theme_css()` - Returns Option<String> with CSS content (graceful on missing file)
  - `has_theme_css()` - Returns bool for existence check
- Module exported from `services/mod.rs`
- Compiles without errors

## Files Changed

| File | Change |
|------|--------|
| `dotfiles/scripts/.local/bin/vulcan-theme` | Added generate_gtk_css() function, call it in apply_theme() |
| `vulcan-appearance-manager/src/services/theme_css.rs` | New file - theme CSS reading service |
| `vulcan-appearance-manager/src/services/mod.rs` | Added `pub mod theme_css;` |

## Generated CSS Example

```css
/* Theme: Vulcan Forge (vulcan-forge) */
@define-color theme_accent #f97316;
@define-color theme_bg_primary #1c1917;
@define-color theme_bg_secondary #292524;
/* ... 24 more color definitions ... */
@define-color window_bg_color #1c1917;
@define-color headerbar_bg_color #292524;
```

## Commits

- `0271c06`: feat(09-02): add GTK CSS generation to vulcan-theme
- `fa0ed0e`: feat(09-02): add theme_css service for reading current theme CSS

## Deviations from Plan

### Auto-added Enhancements

**1. [Rule 2 - Missing Critical] Added Libadwaita color overrides**
- **Found during:** Task 1
- **Issue:** Basic theme colors alone don't fully theme Libadwaita apps
- **Fix:** Added window_bg_color, view_bg_color, headerbar_bg_color, card_bg_color, popover_bg_color
- **Files modified:** dotfiles/scripts/.local/bin/vulcan-theme
- **Commit:** 0271c06

## Verification Results

1. CSS file generated on theme switch
2. Contains @define-color syntax with actual hex colors
3. Rust service compiles without errors
4. 27 color definitions in generated CSS

## Next Phase Readiness

Plan 09-03 (CSS Loading) can now:
- Use `services::theme_css::get_theme_css()` to read the CSS
- Load CSS content into GTK4 CssProvider
- Apply to application display
