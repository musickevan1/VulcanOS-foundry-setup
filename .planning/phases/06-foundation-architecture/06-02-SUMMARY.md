---
phase: 06-foundation-architecture
plan: 02
subsystem: infra
tags: [gtk4, css, theming, rust, relm4, const_format]

# Dependency graph
requires:
  - phase: 06-01
    provides: Unified vulcan-appearance-manager crate foundation
provides:
  - Shared brand_css.rs module with Rust color constants and GTK4 CSS
  - Single source of truth for VulcanOS brand colors matching branding/vulcan-palette.css
  - Merged widget styles from both theme-manager and wallpaper-manager
affects: [06-03, 06-04, 06-05, theme-ui, wallpaper-ui]

# Tech tracking
tech-stack:
  added: [const_format]
  patterns:
    - "Shared CSS module pattern for GTK4 apps"
    - "Rust color constants alongside GTK @define-color declarations"

key-files:
  created:
    - vulcan-appearance-manager/src/brand_css.rs
  modified:
    - vulcan-appearance-manager/src/main.rs
    - vulcan-appearance-manager/Cargo.toml

key-decisions:
  - "Use GTK4 @define-color syntax (not CSS custom properties) for runtime compatibility"
  - "Export both Rust constants (for programmatic access) and CSS strings (for styling)"
  - "Merge widget styles from both old apps into single WIDGET_CSS constant"

patterns-established:
  - "brand_css::colors module provides Rust constants for validation/swatch generation"
  - "brand_css::FULL_CSS provides complete CSS for relm4::set_global_css()"
  - "const_format::concatcp! for compile-time string concatenation"

# Metrics
duration: 3min
completed: 2026-01-25
---

# Phase 6 Plan 2: Shared Brand CSS Summary

**Extracted duplicate CSS from both apps into brand_css.rs module with 15 color constants and unified widget styles**

## Performance

- **Duration:** 3 min
- **Started:** 2026-01-25T04:41:27Z
- **Completed:** 2026-01-25T04:44:24Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Created brand_css.rs module with 15+ Rust color constants matching branding/vulcan-palette.css exactly
- Merged GTK4 @define-color declarations and widget styles from both theme-manager and wallpaper-manager
- Eliminated 150+ lines of duplicate inline CSS from main.rs
- Established single source of truth for VulcanOS brand colors (INFRA-08 requirement)

## Task Commits

Each task was committed atomically:

1. **Task 1: Create brand_css.rs module** - `2a774f0` (feat)
2. **Task 2: Update main.rs to use brand_css module** - `7d26275` (feat)

## Files Created/Modified
- `vulcan-appearance-manager/src/brand_css.rs` - Shared brand CSS module with colors + widget styles (305 lines)
- `vulcan-appearance-manager/src/main.rs` - Entry point loading CSS from brand_css module (removed 150+ lines of inline CSS)
- `vulcan-appearance-manager/Cargo.toml` - Added const_format dependency for compile-time string concatenation
- `vulcan-appearance-manager/src/services/theme_parser.rs` - Fixed missing closing brace in tests module (auto-fix)

## Decisions Made
- **GTK4 @define-color syntax:** Used GTK4 `@define-color` syntax instead of CSS custom properties (`--vulcan-ember`) for runtime compatibility with GTK4 theme engine
- **Dual export pattern:** Export both Rust constants (for programmatic access like swatch generation) and CSS strings (for styling) from the same module
- **Widget style merge:** Combined shared widget styles from both old apps (flowboxchild, theme-card, buttons, scrollbar, etc.) into single WIDGET_CSS constant to eliminate duplication
- **const_format for concatenation:** Use const_format::concatcp! for compile-time string concatenation rather than runtime string allocation

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed missing closing brace in theme_parser.rs tests module**
- **Found during:** Task 2 (cargo check revealed pre-existing compilation error)
- **Issue:** theme_parser.rs tests module (starting line 332) was missing closing brace, preventing compilation
- **Fix:** Added missing `}` at end of tests module (line 552)
- **Files modified:** vulcan-appearance-manager/src/services/theme_parser.rs
- **Verification:** cargo check passes without errors
- **Committed in:** 7d26275 (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Bug fix was necessary to enable compilation. No scope creep - plan executed as written.

## Issues Encountered
None - straightforward CSS extraction and module creation.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Brand CSS module ready for use by all UI components
- Single source of truth established for VulcanOS brand colors
- Widget styles unified - ready for Phase 7 UI component development
- Color constants available for programmatic theme validation and swatch generation

**Ready for:** State machine implementation (06-03), wallpaper backend abstraction (06-04), and UI component development (Phase 7)

---
*Phase: 06-foundation-architecture*
*Completed: 2026-01-25*
