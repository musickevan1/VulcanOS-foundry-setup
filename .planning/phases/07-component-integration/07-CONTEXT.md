# Phase 7: Component Integration - Context

**Gathered:** 2026-01-24
**Status:** Ready for planning

<domain>
## Phase Boundary

Merge existing theme browser and wallpaper manager UIs into a single tabbed application (vulcan-appearance-manager) using libadwaita. Both apps currently work as standalone single-view apps — this phase combines them into one cohesive window. Theme-wallpaper binding, theming propagation, and preset themes are separate phases (8, 9, 10).

</domain>

<decisions>
## Implementation Decisions

### Claude's Discretion

All implementation areas deferred to Claude's judgment. The following are recommended approaches based on analysis of both existing codebases and libadwaita conventions.

### Tab structure & navigation
- Use `adw::ViewStack` + `adw::ViewSwitcher` (not `adw::TabView`) — ViewStack is the libadwaita standard for fixed top-level sections, while TabView is for user-managed dynamic tabs
- Two top-level views: "Themes" and "Wallpapers"
- ViewSwitcher in the header bar for persistent navigation
- Each view preserves its existing internal layout:
  - Themes tab: Horizontal paned (browser left, preview right) — from theme-manager
  - Wallpapers tab: Vertical paned (monitor layout top, picker bottom) — from wallpaper-manager
- Profile manager controls move to the header bar area (accessible from both tabs since unified profiles span both)
- No nested tabs — keep it flat

### Theme browser presentation
- Retain existing factory-based FlowBox grid pattern from theme-manager
- Theme cards show: 8-color palette preview (2x4 grid), theme name, badges (Current, Built-in)
- FlowBox with responsive column count (was 6 columns, keep flexible)
- Clicking a theme card shows preview panel on the right side of the paned view
- No inline editing — "Edit" button opens modal dialog (existing pattern)
- Preview panel uses existing Cairo mock-desktop rendering

### Wallpaper tab migration
- Migrate wallpaper UI components as-is into the Wallpapers view
- Monitor layout visualization (Cairo DrawingArea) stays on top
- Wallpaper picker FlowBox grid stays on bottom
- Split panoramic dialog remains a modal window
- Profile manager dropdown migrates to header bar area (shared across both tabs)
- Preserve existing interaction: click monitor → select wallpaper → apply

### Theme editor experience
- Keep existing modal dialog approach (separate window, not inline)
- Color groups organized in collapsible `adw::ExpanderRow` sections (existing pattern)
- 8 groups: Backgrounds, Foregrounds, Accents, ANSI Colors, Bright ANSI, UI Elements, Gradients, System Themes
- Each color field uses GTK `ColorButton` with hex parsing
- Text fields for system theme names (GTK, icons, Kvantum, Neovim)
- Save/Cancel buttons at bottom of dialog
- No live preview during editing (preview updates after save) — keeps state simple for Phase 6's state machine

</decisions>

<specifics>
## Specific Ideas

- Both apps already use the same Relm4 component architecture with type-safe message passing — merging is structural, not architectural
- The wallpaper manager uses vertical paned layout; the theme manager uses horizontal paned layout. Each tab keeps its own proven layout.
- Global CSS is already unified via Phase 6's shared brand CSS module — no duplicate styling needed
- Profile manager component currently lives in wallpaper-manager; it should become a top-level component accessible from the header bar since unified profiles will eventually span themes + wallpapers (Phase 8)
- The theme-manager's `ThemeItem` factory pattern (`FactoryVecDeque`) is the more sophisticated grid implementation and should be the reference pattern

</specifics>

<deferred>
## Deferred Ideas

- Theme-wallpaper binding (theme suggests wallpaper) — Phase 8
- Unified profiles saving both theme + wallpaper — Phase 8
- Self-theming GUI (app uses active theme colors) — Phase 9
- Preset themes with matching wallpapers — Phase 10
- Third-party app theming discovery — Phase 10

</deferred>

---

*Phase: 07-component-integration*
*Context gathered: 2026-01-24*
