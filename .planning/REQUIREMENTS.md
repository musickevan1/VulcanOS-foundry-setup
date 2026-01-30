# Requirements: VulcanOS v2.0 Vulcan Appearance Manager

**Defined:** 2026-01-24
**Core Value:** Cohesive, recoverable, keyboard-driven

## v2.0 Requirements

Requirements for this milestone. Each maps to roadmap phases.

### Unified App Interface

- [x] **APP-01**: User can launch unified Vulcan Appearance Manager from menu or CLI
- [x] **APP-02**: App displays tabs for Themes and Wallpapers in single window
- [x] **APP-03**: Theme browser shows color preview cards for all available themes
- [x] **APP-04**: Theme browser shows suggested wallpaper thumbnail for each theme
- [x] **APP-05**: Wallpaper tab shows per-monitor layout with current assignments
- [x] **APP-06**: User can assign different wallpapers to each monitor
- [x] **APP-07**: User can split panoramic images across multiple monitors
- [x] **APP-08**: Theme editor allows editing all 50+ color variables in groups

### Theme-Wallpaper Binding

- [x] **BIND-01**: Each theme file includes optional wallpaper suggestion field
- [x] **BIND-02**: Selecting a theme displays its suggested wallpaper (if defined)
- [x] **BIND-03**: User can apply theme's suggested wallpaper with one click
- [x] **BIND-04**: User can override theme's suggested wallpaper with custom choice
- [x] **BIND-05**: Unified profiles save theme + wallpaper configuration together
- [x] **BIND-06**: User can save/load/delete unified profiles by name

### Theming Infrastructure

- [x] **INFRA-01**: Theme changes propagate to waybar (status bar colors)
- [x] **INFRA-02**: Theme changes propagate to wofi (launcher colors)
- [x] **INFRA-03**: Theme changes propagate to swaync (notification colors)
- [x] **INFRA-04**: Theme changes propagate to hyprlock (lock screen colors)
- [x] **INFRA-05**: Theme changes propagate to kitty terminal colors
- [x] **INFRA-06**: Theme changes propagate to alacritty terminal colors
- [x] **INFRA-07**: Appearance Manager GUI uses current theme colors (self-theming)
- [x] **INFRA-08**: Shared CSS/variables file serves as single source of truth

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
| APP-01 | Phase 7 | Complete |
| APP-02 | Phase 7 | Complete |
| APP-03 | Phase 7 | Complete |
| APP-04 | Phase 8 | Complete |
| APP-05 | Phase 7 | Complete |
| APP-06 | Phase 7 | Complete |
| APP-07 | Phase 7 | Complete |
| APP-08 | Phase 7 | Complete |
| BIND-01 | Phase 8 | Complete |
| BIND-02 | Phase 8 | Complete |
| BIND-03 | Phase 8 | Complete |
| BIND-04 | Phase 8 | Complete |
| BIND-05 | Phase 8 | Complete |
| BIND-06 | Phase 8 | Complete |
| INFRA-01 | Phase 9 | Complete |
| INFRA-02 | Phase 9 | Complete |
| INFRA-03 | Phase 9 | Complete |
| INFRA-04 | Phase 9 | Complete |
| INFRA-05 | Phase 9 | Complete |
| INFRA-06 | Phase 9 | Complete |
| INFRA-07 | Phase 9 | Complete |
| INFRA-08 | Phase 6 | Complete |
| PRESET-01 | Phase 10 | Pending |
| PRESET-02 | Phase 10 | Pending |
| PRESET-03 | Phase 10 | Pending |
| PRESET-04 | Phase 10 | Pending |
| PRESET-05 | Phase 10 | Pending |
| PRESET-06 | Phase 10 | Pending |
| THIRD-01 | Phase 10 | Pending |
| THIRD-02 | Phase 10 | Pending |
| THIRD-03 | Phase 10 | Pending |
| THIRD-04 | Phase 10 | Pending |
| DESK-01 | Phase 10 | Pending |
| DESK-02 | Phase 10 | Pending |
| DESK-03 | Phase 10 | Pending |

**Coverage:**
- v2.0 requirements: 33 total
- Mapped to phases: 33 (100% coverage ✓)
- Unmapped: 0

**Phase breakdown:**
- Phase 6 (Foundation Architecture): 1 requirement (INFRA-08)
- Phase 7 (Component Integration): 7 requirements (APP-01, APP-02, APP-03, APP-05, APP-06, APP-07, APP-08)
- Phase 8 (Theme-Wallpaper Binding): 7 requirements (APP-04, BIND-01, BIND-02, BIND-03, BIND-04, BIND-05, BIND-06)
- Phase 9 (Theming Infrastructure): 7 requirements (INFRA-01 through INFRA-07)
- Phase 10 (Preset Themes & Desktop Integration): 11 requirements (PRESET-01 through PRESET-06, THIRD-01 through THIRD-04, DESK-01 through DESK-03)

---
*Requirements defined: 2026-01-24*
*Last updated: 2026-01-30 after Phase 9 completion*
