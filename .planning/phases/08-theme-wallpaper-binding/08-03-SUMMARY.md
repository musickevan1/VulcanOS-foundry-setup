---
phase: 08-theme-wallpaper-binding
plan: 03
status: complete
subsystem: appearance-manager-ui
tags: [gtk4, relm4, css, overlay, theme-card]

# Dependency Graph
requires:
  - 08-01-foundation-data-models (resolve_theme_wallpaper function)
  - 07-02-theme-ui-components (ThemeItem component)
provides:
  - wallpaper-thumbnail-overlay (visual indicator of theme wallpaper binding)
  - override-badge-ui (visual indicator when user overrides theme wallpaper)
affects:
  - 08-06-integration (will wire up override state based on UnifiedProfile binding_mode)

# Tech Stack
tech-stack:
  added: []
  patterns:
    - gtk::Overlay for layering UI elements
    - gtk::Picture with ContentFit::Cover for aspect ratio handling
    - Relm4 factory component input messages for state updates

# File Tracking
key-files:
  created: []
  modified:
    - vulcan-appearance-manager/src/components/theme_card.rs
    - vulcan-appearance-manager/src/brand_css.rs

# Decisions
decisions:
  - decision: Wallpaper thumbnail is 60x40 pixels in bottom-right corner
    rationale: Unobtrusive but visible indicator, aspect ratio close to 16:9 monitors
    date: 2026-01-26
    affects: [theme card layout]

  - decision: Override badge uses emblem-default-symbolic icon
    rationale: Standard GTK icon for indicating user customization
    date: 2026-01-26
    affects: [theme card visual feedback]

  - decision: ThemeItemInput::SetOverride for updating override state
    rationale: Parent component (ThemeBrowser) will control override state based on UnifiedProfile
    date: 2026-01-26
    affects: [08-06 integration, message flow]

# Metrics
metrics:
  duration: 4 minutes
  completed: 2026-01-26
---

# Phase 08 Plan 03: Theme Card Wallpaper Overlay Summary

**One-liner:** Added 60x40 wallpaper thumbnail overlay and override badge to theme cards for visual binding indication

## What Was Built

### Task 1: Wallpaper Thumbnail Overlay (Commit 02d36ca - already complete)
- Wrapped theme card color preview with `gtk::Overlay`
- Added `gtk::Picture` widget (60x40px) in bottom-right corner for wallpaper thumbnail
- Added `gtk::Image` badge (16px) in top-right corner for override indicator
- Implemented wallpaper loading via `resolve_theme_wallpaper()` from 08-01
- Added `is_override` field to `ThemeItem` struct
- Created `ThemeItemInput::SetOverride(bool)` message for parent control
- Set `ContentFit::Cover` for proper aspect ratio handling

**Structure:**
```
gtk::Overlay {
  set_child = gtk::Frame {  // color preview
    ... 8 DrawingArea widgets for colors ...
  },
  add_overlay = gtk::Picture {  // wallpaper thumbnail
    60x40, bottom-right, ContentFit::Cover
  },
  add_overlay = gtk::Image {  // override badge
    16px, top-right, emblem-default-symbolic
  }
}
```

### Task 2: CSS Styling (Commit a47ab8d)
Added two CSS classes to `brand_css.rs`:

1. `.wallpaper-corner-preview` - Wallpaper thumbnail styling
   - 4px border radius for rounded corners
   - Drop shadow for visual separation
   - Subtle white border for contrast

2. `.override-badge` - Override indicator styling
   - Circular badge (50% border radius)
   - Accent color background (`@accent_color`)
   - Drop shadow for visibility

### Task 3: Override State Tracking (Commit 02d36ca - already complete)
- `ThemeCardOutput` kept simple with `Selected(Theme)` only
- `ThemeItemInput::SetOverride(bool)` added for parent updates
- `update()` handler implemented to receive override state
- Override badge visibility controlled by `is_override` field

**Message flow (to be wired in 08-06):**
```
ThemeBrowser checks UnifiedProfile.binding_mode
  → Sends SetOverride(true) if CustomOverride
  → ThemeItem updates is_override field
  → override_badge.set_visible(true)
```

## Files Modified

| File | Changes | Lines |
|------|---------|-------|
| `vulcan-appearance-manager/src/components/theme_card.rs` | Overlay structure, wallpaper Picture, override badge Image, SetOverride message, is_override field | +60 lines |
| `vulcan-appearance-manager/src/brand_css.rs` | CSS classes for thumbnail and badge styling | +15 lines |

## Technical Decisions

**1. Overlay vs. Stack**
- Chose `gtk::Overlay` over `gtk::Stack` for layering
- Rationale: Overlay allows multiple visible layers, Stack is for switching between mutually exclusive views
- Benefit: Can show color preview + wallpaper thumbnail + badge simultaneously

**2. Picture vs. Image for Wallpaper**
- Chose `gtk::Picture` for wallpaper thumbnail
- Rationale: Picture has built-in `ContentFit` property for aspect ratio preservation
- Benefit: Image scales correctly without distortion at 60x40 constraint

