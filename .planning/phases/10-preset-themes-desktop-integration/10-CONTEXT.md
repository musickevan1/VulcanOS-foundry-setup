# Phase 10: Preset Themes & Desktop Integration - Context

**Gathered:** 2026-01-30
**Status:** Ready for planning

<domain>
## Phase Boundary

Polished preset themes with matching wallpapers, third-party app theming discovery, and unified desktop integration. This phase finalizes the Vulcan Appearance Manager by bundling curated content and integrating into the desktop environment.

</domain>

<decisions>
## Implementation Decisions

### Theme Personality & Naming
- Dark-focused theme collection (primarily dark themes, select light variants)
- Mixed naming: community names for ports (Catppuccin Mocha, Dracula), Vulcan-* for originals
- Existing themes to polish: Catppuccin Mocha, Dracula, Nord, Gruvbox Dark, Tokyo Night, Rosé Pine, One Dark, Vulcan-Forge
- Add more community ports AND original Vulcan-* themes
- Light variants for themes that naturally support it (e.g., Catppuccin Latte, Gruvbox Light)

### Wallpaper Sourcing & Storage
- Mixed sourcing: community wallpapers for ports, AI-generated for Vulcan-* originals
- Storage location: `dotfiles/wallpapers/` tracked directly in Git
- Resolution: 1080p minimum (1920x1080 or higher)
- Quantity per theme: Claude's discretion based on availability/generation

### Third-Party App Discovery
- Show installed apps that support theming in a section within the Themes tab
- App list: Claude's discretion based on VulcanOS package list (dev tools focus likely)
- Theming status: Claude's discretion on what to detect (installed vs configured)
- Discovery is informational — no automatic theme application to third-party apps

### Desktop Integration
- Top-level "Appearance" menu in desktop menu structure
- Consolidate old theme/wallpaper launchers into unified menu
- Quick actions for both theme switching AND wallpaper picking (without opening full manager)
- Quick switching UX: Claude's discretion on implementation (submenu vs popup)
- Remove old vulcan-theme-manager and vulcan-wallpaper-manager launchers completely
- Appearance Manager binary and configs synced to archiso skeleton

### Claude's Discretion
- Number of wallpapers per theme
- Which specific third-party apps to detect
- How to display theming status for discovered apps
- Quick theme/wallpaper switching UX implementation
- Which themes warrant light variants

</decisions>

<specifics>
## Specific Ideas

- "Maybe a toplevel appearance menu -> launch manager, quick themes, quick wallpapers"
- Menu should replace/consolidate old separate theme and wallpaper launchers
- Quick actions should allow switching without opening the full Appearance Manager

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 10-preset-themes-desktop-integration*
*Context gathered: 2026-01-30*
