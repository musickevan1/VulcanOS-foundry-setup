---
phase: 05-vulcanos-wallpaper-manager
plan: 02
subsystem: ui
tags: [rust, gtk4, libadwaita, relm4, cairo, drawing-area, monitor-layout]

# Dependency graph
requires:
  - phase: 05-01
    provides: "Rust project scaffold with GTK4/Relm4 dependencies and monitor models"
provides:
  - "Main Relm4 App component with AdwApplicationWindow"
  - "Monitor layout visualization using Cairo DrawingArea"
  - "Click detection for monitor selection"
  - "Visual feedback for selected monitors"
affects: [05-03, wallpaper-ui]

# Tech tracking
tech-stack:
  added:
    - "Relm4 SimpleComponent pattern for reactive UI"
    - "Cairo rendering for custom monitor visualization"
    - "GTK4 DrawingArea widget for graphics"
    - "GTK4 GestureClick for mouse input handling"
  patterns:
    - "Relm4 component hierarchy: App -> MonitorLayoutModel child component"
    - "Message-based UI updates: AppMsg and MonitorLayoutInput enums"
    - "Forward pattern: Child component outputs forwarded to parent via sender"
    - "Cairo coordinate transformation: scale and offset calculations for layout"
    - "Rc<RefCell<T>> pattern for shared mutable state in GTK callbacks"

key-files:
  created:
    - vulcan-wallpaper-manager/src/app.rs
    - vulcan-wallpaper-manager/src/components/mod.rs
    - vulcan-wallpaper-manager/src/components/monitor_layout.rs
    - vulcan-wallpaper-manager/Cargo.lock
  modified:
    - vulcan-wallpaper-manager/src/main.rs

key-decisions:
  - "AdwApplicationWindow with ToolbarView layout (modern GNOME design)"
  - "Cairo DrawingArea for monitor visualization (custom graphics, no SVG overhead)"
  - "Calculate scale factor to fit all monitors in viewport (handles multi-monitor setups)"
  - "Blue highlight for selected monitor (clear visual feedback)"
  - "GestureClick for mouse input (modern GTK4 event handling)"

patterns-established:
  - "MonitorLayoutModel: Relm4 component with DrawingArea and custom Cairo rendering"
  - "calculate_scale(): Finds bounding box and scales monitors to fit viewport with padding"
  - "draw_monitors(): Cairo rendering function with monitor rectangles, labels, and selection state"
  - "find_monitor_at(): Hit detection for click-to-select functionality"

# Metrics
duration: 3m 14s
completed: 2026-01-24
---

# Phase 05 Plan 02: Main Application Window & Monitor Layout Summary

**GTK4/Adwaita window with Cairo-based monitor layout visualization and click-to-select interaction**

## Performance

- **Duration:** 3 min 14 sec
- **Started:** 2026-01-24T06:11:53Z
- **Completed:** 2026-01-24T06:15:06Z
- **Tasks:** 3
- **Files modified:** 5

## Accomplishments

- Created main Relm4 App component with AdwApplicationWindow
- Built monitor layout visualization using GTK4 DrawingArea and Cairo
- Implemented intelligent scaling to fit all monitors in viewport
- Added click detection for monitor selection with visual feedback
- Successfully renders 5-monitor layout with correct positioning and orientation
- Application window launches cleanly with Adwaita styling

## Task Commits

Each task was committed atomically:

1. **Task 1: Create main Relm4 application component** - `7202e69` (feat)
2. **Task 2: Create monitor layout visualization component** - `4ab0dcc` (feat)
3. **Task 3: Test window launch and monitor visualization** - `3483352` (chore)

## Files Created/Modified

- `vulcan-wallpaper-manager/src/app.rs` - Main App component with AdwApplicationWindow, ToolbarView, and MonitorLayoutModel child
- `vulcan-wallpaper-manager/src/components/mod.rs` - Components barrel file
- `vulcan-wallpaper-manager/src/components/monitor_layout.rs` - Monitor layout visualization with DrawingArea, Cairo rendering, and click detection
- `vulcan-wallpaper-manager/src/main.rs` - Updated to launch Relm4 app instead of CLI test
- `vulcan-wallpaper-manager/Cargo.lock` - Dependency lock file for reproducible builds

## Decisions Made

- **AdwApplicationWindow with ToolbarView:** Modern GNOME design pattern with header bar and content area
- **Cairo DrawingArea:** Custom rendering gives full control over monitor visualization (vs using SVG or images)
- **Intelligent scaling:** Calculate bounding box of all monitors and scale to fit viewport with 40px padding
- **Blue highlight on selection:** Clear visual feedback when user clicks a monitor
- **Rc<RefCell<T>> pattern:** Shared mutable state needed for DrawingArea draw_func closure

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed clone! macro import error**
- **Found during:** Task 2 - monitor_layout.rs compilation
- **Issue:** `error: cannot find macro 'clone' in this scope` when using glib clone macro for sender capture
- **Fix:** Changed import from `use gtk::glib;` to `use gtk::glib::clone;`
- **Files modified:** `src/components/monitor_layout.rs`
- **Commit:** Part of `4ab0dcc`

**2. [Rule 3 - Blocking] Fixed GestureClick sender capture in Relm4 view! macro**
- **Found during:** Task 2 - initial compilation attempt
- **Issue:** Cannot use `sender` variable directly in Relm4 `view!` macro add_controller block
- **Fix:** Moved GestureClick setup from view! macro to init() function, using glib clone! macro for proper sender capture
- **Files modified:** `src/components/monitor_layout.rs`
- **Commit:** Part of `4ab0dcc`

## Issues Encountered

None - Cairo rendering and click detection worked as expected.

## User Setup Required

None - application ready to run with `cargo run` from vulcan-wallpaper-manager directory.

## Next Phase Readiness

**Ready for wallpaper selection UI (Plan 03):**
- Main window structure established with Adwaita styling
- Monitor layout visualization working correctly
- Click-to-select interaction implemented
- Monitor data flows properly through Relm4 components
- Foundation ready for adding wallpaper picker sidebar

**Verification output:**
```
Application window opens with:
- AdwApplicationWindow with ToolbarView layout
- Header bar with "Wallpaper Manager" title
- Monitor layout visualization showing all 5 monitors
- Monitors positioned correctly with eDP-1 centered
- Click detection working (prints "Selected monitor: {name}")
- Refresh button functional (reloads monitor list)
```

**No blockers or concerns** - UI foundation is solid for adding wallpaper selection in next phase.

---
*Phase: 05-vulcanos-wallpaper-manager*
*Completed: 2026-01-24*
