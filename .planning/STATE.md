# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-24)

**Core value:** Cohesive, recoverable, keyboard-driven
**Current focus:** Phase 7 Complete - Component Integration (v2.0 Vulcan Appearance Manager)

## Current Position

Phase: 8 of 10 (Theme-Wallpaper Binding) — IN PROGRESS
Plan: 5 of 6 in current phase
Status: Plans 08-01, 08-02, 08-03, 08-05 complete
Last activity: 2026-01-26 — Completed 08-05-PLAN.md (Profiles Tab UI)

Progress: [███████████░░░░░░░░░░░░░░░] 29% (25/86 total plans from v1.0 complete)

## Performance Metrics

**Velocity:**
- Total plans completed: 21
- Average duration: ~25 min (estimated from Phases 1, 5, 6, 7)
- Total execution time: ~9 hours (estimated)

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. T2 Kernel Protection | 3 | ~2h | ~40 min |
| 5. VulcanOS Wallpaper Manager | 8 | ~6h | ~45 min |
| 6. Foundation Architecture | 5 | 17min | 3.4 min |
| 7. Component Integration | 5 | ~15min | ~3 min |

**Recent Trend:**
- Last completed: Phase 7 Plan 5 (07-05)
- Trend: Phase 7 COMPLETE - unified app verified by human testing

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

**From Phase 8 Plan 5 (08-05):**
- ProfileCardOutput enum for Load/Delete actions from profile cards
- Mini color preview shows 4 colors instead of 8 for compact display
- Load button gets suggested-action class when profile is active
- UpdateCurrentState message for tracking app state to save as profile
- Auto-suggested profile name from theme_id in save dialog
- First monitor's wallpaper used for profile card preview

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

**v2.0 Theme-Wallpaper Binding - Phase 8 IN PROGRESS:**
- Plan 1: Foundation data models (BindingMode enum, UnifiedProfile struct, THEME_WALLPAPER validation) ✓
- Plan 2: Profile Storage (UnifiedProfile CRUD, migration from WallpaperProfile, legacy directory support) ✓
- Plan 3: Theme Card Wallpaper Overlay (60x40 thumbnail, override badge, gtk::Overlay structure) ✓
- Plan 5: Profiles Tab UI (ProfileItem cards, ProfileView container, save/load/delete operations) ✓

### Pending Todos

None yet (v2.0 milestone just initialized).

### Blockers/Concerns

**Phase 7 Complete - All verified:**
- ViewSwitcher tabs work ✓
- Theme browser with color preview cards ✓
- Theme application changes system colors ✓
- Wallpaper tab with monitor layout ✓
- Keyboard shortcuts Ctrl+1/Ctrl+2 ✓
- Toast notifications ✓

**Phase 8 Plans 1-3, 5 Complete:**
- BindingMode enum and UnifiedProfile struct created ✓
- THEME_WALLPAPER security validation (relative paths only) ✓
- resolve_theme_wallpaper() helper for path resolution ✓
- UnifiedProfile CRUD operations with TOML persistence ✓
- Automatic migration from WallpaperProfile format ✓
- Legacy directory support (vulcan-wallpaper → vulcan-appearance-manager) ✓
- Theme cards show wallpaper thumbnails (60x40, bottom-right) ✓
- Override badge prepared (16px icon, top-right) ✓
- gtk::Overlay structure for layered UI elements ✓
- ProfileItem cards display theme colors and wallpaper thumbnails ✓
- ProfileView container with save/load/delete operations ✓
- Save Current dialog with auto-suggested profile names ✓

**Ready for Plan 08-04 or 08-06:** Theme Application Wallpaper Sync or App Integration

## Session Continuity

Last session: 2026-01-26 (Phase 8 Plan 5 execution complete)
Stopped at: Completed 08-05-PLAN.md (Profiles Tab UI)
Resume file: None

**Next action:** Execute Phase 8 Plan 4 or Plan 6 - either sync wallpaper on theme apply, or integrate ProfileView into app shell.
