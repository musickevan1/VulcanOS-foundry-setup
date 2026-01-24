---
phase: 05-vulcanos-wallpaper-manager
plan: 01
subsystem: ui
tags: [rust, gtk4, libadwaita, relm4, hyprland, hyprctl, serde, toml]

# Dependency graph
requires:
  - phase: none
    provides: "Initial phase - no dependencies"
provides:
  - "Rust project scaffold with GTK4/Relm4 dependencies"
  - "Monitor data model with hyprctl JSON parsing"
  - "Wallpaper and WallpaperProfile data models"
  - "hyprctl service layer for monitor detection and wallpaper control"
affects: [05-02, 05-03, wallpaper-ui]

# Tech tracking
tech-stack:
  added:
    - gtk4 v0.9 with v4_16 features
    - libadwaita v0.7 with v1_6 features
    - relm4 v0.9 with libadwaita integration
    - image v0.25 for image handling
    - serde/serde_json/toml for serialization
    - anyhow for error handling
    - dirs v5 for directory paths
    - tokio v1 for async runtime
  patterns:
    - "Barrel file pattern: models/mod.rs re-exports types"
    - "Service layer pattern: services/hyprctl.rs wraps external commands"
    - "Result<T> error handling with anyhow::Context"
    - "Serde derives for JSON/TOML serialization"

key-files:
  created:
    - vulcan-wallpaper-manager/Cargo.toml
    - vulcan-wallpaper-manager/src/main.rs
    - vulcan-wallpaper-manager/src/models/monitor.rs
    - vulcan-wallpaper-manager/src/models/wallpaper.rs
    - vulcan-wallpaper-manager/src/models/profile.rs
    - vulcan-wallpaper-manager/src/models/mod.rs
    - vulcan-wallpaper-manager/src/services/hyprctl.rs
    - vulcan-wallpaper-manager/src/services/mod.rs
  modified: []

key-decisions:
  - "GTK4/Libadwaita for native GNOME-style UI (follows VulcanOS design language)"
  - "Relm4 for reactive UI architecture (Elm-inspired, type-safe)"
  - "hyprctl JSON parsing for monitor detection (Hyprland IPC protocol)"
  - "TOML for profile serialization (human-readable, standard for Rust)"
  - "anyhow for flexible error handling (better than std::error for CLI)"

patterns-established:
  - "Monitor struct: Parses hyprctl JSON with logical_size() and is_vertical() helpers"
  - "WallpaperProfile: TOML-based profiles with save/load/list operations"
  - "Service layer: Command wrappers with proper error context"

# Metrics
duration: 6m 51s
completed: 2026-01-24
---

# Phase 05 Plan 01: Foundation Setup Summary

**Rust wallpaper manager with GTK4/Relm4 UI framework, hyprctl monitor detection, and TOML profile system**

## Performance

- **Duration:** 6 min 51 sec
- **Started:** 2026-01-24T05:59:50Z
- **Completed:** 2026-01-24T06:06:41Z
- **Tasks:** 3
- **Files modified:** 8

## Accomplishments

- Created Rust project with GTK4/Libadwaita/Relm4 dependencies
- Implemented Monitor data model that parses hyprctl JSON output
- Built hyprctl service layer for monitor detection and hyprpaper control
- Successfully detected 5 monitors with correct dimensions and scaling factors
- Established TOML-based profile save/load system

## Task Commits

Each task was committed atomically:

1. **Task 1: Create project scaffold with Cargo.toml** - `04d0025` (chore)
2. **Task 2: Create data models (Monitor, Wallpaper, Profile)** - `43f61eb` (feat)
3. **Task 3: Create hyprctl service layer** - `286af4d` (feat)

## Files Created/Modified

- `vulcan-wallpaper-manager/Cargo.toml` - Project manifest with GTK4/Relm4 dependencies
- `vulcan-wallpaper-manager/src/main.rs` - Entry point with monitor detection test
- `vulcan-wallpaper-manager/src/models/monitor.rs` - Monitor struct with serde deserialization
- `vulcan-wallpaper-manager/src/models/wallpaper.rs` - Wallpaper path+name wrapper
- `vulcan-wallpaper-manager/src/models/profile.rs` - WallpaperProfile with TOML serialization
- `vulcan-wallpaper-manager/src/models/mod.rs` - Barrel file re-exporting model types
- `vulcan-wallpaper-manager/src/services/hyprctl.rs` - hyprctl command wrappers
- `vulcan-wallpaper-manager/src/services/mod.rs` - Services barrel file

## Decisions Made

- **GTK4/Libadwaita:** Modern GNOME design language, native Wayland support, matches VulcanOS aesthetics
- **Relm4 framework:** Elm-inspired reactive architecture provides type-safe UI state management
- **hyprctl JSON parsing:** Uses existing Hyprland IPC for monitor detection (no custom implementation needed)
- **TOML profiles:** Human-readable format, standard in Rust ecosystem, easy manual editing
- **anyhow for errors:** More flexible than std::error, better context propagation for CLI tools

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None - hyprctl integration and serde deserialization worked as expected on first try.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

**Ready for UI implementation (Plan 02):**
- GTK4/Relm4 dependencies installed and verified
- Data models established with proper serialization
- hyprctl service layer tested and working
- Monitor detection confirmed on 5-monitor setup

**Verification output:**
```
VulcanOS Wallpaper Manager
Detected 5 monitors:
  eDP-1 @ 3072x1920 (logical: 1920x1200, scale: 1.6)
  DP-4 @ 1920x1080 (logical: 1920x1080, scale: 1)
  DP-9 @ 1920x1080 (logical: 1920x1080, scale: 1)
  DP-10 @ 1920x1080 (logical: 1920x1080, scale: 1)
  DP-12 @ 1920x1080 (logical: 1920x1080, scale: 1)
```

**No blockers or concerns** - foundation is solid for building the GTK4 UI in next phase.

---
*Phase: 05-vulcanos-wallpaper-manager*
*Completed: 2026-01-24*
