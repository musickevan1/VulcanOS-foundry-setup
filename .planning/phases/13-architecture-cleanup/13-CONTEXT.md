# Phase 13: Architecture Cleanup - Context

**Gathered:** 2026-02-01
**Status:** Ready for planning

<domain>
## Phase Boundary

Wire the existing AppState state machine into UI components for proper preview/apply/cancel workflow. The state machine already exists in `state.rs` with Idle, Previewing, Applying, and Error states — this phase connects it to the theme browser UI so buttons respond correctly and Cancel restores previous state.

</domain>

<decisions>
## Implementation Decisions

### Preview workflow
- Single click on theme card triggers preview immediately (no explicit Preview button)
- Preview includes both theme AND wallpaper together (full coordinated preview)
- Preview applies to live desktop immediately (not a preview pane)
- Theme colors apply first, wallpaper follows (instant feedback, wallpaper catches up)

### Cancel behavior
- Cancel restores BOTH theme and wallpaper (full restore)
- Per-monitor wallpaper state is restored exactly (each monitor returns to its previous wallpaper)
- No auto-cancel timeout — preview stays until user explicitly applies or cancels
- Closing app while previewing = implicit apply (keep the new look)

### Button state logic
- Action bar appears at bottom of theme view
- Action bar only visible during preview (slides in when preview starts, disappears when idle)
- Apply button shows spinner/loading state during persistence
- Apply failure shows inline error, stays in preview mode (user can retry or cancel)

### Multi-preview handling
- Clicking different themes while previewing switches preview, keeps ORIGINAL state to restore
- Original state = state before any preview started (not previous preview)
- Show both indicators: "Applied: X" and "Previewing: Y" in action bar
- After Apply completes, return immediately to idle state (action bar slides away)

### Claude's Discretion
- Re-clicking same theme card while previewing (no-op vs double-click-to-apply)
- Exact animation timing for action bar slide in/out
- Spinner/loading indicator styling
- Error message wording and styling

</decisions>

<specifics>
## Specific Ideas

- Action bar behavior inspired by "unsaved changes" bars — appears when needed, disappears when resolved
- "Applied: X / Previewing: Y" pattern gives user confidence about what Cancel will do
- Instant theme color feedback with wallpaper following creates responsive feel without blocking

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 13-architecture-cleanup*
*Context gathered: 2026-02-01*
