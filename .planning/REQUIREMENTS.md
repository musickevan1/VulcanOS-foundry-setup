# Requirements: VulcanOS v2.0 Vulcan Appearance Manager

**Defined:** 2026-01-24
**Core Value:** Cohesive, recoverable, keyboard-driven

## v2.0 Requirements

Requirements for this milestone. Each maps to roadmap phases.

### Unified App Interface

- [ ] **APP-01**: User can launch unified Vulcan Appearance Manager from menu or CLI
- [ ] **APP-02**: App displays tabs for Themes and Wallpapers in single window
- [ ] **APP-03**: Theme browser shows color preview cards for all available themes
- [ ] **APP-04**: Theme browser shows suggested wallpaper thumbnail for each theme
- [ ] **APP-05**: Wallpaper tab shows per-monitor layout with current assignments
- [ ] **APP-06**: User can assign different wallpapers to each monitor
- [ ] **APP-07**: User can split panoramic images across multiple monitors
- [ ] **APP-08**: Theme editor allows editing all 50+ color variables in groups

### Theme-Wallpaper Binding

- [ ] **BIND-01**: Each theme file includes optional wallpaper suggestion field
- [ ] **BIND-02**: Selecting a theme displays its suggested wallpaper (if defined)
- [ ] **BIND-03**: User can apply theme's suggested wallpaper with one click
- [ ] **BIND-04**: User can override theme's suggested wallpaper with custom choice
- [ ] **BIND-05**: Unified profiles save theme + wallpaper configuration together
- [ ] **BIND-06**: User can save/load/delete unified profiles by name

### Theming Infrastructure

- [ ] **INFRA-01**: Theme changes propagate to waybar (status bar colors)
- [ ] **INFRA-02**: Theme changes propagate to wofi (launcher colors)
- [ ] **INFRA-03**: Theme changes propagate to swaync (notification colors)
- [ ] **INFRA-04**: Theme changes propagate to hyprlock (lock screen colors)
- [ ] **INFRA-05**: Theme changes propagate to kitty terminal colors
- [ ] **INFRA-06**: Theme changes propagate to alacritty terminal colors
- [ ] **INFRA-07**: Appearance Manager GUI uses current theme colors (self-theming)
- [ ] **INFRA-08**: Shared CSS/variables file serves as single source of truth

### Preset Themes & Wallpapers

- [ ] **PRESET-01**: 8-10 polished preset themes with distinct visual personalities
- [ ] **PRESET-02**: Each preset theme includes at least one matching wallpaper
- [ ] **PRESET-03**: Well-known themes (Catppuccin, Dracula, etc.) use community wallpapers
- [ ] **PRESET-04**: Custom VulcanOS themes have AI-generated wallpapers (pre-made)
- [ ] **PRESET-05**: Wallpaper library stored in dotfiles directory structure
- [ ] **PRESET-06**: Wallpaper library syncs with git/backup system

### Third-Party App Discovery

- [ ] **THIRD-01**: App scans for installed apps that support theming
- [ ] **THIRD-02**: Discovery includes VS Code, Neovim, Firefox, and other common apps
- [ ] **THIRD-03**: App shows theming status for each discovered app (configured/not configured)
- [ ] **THIRD-04**: App provides links to theme marketplaces/resources for each discovered app

### Desktop Integration

- [ ] **DESK-01**: Appearance Manager accessible from vulcan-menu
- [ ] **DESK-02**: .desktop file created for application launchers
- [ ] **DESK-03**: App synced to archiso skeleton for fresh installs

## Future Requirements

Deferred to later milestones. Tracked but not in current roadmap.

### Automation

- **AUTO-01**: Theme changes automatically on time of day (light/dark)
- **AUTO-02**: Theme changes based on wallpaper colors (pywal-style)
- **AUTO-03**: Automatic third-party theme installation

### Advanced Features

- **ADV-01**: Theme import from other formats (base16, terminal.sexy)
- **ADV-02**: Theme export to shareable format
- **ADV-03**: AI wallpaper generation on-demand
- **ADV-04**: Color extraction from wallpapers for theme suggestions

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Automatic third-party theme installation | Complexity; discovery + links is sufficient for v2.0 |
| AI wallpaper generation on-demand | Pre-made wallpapers bundled; on-demand adds complexity |
| Dynamic theme generation from wallpapers | pywal/matugen produce inconsistent quality |
| Cloud sync for themes/wallpapers | Local dotfiles + git sync is sufficient |
| Multi-monitor spanning themes | Different colors per monitor adds complexity |
| Real-time theme preview (before apply) | Apply is fast enough; live preview is complex |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| APP-01 | TBD | Pending |
| APP-02 | TBD | Pending |
| APP-03 | TBD | Pending |
| APP-04 | TBD | Pending |
| APP-05 | TBD | Pending |
| APP-06 | TBD | Pending |
| APP-07 | TBD | Pending |
| APP-08 | TBD | Pending |
| BIND-01 | TBD | Pending |
| BIND-02 | TBD | Pending |
| BIND-03 | TBD | Pending |
| BIND-04 | TBD | Pending |
| BIND-05 | TBD | Pending |
| BIND-06 | TBD | Pending |
| INFRA-01 | TBD | Pending |
| INFRA-02 | TBD | Pending |
| INFRA-03 | TBD | Pending |
| INFRA-04 | TBD | Pending |
| INFRA-05 | TBD | Pending |
| INFRA-06 | TBD | Pending |
| INFRA-07 | TBD | Pending |
| INFRA-08 | TBD | Pending |
| PRESET-01 | TBD | Pending |
| PRESET-02 | TBD | Pending |
| PRESET-03 | TBD | Pending |
| PRESET-04 | TBD | Pending |
| PRESET-05 | TBD | Pending |
| PRESET-06 | TBD | Pending |
| THIRD-01 | TBD | Pending |
| THIRD-02 | TBD | Pending |
| THIRD-03 | TBD | Pending |
| THIRD-04 | TBD | Pending |
| DESK-01 | TBD | Pending |
| DESK-02 | TBD | Pending |
| DESK-03 | TBD | Pending |

**Coverage:**
- v2.0 requirements: 33 total
- Mapped to phases: 0 (pending roadmap)
- Unmapped: 33

---
*Requirements defined: 2026-01-24*
*Last updated: 2026-01-24 after initial definition*
