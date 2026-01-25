---
phase: 07-component-integration
plan: 04
subsystem: wallpaper-ui
tags: [gtk4, relm4, wallpapers, ui-integration, vertical-paned]
requires: [07-01, 07-02, 07-03]
provides:
  - Functional Wallpapers tab in ViewStack
  - WallpaperView container component
  - Monitor selection and wallpaper assignment UI
  - Split panoramic dialog integration
  - Profile manager wallpaper sync
affects: [07-05]
tech-stack:
  added: []
  patterns:
    - Vertical paned layout (monitor top, picker bottom)
    - Component message forwarding (child → parent → sibling)
    - Wallpaper backend abstraction (swww/hyprpaper)
    - Profile state synchronization
key-files:
  created:
    - vulcan-appearance-manager/src/components/wallpaper_view.rs
  modified:
    - vulcan-appearance-manager/src/components/mod.rs
    - vulcan-appearance-manager/src/app.rs
decisions:
  - title: "Wallpaper view uses vertical paned layout"
    rationale: "Monitor layout naturally sits at top for spatial reference, picker at bottom for browsing"
    alternatives: ["Horizontal split", "Separate modal windows"]
  - title: "Profile state synchronized via WallpapersChanged output"
    rationale: "Keeps profile manager in sync with wallpaper assignments without tight coupling"
    alternatives: ["Direct profile_manager access", "Shared state model"]
  - title: "Wallpaper backend abstraction with runtime detection"
    rationale: "Supports both swww (preferred) and hyprpaper (fallback) with same interface"
    alternatives: ["Hard-code swww", "User config selection"]
metrics:
  duration: "3 minutes"
  files-changed: 3
  lines-added: 435
  completed: 2026-01-25
---

# Phase 7 Plan 4: Wallpaper View Integration Summary

> Container component for wallpaper management UI with monitor visualization and picker grid

## One-liner

Vertical paned wallpaper view with monitor layout visualization, wallpaper picker grid, and profile integration

## What Was Built

### WallpaperView Container Component (`wallpaper_view.rs`)

**Purpose:** Wraps all wallpaper UI components into a cohesive tab experience with vertical paned layout.

**Key features:**
- **Vertical paned layout:** Monitor layout top (350px), wallpaper picker bottom (expandable)
- **Monitor selection:** Click monitors in layout to select target
- **Wallpaper assignment:** Click wallpaper thumbnail, then Apply button
- **Split panoramic dialog:** Import wide images and split across monitors
- **Profile integration:** Responds to ApplyProfile message to set wallpapers from saved profiles
- **Backend abstraction:** Works with swww (preferred) or hyprpaper (fallback)

**Component composition:**
```rust
WallpaperViewModel
├── MonitorLayoutModel (top pane - Cairo drawing of monitor positions)
├── WallpaperPickerModel (bottom pane - FlowBox grid of thumbnails)
└── SplitDialogModel (modal - panoramic splitter)
```

**Message flow:**
```
MonitorLayoutOutput::Selected(name)
  → WallpaperViewMsg::MonitorSelected(name)
  → Store selected_monitor

WallpaperPickerOutput::Selected(path)
  → WallpaperViewMsg::WallpaperSelected(path)
  → Store selected_wallpaper

Apply button clicked
  → WallpaperViewMsg::ApplyWallpaper
  → backend.apply(monitor, wallpaper)
  → WallpaperViewOutput::WallpapersChanged(...)
  → AppMsg::WallpapersChanged(...)
  → ProfileManagerInput::UpdateWallpapers(...)
```

**Operations supported:**
- `ApplyWallpaper` - Set selected wallpaper on selected monitor
- `RefreshMonitors` - Reload monitor config and current wallpapers
- `OpenDirectory` - Open wallpaper folder in file manager (xdg-open)
- `ShowSplitDialog` - Launch panoramic image splitter
- `SplitGenerated` - Apply all split wallpapers to monitors
- `ApplyProfile` - Load wallpapers from saved profile

### App Integration

**Updated files:**
- `components/mod.rs` - Export wallpaper_view module
- `app.rs` - Replace wallpaper placeholder with WallpaperViewModel

**ViewStack structure:**
```
Themes Tab (ThemeView)
  ├── Theme browser (left pane)
  └── Theme preview (right pane)

Wallpapers Tab (WallpaperView) ← THIS PLAN
  ├── Monitor layout (top pane)
  └── Wallpaper picker (bottom pane)
```

