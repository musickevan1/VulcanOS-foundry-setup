---
phase: 05-vulcanos-wallpaper-manager
plan: 06
subsystem: ui
tags: [gtk4, relm4, image-processing, rust, hyprpaper, wallpaper]

# Dependency graph
requires:
  - phase: 05-04
    provides: "Monitor layout canvas with visual feedback"
  - phase: 05-05
    provides: "Profile management for saving/loading wallpaper sets"
provides:
  - "Panoramic image splitting service using image crate"
  - "Split dialog for importing wide images"
  - "Automatic per-monitor wallpaper generation from panoramic sources"
affects: [05-07-cli-integration, future-wallpaper-features]

# Tech tracking
tech-stack:
  added: [image crate v0.25]
  patterns: [canvas bounds calculation, Lanczos3 scaling, popup window dialogs]

key-files:
  created:
    - vulcan-wallpaper-manager/src/services/image_splitter.rs
    - vulcan-wallpaper-manager/src/components/split_dialog.rs
  modified:
    - vulcan-wallpaper-manager/src/services/mod.rs
    - vulcan-wallpaper-manager/src/components/mod.rs
    - vulcan-wallpaper-manager/src/app.rs

key-decisions:
  - "Lanczos3 filter for image scaling (quality over speed for wallpapers)"
  - "Center crop strategy when panoramic doesn't match canvas aspect ratio"
  - "Output to ~/Pictures/Wallpapers/spanning/<name>/ directory structure"
  - "Auto-populate name from filename for convenience"
  - "Keep dialog open on error to allow retry"

patterns-established:
  - "Popup window pattern: create gtk::Window to host child component dialogs"
  - "Canvas bounds calculation from monitor layout (handles negative offsets)"
  - "Vertical monitor detection via transform field (1 or 3)"

# Metrics
duration: 4min
completed: 2026-01-24
---

# Phase 05 Plan 06: Adaptive Wallpaper Generation Summary

**Panoramic image splitter with monitor layout-aware cropping, Lanczos3 scaling, and automatic hyprpaper application**

## Performance

- **Duration:** 4 min
- **Started:** 2026-01-24T06:35:41Z
- **Completed:** 2026-01-24T06:40:20Z
- **Tasks:** 3
- **Files modified:** 5

## Accomplishments
- Image splitter calculates canvas bounds from actual monitor positions and handles negative offsets
- Panoramic images scaled with Lanczos3 for quality, center-cropped to fit multi-monitor layout
- Split dialog with FileDialog, auto-populated name, and monitor preview
- Generated wallpapers automatically applied via hyprpaper and saved to profile
- Import button integrated into header bar for easy access

## Task Commits

Each task was committed atomically:

1. **Task 1: Create image splitter service** - `0ccb760` (feat)
2. **Task 2: Create split dialog component** - `68c7f6a` (feat)
3. **Task 3: Integrate split dialog into main app** - `bae5c15` (feat)

## Files Created/Modified
- `vulcan-wallpaper-manager/src/services/image_splitter.rs` - Panoramic splitting with canvas bounds calculation
- `vulcan-wallpaper-manager/src/components/split_dialog.rs` - Dialog UI for panoramic import
- `vulcan-wallpaper-manager/src/services/mod.rs` - Added image_splitter module export
- `vulcan-wallpaper-manager/src/components/mod.rs` - Added split_dialog module export
- `vulcan-wallpaper-manager/src/app.rs` - Import button, popup window, wallpaper application integration

## Decisions Made

**Image quality:**
- Lanczos3 filter chosen for scaling (higher quality at cost of speed, acceptable for wallpapers)
- Center crop strategy when aspect ratios don't match exactly

**UX patterns:**
- Auto-populate wallpaper name from filename for convenience
- Keep dialog open on error to allow user to retry without losing context
- Close dialog automatically on success or cancel

**File organization:**
- Output to `~/Pictures/Wallpapers/spanning/<name>/` for organized storage
- Naming pattern: `<name>-<monitor>.png` for clear monitor association

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

**Relm4 syntax:**
- Initial attempt to use `connect_changed[sender]` syntax failed - Relm4 doesn't support that pattern
- Solution: Named widget with manual connection in init() using sender.clone()

**Sender parameter:**
- Update method had `_sender` parameter (unused), caused error when ShowSplitDialog tried to use it
- Solution: Removed underscore prefix to make sender available

Both were simple Rust/Relm4 syntax corrections, resolved quickly.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

**Wallpaper Manager GUI complete:**
- Monitor layout visualization ✓
- Wallpaper picker with thumbnails ✓
- Profile management (save/load/delete) ✓
- Panoramic image splitting ✓

**Ready for:**
- CLI integration (05-07) - command-line interface for scripting/automation
- Distribution packaging - standalone binary ready for VulcanOS inclusion

**Note:** Canvas bounds calculation correctly handles:
- Negative monitor offsets (monitors positioned left/above origin)
- Vertical monitors (transform == 1 or 3)
- Mixed resolutions and positions
- HiDPI scaling via logical_size()

This replicates the functionality of `scripts/split-wallpaper.sh` but with:
- Real-time monitor detection via hyprctl (no hardcoded layout)
- Pure Rust (no ImageMagick dependency)
- GUI integration (no CLI interaction needed)

---
*Phase: 05-vulcanos-wallpaper-manager*
*Completed: 2026-01-24*
