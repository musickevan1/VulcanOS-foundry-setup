# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-24)

**Core value:** Cohesive, recoverable, keyboard-driven
**Current focus:** Phase 9 - Theming Infrastructure (v2.0 Vulcan Appearance Manager)

## Current Position

Phase: 10 of 10 (Preset Themes & Desktop Integration)
Plan: 7 of 8 in current phase
Status: Human verification checkpoint (10-08)
Last activity: 2026-01-30 — Plans 10-01 through 10-07 complete, awaiting 10-08 verification

Progress: [██████████████████████████████░] 97% (34/35 total plans complete)

## Performance Metrics

**Velocity:**
- Total plans completed: 28
- Average duration: ~25 min (estimated from Phases 1, 5, 6, 7)
- Total execution time: ~9 hours (estimated)

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. T2 Kernel Protection | 3 | ~2h | ~40 min |
| 5. VulcanOS Wallpaper Manager | 8 | ~6h | ~45 min |
| 6. Foundation Architecture | 5 | 17min | 3.4 min |
| 7. Component Integration | 5 | ~15min | ~3 min |
| 8. Theme-Wallpaper Binding | 6 | ~20min | ~3.3 min |
| 9. Theming Infrastructure | 4 | ~8min | ~2 min |
| 10. Preset Themes & Desktop Integration | 6/8 | ~18min | ~3 min |

**Recent Trend:**
- Last completed: Phase 10 Plan 6 (10-06)
- Trend: Fast execution with minimal deviations

## Accumulated Context

### Decisions

**From v1.0 (still relevant):**
- GTK4/Libadwaita for native GNOME-style UI
- Relm4 for reactive UI architecture
- swww backend for wallpapers (not hyprpaper)
- TOML for profile serialization
- anyhow for error handling in Rust apps
- Clone macro for GTK signal handlers

**New for v2.0 (from PROJECT.md):**
- Merge theme-manager + wallpaper-manager into unified app
- Theme suggests wallpaper (user can override)
- Pre-made wallpapers bundled with themes
- Discovery-only for third-party app theming
- Shared CSS infrastructure for theming propagation
- Delegate theme application to vulcan-theme CLI (not reimplement)

**From Phase 6 Plan 1 (06-01):**
- Renamed vulcan-wallpaper-manager to vulcan-appearance-manager as unified base
- Wallpaper codebase chosen as foundation (8 shipped plans, more mature)
- Theme-manager components NOT moved yet (Phase 7 UI work)
- Old vulcan-theme-manager directory preserved for reference during Phase 7

**From Phase 6 Plan 2 (06-02):**
- GTK4 @define-color syntax for brand colors (not CSS custom properties)
- Dual export: Rust constants for programmatic access + CSS strings for styling
- Merged widget styles from both theme-manager and wallpaper-manager into single module
- const_format for compile-time string concatenation

**From Phase 6 Plan 5 (06-05):**
- Reject dangerous shell patterns in theme files (command substitution, backticks, eval, source, exec, pipes)
- Enforce alphanumeric theme_id starting with letter/number (prevents CLI injection)
- Validate all color fields as strict #RRGGBB hex format
- Require THEME_NAME and THEME_ID presence for theme identification

**From Phase 7 Plan 1 (07-01):**
- Use ViewStack instead of TabView for fixed application views (Themes/Wallpapers)
- ToastOverlay for app-level notifications instead of direct dialogs
- Shell-level AppMsg enum for app-wide concerns only (view-specific state in child components)

**From Phase 7 Plan 2 (07-02):**
- Extract ThemeItem into separate theme_card.rs file (better organization than embedding)
- Continue using gtk::ColorButton despite deprecation (deferred ColorDialogButton migration)
- Message forwarding closure for ThemeCardOutput → ThemeBrowserOutput conversion

**From Phase 7 Plan 3 (07-03):**
- Container component pattern: ThemeView owns all child controllers (browser, preview, editor)
- Modal dialog lifecycle management: Explicit window.close() on save/cancel
- Three-level message forwarding: Card → Browser → ThemeView → App
- FileDialog for theme imports with .sh file filtering

**From Phase 7 Plan 4 (07-04):**
- Vertical paned layout for Wallpapers tab (monitor layout top, wallpaper picker bottom)
- Profile state synchronization via WallpapersChanged output message
- Wallpaper backend runtime detection (swww → hyprpaper → dummy graceful degradation)
- Container component pattern: WallpaperView coordinates monitor_layout, wallpaper_picker, split_dialog

**From Phase 7 Plan 5 (07-05):**
- Human verification confirmed all Phase 7 functionality works
- App self-theming (GUI matching active theme) correctly scoped to Phase 9
- Binary installation required: `cp target/release/vulcan-appearance-manager ~/.local/bin/`

**From Phase 8 Plan 1 (08-01):**
- BindingMode enum with three states: ThemeBound, CustomOverride, Unbound (not boolean)
- THEME_WALLPAPER must be relative path - reject absolute paths and .. traversal (security)
- resolve_theme_wallpaper() returns None for missing files (graceful degradation)
- UnifiedProfile struct extends WallpaperProfile pattern with theme_id and binding_mode

