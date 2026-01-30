# Roadmap: VulcanOS

## Milestones

- **v1.0 VulcanOS Foundation** - Phases 1-5 (Phase 5 shipped 2026-01-24)
- **v2.0 Vulcan Appearance Manager** - Phases 6-10 (in progress)

## Phases

<details>
<summary>v1.0 VulcanOS Foundation (Phases 1-5) - Phase 5 shipped 2026-01-24</summary>

### Phase 1: T2 Kernel Protection
**Goal**: System is protected from catastrophic kernel update failures that make T2 MacBook Pro unbootable
**Depends on**: Nothing (first phase)
**Requirements**: T2-01, T2-02, T2-03, T2-04
**Success Criteria** (what must be TRUE):
  1. User cannot accidentally install mainline linux kernel (pacman refuses upgrade)
  2. User sees warning if kernel package would change during system update
  3. Pacman operation aborts if /boot is not mounted before kernel update
  4. User can verify initramfs was generated successfully after kernel operations
  5. GRUB menu shows previous kernel version as fallback boot option
**Plans**: 3 plans

Plans:
- [x] 01-01-PLAN.md - Protection hooks and scripts
- [x] 01-02-PLAN.md - Verification and fallback
- [x] 01-03-PLAN.md - Archiso sync and testing

### Phase 2: Core Backup Engine
**Goal**: User can create, list, restore, and manage system snapshots manually using proven backup tools
**Depends on**: Phase 1
**Requirements**: SNAP-01, SNAP-02, SNAP-03, SNAP-04, SNAP-05, SYNC-01, SYNC-02, SYNC-03, SYNC-04, DATA-01, DATA-02
**Plans**: Not started

### Phase 3: Desktop Integration
**Goal**: Backup operations are accessible through native VulcanOS desktop interface with status visibility
**Depends on**: Phase 2
**Requirements**: UI-01, UI-02, UI-03, UI-04, ORG-01, ORG-02, ORG-03
**Plans**: Not started

### Phase 4: Automation & Validation
**Goal**: Backup system runs automatically before risky operations with scheduled snapshots and tested restoration
**Depends on**: Phase 3
**Plans**: Not started

### Phase 5: VulcanOS Wallpaper Manager
**Goal**: Native GTK4/Adwaita GUI for multi-monitor wallpaper management with per-monitor assignment, profiles, and adaptive wallpaper generation
**Depends on**: Phase 3 (Desktop Integration patterns)
**Requirements**: WALL-01, WALL-02, WALL-03, WALL-04, WALL-05
**Success Criteria** (what must be TRUE):
  1. User can launch vulcan-wallpaper-manager GUI from menu or command line
  2. GUI displays visual representation of current monitor layout with live wallpaper previews
  3. User can assign different wallpapers to each monitor via drag-and-drop or file picker
  4. User can save/load wallpaper profiles by name (matching hyprmon monitor profiles)
  5. User can generate adaptive/split wallpapers that span multiple monitors seamlessly
  6. Changes apply immediately to hyprpaper without restart
  7. Profiles are synced to archiso skeleton for fresh installs
  8. Wallpaper settings accessible from vulcan-menu submenu
**Plans**: 8 plans

Plans:
- [x] 05-01-PLAN.md - Project scaffold and core infrastructure
- [x] 05-02-PLAN.md - Main app window and monitor layout visualization
- [x] 05-03-PLAN.md - Wallpaper picker with thumbnail grid and caching
- [x] 05-04-PLAN.md - Component integration and swww IPC
- [x] 05-05-PLAN.md - Profile management (save/load/sync)
- [x] 05-06-PLAN.md - Adaptive wallpaper generation (panoramic splitting)
- [x] 05-07-PLAN.md - Desktop integration (menu, .desktop file, archiso sync)
- [x] 05-08-PLAN.md - Human verification checkpoint

</details>

### v2.0 Vulcan Appearance Manager (In Progress)

**Milestone Goal:** Unified theme and wallpaper management with preset theme bundles and consistent theming across all VulcanOS tools.

