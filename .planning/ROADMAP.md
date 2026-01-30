# Roadmap: VulcanOS

## Milestones

- ✅ **v1.0 Foundation** - Phases 1, 5 (shipped 2026-01-24)
- ✅ **v2.0 Vulcan Appearance Manager** - Phases 6-10 (shipped 2026-01-30)
- 🚧 **v2.1 Maintenance** - Phases 11-13 (in progress)

## Phases

<details>
<summary>✅ v1.0 Foundation (Phases 1, 5) - SHIPPED 2026-01-24</summary>

### Phase 1: T2 Kernel Protection
**Goal**: Protect linux-t2 kernel from accidental removal or corruption
**Plans**: 3 plans

Plans:
- [x] 01-01: Pacman hooks for kernel protection
- [x] 01-02: Boot verification system
- [x] 01-03: Fallback kernel mechanism

### Phase 5: Wallpaper Manager
**Goal**: Per-monitor wallpaper management with profile system
**Plans**: 8 plans

Plans:
- [x] 05-01: GTK4/Relm4 foundation
- [x] 05-02: Monitor detection via hyprctl
- [x] 05-03: Wallpaper preview grid
- [x] 05-04: Per-monitor assignment
- [x] 05-05: Profile save/load system
- [x] 05-06: swww integration
- [x] 05-07: Desktop integration
- [x] 05-08: Archiso skeleton sync

</details>

<details>
<summary>✅ v2.0 Vulcan Appearance Manager (Phases 6-10) - SHIPPED 2026-01-30</summary>

### Phase 6: Foundation Architecture
**Goal**: Core infrastructure for unified theme + wallpaper management
**Plans**: 7 plans

Plans:
- [x] 06-01: App skeleton with ViewStack navigation
- [x] 06-02: Theme data models and storage
- [x] 06-03: State machine (AppState)
- [x] 06-04: Theme parser with security validation
- [x] 06-05: Wallpaper view integration
- [x] 06-06: Unified profile system (theme + wallpaper + binding)
- [x] 06-07: BindingMode architecture

### Phase 7: Theme Browser
**Goal**: User can discover and preview preset themes
**Plans**: 4 plans

Plans:
- [x] 07-01: Theme card component with color preview
- [x] 07-02: Theme grid layout with ScrolledWindow
- [x] 07-03: Theme selection and detail view
- [x] 07-04: Theme application workflow

### Phase 8: Theme-Wallpaper Binding
**Goal**: Themes suggest coordinated wallpapers with user override
**Plans**: 5 plans

Plans:
- [x] 08-01: Theme-wallpaper resolution logic
- [x] 08-02: BindingMode UI controls
- [x] 08-03: Binding state persistence in profiles
- [x] 08-04: Auto-apply wallpaper on theme selection (ThemeBound mode)
- [x] 08-05: Manual override workflow (CustomOverride mode)

### Phase 9: Preset Theme Library
**Goal**: 10 polished preset themes with verified color palettes
**Plans**: 6 plans

Plans:
- [x] 09-01: Theme file structure and metadata
- [x] 09-02: Research official color palettes (8 themes)
- [x] 09-03: Create theme files with verified colors
- [x] 09-04: Download matching wallpapers (3 sources)
- [x] 09-05: Theme-wallpaper bindings configuration
- [x] 09-06: App self-theming implementation

### Phase 10: Preset Themes + Desktop Integration
**Goal**: Complete theme library and desktop menu integration
**Plans**: 6 plans

Plans:
- [x] 10-01: Remaining theme definitions (Gruvbox Light, One Dark)
- [x] 10-02: Third-party app discovery (marketplace links)
- [x] 10-03: Theme propagation to 6 components
- [x] 10-04: UI polish (icons, spacing, labels)
- [x] 10-05: vulcan-menu Appearance submenu
- [x] 10-06: Desktop menu integration (wofi launcher)
- [x] 10-07: Archiso skeleton sync
- [x] 10-08: Human verification and docs

</details>

### 🚧 v2.1 Maintenance (In Progress)

**Milestone Goal:** Clean up technical debt from v2.0 to solidify the Appearance Manager codebase before adding new features.

**Coverage:** 9 v2.1 requirements mapped across 3 phases

#### Phase 11: Security Hardening
**Goal**: Theme import security via parse_and_validate() integration
**Depends on**: Nothing (maintenance work on existing code)
**Requirements**: SEC-01, SEC-02, SEC-03
**Success Criteria** (what must be TRUE):
  1. All theme loading paths (builtin, custom, import) use parse_and_validate() function
  2. Theme import rejects files with dangerous patterns (command injection, eval, pipes)
  3. Theme import rejects files with path traversal attempts (../, absolute paths)
  4. User receives clear error message when importing malformed or malicious theme
**Plans**: 1 plan

Plans:
- [ ] 11-01-PLAN.md — Wire parse_and_validate() into all theme loading paths

#### Phase 12: UX Polish
**Goal**: Complete theme/wallpaper experience with binding detection and wallpaper library
**Depends on**: Nothing (independent UX improvements)
**Requirements**: UX-01, UX-02, UX-03
**Success Criteria** (what must be TRUE):
  1. BindingMode automatically transitions to CustomOverride when user manually changes wallpaper after applying theme
  2. All 10 preset themes have matching wallpapers bundled
  3. Wallpaper LICENSE files contain proper attribution for all downloaded sources
  4. User can apply any preset theme and see coordinated wallpaper immediately
**Plans**: TBD

Plans:
- [ ] 12-01: TBD

#### Phase 13: Architecture Cleanup
**Goal**: AppState integration for proper preview/apply/cancel workflow
**Depends on**: Nothing (internal state management)
**Requirements**: ARCH-01, ARCH-02, ARCH-03
**Success Criteria** (what must be TRUE):
  1. App uses AppState state machine to track preview/apply lifecycle
  2. Cancel Preview button restores previous theme AND wallpapers
  3. Preview/Apply/Cancel buttons are disabled during invalid states (cannot preview while already previewing)
  4. User can preview multiple themes, cancel to restore original state, then apply desired theme
**Plans**: TBD

Plans:
- [ ] 13-01: TBD

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. T2 Kernel Protection | v1.0 | 3/3 | Complete | 2026-01-24 |
| 5. Wallpaper Manager | v1.0 | 8/8 | Complete | 2026-01-24 |
| 6. Foundation Architecture | v2.0 | 7/7 | Complete | 2026-01-26 |
| 7. Theme Browser | v2.0 | 4/4 | Complete | 2026-01-27 |
| 8. Theme-Wallpaper Binding | v2.0 | 5/5 | Complete | 2026-01-28 |
| 9. Preset Theme Library | v2.0 | 6/6 | Complete | 2026-01-29 |
| 10. Preset Themes + Desktop Integration | v2.0 | 6/6 | Complete | 2026-01-30 |
| 11. Security Hardening | v2.1 | 0/1 | Ready | - |
| 12. UX Polish | v2.1 | 0/TBD | Not started | - |
| 13. Architecture Cleanup | v2.1 | 0/TBD | Not started | - |

---
*Last updated: 2026-01-30 (Phase 11 planned)*