**From Phase 8 Plan 2 (08-02):**
- Profile directory changed from vulcan-wallpaper to vulcan-appearance-manager
- Automatic migration saves to new location when loading from legacy path
- UnifiedProfile format is primary, WallpaperProfile support maintained for migration
- Format detection via try-parse with fallback to old format

**From Phase 8 Plan 3 (08-03):**
- Wallpaper thumbnail is 60x40 pixels in bottom-right corner of theme cards
- Override badge uses emblem-default-symbolic icon in top-right corner
- ThemeItemInput::SetOverride for parent-controlled override state updates
- gtk::Overlay for layering wallpaper thumbnail and badge over color preview
- gtk::Picture with ContentFit::Cover for proper aspect ratio handling

**From Phase 8 Plan 4 (08-04):**
- BindingDialogModel with side-by-side theme colors and wallpaper preview
- show_dialog returns Connector (not Controller) for flexible output forwarding
- Three action buttons: Cancel, Theme Only, Apply Both (suggested-action)
- Simplified color preview (8 swatches in 2 rows of 4) for dialog constraints
- gtk::Picture with ContentFit::Contain for wallpaper preview aspect ratio

**From Phase 8 Plan 5 (08-05):**
- ProfileCardOutput enum for Load/Delete actions from profile cards
- Mini color preview shows 4 colors instead of 8 for compact display
- Load button gets suggested-action class when profile is active
- UpdateCurrentState message for tracking app state to save as profile
- Auto-suggested profile name from theme_id in save dialog
- First monitor's wallpaper used for profile card preview

**From Phase 8 Plan 6 (08-06):**
- Profiles tab third position with user-bookmarks-symbolic icon and Ctrl+3 shortcut
- Binding dialog shown inline during theme application (not separate action)
- Manual wallpaper change switches binding mode from ThemeBound to CustomOverride
- State sync via UpdateCurrentState messages (app pushes to profile view on theme/wallpaper changes)
- Three-level message forwarding: theme_view → app → wallpaper_view for coordinated state

**From Phase 9 Plan 2 (09-02):**
- GTK CSS path: ~/.config/vulcan/current-theme.css
- Include Libadwaita color overrides (window_bg_color, view_bg_color, headerbar_bg_color, etc.)
- heredoc syntax for CSS generation in vulcan-theme

**From Phase 9 Plan 3 (09-03):**
- STYLE_PROVIDER_PRIORITY_USER (600) for runtime CSS to override brand defaults (APPLICATION=400)
- theme_* prefix for overrideable colors, vulcan_* for brand-fixed elements
- CSS reload on ThemeApplied message covers all theme application paths

**From Phase 10 Plan 1 (10-01):**
- Use official palettes without modification for authenticity
- Export 40+ colors per theme including official names (NORD0-15, MAUVE, etc.)
- Preserve theme-specific color naming with prefixes (NORD_, TN_, RP_, OD_, GB_)
- All themes have THEME_WALLPAPER references for future pairing
- Verified colors against official documentation (catppuccin.com, nordtheme.com, etc.)
- 8 polished preset themes with complete official color palettes

**From Phase 10 Plan 3 (10-03):**
- Theme-specific wallpaper directories under dotfiles/wallpapers/
- LICENSE file per theme directory for attribution and compliance
- Official MIT-licensed wallpapers from theme repos (Catppuccin, Dracula)
- Document sources in LICENSE for manual addition when downloads fail
- GPL-compatible licensing required for all wallpapers
- 3 high-resolution wallpapers downloaded (4K and 8K)

**From Phase 10 Plan 4 (10-04):**
- which crate for binary detection via PATH lookup
- open crate for non-blocking URL opening in default browser
- 6 app detectors: Neovim, Kitty, Alacritty, btop, VS Code, Firefox
- Detection checks installation (binary presence) and configuration (theme keywords in config files)
- Discovery-only approach: provide marketplace URLs, don't implement theme installation
- XDG config path resolution with HOME fallback for cross-platform compatibility

**From Phase 10 Plan 5 (10-05):**
- gtk::Expander placement for discovery section (collapsible, non-dominant)
- Badge-based status UI with conditional visibility (Themed badge only when installed)
- Controller list pattern (Vec<Controller<AppRow>>) over Factory for fixed app list
- CSS badge classes: badge-success, badge-muted, badge-accent, badge-warning

**From Phase 10 Plan 6 (10-06):**
- vulcan-menu "Style" renamed to "Appearance" for consistency
- Appearance submenu launches unified vulcan-appearance-manager
- Simplified wallpaper menu: quick actions (Random, Rotate) without full GUI
- CLI backward compatibility: both 'style' and 'appearance' arguments supported
- Removed references to vulcan-theme-manager, vulcan-wallpaper-manager, vulcan-wallpaper-picker

**From Phase 10 Plan 7 (10-07):**
- Archiso skeleton synchronization from dotfiles to /etc/skel structure
- All 10 preset themes synced to archiso for fresh installs
- Wallpaper directory structure created with README (images deferred for ISO size)
- Updated vulcan-menu with Appearance Manager synced to archiso
- Legacy desktop entry cleanup (removed old vulcan-wallpaper-manager.desktop)

