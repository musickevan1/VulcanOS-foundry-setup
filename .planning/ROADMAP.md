# Roadmap: VulcanOS

## Milestones

- ✅ **v1.0 Foundation** - Phases 1, 5 (shipped 2026-01-24)
- ✅ **v2.0 Vulcan Appearance Manager** - Phases 6-10 (shipped 2026-01-30)
- ✅ **v2.1 Maintenance** - Phases 11-13 (shipped 2026-02-01)
- 🚧 **v3.0 Multi-Profile + AI Workstation** - Phases 14-22 (in progress)

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

<details>
<summary>✅ v2.1 Maintenance (Phases 11-13) - SHIPPED 2026-02-01</summary>

**Milestone Goal:** Clean up technical debt from v2.0 to solidify the Appearance Manager codebase before adding new features.

**Coverage:** 9 v2.1 requirements mapped across 3 phases

### Phase 11: Security Hardening
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
- [x] 11-01: Wire parse_and_validate() into all theme loading paths

### Phase 12: UX Polish
**Goal**: Complete theme/wallpaper experience with binding detection and wallpaper library
**Depends on**: Nothing (independent UX improvements)
**Requirements**: UX-01, UX-02, UX-03
**Success Criteria** (what must be TRUE):
  1. BindingMode automatically transitions to CustomOverride when user manually changes wallpaper after applying theme
  2. All 10 preset themes have matching wallpapers bundled
  3. Wallpaper LICENSE files contain proper attribution for all downloaded sources
  4. User can apply any preset theme and see coordinated wallpaper immediately
**Plans**: 3 plans

Plans:
- [x] 12-01: BindingMode auto-transition (ThemeBound -> CustomOverride)
- [x] 12-02: Fix wallpaper path resolution
- [x] 12-03: Download missing wallpapers and update LICENSE attribution

### Phase 13: Architecture Cleanup
**Goal**: AppState integration for proper preview/apply/cancel workflow
**Depends on**: Nothing (internal state management)
**Requirements**: ARCH-01, ARCH-02, ARCH-03
**Success Criteria** (what must be TRUE):
  1. App uses AppState state machine to track preview/apply lifecycle
  2. Cancel Preview button restores previous theme AND wallpapers
  3. Preview/Apply/Cancel buttons are disabled during invalid states (cannot preview while already previewing)
  4. User can preview multiple themes, cancel to restore original state, then apply desired theme
**Plans**: 5 plans

Plans:
- [x] 13-01: AppState integration into ThemeViewModel
- [x] 13-02: Action bar UI with Revealer animation
- [x] 13-03: Cancel restore logic with RestoreWallpapers message
- [x] 13-04: Implicit apply on close and human verification
- [x] 13-05: Apply state transitions with rollback on failure

</details>

### 🚧 v3.0 Multi-Profile + AI Workstation (In Progress)

**Milestone Goal:** Restructure VulcanOS into a multi-profile architecture supporting Vulcan Foundry (AMD AI workstation with NVIDIA RTX 5070 Ti) and Vulcan T2 (T2 MacBook Pro), with full NVIDIA/CUDA AI stack and gaming support.

**Coverage:** 69 v3.0 requirements mapped across 9 phases

#### Phase 14: Multi-Profile Build Infrastructure
**Goal**: Build system supports multiple archiso profiles (foundry, t2) with shared base
**Depends on**: Nothing (foundation for all v3.0 work)
**Requirements**: PROF-01, PROF-02, PROF-03, PROF-04, PROF-05, PROF-06, PROF-07
**Success Criteria** (what must be TRUE):
  1. User can run `./scripts/build-t2.sh` and produce `vulcanos-t2-YYYY.MM.DD-x86_64.iso`
  2. User can run `./scripts/build-foundry.sh` and produce `vulcanos-foundry-YYYY.MM.DD-x86_64.iso`
  3. Shared packages in `archiso/base/packages.base` appear in both ISOs
  4. Profile-specific packages in `archiso/profiles/{profile}/packages.profile` only appear in their respective ISOs
  5. T2-specific repos (arch-mact2) only in T2 ISO, not Foundry
**Plans**: 8 plans

