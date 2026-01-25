---
phase: 07-component-integration
plan: 03
subsystem: ui
tags: [relm4, gtk4, component-integration, theme-view]
requires: ["07-01", "07-02"]
provides: ["Functional Themes tab in ViewStack"]
affects: ["07-04"]
tech-stack:
  added: []
  patterns: ["Container component composition", "Message forwarding chains"]
key-files:
  created:
    - "vulcan-appearance-manager/src/components/theme_view.rs"
  modified:
    - "vulcan-appearance-manager/src/components/mod.rs"
    - "vulcan-appearance-manager/src/app.rs"
decisions:
  - id: theme-view-container
    what: "Create ThemeView as a standalone container component"
    why: "Encapsulates all theme functionality, maintains separation of concerns from app shell"
    alternatives: "Could have embedded paned layout directly in app.rs"
    impact: "Cleaner architecture, easier to test and maintain theme-specific logic"

  - id: output-forwarding-pattern
    what: "Use ThemeViewOutput to communicate with app shell via ShowToast and ThemeApplied events"
    why: "Enables app-level toast notifications and profile manager updates"
    alternatives: "Could use direct references to ToastOverlay"
    impact: "Better decoupling, components don't need direct access to app widgets"

  - id: refresh-forwarding
    what: "Forward Refresh from app to active view based on view_stack.visible_child_name()"
    why: "Allows header refresh button to trigger view-specific refresh logic"
    alternatives: "Could have each view register its own refresh button"
    impact: "Centralized refresh UX, consistent behavior across tabs"
metrics:
  duration: "3 minutes"
  completed: "2026-01-25"
---

# Phase 07 Plan 03: ThemeView Container Integration Summary

**One-liner:** Horizontal paned theme view with browser, preview, and editor integrated into ViewStack

## What Was Built

### ThemeView Container Component

Created `theme_view.rs` as the complete container for the Themes tab:

**Component structure:**
- **Left pane:** Theme browser with action buttons (New/Import)
- **Right pane:** Preview panel with theme operations (Edit/Preview/Cancel/Apply)
- **Modal dialog:** Theme editor for create/edit operations
- **File import:** GTK FileDialog for importing .sh theme files

**Ownership model:**
- Owns `Controller<ThemeBrowserModel>` for theme grid
- Owns `Controller<PreviewPanelModel>` for visual preview
- Owns `Option<Controller<ThemeEditorModel>>` for modal editor
- Owns `Option<gtk::Window>` for editor window lifecycle

**Message handling:**
- `ThemeSelected`: Updates preview panel, stores selected theme
- `PreviewTheme`: Calls theme_applier::preview_theme()
- `ApplyTheme`: Calls theme_applier::apply_theme(), updates browser current indicator
- `CancelPreview`: Calls theme_applier::revert_theme()
- `NewTheme`/`EditTheme`: Opens modal editor dialog
- `ThemeSaved`: Saves via storage service, refreshes browser, closes editor
- `EditorCancelled`: Closes editor window
- `Refresh`: Triggers browser refresh
- `Import`: Opens file picker dialog

### App Shell Integration

**Updated `app.rs`:**
1. Added `theme_view: Controller<ThemeViewModel>` field
2. Created ThemeViewModel with output forwarding:
   - `ShowToast` → `AppMsg::ShowToast`
   - `ThemeApplied` → `AppMsg::ThemeApplied`
3. Replaced themes placeholder with `theme_view.widget()` in ViewStack
4. Added `ThemeApplied` message handler for toast notifications
5. Implemented Refresh forwarding to active view

**Updated `components/mod.rs`:**
- Added `pub mod theme_view;` export

## Code Statistics

**Files created:** 1 (theme_view.rs, 331 lines)
**Files modified:** 2 (app.rs, mod.rs)
**New dependencies:** None
**New patterns:** Container component composition, modal dialog lifecycle management

## Technical Decisions

### Container Component Pattern

Chose to create ThemeView as a standalone component rather than embedding logic in app.rs:

**Benefits:**
- Encapsulation: All theme logic in one module
- Testability: Can test theme operations independently
- Maintainability: Changes to theme functionality don't affect app shell

**Implementation:**
```rust
pub struct ThemeViewModel {
    theme_browser: Controller<ThemeBrowserModel>,
    preview_panel: Controller<PreviewPanelModel>,
    editor_dialog: Option<Controller<ThemeEditorModel>>,
    editor_window: Option<gtk::Window>,
    selected_theme: Option<Theme>,
    original_theme_id: String,
}
```

### Message Forwarding Chain

Implemented three-level message forwarding:
1. **ThemeCard** → ThemeBrowser (theme selection)
2. **ThemeBrowser** → ThemeView (theme selected output)
3. **ThemeView** → App (toast/theme applied outputs)

This maintains component boundaries while enabling app-level notifications.

### Modal Dialog Lifecycle

Chose to manage editor window lifecycle manually:
```rust
let window = gtk::Window::builder()
    .title(title)
    .modal(true)
    .child(editor.widget())
    .build();
window.present();
self.editor_window = Some(window);
```

**Why manual lifecycle:**
- GTK4 modal dialogs require explicit window management
- Need to close window on both Save and Cancel
- Allows cleaning up controller and window references

### File Import Dialog

