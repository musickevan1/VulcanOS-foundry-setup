# Phase 8: Theme-Wallpaper Binding - Context

**Gathered:** 2026-01-25
**Status:** Ready for planning

<domain>
## Phase Boundary

Themes suggest wallpapers and unified profiles save coordinated appearance as single unit. User can override theme's suggested wallpaper with custom choice. The binding relationship between theme and wallpaper is tracked and visible.

</domain>

<decisions>
## Implementation Decisions

### Binding behavior
- When user selects a theme with suggested wallpaper: **ask each time** ("Apply suggested wallpaper too?" dialog)
- When user has overridden theme's wallpaper: **badge/icon on theme card** shows override state
- User can reset to theme's suggested wallpaper via:
  - Explicit "Use theme wallpaper" reset button when in override mode
  - Re-applying the same theme (both methods work)

### Wallpaper suggestions
- Wallpapers **bundled with themes** — each theme folder contains its wallpapers (e.g., `themes/catppuccin/wallpapers/`)
- If theme's suggested wallpaper file doesn't exist: **keep current wallpaper** unchanged (don't error, don't change)

### Unified profiles
- Profile contents: **full snapshot** — theme ID, per-monitor wallpaper paths, binding mode, wallpaper backend settings, transition effects
- Profile naming: **auto-generate from theme name, but user can edit** before saving (e.g., auto-suggests "Catppuccin Mocha", user can change to "Work Dark")

### UI presentation
- Suggested wallpaper thumbnail: **corner preview** on theme cards (small picture-in-picture style)
- "Apply with wallpaper?" dialog: **full preview** showing theme colors + wallpaper thumbnail side by side
- Profile management: **dedicated third tab** alongside Themes and Wallpapers
- Profiles tab: **full preview cards** showing theme colors + wallpaper thumbnail for each saved profile

### Claude's Discretion
- Whether themes can suggest multiple wallpapers or just one (lean toward simplicity)
- Behavior when manually changing wallpaper breaks binding (silent override vs confirmation)
- Behavior for themes without THEME_WALLPAPER field (treat as no preference)
- Migration strategy for existing wallpaper-only profiles from Phase 5
- Handling profiles that reference missing themes (partial load vs fail gracefully)

</decisions>

<specifics>
## Specific Ideas

- Dialog preview should show theme + wallpaper side by side — gives user full picture before committing
- Badge on theme card for override state should be subtle but visible (like a small pin or edit icon)
- Profile cards in Profiles tab should be scannable — user quickly sees "oh that's my dark coding setup"

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 08-theme-wallpaper-binding*
*Context gathered: 2026-01-25*