**3. Parent-Controlled Override State**
- Override state set by parent via `SetOverride` input message
- Rationale: ThemeItem is a factory component, parent (ThemeBrowser) has access to UnifiedProfile
- Benefit: Separation of concerns - card displays, browser manages state

**4. Thumbnail Size: 60x40 pixels**
- Chose 60x40 instead of 80x60 or smaller 40x30
- Rationale:
  - Visible but unobtrusive (doesn't dominate 180px wide card)
  - Aspect ratio (1.5:1) close to 16:9 monitors (1.77:1)
  - 4px margins fit within color preview frame (160px)
- Trade-off: Small enough to not interfere, large enough to recognize wallpaper

## Integration Points

**From 08-01 (Foundation Data Models):**
- Uses `resolve_theme_wallpaper(&theme) -> Option<PathBuf>`
- Returns `None` if theme has no wallpaper or file doesn't exist
- Graceful degradation: thumbnail hidden if no wallpaper

**From 07-02 (Theme Card Component):**
- Extended existing `ThemeItem` factory component
- Maintained compatibility with `ThemeCardOutput::Selected(Theme)`
- Added input handling without breaking parent message flow

**To 08-06 (Integration):**
- `ThemeBrowser` will check `UnifiedProfile.binding_mode`
- If `BindingMode::CustomOverride`, send `SetOverride(true)` to card
- Override badge becomes visible when user has custom wallpaper

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Import path for resolve_theme_wallpaper**
- **Found during:** Task 1 compilation
- **Issue:** Initial import used private `binding` module path
- **Fix:** Changed to public re-export from `models` module
- **Files modified:** `theme_card.rs`
- **Commit:** 02d36ca (same commit as 08-02, bundled fix)

**2. [Rule 1 - Bug] Automatic import consolidation**
- **Found during:** Task 1 editing
- **Issue:** Rust analyzer auto-consolidated imports to single line
- **Fix:** Accepted auto-formatting to `use crate::models::{Theme, resolve_theme_wallpaper};`
- **Files modified:** `theme_card.rs`
- **Commit:** Part of 02d36ca

## Verification

**Compilation:**
```bash
cargo check --package vulcan-appearance-manager
# Result: 0 errors, 97 warnings (all dead code/unused imports)
```

**Overlay structure:**
```bash
grep -n "gtk::Overlay\|wallpaper_thumb\|override_badge" theme_card.rs
# Found: Overlay at line 45, wallpaper_thumb at 115, override_badge at 129
```

**CSS classes:**
```bash
grep -n "wallpaper-corner-preview\|override-badge" brand_css.rs
# Found: wallpaper-corner-preview at 299, override-badge at 306
```

**All success criteria met:**
- ✅ Theme card uses `gtk::Overlay` to layer wallpaper thumbnail
- ✅ Wallpaper `Picture` is 60x40 pixels, bottom-right, `ContentFit::Cover`
- ✅ Override badge `Image` is 16px icon, top-right corner
- ✅ CSS classes provide visual styling (rounded corners, shadows)
- ✅ `is_override` field exists for future binding state tracking
- ✅ No compilation errors

## Next Phase Readiness

**Ready for 08-04 (Theme Application Wallpaper Sync):**
- Theme cards now display wallpaper thumbnails visually
- Override badge prepared for binding state indication
- No blockers

**Ready for 08-06 (Integration):**
- `ThemeItemInput::SetOverride(bool)` message available
- `is_override` field ready to be updated based on `UnifiedProfile.binding_mode`
- Badge visibility already wired to field

## Key Insights

**1. Overlay enables layered UI without widget nesting complexity**
- Single Overlay wraps color preview
- Multiple `add_overlay` widgets stack on top
- Clean separation: base layer (colors) + indicators (wallpaper, badge)

**2. ContentFit property eliminates manual aspect ratio math**
- `ContentFit::Cover` scales image to fill 60x40 while preserving aspect
- GTK4 handles cropping automatically
- No custom drawing code needed

**3. Factory component input messages decouple parent/child state**
- Parent (ThemeBrowser) manages binding mode logic
- Child (ThemeItem) displays what it's told
- Testable: can send SetOverride(true) in isolation

**4. Visual feedback improves discoverability**
- Wallpaper thumbnail: "This theme suggests a wallpaper"
- Override badge: "You've customized this theme's wallpaper"
- No tooltips needed - visual cues are self-explanatory

## Lessons Learned

**1. CSS in const strings requires proper escaping**
- Used `r#"..."#` raw strings for CSS
- Avoids issues with quotes inside CSS rules
- Consistent with existing `WIDGET_CSS` pattern

**2. GTK4 Picture vs Image distinction important**
- Picture: For displaying images with aspect ratio control
- Image: For icons and symbolic graphics
- Right tool for right job: Picture for wallpaper, Image for badge

**3. Relm4 widget naming with `#[name]` crucial for init_widgets**
- Named widgets accessible via `widgets.wallpaper_thumb`
- Enables setting visibility, file path after view! macro
- Pattern: declare in view!, configure in init_widgets

**4. Graceful degradation for missing wallpapers**
- `resolve_theme_wallpaper` returns `Option<PathBuf>`
- `if let Some(path)` pattern sets file, else hides widget
- No crashes if wallpaper file moved/deleted
