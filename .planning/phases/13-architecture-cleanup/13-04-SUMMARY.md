---
phase: 13-architecture-cleanup
plan: 04
subsystem: appearance-manager-ui
tags: [gtk4, relm4, window-management, implicit-apply]
requires: [13-02, 13-03, 13-05]
provides:
  - "Implicit apply on window close during preview"
  - "Preview state tracking in App coordinator"
affects: []
tech-stack:
  added: []
  patterns:
    - "GTK4 close-request signal handling"
    - "Preview state propagation via Relm4 outputs"
    - "Synchronous theme application before window destruction"
key-files:
  created: []
  modified:
    - vulcan-appearance-manager/src/app.rs
    - vulcan-appearance-manager/src/components/theme_view.rs
decisions:
  - id: close-applies-preview
    desc: "Closing app while previewing implicitly applies the preview"
    rationale: "Prevents loss of user's preview work, matches user expectation"
    alternatives: "Could require explicit Apply before close, but frustrates users"
  - id: sync-apply-on-close
    desc: "Window close-request handler performs synchronous theme application"
    rationale: "close-request signal fires before destruction, ensuring apply completes"
    alternatives: "Async apply would race with window destruction"
metrics:
  duration: 12min
  completed: 2026-02-01
---

# Phase 13 Plan 04: Implicit Apply on Close Summary

**One-liner:** Window close during preview implicitly applies previewed theme via close-request handler and state tracking.

## What Was Built

### 1. Preview State Tracking in App Coordinator
- Added `is_previewing` and `previewing_theme_id` fields to App struct
- ThemeView emits `PreviewStateChanged` output when entering/leaving Previewing state
- App tracks preview state via message forwarding from ThemeView outputs

**Key pattern:** Coordinator tracks child component state via outputs for cross-cutting concerns (window lifecycle).

### 2. Window Close Handler with Implicit Apply
- Implemented GTK4 `close-request` signal handler on ApplicationWindow
- Handler checks `is_previewing` flag before allowing window close
- If previewing, synchronously applies theme via `theme_applier::apply_theme()`
- Window closes after apply completes (close-request is synchronous)

**Behavior:** Closing app while previewing keeps the new theme. Closing app while Idle does nothing special.

### 3. State Propagation Events
ThemeView emits PreviewStateChanged in:
- `ThemeSelected` handler after successful preview (is_previewing: true)
- `ThemeSelected` handler when switching previews (still true, new theme_id)
- `CancelPreview` handler after cancel (is_previewing: false)
- `ApplyTheme` handler after apply (is_previewing: false)

## Deviations from Plan

None - plan executed exactly as written.

## Verification Results

### Build Verification
- `cargo check -p vulcan-appearance-manager` passed
- `cargo build -p vulcan-appearance-manager` succeeded
- All compiler warnings resolved

### Human Verification (Checkpoint Task 4)
User tested complete workflow:
1. ✓ Preview workflow - action bar slides in, theme changes
2. ✓ Multi-preview - switching between themes works
3. ✓ Cancel - reverts to original theme and wallpapers
4. ✓ Apply - persists theme after close/reopen
5. ✓ Implicit apply on close - previewed theme persists after window close
6. ✓ Button sensitivity - action bar hidden when not previewing

**Result:** All verifications passed, workflow approved.

## Commits

| Hash | Type | Description |
|------|------|-------------|
| aacfce7 | feat | Add PreviewStateChanged output to ThemeView |
| 621da0b | feat | Track preview state in App coordinator |
| 91ee8c6 | feat | Add window close handler for implicit apply |

## Technical Notes

### GTK4 Close Request Signal
The `close-request` signal returns `gtk::glib::Propagation`:
- `Proceed` - Allow window to close (default)
- `Stop` - Prevent close (e.g., unsaved changes prompt)

We return Proceed but perform synchronous work before the signal completes, ensuring theme applies before destruction.

### Why Synchronous Apply
- `close-request` is emitted synchronously before window.destroy()
- Async apply would race with window destruction
- ThemeView may be destroyed before async apply completes
- Direct `theme_applier::apply_theme()` call ensures completion

### State Tracking Pattern
Rather than querying ThemeView state (which requires exposing getters), we track state in the coordinator via outputs. This:
- Keeps ThemeView encapsulated (no state queries)
- Follows Relm4 unidirectional data flow
- Enables multiple consumers of preview state changes

## Files Modified

### vulcan-appearance-manager/src/components/theme_view.rs
- Added `PreviewStateChanged` variant to ThemeViewOutput
- Emitted in ThemeSelected (enter/switch preview), CancelPreview, ApplyTheme handlers
- Tracks is_previewing boolean and current previewing_theme_id

### vulcan-appearance-manager/src/app.rs
- Added `is_previewing` and `previewing_theme_id` fields to App struct
- Added `PreviewStateChanged` and `WindowCloseRequested` AppMsg variants
- Wired ThemeViewOutput::PreviewStateChanged to AppMsg forwarding
- Implemented close-request handler that sends WindowCloseRequested
- WindowCloseRequested handler performs implicit apply if is_previewing

## Integration with Phase 13

This plan completes the AppState integration wave:
- **13-01:** State machine wired into ThemeViewModel
- **13-02:** Action bar visibility respects state
- **13-03:** Cancel restores original theme + wallpapers
- **13-04:** Implicit apply on window close (this plan)
- **13-05:** Apply transitions through Applying state with rollback

All v2.1 ARCH requirements now met:
- ARCH-01: AppState state machine integrated ✓
- ARCH-02: Cancel Preview restores wallpapers ✓
- ARCH-03: Buttons respect state transitions ✓

## Next Phase Readiness

**Blockers:** None

**Concerns:** None

**Ready for:** Phase completion - all 5 plans in Phase 13 are complete.

**Future work:** v2.2 could add:
- User confirmation dialog before implicit apply (optional)
- Toast notification showing "Theme applied" on close
- Setting to disable implicit apply (restore on close instead)