Used GTK4 FileDialog for theme imports:
```rust
let dialog = gtk::FileDialog::builder()
    .title("Import Theme")
    .accept_label("Import")
    .build();

let filter = gtk::FileFilter::new();
filter.add_pattern("*.sh");
dialog.set_filters(Some(&filters));
```

**Pattern benefits:**
- Native file picker UI
- Automatic .sh file filtering
- Async callback for import completion

## Integration Points

### With Theme Browser
- Receives `ThemeBrowserOutput::ThemeSelected` events
- Sends `ThemeBrowserInput::Refresh` for reloading
- Sends `ThemeBrowserInput::SetCurrentTheme` after apply

### With Preview Panel
- Sends `PreviewPanelInput::SetTheme` on selection
- Detached (no output needed)

### With Theme Editor
- Forwards `ThemeEditorOutput::Saved` to save handler
- Forwards `ThemeEditorOutput::Cancelled` to close handler
- Manages editor window lifecycle

### With App Shell
- Outputs `ShowToast` for user notifications
- Outputs `ThemeApplied` for profile manager updates
- Receives `ThemeViewMsg::Refresh` from app

### With Services
- `theme_applier::preview_theme()` - temporary theme application
- `theme_applier::apply_theme()` - permanent theme application
- `theme_applier::revert_theme()` - cancel preview
- `theme_applier::get_current_theme()` - initial theme state
- `theme_storage::save_theme()` - persist theme edits
- `theme_storage::import_theme()` - import from file

## User Experience Flow

### Theme Selection
1. User clicks theme card in browser
2. ThemeCard emits Selected
3. ThemeBrowser forwards to ThemeView
4. ThemeView updates preview panel
5. Preview shows mock desktop with theme colors

### Theme Application
1. User clicks "Apply" button
2. ThemeView calls theme_applier::apply_theme()
3. Theme applied system-wide (Hyprland, GTK, terminals, etc.)
4. ThemeView updates browser current indicator
5. ThemeView sends ThemeApplied to app
6. App shows toast notification

### Theme Editing
1. User clicks "Edit" button (enabled for non-builtin themes)
2. ThemeView opens modal editor window
3. User modifies colors in editor
4. User clicks "Save Theme"
5. ThemeView calls theme_storage::save_theme()
6. ThemeView refreshes browser (shows updated theme)
7. ThemeView closes editor window

### Theme Import
1. User clicks "Import" button (top toolbar)
2. ThemeView opens file picker dialog
3. User selects .sh theme file
4. ThemeView calls theme_storage::import_theme()
5. ThemeView refreshes browser (shows new theme)
6. Toast shows success/failure

## Deviations from Plan

None - plan executed exactly as written. All specified functionality implemented:
- Horizontal paned layout ✓
- Browser on left, preview on right ✓
- Edit button for non-builtin themes ✓
- Apply/Preview/Cancel buttons ✓
- Modal editor dialog ✓
- Import file dialog ✓
- Message forwarding to app ✓

## Testing Notes

**Compilation:** ✓ Passed `cargo check` with only warnings (no errors)

**Expected behavior (requires X11/Wayland display):**
- Themes tab appears in ViewStack
- Theme browser shows grid of theme cards
- Clicking theme updates preview panel
- Apply button triggers theme application
- Edit button opens modal editor (non-builtin only)
- Import button opens file picker
- All operations show toast notifications

**Cannot test UI without display**, but component structure is sound based on:
- Successful compilation
- Proper message forwarding chains
- Correct service integration
- Valid GTK4 widget hierarchy

## Next Phase Readiness

**Ready for Phase 07 Plan 04 (Wallpaper View Integration):**
- Container component pattern established
- Message forwarding pattern demonstrated
- ViewStack integration pattern clear
- Toast notification pattern defined

**Blockers:** None

**Concerns:** None - theme functionality is fully encapsulated in ThemeView

## Lessons Learned

1. **Container components scale well** - ThemeView cleanly owns all theme-related child controllers
2. **Modal dialogs need manual cleanup** - Must explicitly manage window lifecycle to prevent leaks
3. **Message forwarding adds indirection** - Clear naming (Input/Output) helps track message flow
4. **ViewStack integration is straightforward** - Just add widget with add_titled_with_icon()

## Files Changed

### Created
- `vulcan-appearance-manager/src/components/theme_view.rs` (331 lines)
  - ThemeViewModel struct with child controllers
  - ThemeViewMsg enum for all operations
  - ThemeViewOutput enum for app communication
  - SimpleComponent implementation with paned layout
  - Modal editor management methods
  - File import dialog helper

### Modified
- `vulcan-appearance-manager/src/components/mod.rs`
  - Added theme_view module export

- `vulcan-appearance-manager/src/app.rs`
  - Added ThemeViewModel import and controller field
  - Added ThemeApplied message variant
  - Created theme_view with output forwarding
  - Replaced placeholder with theme_view widget in ViewStack
  - Implemented Refresh forwarding to active view
  - Added ThemeApplied toast handler

## Commit

```
feat(07-03): integrate ThemeView into ViewStack

- Create theme_view.rs container component with horizontal paned layout
- Wrap theme browser, preview panel, and editor into coherent tab
- Forward ThemeViewOutput to app-level toasts and theme_applied events
- Add Refresh message forwarding from app to active view
- Handle theme apply/preview/edit/import operations

Commit: 0b786c4
```