Plans:
- [x] 14-01-PLAN.md — Directory structure + shared build library
- [x] 14-02-PLAN.md — Package split (base + T2 + Foundry)
- [x] 14-03-PLAN.md — T2 profile configs (pacman.conf, profiledef.sh, boot menus)
- [x] 14-04-PLAN.md — Foundry profile configs (pacman.conf, profiledef.sh, boot menus)
- [x] 14-05-PLAN.md — Migrate airootfs to base + T2 overlay
- [x] 14-06-PLAN.md — Create build-t2.sh entry point
- [x] 14-07-PLAN.md — Create build-foundry.sh entry point
- [x] 14-08-PLAN.md — Deprecate build.sh + verification checkpoint

#### Phase 15: NVIDIA Driver Foundation
**Goal**: Foundry profile boots with working NVIDIA RTX 5070 Ti drivers and proper suspend/resume
**Depends on**: Phase 14 (needs foundry profile to exist)
**Requirements**: DRV-01, DRV-02, DRV-03, DRV-04, DRV-05, DRV-06, SUSP-01, SUSP-02, SUSP-03, SUSP-04
**Success Criteria** (what must be TRUE):
  1. `nvidia-smi` shows RTX 5070 Ti detected with driver version 590.48.01+
  2. `nvidia-smi -q | grep "Link Gen"` shows Gen4 (not Gen1 fallback)
  3. System suspends and resumes without Hyprland crash or black screen
  4. Kernel update triggers automatic initramfs rebuild via pacman hook
  5. 32-bit NVIDIA libs present (`ls /usr/lib32/libnvidia*` succeeds)
**Plans**: TBD

Plans:
- [ ] 15-01: TBD
- [ ] 15-02: TBD
- [ ] 15-03: TBD

#### Phase 16: CUDA/AI Stack
**Goal**: Full CUDA toolkit and AI/ML frameworks functional with RTX 5070 Ti (sm_120)
**Depends on**: Phase 15 (drivers must work first)
**Requirements**: CUDA-01, CUDA-02, CUDA-03, CUDA-04, CUDA-05, CUDA-06, LLM-01, LLM-02, LLM-03, LLM-04, IMG-01, IMG-02, IMG-03
**Success Criteria** (what must be TRUE):
  1. `nvcc --version` shows CUDA 13.x installed
  2. `ollama run llama3.2` executes on GPU (not CPU fallback)
  3. PyTorch nightly detects CUDA and runs tensor operations on GPU
  4. `docker run --gpus all nvidia/cuda:12.8.0-base-ubuntu22.04 nvidia-smi` shows GPU inside container
  5. ComfyUI installation path documented and tested with sm_120
**Plans**: TBD

Plans:
- [ ] 16-01: TBD
- [ ] 16-02: TBD
- [ ] 16-03: TBD

#### Phase 17: Gaming Stack
**Goal**: Steam, Proton, and gaming utilities ready for Foundry profile
**Depends on**: Phase 15 (needs 32-bit NVIDIA libs from driver phase)
**Requirements**: GAME-01, GAME-02, GAME-03, GAME-04, GAME-05, GAME-06, GAME-07
**Success Criteria** (what must be TRUE):
  1. Steam launches and can install games
  2. `gamemoderun %command%` launch option works in Steam
  3. MangoHud overlay appears when running games with `mangohud %command%`
  4. Gamescope wraps games without crashing (`gamescope -- %command%`)
  5. Proton-GE appears as a Proton version option in Steam
**Plans**: TBD

Plans:
- [ ] 17-01: TBD
- [ ] 17-02: TBD

#### Phase 18: Desktop Improvements
**Goal**: Kitty as default terminal, yazi as file manager, Alacritty removed
**Depends on**: Nothing (independent desktop polish)
**Requirements**: DESK-01, DESK-02, DESK-03, DESK-04, DESK-05, DESK-06
**Success Criteria** (what must be TRUE):
  1. `Super+Return` opens Kitty (not Alacritty)
  2. `yazi` launches and navigates filesystem
  3. `y` shell function (yazi with cd-on-exit) works in bash/zsh
  4. Alacritty is not installed (`which alacritty` fails)
  5. Thunar available as GUI file manager fallback
**Plans**: TBD

Plans:
- [ ] 18-01: TBD
- [ ] 18-02: TBD

