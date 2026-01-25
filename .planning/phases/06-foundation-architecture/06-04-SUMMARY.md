---
phase: 06-foundation-architecture
plan: 04
subsystem: wallpaper
tags: [rust, swww, hyprpaper, trait, abstraction, cli]

# Dependency graph
requires:
  - phase: 06-01
    provides: "Unified vulcan-appearance-manager crate with wallpaper and theme models/services"
provides:
  - "WallpaperBackend trait abstracting swww and hyprpaper operations"
  - "Runtime backend detection with swww preference"
  - "Tested output parsing for both backends"
affects: [06-05, 06-06, Phase 7 UI integration]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Trait abstraction for external CLI tools (swww, hyprctl)"
    - "Testable parsing functions extracted from trait implementations"
    - "Runtime backend detection via command probing"

key-files:
  created:
    - vulcan-appearance-manager/src/services/wallpaper_backend.rs
  modified:
    - vulcan-appearance-manager/src/services/mod.rs

key-decisions:
  - "Trait uses synchronous std::process::Command (matches existing pattern, GTK single-threaded)"
  - "Prefer swww over hyprpaper in detect_backend() for smoother transitions"
  - "Extract parsing logic into standalone functions for unit testing"

patterns-established:
  - "CLI backend abstraction: trait + detect function + testable parsers"
  - "Unit tests focus on parsing logic, not CLI execution (avoid external dependencies)"

# Metrics
duration: 2min
completed: 2026-01-25
---

# Phase 6 Plan 4: Wallpaper Backend Abstraction Summary

**WallpaperBackend trait with swww and hyprpaper implementations, plus runtime detection and tested output parsing**

## Performance

- **Duration:** 2 min
- **Started:** 2026-01-25T04:41:25Z
- **Completed:** 2026-01-25T04:43:07Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments

- Created WallpaperBackend trait abstracting apply(), query_active(), name() operations
- Implemented SwwwBackend using swww CLI with fade transitions
- Implemented HyprpaperBackend using hyprctl hyprpaper CLI (preload + wallpaper steps)
- Added detect_backend() function probing for available backend (prefers swww)
- Extracted and tested output parsing for both swww query and hyprpaper listactive formats
- All 4 unit tests pass (parsing both backends + empty output cases)

## Task Commits

Each task was committed atomically:

1. **Task 1: Create WallpaperBackend trait and implementations** - `6c41a05` (feat)
   - WallpaperBackend trait with 3 methods
   - SwwwBackend and HyprpaperBackend structs
   - detect_backend() with swww preference
   - parse_swww_query() and parse_hyprpaper_query() helpers
   - 4 unit tests

2. **Task 2: Register module** - `d96668c` (feat)
   - Added pub mod wallpaper_backend to services/mod.rs
   - Verified all tests pass

## Files Created/Modified

- `vulcan-appearance-manager/src/services/wallpaper_backend.rs` - Trait abstraction for wallpaper backends (swww and hyprpaper)
- `vulcan-appearance-manager/src/services/mod.rs` - Registered wallpaper_backend module

## Decisions Made

- **Synchronous Command API:** Used std::process::Command (not async) to match existing hyprpaper.rs and hyprctl.rs patterns and because GTK app is single-threaded
- **Swww preference:** detect_backend() tries swww first for smoother transitions, falls back to hyprpaper
- **Testable parsing:** Extracted parse_swww_query() and parse_hyprpaper_query() as standalone functions so unit tests don't require running daemons
- **Preserve existing code:** Did NOT modify hyprpaper.rs or hyprctl.rs - new trait coexists alongside them until Phase 7 migration

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None - straightforward trait abstraction with clear existing patterns to follow from hyprpaper.rs and hyprctl.rs.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

**Ready for Phase 7 UI integration:**
- WallpaperBackend trait provides clean abstraction for app.rs to use
- Both swww and hyprpaper supported via same interface
- detect_backend() handles runtime selection automatically
- Parsing tested independently of CLI availability

**Next steps (Phase 7):**
- Migrate app.rs to use WallpaperBackend trait instead of calling hyprpaper.rs directly
- Consider deprecating old hyprpaper.rs module functions once migration complete

**Note:** Plan 06-05 (unified state management) should happen before Phase 7 UI work to ensure backend selection state is tracked properly.

---
*Phase: 06-foundation-architecture*
*Completed: 2026-01-25*