**Message routing:**
- `AppMsg::Refresh` → `WallpaperViewMsg::RefreshMonitors`
- `AppMsg::ProfileApply` → `WallpaperViewMsg::ApplyProfile`
- `WallpaperViewOutput::ShowToast` → `AppMsg::ShowToast` → toast notification
- `WallpaperViewOutput::WallpapersChanged` → `AppMsg::WallpapersChanged` → `ProfileManagerInput::UpdateWallpapers`

**Header bar controls:**
- ViewSwitcher (Themes | Wallpapers tabs)
- Profile manager dropdown (shared, saves wallpaper assignments)
- Refresh button (reloads monitors and wallpapers)

### Wallpaper Backend Abstraction

**Detection order:**
1. Try `swww query` (preferred - smooth transitions)
2. Fall back to `hyprctl hyprpaper listactive`
3. Use DummyBackend if neither available (graceful degradation)

**Backend trait:**
```rust
trait WallpaperBackend {
    fn apply(&self, monitor: &str, path: &Path) -> Result<()>;
    fn query_active(&self) -> Result<HashMap<String, String>>;
    fn name(&self) -> &str;
}
```

**Backends implemented:**
- `SwwwBackend` - Uses `swww img --outputs <monitor> --transition-type fade`
- `HyprpaperBackend` - Uses `hyprctl hyprpaper preload/wallpaper`
- `DummyBackend` - Returns errors gracefully if no backend found

## Decisions Made

### Vertical Paned Layout for Wallpaper Tab

**Decision:** Use vertical split with monitor layout on top, wallpaper picker on bottom.

**Rationale:**
- Monitor layout benefits from horizontal space (multi-monitor setups are wide)
- Wallpaper picker grid flows better vertically (thumbnail browsing)
- Top pane provides spatial context for "where am I applying this?"
- Bottom pane provides browsing context for "what should I apply?"

**Alternatives considered:**
- Horizontal split: Would constrain monitor layout width, make picker too narrow
- Separate windows: More clicks, breaks cohesion
- Single view with tabs: Extra navigation overhead

**Result:** Clean separation of "target selection" (top) and "wallpaper selection" (bottom).

### Profile State Synchronization via Output Messages

**Decision:** WallpaperView emits `WallpapersChanged` output, App forwards to ProfileManager.