#### Phase 19: Hyprland Plugins + Waybar Enhancements
**Goal**: Hyprland plugins installed and GPU monitoring in Waybar
**Depends on**: Phase 15 (GPU monitoring needs drivers)
**Requirements**: PLUG-01, PLUG-02, PLUG-03, PLUG-04, PLUG-05, BAR-01, BAR-02, BAR-03
**Success Criteria** (what must be TRUE):
  1. `hyprpm list` shows hyprexpo, hyprspace, hyprtrails installed
  2. `Super+Tab` triggers workspace overview (hyprexpo)
  3. Waybar shows GPU VRAM usage (e.g., "8.2/16GB")
  4. Waybar shows GPU temperature (e.g., "45C")
  5. Plugins auto-load on Hyprland start
**Plans**: TBD

Plans:
- [ ] 19-01: TBD
- [ ] 19-02: TBD

#### Phase 20: T2 Profile Maintenance
**Goal**: T2 profile continues working with linux-t2 kernel and T2-specific configs
**Depends on**: Phase 14 (needs profile structure)
**Requirements**: T2-01, T2-02, T2-03, T2-04, T2-05
**Success Criteria** (what must be TRUE):
  1. T2 ISO boots on MacBook Pro with working keyboard/trackpad
  2. WiFi connects via `iwctl` (brcmfmac driver functional)
  3. T2 kernel params (`intel_iommu=on iommu=pt pcie_ports=compat`) in grub.cfg
  4. `uname -r` shows linux-t2 kernel, not mainline
  5. Touch Bar functional via tiny-dfr
**Plans**: TBD

Plans:
- [ ] 20-01: TBD
- [ ] 20-02: TBD

#### Phase 21: Foundry-T2 Sync & Remote Access
**Goal**: T2 can wake Foundry, use its GPU remotely, and sync projects via Syncthing
**Depends on**: Phase 16 (Ollama must work for remote API)
**Requirements**: SYNC-01, SYNC-02, SYNC-03, SYNC-04, REM-01, REM-02, REM-03, REM-04, REM-05, REM-06, REM-07, REM-08
**Success Criteria** (what must be TRUE):
  1. Syncthing running on both profiles, syncing `~/Projects`
  2. T2 can SSH to Foundry when awake (`ssh foundry`)
  3. T2 can wake Foundry via `vulcan-foundry-wake` script (WoL)
  4. T2 can use Foundry's Ollama (`OLLAMA_HOST=foundry:11434 ollama run llama3.2`)
  5. Foundry auto-suspends after configurable idle timeout
**Plans**: TBD

Plans:
- [ ] 21-01: TBD
- [ ] 21-02: TBD
- [ ] 21-03: TBD

#### Phase 22: Profile-Aware Dotfiles
**Goal**: Dotfiles adapt to profile (hostname detection, conditional configs)
**Depends on**: Phase 14 (profiles must exist), Phase 15 (NVIDIA env vars for Foundry)
**Requirements**: DOT-01, DOT-02, DOT-03, DOT-04, DOT-05
**Success Criteria** (what must be TRUE):
  1. Hyprland config sources profile-specific monitor layout based on `$HOSTNAME`
  2. Foundry has triple+ monitor config, T2 has laptop display config
  3. NVIDIA environment variables only set on Foundry (not T2)
  4. Keybindings, theming, and app configs identical across profiles
  5. Dotfiles managed via git in VulcanOS repo (not Syncthing)
**Plans**: TBD

Plans:
- [ ] 22-01: TBD
- [ ] 22-02: TBD

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
| 11. Security Hardening | v2.1 | 1/1 | Complete | 2026-01-30 |
| 12. UX Polish | v2.1 | 3/3 | Complete | 2026-02-01 |
| 13. Architecture Cleanup | v2.1 | 5/5 | Complete | 2026-02-01 |
| 14. Multi-Profile Build Infrastructure | v3.0 | 8/8 | Complete | 2026-02-03 |
| 15. NVIDIA Driver Foundation | v3.0 | 0/TBD | Not started | - |
| 16. CUDA/AI Stack | v3.0 | 0/TBD | Not started | - |
| 17. Gaming Stack | v3.0 | 0/TBD | Not started | - |
| 18. Desktop Improvements | v3.0 | 0/TBD | Not started | - |
| 19. Hyprland Plugins + Waybar Enhancements | v3.0 | 0/TBD | Not started | - |
| 20. T2 Profile Maintenance | v3.0 | 0/TBD | Not started | - |
| 21. Foundry-T2 Sync & Remote Access | v3.0 | 0/TBD | Not started | - |
| 22. Profile-Aware Dotfiles | v3.0 | 0/TBD | Not started | - |

---
*Last updated: 2026-02-03 (Phase 14 complete - 8 plans executed)*
