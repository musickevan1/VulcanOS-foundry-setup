# Phase 6: Foundation Architecture - Context

**Gathered:** 2026-01-24
**Status:** Ready for planning

<domain>
## Phase Boundary

Establish unified codebase with hardened state management, backend abstraction, and shared theming infrastructure. Merges vulcan-wallpaper-manager and vulcan-theme-manager into vulcan-appearance-manager. Delivers: merged crate, explicit state machine, wallpaper backend trait, theme bash parser, shared brand CSS module.

This is foundation/infrastructure — no new UI in this phase.

</domain>

<decisions>
## Implementation Decisions

### Merge Strategy
- **Extend vulcan-wallpaper-manager** — do NOT create a new crate from scratch
- Rename the wallpaper-manager crate to vulcan-appearance-manager (timing at Claude's discretion — research whether renaming in Phase 6 or Phase 7 causes less churn)
- **Absorb and delete** vulcan-theme-manager — move useful code into the renamed crate, then delete the old crate entirely. One crate to maintain.
- What's worth keeping from theme-manager is at Claude's discretion — evaluate code quality during research and decide what to cherry-pick vs rewrite

### State Management Model
- State machine with explicit transitions: Idle, Previewing, Applying (and potentially Error)
- **Strict with errors** — invalid state transitions return `Result::Err`. Callers must handle. No silent ignoring.
- Live system truth detection method at Claude's discretion — research what hyprpaper and swww actually support (IPC queries vs config file reads vs both)
- On close-with-unsaved-preview: **prompt the user** with three choices — Apply, Revert, or Keep as-is
- Error recovery on failed apply: at Claude's discretion — pick between rollback, error-state-with-retry, or error-state-only based on what's practical

### Theme File Format
- **Pure bash scripts** — theme files are bash scripts that export variables. No TOML intermediary.
- Theme variable scope (colors only vs colors+fonts+paths): at Claude's discretion — analyze what vulcan-theme-manager already defines and build from that
- **Parse and validate** — no eval/source. Extract expected variables from bash scripts safely (no shell execution). Error on missing required variables.
- **No inheritance** for Phase 6 — each theme is self-contained, flat files only. Inheritance deferred to future phase if needed.

### Brand CSS Architecture
- Source of truth: **branding/ directory** (vulcan-palette.css already exists there). External to any app crate.
- How apps consume brand colors (CSS file vs Rust constants vs both): at Claude's discretion — research what Iced actually supports and what existing apps use
- Static vs dynamic palette: at Claude's discretion — decide based on what makes sense for the theme system
- Scope: **colors only** — no spacing, border-radius, or other design tokens in the shared module. Those are per-app decisions.

### Claude's Discretion
- Crate rename timing (Phase 6 vs Phase 7)
- What to absorb from vulcan-theme-manager (code quality evaluation)
- Live system truth detection method (depends on hyprpaper/swww capabilities)
- Error recovery strategy on failed apply operations
- Theme variable scope (what variables a theme file should define)
- Brand color consumption layer (CSS, Rust consts, or both)
- Whether brand palette is static or theme-driven

</decisions>

<specifics>
## Specific Ideas

- The wallpaper manager is the more mature codebase (8 plans shipped in Phase 5) — it's the foundation to build on
- branding/vulcan-palette.css already exists and should be leveraged as the starting point for brand colors
- The theme parser must be safe — no shell execution during parsing. This is a security decision, not just a preference.
- State machine strictness is intentional — bugs should surface early via Result::Err, not silently corrupt state

</specifics>

<deferred>
## Deferred Ideas

- Theme inheritance (extending base themes) — revisit after flat theme system is proven
- Design tokens beyond colors (spacing, border-radius) — per-app for now, may unify later
- Theme-wallpaper binding — Phase 8
- UI integration of merged components — Phase 7

</deferred>

---

*Phase: 06-foundation-architecture*
*Context gathered: 2026-01-24*