### Previous Milestone Summary

**v1.0 VulcanOS Foundation** (Phase 5 shipped 2026-01-24):
- Phase 1: T2 Kernel Protection (3/3 plans complete)
- Phase 5: VulcanOS Wallpaper Manager (8/8 plans complete)
- Established GTK4/Relm4 patterns
- swww integration working
- Profile system proven

**v2.0 Foundation Architecture - Phase 6 COMPLETE:**
- Plan 1: Unified vulcan-appearance-manager crate (wallpaper + theme models/services merged)
- Plan 2: Shared brand CSS module (single source of truth for VulcanOS colors, merged widget styles)
- Plan 3: State management system (application state tracking with save/load)
- Plan 4: Wallpaper backend abstraction (SwwwBackend + HyprpaperBackend with auto-detection)
- Plan 5: Theme parser hardening (security validation, dangerous pattern detection)

**v2.0 Component Integration - Phase 7 COMPLETE:**
- Plan 1: Unified app shell (ViewStack + ViewSwitcher navigation, placeholder views, profile manager in header) ✓
- Plan 2: Theme UI component migration (theme_card, theme_browser, preview_panel, theme_editor) ✓
- Plan 3: ThemeView container integration (horizontal paned layout, message forwarding, modal editor) ✓
- Plan 4: WallpaperView container integration (vertical paned layout, backend abstraction, profile sync) ✓
- Plan 5: Final integration, polish, desktop entry (human verified) ✓

**v2.0 Theme-Wallpaper Binding - Phase 8 COMPLETE:**
- Plan 1: Foundation data models (BindingMode enum, UnifiedProfile struct, THEME_WALLPAPER validation) ✓
- Plan 2: Profile Storage (UnifiedProfile CRUD, migration from WallpaperProfile, legacy directory support) ✓
- Plan 3: Theme Card Wallpaper Overlay (60x40 thumbnail, override badge, gtk::Overlay structure) ✓
- Plan 4: Binding Confirmation Dialog (BindingDialogModel modal with side-by-side preview, user choice output) ✓
- Plan 5: Profiles Tab UI (ProfileItem cards, ProfileView container, save/load/delete operations) ✓
- Plan 6: App Integration (Profiles tab, binding dialog flow, state sync, profile save/load) ✓

**v2.0 Theming Infrastructure - Phase 9 COMPLETE:**
- Plan 1: Research (domain research complete) ✓
- Plan 2: Theme CSS Generation (vulcan-theme generates GTK CSS, Rust service reads it) ✓
- Plan 3: CSS Loading (GTK4 CssProvider integration, brand_css fallbacks) ✓
- Plan 4: Human Verification (all 7 components confirmed working) ✓

**v2.0 Preset Themes & Desktop Integration - Phase 10 IN PROGRESS:**
- Plan 1: Polish Preset Themes (verified official colors, 40+ exports each) ✓
- Plan 2: Light Theme Variants (catppuccin-latte.sh, gruvbox-light.sh created) ✓
- Plan 3: Wallpaper Library Structure (10 theme directories, 3 wallpapers, LICENSE docs) ✓
- Plan 4: Third-Party App Discovery (6 app detectors, marketplace URLs, installation/config status) ✓
- Plan 5: Discovery Section UI (badge-based app status display, expander in Themes tab) ✓
- Plan 6: Desktop Menu Integration (vulcan-menu updated with Appearance submenu) ✓
- Plan 7: Archiso Skeleton Sync (10 themes, vulcan-menu, wallpapers directory) ✓
- Plan 8: Human Verification (checkpoint - awaiting approval)

### Pending Todos

None.

### Blockers/Concerns

**Gap from human verification (10-08):**
- Light themes (Catppuccin Latte, Gruvbox Light) files exist in dotfiles/themes/colors/ but NOT showing in Appearance Manager UI
- Only 8 themes visible in app: Catppuccin Mocha, Dracula, Gruvbox Dark, Nord, One Dark, Rose Pine, Tokyo Night, Vulcan Forge
- Need to investigate why theme_storage.rs or theme discovery isn't picking up the 2 new light themes

**What IS working (confirmed via screenshots):**
- ✓ Third-Party Apps discovery section visible and functional
- ✓ App detection correct: Neovim/Kitty (Installed, Not Themed), Alacritty/btop/VS Code (Installed, Themed), Firefox (Not Installed)
- ✓ Wallpapers tab working with monitor layout and wallpaper picker
- ✓ Profiles tab working (empty state shown correctly)
- ✓ vulcan-menu has Appearance submenu (confirmed working)

## Session Continuity

Last session: 2026-01-30 (Phase 10 human verification)
Stopped at: Plan 10-08 checkpoint - user reviewed screenshots, found light themes missing from UI
Resume file: None

**Next action:**
1. Investigate why catppuccin-latte.sh and gruvbox-light.sh aren't discovered by theme_storage.rs
2. Fix light theme discovery issue
3. Complete 10-08 verification
4. Finish Phase 10 and milestone
