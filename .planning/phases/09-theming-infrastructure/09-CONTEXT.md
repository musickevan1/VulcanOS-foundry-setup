# Phase 9: Theming Infrastructure - Context

**Gathered:** 2026-01-25
**Status:** Ready for planning

<domain>
## Phase Boundary

Theme changes propagate automatically to all desktop components (waybar, wofi, swaync, hyprlock, kitty, alacritty) and the Appearance Manager GUI reflects the active theme colors (self-theming).

This phase implements the propagation mechanisms — not the themes themselves (Phase 10).

</domain>

<decisions>
## Implementation Decisions

### User Feedback
- Single "Theme applied" toast notification — not per-component status
- Trust the propagation works; only surface errors if something fails
- No progress indicator needed for component updates (should be fast)

### Claude's Discretion
- Propagation timing (instant vs batched)
- Reload mechanisms for each component (SIGHUP, config rewrite, IPC)
- Order of component updates
- Self-theming implementation approach (CSS variables, GTK provider, etc.)
- Error handling strategy (retry, skip, notify)
- Which components need restart vs hot-reload

</decisions>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches for each component's reload mechanism.

**Known component reload patterns (for researcher reference):**
- waybar: `pkill -SIGUSR2 waybar` for style reload
- wofi: CSS file rewrite, picks up on next launch
- swaync: `swaync-client -rs` for style reload
- hyprlock: Config rewrite, picks up on next lock
- kitty: `kitty @ set-colors` for live update or config rewrite
- alacritty: `alacritty msg config` or config rewrite

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 09-theming-infrastructure*
*Context gathered: 2026-01-25*