**Rationale:**
- Maintains component independence (WallpaperView doesn't know about ProfileManager)
- App acts as message router for cross-component communication
- ProfileManager can react to wallpaper changes to keep "current state" accurate
- Enables future features like "auto-save on change" or "unsaved changes warning"

**Alternatives considered:**
- Direct coupling: WallpaperView holds ProfileManager reference → tight coupling, harder to test
- Shared state model: Global wallpaper state → more complex state management
- No sync: Profile manager unaware of changes → stale state, user confusion

**Result:** Clear message flow, easy to trace, testable in isolation.

### Wallpaper Backend Runtime Detection

**Decision:** Detect backend at startup by trying swww first, falling back to hyprpaper, then dummy.

**Rationale:**
- VulcanOS prefers swww (smoother transitions, better animations)
- Some users may use hyprpaper (built into Hyprland, simpler)
- Runtime detection allows app to work in both environments
- Dummy backend provides graceful degradation (app launches, shows error on apply)

**Alternatives considered:**
- Hard-code swww: Breaks for users with hyprpaper
- User config setting: Extra configuration burden, can auto-detect
- Compile-time feature flags: Harder to distribute, less flexible

**Result:** App works in both swww and hyprpaper environments without user configuration.

## Deviations from Plan

None - plan executed exactly as written.

## What Exists Now

### File Structure

```
vulcan-appearance-manager/
├── src/
│   ├── app.rs                         # App shell with ViewStack
│   ├── components/
│   │   ├── mod.rs                     # Module exports
│   │   ├── wallpaper_view.rs          # THIS PLAN - container view
│   │   ├── monitor_layout.rs          # Monitor visualization (Plan 6)
│   │   ├── wallpaper_picker.rs        # Thumbnail grid (Plan 6)
│   │   ├── split_dialog.rs            # Panoramic splitter (Plan 6)
│   │   ├── profile_manager.rs         # Profile save/load (Plan 6)
│   │   ├── theme_view.rs              # Theme tab container (Plan 3)
│   │   ├── theme_browser.rs           # Theme grid (Plan 2)
│   │   ├── preview_panel.rs           # Theme preview (Plan 2)
│   │   └── theme_editor.rs            # Theme editor dialog (Plan 2)
│   ├── services/
│   │   ├── wallpaper_backend.rs       # Backend abstraction
│   │   ├── hyprctl.rs                 # Monitor queries
│   │   ├── thumbnail.rs               # Wallpaper scanning
│   │   └── image_splitter.rs          # Panoramic splitter logic
│   └── models/
│       └── monitor.rs                 # Monitor data structure
```

### Component Hierarchy

```
App (ViewStack with ToastOverlay)
├── HeaderBar
│   ├── ViewSwitcher (Themes | Wallpapers)
│   ├── ProfileManagerModel (shared dropdown)
│   └── Refresh button
├── ThemeView (Plan 3)
│   ├── ThemeBrowser (left pane)
│   └── PreviewPanel (right pane)
└── WallpaperView (THIS PLAN)
    ├── Monitor Layout (top pane)
    │   └── Cairo DrawingArea with click handler
    ├── Wallpaper Picker (bottom pane)
    │   └── FlowBox grid of thumbnails
    └── Split Dialog (modal)
        └── File chooser + name entry + split button
```

### Message Flow Architecture

```
User clicks monitor in layout
  → MonitorLayoutOutput::Selected(name)
  → WallpaperViewMsg::MonitorSelected(name)
  → model.selected_monitor = Some(name)

User clicks wallpaper thumbnail
  → WallpaperPickerOutput::Selected(path)
  → WallpaperViewMsg::WallpaperSelected(path)
  → model.selected_wallpaper = Some(path)

User clicks Apply button
  → WallpaperViewMsg::ApplyWallpaper
  → backend.apply(monitor, wallpaper)
  → Update monitor_wallpapers HashMap
  → Emit WallpaperViewOutput::WallpapersChanged
  → App receives AppMsg::WallpapersChanged
  → Forward to ProfileManagerInput::UpdateWallpapers
  → Profile manager state synchronized

User clicks profile dropdown → Load
  → ProfileManagerOutput::ApplyProfile(wallpapers)
  → AppMsg::ProfileApply(wallpapers)
  → WallpaperViewMsg::ApplyProfile(wallpapers)
  → backend.apply() for each monitor
  → Update monitor layout visualization
```

### User Workflow

**Assign wallpaper to monitor:**
1. Click monitor in top pane (selects target)
2. Click wallpaper in bottom pane (selects image)
3. Click Apply button (sets wallpaper, updates visualization)
4. Toast notification confirms application

**Split panoramic image:**
1. Click "Import panoramic image" button (📷 icon)
2. Modal dialog opens
3. Click "Browse..." to select wide image
4. Enter name for wallpaper set
5. Click "Split & Apply"
6. Wallpapers generated and applied to all monitors
7. New wallpapers appear in picker grid

**Save/load profile:**
1. Assign wallpapers to monitors
2. Click profile dropdown → Save
3. Enter profile name → Save
4. Later: Select profile from dropdown → Load
5. All wallpapers restored to monitors

## Next Phase Readiness

### Ready

✅ **For Phase 7 Plan 5 (Final Integration Testing):**
- Both tabs (Themes, Wallpapers) fully functional
- Profile manager integrated with both views
- Message routing complete and tested
- All components wired together in ViewStack

### Enables

✅ **Phase 8 (Theme-Wallpaper Binding):**
- WallpaperView can receive external wallpaper suggestions from ThemeView
- Profile storage can be extended to include theme_id + wallpaper mappings
- Apply workflow exists: `ApplyProfile` message already implemented

✅ **Phase 9 (Theme Propagation):**
- Theme application works independently in ThemeView
- Wallpaper application works independently in WallpaperView
- Unified "Apply All" can orchestrate both via message passing

✅ **Phase 10 (Preset Themes):**
- Theme browser can display preset vs. custom badges
- Wallpaper picker can filter by theme association
- Profile manager can save theme+wallpaper bundles

### Concerns

None - plan completed successfully, all integration points working.

## Technical Notes

### Wallpaper Backend Detection Flow

```rust
// At WallpaperViewModel::init()
let backend = detect_backend()
    .unwrap_or_else(|_| {
        eprintln!("Warning: No wallpaper backend detected.");
        Box::new(DummyBackend)
    });

// detect_backend() tries in order:
// 1. swww query (preferred)
// 2. hyprctl hyprpaper listactive (fallback)
// 3. Returns error if neither works

// DummyBackend provides graceful degradation:
// - App launches successfully
// - Operations return errors
// - User sees toast: "No wallpaper backend available"
```

**Why this matters:** Allows app to work in development environments (no swww) and production (with swww).

### Profile Manager Integration

```rust
// When wallpapers change in WallpaperView:
WallpaperViewMsg::ApplyWallpaper => {
    backend.apply(monitor, wallpaper)?;
    self.monitor_wallpapers.insert(monitor, wallpaper);

    // Notify parent
    sender.output(WallpaperViewOutput::WallpapersChanged(
        self.monitor_wallpapers.clone()
    ));
}

// App forwards to ProfileManager:
AppMsg::WallpapersChanged(wallpapers) => {
    self.profile_manager.emit(
        ProfileManagerInput::UpdateWallpapers(wallpapers)
    );
}

// ProfileManager now knows current state for saving
```

**Why this matters:** Profile manager always has accurate current state, "Save" captures what's actually applied.

### Split Dialog Modal Window

```rust
// SplitDialogModel is SimpleComponent (just content)
// WallpaperView creates gtk::Window wrapper:
let window = gtk::Window::builder()
    .title("Import Panoramic Wallpaper")
    .modal(true)
    .resizable(false)
    .build();

window.set_child(Some(split_dialog.widget()));
window.present();

// Dialog emits SplitDialogOutput → WallpaperViewMsg
// WallpaperView handles Generated/Cancelled/Error
// Window is not stored (auto-closes on dialog output)
```

**Why this matters:** Modal window prevents interaction with main window, clear UX for blocking operation.

### Vertical Paned Position

```rust
gtk::Paned {
    set_orientation: gtk::Orientation::Vertical,
    set_position: 350,  // Monitor layout gets 350px
    // ... panes ...
}
```

**Default split:** 350px for monitor layout, remainder for wallpaper picker.

**User can adjust:** Dragging pane separator persists across sessions (GTK default behavior).

**Why 350px:** Typical monitor layout fits comfortably, allows 2-3 rows of wallpaper thumbnails below.

## Lessons Learned

### Component Message Forwarding Pattern

**Pattern used:**
```rust
// In App::init()
let wallpaper_view = WallpaperViewModel::builder()
    .launch(())
    .forward(sender.input_sender(), |msg| {
        match msg {
            WallpaperViewOutput::ShowToast(text) => AppMsg::ShowToast(text),
            WallpaperViewOutput::WallpapersChanged(wps) => AppMsg::WallpapersChanged(wps),
        }
    });
```

**Why effective:**
- Child component doesn't know about parent message types
- Parent defines mapping (OutputMsg → InputMsg)
- Easy to add new message routes without modifying child
- Clear ownership: parent controls how child messages are handled

**When to use:** Any parent-child component relationship where child needs to notify parent.

### Dummy Backend for Graceful Degradation

**Pattern used:**
```rust
struct DummyBackend;

impl WallpaperBackend for DummyBackend {
    fn apply(&self, _: &str, _: &Path) -> Result<()> {
        bail!("No wallpaper backend available")
    }
    // ... other methods return empty/error ...
}
```

**Why effective:**
- App compiles and runs even without swww/hyprpaper
- Testing in development environments easier
- User gets clear error message instead of crash
- Can be extended to log backend issues for debugging

**When to use:** Any external dependency that may not be available at runtime.

### Vertical Paned for Spatial + Grid Layouts

**Pattern used:**
- Top pane: Cairo DrawingArea (spatial - monitor positions)
- Bottom pane: FlowBox (grid - wallpaper thumbnails)

**Why effective:**
- Top pane benefits from width (multi-monitor layouts are wide)
- Bottom pane benefits from height (vertical scrolling for many wallpapers)
- Natural visual hierarchy: "target" above "source"

**When to use:** Combining spatial visualization with item selection grid.

## Commits

- `f8b2d99` - feat(07-04): create WallpaperView container component
- `7e489db` - feat(07-04): integrate WallpaperView into app

---

**Phase:** 07-component-integration | **Plan:** 04 | **Status:** ✅ Complete
