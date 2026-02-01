---
phase: 13-architecture-cleanup
plan: 01
subsystem: ui
tags: [rust, gtk4, relm4, state-machine, preview-workflow]

# Dependency graph
requires:
  - phase: 08-theme-wallpaper-ui
    provides: AppState state machine and PreviewSnapshot structure
provides:
  - ThemeViewModel with AppState integration for preview lifecycle
  - Multi-preview support (clicking different themes preserves original state)
  - State tracking foundation for action bar visibility (Plan 02)
affects: [13-02, 13-03]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "State machine integration in Relm4 components"
    - "Multi-preview workflow (keep original snapshot, not previous preview)"

key-files:
  created: []
  modified:
    - vulcan-appearance-manager/src/components/theme_view.rs

key-decisions:
  - "First click from Idle creates snapshot and transitions to Previewing"
  - "Subsequent clicks while Previewing switch preview but keep ORIGINAL snapshot"
  - "Preview applies immediately on theme selection (not separate button)"

patterns-established:
  - "create_preview_snapshot() queries wallpaper backend for current system state"
  - "State transitions use AppState.start_preview() with Result handling"
  - "On preview failure, revert state to Idle to prevent inconsistent state"

# Metrics
duration: 2min
completed: 2026-02-01
---

# Phase 13 Plan 01: AppState Integration Summary

**ThemeViewModel wired to AppState state machine with multi-preview support preserving original snapshot for cancel/restore**

## Performance

- **Duration:** 2 min
- **Started:** 2026-02-01T21:57:07Z
- **Completed:** 2026-02-01T21:59:27Z
- **Tasks:** 3
- **Files modified:** 1

## Accomplishments
- ThemeViewModel tracks state with app_state, preview_snapshot, and previewing_theme_id fields
- Clicking theme from Idle captures snapshot and transitions to Previewing
- Subsequent clicks switch preview without creating new snapshots (preserves original)
- State machine foundation ready for action bar visibility and button sensitivity (Plan 02)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add AppState and PreviewSnapshot to ThemeViewModel** - `10e7aa2` (feat)
2. **Task 2: Create snapshot helper method** - `2836044` (feat)
3. **Task 3: Wire state transitions to ThemeSelected handler** - `d0d6854` (feat)

## Files Created/Modified
- `vulcan-appearance-manager/src/components/theme_view.rs` - Added app_state field, create_preview_snapshot() method, and state machine logic in ThemeSelected handler

## Decisions Made

**1. Preview applies immediately on theme selection**
- Rationale: Clicking a theme card already signals intent to preview. No need for separate "Preview" button click.
- Impact: Idle -> Previewing transition happens on first theme click, not explicit preview action.

**2. Multi-preview keeps ORIGINAL snapshot, not previous preview**
- Rationale: User expects "Cancel" to return to state BEFORE any preview, not the previous preview theme.
- Implementation: Subsequent clicks while Previewing update previewing_theme_id but don't create new snapshots.

**3. Revert state on preview failure**
- Rationale: If theme_applier::preview_theme() fails, keeping Previewing state would be inconsistent (UI thinks preview active but system unchanged).
- Implementation: Set app_state back to Idle and clear snapshot/previewing_theme_id on error.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None - wallpaper backend query and state transitions worked as expected.

## Next Phase Readiness

- ThemeViewModel has state tracking for Idle/Previewing/Applying states
- Preview snapshot creation queries wallpaper backend successfully
- Plan 02 can now implement action bar visibility based on app_state.is_previewing()
- Plan 03 can use preview_snapshot for cancel/restore functionality

**Blockers:** None

---
*Phase: 13-architecture-cleanup*
*Completed: 2026-02-01*