#### Phase 6: Foundation Architecture
**Goal**: Establish unified codebase with hardened state management, backend abstraction, and shared theming infrastructure
**Depends on**: Phase 5 (existing apps to merge)
**Requirements**: INFRA-08
**Success Criteria** (what must be TRUE):
  1. vulcan-appearance-manager crate exists with merged models from both existing apps
  2. State management follows explicit transitions (Idle/Previewing/Applying) with live system as truth
  3. Wallpaper backend abstraction supports both swww and hyprpaper via trait
  4. Theme parser validates bash scripts before parsing and rejects malformed themes
  5. Shared CSS module provides single source of truth for Vulcan brand colors
**Plans**: 5 plans

Plans:
- [x] 06-01-PLAN.md - Rename crate and merge theme models/services
- [x] 06-02-PLAN.md - Shared brand CSS module
- [x] 06-03-PLAN.md - Explicit state machine (AppState)
- [x] 06-04-PLAN.md - Wallpaper backend trait abstraction
- [x] 06-05-PLAN.md - Theme parser hardening and validation

#### Phase 7: Component Integration
**Goal**: Tab-based UI merges existing theme and wallpaper components into single cohesive application
**Depends on**: Phase 6
**Requirements**: APP-01, APP-02, APP-03, APP-05, APP-06, APP-07, APP-08
**Success Criteria** (what must be TRUE):
  1. User can launch vulcan-appearance-manager from menu or CLI (replaces both old apps)
  2. App displays tabs for Themes and Wallpapers in single window using libadwaita ViewStack
  3. Theme browser shows color preview cards for all available themes
  4. Wallpaper tab shows per-monitor layout with current assignments (migrated from wallpaper-manager)
  5. User can assign different wallpapers to each monitor (existing feature preserved)
  6. User can split panoramic images across multiple monitors (existing feature preserved)
  7. Theme editor allows editing all 50+ color variables in groups (migrated from theme-manager)
  8. Memory usage remains stable during repeated previews (no leaks from GTK4 components)
**Plans**: 5 plans

Plans:
- [x] 07-01-PLAN.md - App shell with ViewStack + ViewSwitcher navigation
- [x] 07-02-PLAN.md - Migrate theme UI components (browser, card, preview, editor)
- [x] 07-03-PLAN.md - Create ThemeView container and integrate into ViewStack
- [x] 07-04-PLAN.md - Create WallpaperView container and integrate into ViewStack
- [x] 07-05-PLAN.md - Final integration, polish, and desktop entry

#### Phase 8: Theme-Wallpaper Binding
**Goal**: Themes suggest wallpapers and unified profiles save coordinated appearance as single unit
**Depends on**: Phase 7
**Requirements**: BIND-01, BIND-02, BIND-03, BIND-04, BIND-05, BIND-06, APP-04
**Success Criteria** (what must be TRUE):
  1. Each theme file includes optional THEME_WALLPAPER field for suggested wallpaper path
  2. Selecting a theme displays its suggested wallpaper thumbnail (if defined)
  3. User can apply theme's suggested wallpaper with one click from theme browser
  4. User can override theme's suggested wallpaper with custom choice (binding mode shown in UI)
  5. Unified profiles save theme ID + wallpaper configuration + binding mode together
  6. User can save/load/delete unified profiles by name from profiles tab
  7. Theme browser shows suggested wallpaper thumbnail for each theme card
**Plans**: 6 plans

Plans:
- [x] 08-01-PLAN.md - BindingMode enum, UnifiedProfile model, theme parser THEME_WALLPAPER extraction
- [x] 08-02-PLAN.md - Unified profile storage with migration from old format
- [x] 08-03-PLAN.md - Theme card wallpaper thumbnail overlay
- [x] 08-04-PLAN.md - Binding dialog for "Apply theme wallpaper?" confirmation
- [x] 08-05-PLAN.md - Profiles tab with profile cards (save/load/delete)
- [x] 08-06-PLAN.md - Integration wiring and human verification

