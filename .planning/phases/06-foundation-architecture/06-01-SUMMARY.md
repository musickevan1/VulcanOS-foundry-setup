---
phase: 06-foundation-architecture
plan: 01
subsystem: infra
tags: [rust, gtk4, relm4, libadwaita, appearance, themes, wallpapers]

# Dependency graph
requires:
  - phase: 05-wallpaper-manager
    provides: "Wallpaper manager with multi-monitor support, profile system, GTK4/Relm4 architecture"
provides:
  - "Unified vulcan-appearance-manager crate with all wallpaper and theme models/services"
  - "Theme struct (50+ fields) for color/config management"
  - "Theme parser service for bash script themes"
  - "Theme storage service for CRUD operations"
  - "Theme applier service (preview/apply/revert via vulcan-theme CLI)"
affects: [06-foundation-architecture, 07-theme-editor-ui, 08-wallpaper-ui, 09-integration, 10-polish]

# Tech tracking
tech-stack:
  added: ["regex = 1 (for theme parsing)"]
  patterns: ["Unified appearance crate structure (models + services for both domains)"]

key-files:
  created:
    - "vulcan-appearance-manager/src/models/theme.rs"
    - "vulcan-appearance-manager/src/models/color_group.rs"
    - "vulcan-appearance-manager/src/services/theme_parser.rs"
    - "vulcan-appearance-manager/src/services/theme_storage.rs"
    - "vulcan-appearance-manager/src/services/theme_applier.rs"
  modified:
    - "vulcan-appearance-manager/Cargo.toml"
    - "vulcan-appearance-manager/src/main.rs"
    - "vulcan-appearance-manager/src/models/mod.rs"
    - "vulcan-appearance-manager/src/services/mod.rs"

key-decisions:
  - "Renamed vulcan-wallpaper-manager to vulcan-appearance-manager as the unified base"
  - "Kept wallpaper codebase as foundation (8 plans shipped, more mature)"
  - "Theme-manager components NOT moved yet (Phase 7 UI work)"
  - "Old vulcan-theme-manager directory left intact (cleanup is separate concern)"

patterns-established:
  - "Models organized by domain (wallpaper: Monitor/Wallpaper/WallpaperProfile, theme: Theme/ColorGroup)"
  - "Services organized by domain (5 wallpaper services + 3 theme services)"
  - "Theme parsing delegates to bash script validator (vulcan-theme CLI)"

# Metrics
duration: 2min
completed: 2026-01-25
---

# Phase 6 Plan 1: Foundation Architecture Summary

**vulcan-appearance-manager unified crate with wallpaper + theme models/services, Theme struct (50+ fields), and bash script theme parser**

## Performance

- **Duration:** 2 minutes
- **Started:** 2026-01-25T04:35:42Z
- **Completed:** 2026-01-25T04:37:50Z
- **Tasks:** 3
- **Files modified:** 10

## Accomplishments
- Renamed vulcan-wallpaper-manager to vulcan-appearance-manager (established unified crate)
- Merged theme models from vulcan-theme-manager (Theme struct, ColorGroup, ColorField)
- Merged theme services from vulcan-theme-manager (parser, storage, applier)
- All 9 tests pass including theme parser tests
- cargo check passes with zero errors

## Task Commits

Each task was committed atomically:

1. **Task 1: Rename crate and merge Cargo.toml** - `c424660` (chore)
2. **Task 2: Move theme models into merged crate** - `85a5f8a` (feat)
3. **Task 3: Move theme services into merged crate** - `c9e67a1` (feat)

## Files Created/Modified
- `vulcan-appearance-manager/Cargo.toml` - Updated name, description, added regex dependency
- `vulcan-appearance-manager/src/main.rs` - Updated app ID to com.vulcanos.appearance-manager
- `vulcan-appearance-manager/src/models/theme.rs` - Theme struct with 50+ color/config fields
- `vulcan-appearance-manager/src/models/color_group.rs` - ColorGroup and ColorField types for editor UI
- `vulcan-appearance-manager/src/models/mod.rs` - Re-exports for all 5 model types (wallpaper + theme)
- `vulcan-appearance-manager/src/services/theme_parser.rs` - Bash script parser for theme files
- `vulcan-appearance-manager/src/services/theme_storage.rs` - Theme file CRUD operations
- `vulcan-appearance-manager/src/services/theme_applier.rs` - Theme apply/preview/revert via vulcan-theme CLI
- `vulcan-appearance-manager/src/services/mod.rs` - Declarations for all 8 service modules (5 wallpaper + 3 theme)

## Decisions Made

**Wallpaper codebase chosen as foundation:**
- Wallpaper-manager had 8 complete plans shipped (Phase 5)
- Theme-manager was newer with less mature patterns
- Rename rather than merge-into preserves git history

**Theme-manager components NOT moved yet:**
- Phase 7 will handle UI component migration
- This plan focused purely on models + services (foundation only)
- Components have dependencies on GTK widgets that need careful integration

**Old vulcan-theme-manager directory preserved:**
- Deletion is a cleanup concern, not foundation concern
- Keeping it allows reference during Phase 7 UI work
- Can be removed after v2.0 fully shipped

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None - all tasks executed smoothly. Theme models and services integrated without conflicts because both crates used the same dependencies (GTK4, Relm4, anyhow, serde).

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

**Ready for Phase 6 remaining plans:**
- Unified crate foundation established
- All wallpaper models/services preserved and accessible
- All theme models/services imported and accessible
- Tests passing (9/9 including theme parser tests)
- Zero compilation errors

**Blockers/Concerns:**
None - foundation is stable and ready for subsequent Phase 6 plans.

---
*Phase: 06-foundation-architecture*
*Completed: 2026-01-25*