#### Phase 9: Theming Infrastructure
**Goal**: Theme changes propagate automatically to all desktop components with self-theming GUI
**Depends on**: Phase 8
**Requirements**: INFRA-01, INFRA-02, INFRA-03, INFRA-04, INFRA-05, INFRA-06, INFRA-07
**Success Criteria** (what must be TRUE):
  1. Theme changes propagate to waybar status bar colors (reloaded automatically)
  2. Theme changes propagate to wofi launcher colors (CSS updated)
  3. Theme changes propagate to swaync notification center colors (CSS updated)
  4. Theme changes propagate to hyprlock lock screen colors (config regenerated)
  5. Theme changes propagate to kitty terminal colors (config regenerated, new windows themed)
  6. Theme changes propagate to alacritty terminal colors (config regenerated, new windows themed)
  7. Appearance Manager GUI uses current active theme colors (self-theming via shared CSS)
**Plans**: 4 plans

Plans:
- [x] 09-01-PLAN.md - Verify and fix theme propagation chain to all 6 components
- [x] 09-02-PLAN.md - GTK CSS generation for active theme (vulcan-theme + Rust service)
- [x] 09-03-PLAN.md - App self-theming integration (CssProvider runtime loading)
- [x] 09-04-PLAN.md - Human verification of complete theming infrastructure

#### Phase 10: Preset Themes & Desktop Integration
**Goal**: Polished preset themes with matching wallpapers, third-party app discovery, and complete desktop integration
**Depends on**: Phase 9
**Requirements**: PRESET-01, PRESET-02, PRESET-03, PRESET-04, PRESET-05, PRESET-06, THIRD-01, THIRD-02, THIRD-03, THIRD-04, DESK-01, DESK-02, DESK-03
**Success Criteria** (what must be TRUE):
  1. 8-10 polished preset themes exist with distinct visual personalities
  2. Each preset theme includes at least one matching wallpaper in library
  3. Well-known themes (Catppuccin, Dracula, Nord, etc.) use community wallpapers from sources
  4. Custom VulcanOS themes have AI-generated wallpapers pre-bundled in repository
  5. Wallpaper library stored in dotfiles/wallpapers/ directory structure
  6. Wallpaper library files tracked in Git for backup and sync
  7. App scans for installed apps that support theming (VS Code, Neovim, Firefox, Thunderbird)
  8. Discovery tab shows theming status for each app (configured vs not configured)
  9. App provides clickable links to theme marketplaces for each discovered app
  10. Appearance Manager accessible from vulcan-menu "Appearance" submenu
  11. .desktop file created and installed for application launchers
  12. App binary and configs synced to archiso skeleton for fresh installs
**Plans**: 8 plans

Plans:
- [ ] 10-01-PLAN.md - Polish existing 8 preset themes with official palettes
- [ ] 10-02-PLAN.md - Add 2 light theme variants (Catppuccin Latte, Gruvbox Light)
- [ ] 10-03-PLAN.md - Create wallpaper library and curate community wallpapers
- [ ] 10-04-PLAN.md - Implement third-party app discovery service
- [ ] 10-05-PLAN.md - Create discovery section UI component
- [ ] 10-06-PLAN.md - Update vulcan-menu with Appearance submenu
- [ ] 10-07-PLAN.md - Sync themes and configs to archiso skeleton
- [ ] 10-08-PLAN.md - Human verification checkpoint

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9 -> 10

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. T2 Kernel Protection | v1.0 | 3/3 | Complete | 2026-01-23 |
| 2. Core Backup Engine | v1.0 | 0/? | Deferred | - |
| 3. Desktop Integration | v1.0 | 0/? | Deferred | - |
| 4. Automation & Validation | v1.0 | 0/? | Deferred | - |
| 5. VulcanOS Wallpaper Manager | v1.0 | 8/8 | Complete | 2026-01-24 |
| 6. Foundation Architecture | v2.0 | 5/5 | Complete | 2026-01-25 |
| 7. Component Integration | v2.0 | 5/5 | Complete | 2026-01-25 |
| 8. Theme-Wallpaper Binding | v2.0 | 6/6 | Complete | 2026-01-25 |
| 9. Theming Infrastructure | v2.0 | 4/4 | Complete | 2026-01-30 |
| 10. Preset Themes & Desktop Integration | v2.0 | 0/8 | Planned | - |

---
*Roadmap created: 2026-01-23*
*Last updated: 2026-01-30 after Phase 10 planning complete*
