# AppState Analysis - State Machine Integration Research

**Domain:** GTK4/Relm4 UI State Management
**Researched:** 2026-01-30
**Overall confidence:** HIGH (code exists, verified by direct inspection)

## Executive Summary

The AppState state machine was implemented in Phase 6 (06-03-PLAN.md) but never integrated into UI components. It exists as a fully-tested, working module (`src/state.rs`, 320 lines, 17 passing tests) but is completely bypassed by the current UI implementation.

**Current reality:**
- AppState defines 4 states (Idle, Previewing, Applying, Error) with validated transitions
- UI components (ThemeViewModel, WallpaperViewModel) do NOT use AppState
- Preview/Apply operations call services directly without state tracking
- No revert mechanism - `revert_theme()` re-applies current theme, doesn't restore previous state
- Profile operations bypass preview/apply state entirely

**Integration challenge:**
The Relm4 architecture uses per-component state in separate `Model` structs. AppState was designed as a global singleton state machine, but Relm4 doesn't have a natural "global state" location. The current App struct tracks appearance state (theme_id, wallpapers, binding_mode) but not preview/apply lifecycle state.

## Current Implementation

### 1. AppState Definition (src/state.rs)

**Location:** `/home/evan/VulcanOS/vulcan-appearance-manager/src/state.rs`

**States:**
```rust
pub enum AppState {
    Idle,                              // No preview active
    Previewing { previous: PreviewSnapshot },  // Preview active, can revert
    Applying,                          // Currently applying changes
    Error { message: String, recovery: Box<AppState> }, // Error occurred
}
```

**PreviewSnapshot:**
```rust
pub struct PreviewSnapshot {
    pub wallpapers: HashMap<String, PathBuf>,  // Monitor -> wallpaper before preview
    pub theme_id: Option<String>,              // Theme ID before preview
}
```

**Transitions (all return `Result<AppState>`):**
- `start_preview(snapshot)` - Idle → Previewing (only valid from Idle)
- `start_apply()` - Idle|Previewing → Applying
- `finish()` - Previewing|Applying → Idle
- `cancel_preview()` - Previewing → Idle (reverts to snapshot)
- `fail(message)` - Any → Error (always succeeds)
- `recover()` - Error → recovery state

**Query methods:**
- `is_idle()`, `is_previewing()`, `is_applying()`, `is_error()` - State checks
- `previous_snapshot()` - Get snapshot if previewing

**Test coverage:** 17 tests, all passing
- Valid transitions (8 tests)
- Invalid transitions (6 tests)
- Query methods (3 tests)

**Usage:** NONE. Module is declared in main.rs but never imported by any component.

### 2. Current UI State Management

#### App Struct (src/app.rs)

**Tracks appearance state (NOT lifecycle state):**
```rust
pub struct App {
    current_binding_mode: BindingMode,           // How theme/wallpaper are bound
    current_theme_id: Option<String>,            // Active theme
    current_wallpapers: HashMap<String, PathBuf>, // Active wallpapers per monitor
    // ... component controllers ...
}
```

**What's missing:**
- No AppState field
- No preview vs. applied distinction
- No snapshot of state before preview
- No error state tracking

#### ThemeViewModel (src/components/theme_view.rs)

**State fields:**
```rust
pub struct ThemeViewModel {
    selected_theme: Option<Theme>,  // Theme selected in browser
    original_theme_id: String,      // Theme at startup (for Cancel button)
    // ... component controllers ...
}
```

**Operations that should use AppState:**

1. **PreviewTheme (line 214-222):**
   - Currently: Calls `theme_applier::preview_theme()` directly
   - Should: Transition to `Previewing` state, capture snapshot
   - Problem: No snapshot captured, can't restore previous wallpapers

2. **ApplyTheme (line 225-237):**
   - Currently: Shows binding dialog, then calls `apply_theme_only()`
   - Should: Transition to `Applying` state, then `finish()` on success
   - Problem: Can apply while previewing without capturing that transition

3. **CancelPreview (line 239-245):**
   - Currently: Calls `theme_applier::revert_theme()` (re-applies current theme)
   - Should: Call `AppState::cancel_preview()` and restore snapshot
   - Problem: `revert_theme()` doesn't restore wallpapers, only theme

4. **Error handling:**
   - Currently: Toast message only
   - Should: Transition to `Error` state, allow recovery

**Missing integration:**
- No AppState field in ThemeViewModel
- Operations don't capture snapshots
- No state-based button disabling (Preview enabled while already previewing)

#### WallpaperViewModel (src/components/wallpaper_view.rs)

**State fields:**
```rust
pub struct WallpaperViewModel {
    selected_monitor: Option<String>,
    selected_wallpaper: Option<PathBuf>,
    monitor_wallpapers: HashMap<String, PathBuf>, // Current state
    // ... component controllers ...
}
```

**Operations that should use AppState:**

1. **ApplyWallpaper (line 203-231):**
   - Currently: Calls `backend.apply()` directly
   - Should: Check if in Previewing state, handle accordingly
   - Problem: Can apply wallpaper while theme preview is active

2. **ApplyProfile (line 340-368):**
   - Currently: Applies all wallpapers immediately
   - Should: Respect global preview/apply state
   - Problem: Profile loading bypasses any preview state entirely

**Missing integration:**
- No AppState awareness at all
- No coordination with theme preview state
- No snapshot mechanism for wallpaper-only operations

### 3. Service Layer (theme_applier.rs)

**Functions called by UI:**

1. `preview_theme(theme_id)` - Calls `vulcan-theme preview <id>`
2. `apply_theme(theme_id)` - Calls `vulcan-theme set <id>`
3. `revert_theme()` - Gets current theme via `get_current_theme()`, re-applies it

**Problem with revert_theme():**
```rust
pub fn revert_theme() -> Result<()> {
    let current = get_current_theme()?;  // Reads saved theme from disk
    apply_theme(&current)                 // Re-applies it
}
```

This doesn't restore the previous snapshot. It re-applies the **saved** theme, which:
- Loses any preview changes (intended)
- DOESN'T restore previous wallpapers (bug if theme changed wallpapers)
- DOESN'T restore previous binding mode

**What it should do:**
```rust
// In App or ThemeViewModel:
if let Some(snapshot) = app_state.previous_snapshot() {
    // Restore theme
    if let Some(theme_id) = &snapshot.theme_id {
        theme_applier::apply_theme(theme_id)?;
    }
    // Restore wallpapers
    for (monitor, wallpaper) in &snapshot.wallpapers {
        wallpaper_backend.apply(monitor, wallpaper)?;
    }
    // Transition state
    app_state = app_state.cancel_preview()?;
}
```

## UI Components Needing Integration

### Priority 1: ThemeViewModel (CRITICAL)

**Why critical:**
- Preview/Apply/Cancel operations are core theme workflow
- Currently has no revert-to-snapshot mechanism
- Error states not properly handled

**Changes needed:**

1. **Add AppState field:**
   ```rust
   pub struct ThemeViewModel {
       app_state: AppState,  // Add this
       selected_theme: Option<Theme>,
       original_theme_id: String,
   }
   ```

2. **Capture snapshot before preview:**
   ```rust
   ThemeViewMsg::PreviewTheme => {
       // Get current state from parent App
       let snapshot = PreviewSnapshot {
           theme_id: self.current_theme_id.clone(),
           wallpapers: self.current_wallpapers.clone(),
       };

       // Transition to Previewing
       match self.app_state.clone().start_preview(snapshot) {
           Ok(new_state) => {
               self.app_state = new_state;
               // Now call theme_applier::preview_theme()
           }
           Err(e) => {
               // Show error toast
           }
       }
   }
   ```

3. **Handle apply transition:**
   ```rust
   ThemeViewMsg::ApplyTheme => {
       match self.app_state.clone().start_apply() {
           Ok(new_state) => {
               self.app_state = new_state;
               // Show binding dialog, apply theme
               // On success: self.app_state = self.app_state.clone().finish()?;
           }
           Err(e) => { /* error handling */ }
       }
   }
   ```

4. **Restore snapshot on cancel:**
   ```rust
   ThemeViewMsg::CancelPreview => {
       if let Some(snapshot) = self.app_state.previous_snapshot() {
           // Restore theme
           if let Some(theme_id) = &snapshot.theme_id {
               theme_applier::apply_theme(theme_id)?;
           }
           // Request parent to restore wallpapers
           sender.output(ThemeViewOutput::RestoreWallpapers(snapshot.wallpapers.clone()))?;

           // Transition to Idle
           self.app_state = self.app_state.clone().cancel_preview()?;
       }
   }
   ```

5. **State-based button sensitivity:**
   ```rust
   gtk::Button {
       set_label: "Preview",
       #[watch]
       set_sensitive: model.selected_theme.is_some() && model.app_state.is_idle(),
   }

   gtk::Button {
       set_label: "Cancel",
       #[watch]
       set_sensitive: model.app_state.is_previewing(),
   }

   gtk::Button {
       set_label: "Apply",
       #[watch]
       set_sensitive: model.selected_theme.is_some() && !model.app_state.is_applying(),
   }
   ```

**Data flow problem:**
ThemeViewModel needs current wallpaper state to create snapshots, but wallpapers are managed by WallpaperViewModel. Need App to provide snapshot data.

### Priority 2: App (COORDINATOR)

**Why needed:**
- App has the global appearance state (theme_id, wallpapers, binding_mode)
- App coordinates between ThemeViewModel and WallpaperViewModel
- AppState should live in App, not individual view components

**Changes needed:**

1. **Add AppState field:**
   ```rust
   pub struct App {
       app_state: AppState,  // Add this
       current_binding_mode: BindingMode,
       current_theme_id: Option<String>,
       current_wallpapers: HashMap<String, PathBuf>,
       // ... component controllers ...
   }
   ```

2. **Create new message types:**
   ```rust
   pub enum AppMsg {
       // Preview/Apply lifecycle
       StartPreview,        // Create snapshot, transition to Previewing
       ApplyChanges,        // Transition to Applying
       CancelPreview,       // Restore snapshot, transition to Idle
       FinishOperation,     // Transition to Idle after apply succeeds

       // Existing messages...
   }
   ```

3. **Handle state transitions:**
   ```rust
   AppMsg::StartPreview => {
       let snapshot = PreviewSnapshot {
           theme_id: self.current_theme_id.clone(),
           wallpapers: self.current_wallpapers.clone(),
       };

       match self.app_state.clone().start_preview(snapshot) {
           Ok(new_state) => {
               self.app_state = new_state;
               // Notify child components preview started
           }
           Err(e) => {
               self.show_toast(&format!("Cannot preview: {}", e));
           }
       }
   }
   ```

4. **Restore snapshot on cancel:**
   ```rust
   AppMsg::CancelPreview => {
       if let Some(snapshot) = self.app_state.previous_snapshot() {
           // Restore theme
           if let Some(theme_id) = &snapshot.theme_id {
               self.theme_view.emit(ThemeViewMsg::ApplyThemeById(theme_id.clone()));
           }

           // Restore wallpapers
           self.wallpaper_view.emit(WallpaperViewMsg::ApplyProfile(snapshot.wallpapers.clone()));

           // Transition to Idle
           match self.app_state.clone().cancel_preview() {
               Ok(new_state) => self.app_state = new_state,
               Err(e) => eprintln!("Cancel preview failed: {}", e),
           }
       }
   }
   ```

**Benefits:**
- Centralized state management
- Child components query state via App
- Snapshot creation/restoration coordinated by App
- State transitions validated before operations

### Priority 3: WallpaperViewModel (OPTIONAL)

**Why optional:**
- Wallpaper operations are independent of theme preview in current UX
- No preview/apply/cancel workflow for wallpapers alone
- Could be added later if wallpaper-only preview is desired

**Potential changes:**
- Query App's AppState before applying wallpaper
- Disable Apply button if theme preview is active
- Participate in snapshot restoration

## Integration Approaches

### Approach 1: Centralized State (RECOMMENDED)

**Pattern:** AppState lives in App, child components query/request transitions

**Pros:**
- Single source of truth
- App coordinates snapshot creation/restoration
- Child components simplified (no state machine logic)
- Natural fit for Relm4's message-passing architecture

**Cons:**
- More App messages
- Requires plumbing state queries to children

**Implementation:**
1. Add `app_state: AppState` to App
2. Add state transition messages (StartPreview, ApplyChanges, etc.)
3. Child components send messages to App for state changes
4. App handles snapshot creation/restoration
5. Child components query state for button sensitivity

**Example message flow:**
```
User clicks "Preview" button
→ ThemeViewModel sends AppMsg::StartPreview
→ App creates snapshot, transitions to Previewing
→ App sends ThemeViewMsg::ExecutePreview back to ThemeViewModel
→ ThemeViewModel calls theme_applier::preview_theme()
```

### Approach 2: Distributed State (NOT RECOMMENDED)

**Pattern:** Each view component has its own AppState copy

**Pros:**
- Components are self-contained
- No central coordination needed

**Cons:**
- Multiple AppState instances can desync
- No global snapshot (theme preview doesn't know about wallpaper state)
- Violates single-source-of-truth principle
- Coordination bugs likely

**Conclusion:** Don't use this approach. AppState was designed as a global state machine.

### Approach 3: Shared State via Relm4 Connector (COMPLEX)

**Pattern:** Use Relm4's `Connector` to share AppState between components

**Pros:**
- Reactive state updates
- No manual message passing for state queries

**Cons:**
- Overly complex for the use case
- Relm4 connectors designed for simpler data sharing
- State transitions require message coordination anyway

**Conclusion:** Centralized State (Approach 1) is simpler and sufficient.

## Architectural Concerns

### 1. Snapshot Data Ownership

**Problem:** AppState's `PreviewSnapshot` contains wallpaper HashMap and theme_id, but:
- Wallpapers are managed by WallpaperViewModel
- Theme is managed by ThemeViewModel
- App tracks both but doesn't "own" the canonical state

**Current reality:**
```
WallpaperViewModel.monitor_wallpapers (HashMap<String, PathBuf>)
   ↓ updates via WallpapersChanged message
App.current_wallpapers (HashMap<String, PathBuf>)
   ↓ read by ProfileView for saving
```

**Solution:**
App is already the coordinator. Make it the snapshot source:
```rust
// In App:
AppMsg::StartPreview => {
    let snapshot = PreviewSnapshot {
        theme_id: self.current_theme_id.clone(),
        wallpapers: self.current_wallpapers.clone(),  // App already has this
    };
    self.app_state = self.app_state.clone().start_preview(snapshot)?;
}
```

App already receives `WallpapersChanged` events and tracks current state. No additional plumbing needed.

### 2. State Transition Message Flow

**Question:** Who initiates state transitions?

**Answer:** Child components request transitions, App executes them.

**Pattern:**
```
ThemeViewModel (user clicks Preview)
  → sends AppMsg::RequestPreview(theme_id)
App (receives message)
  → validates transition (app_state.start_preview())
  → if Ok: sends ThemeViewMsg::ExecutePreview(theme_id)
  → if Err: shows error toast
ThemeViewModel (receives ExecutePreview)
  → calls theme_applier::preview_theme(theme_id)
  → shows success/error toast
```

**Why this pattern:**
- App validates state transitions (enforces state machine rules)
- Child components execute operations (call services)
- Clear separation of concerns

### 3. Button Sensitivity and State Reactivity

**Problem:** Buttons need to be sensitive/insensitive based on AppState, but AppState lives in App.

**Relm4 solution:** Parent-to-child message for state updates

**Pattern:**
```rust
// In App:
AppMsg::StartPreview => {
    self.app_state = ...;
    // Notify children of state change
    self.theme_view.emit(ThemeViewMsg::StateChanged(self.app_state.clone()));
}

// In ThemeViewModel:
pub struct ThemeViewModel {
    app_state: AppState,  // Local copy for UI reactivity
}

ThemeViewMsg::StateChanged(new_state) => {
    self.app_state = new_state;  // Triggers #[watch] macros to re-evaluate
}

// In view! macro:
gtk::Button {
    set_label: "Preview",
    #[watch]
    set_sensitive: model.app_state.is_idle() && model.selected_theme.is_some(),
}
```

**Benefits:**
- App owns canonical AppState
- Child components have read-only copy for UI reactivity
- #[watch] macro automatically updates button sensitivity

### 4. Error State Handling

**Current:** Errors shown as toasts, no state tracking

**With AppState:**
```rust
// In App:
AppMsg::ThemePreviewFailed(error_msg) => {
    self.app_state = self.app_state.clone().fail(error_msg.clone());
    // Show error dialog
    self.show_error_dialog(&error_msg);
}

// Error dialog has "OK" button:
AppMsg::AcknowledgeError => {
    match self.app_state.clone().recover() {
        Ok(new_state) => {
            self.app_state = new_state;  // Returns to Idle
            // Restore snapshot if we were previewing
            if let Some(snapshot) = self.app_state.previous_snapshot() {
                self.restore_snapshot(snapshot);
            }
        }
        Err(e) => eprintln!("Recovery failed: {}", e),
    }
}
```

**Benefits:**
- Errors don't leave app in undefined state
- Can restore snapshot after error during preview
- Recovery path is explicit

### 5. Profile Loading and AppState

**Current:** Profile loading (AppMsg::ProfileLoad) applies theme and wallpapers immediately, bypassing any preview state.

**Question:** Should profile loading respect preview state?

**Options:**

1. **Profile loading always succeeds (ignores preview state)**
   - If previewing: cancel preview, load profile
   - Rationale: Loading profile is a "reset" operation

2. **Profile loading fails if previewing/applying**
   - User must cancel preview first
   - Rationale: Explicit state management, no surprises

3. **Profile loading creates its own preview**
   - Load profile into preview state
   - User can cancel to return to previous state
   - Rationale: Consistent with theme preview workflow

**Recommendation:** Option 1 (always succeeds)
- Profile loading is a high-level operation (reset to saved state)
- Canceling preview automatically is acceptable UX
- Implementation:
  ```rust
  AppMsg::ProfileLoad(profile) => {
      // Cancel any active preview
      if self.app_state.is_previewing() {
          self.app_state = self.app_state.clone().cancel_preview()?;
      }
      // Load profile...
  }
  ```

## Integration Plan Summary

### Phase 1: Add AppState to App (Foundation)

**Files:**
- `src/app.rs`

**Changes:**
1. Add `app_state: AppState` field to App struct
2. Initialize to `AppState::default()` (Idle)
3. Add state transition messages (StartPreview, ApplyChanges, CancelPreview, etc.)
4. Implement snapshot creation in App (uses existing current_theme_id, current_wallpapers)
5. Implement snapshot restoration in App (delegates to ThemeViewModel, WallpaperViewModel)

**Verification:**
- App compiles
- cargo check passes
- No functional changes (state transitions not yet triggered)

### Phase 2: Wire ThemeViewModel to Use AppState (Core Workflow)

**Files:**
- `src/components/theme_view.rs`
- `src/app.rs` (message handling)

**Changes:**
1. Add `app_state: AppState` field to ThemeViewModel (read-only copy)
2. Add `StateChanged(AppState)` message to ThemeViewModel
3. Update PreviewTheme to send `AppMsg::RequestPreview` instead of calling service directly
4. Update ApplyTheme to send `AppMsg::RequestApply`
5. Update CancelPreview to send `AppMsg::RequestCancel`
6. Add button sensitivity based on `model.app_state` (via #[watch])

**App-side changes:**
7. Handle RequestPreview: create snapshot, transition to Previewing, send ExecutePreview
8. Handle RequestApply: transition to Applying, send ExecuteApply
9. Handle RequestCancel: restore snapshot, transition to Idle
10. Send StateChanged to ThemeViewModel after each transition

**Verification:**
- Preview → Cancel workflow restores theme AND wallpapers
- Apply workflow transitions Previewing → Applying → Idle
- Buttons disable/enable based on state
- Error handling transitions to Error state

### Phase 3: Optional Enhancements

**Error state UI:**
- Modal error dialog instead of toast
- "Retry" button for transient errors
- Restore snapshot on error recovery

**WallpaperViewModel integration:**
- Disable Apply button if theme preview is active
- Optional wallpaper-only preview workflow

**State persistence:**
- Save AppState to prevent losing preview on app restart
- Restore on launch (questionable UX, probably skip)

## Risks and Mitigations

### Risk 1: State Machine Overhead for Simple Operations

**Concern:** Adding state machine transitions for every Preview/Apply adds complexity.

**Mitigation:**
- AppState is opt-in: wallpaper-only operations can bypass it
- State transitions are validated, preventing bugs
- Complexity is in App (coordinator), not scattered across components

**Verdict:** Acceptable. The explicit state machine prevents subtle bugs (e.g., preview during preview, apply during apply).

### Risk 2: Snapshot Size and Performance

**Concern:** Copying wallpaper HashMap (potentially many entries) for every preview.

**Reality check:**
- Typical system: 1-3 monitors
- HashMap clone is cheap (Arc<PathBuf> internally)
- Preview is user-initiated, not a hot path

**Verdict:** Not a concern. Snapshot clone is negligible.

### Risk 3: Relm4 Message Passing Latency

**Concern:** Request → App → Response message flow adds latency.

**Reality:**
- Relm4 messages are synchronous within same thread
- Latency is microseconds, imperceptible
- Service calls (theme_applier) are the bottleneck, not message passing

**Verdict:** Not a concern.

### Risk 4: State Desync Between App and Children

**Concern:** Child components have stale AppState copy.

**Mitigation:**
- App sends StateChanged message after every transition
- Children update local copy immediately
- Single source of truth: App's app_state

**Verification strategy:**
- Add debug logging for state transitions
- Unit test: verify StateChanged emitted after each transition

**Verdict:** Manageable with proper message handling.

## Open Questions

### Q1: Should wallpaper changes participate in preview state?

**Current:** Wallpaper changes apply immediately, no preview/cancel workflow.

**Option A:** Keep current behavior (wallpapers independent)
- Simpler implementation
- Matches current UX expectations
- AppState only used for theme operations

**Option B:** Add wallpaper preview workflow
- Consistent with theme preview
- Allows "try before apply" for wallpapers
- More complex (WallpaperViewModel needs state machine logic)

**Recommendation:** Option A for v2.1 (maintenance milestone). Option B for future enhancement.

### Q2: How to handle concurrent operations?

**Scenario:** User previews theme A, then immediately previews theme B without canceling.

**Current AppState behavior:**
- `Previewing.start_preview()` returns Err (invalid transition)

**Options:**
1. **Reject second preview (current behavior)**
   - User must cancel A before previewing B
   - Explicit state management

2. **Auto-cancel previous preview**
   - `start_preview()` checks if currently previewing, auto-cancels
   - More permissive UX

**Recommendation:** Option 1 (reject). Enforce explicit state transitions for clarity.

**UI consideration:** Disable Preview button while previewing to prevent confusion.

### Q3: Should state persist across app restarts?

**Scenario:** User previews theme, closes app, reopens.

**Options:**
1. **Don't persist (recommended)**
   - App always starts in Idle state
   - Preview is ephemeral
   - Simple implementation

2. **Persist preview state**
   - Serialize AppState to disk
   - Restore on launch
   - Complex: must validate snapshot is still valid

**Recommendation:** Don't persist. Preview is a temporary workflow, not a saved state.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| AppState Implementation | HIGH | Verified by reading src/state.rs, all 17 tests pass |
| Current UI Bypass | HIGH | Verified by reading component source, no imports of AppState |
| Integration Approach | HIGH | Centralized state is idiomatic Relm4 pattern |
| Snapshot Mechanism | HIGH | App already tracks all necessary state (theme_id, wallpapers) |
| Message Flow | MEDIUM | Relm4 message patterns are well-understood, but haven't tested in this codebase |
| Button Sensitivity | HIGH | #[watch] macro is standard Relm4 feature for reactive UI |

## Gaps to Address

**During implementation:**
- Test message flow thoroughly (RequestPreview → StateChanged → ExecutePreview)
- Verify snapshot restoration restores ALL state (theme + wallpapers + binding_mode?)
- Add error state UI (currently only Error state exists, no UI for it)
- Consider loading spinner during Applying state (prevent double-apply)

**Future enhancements:**
- Wallpaper preview workflow (if desired)
- Undo/redo stack (multiple snapshots)
- State machine visualization for debugging (optional)

## Success Criteria

Integration is complete when:

- [ ] App has `app_state: AppState` field
- [ ] Preview button creates snapshot and transitions to Previewing
- [ ] Apply button transitions Previewing → Applying → Idle
- [ ] Cancel button restores snapshot (theme + wallpapers) and transitions to Idle
- [ ] Buttons are sensitive/insensitive based on AppState
- [ ] Error during preview transitions to Error state
- [ ] Profile loading cancels preview if active
- [ ] No regressions: existing wallpaper-only operations still work
- [ ] All existing tests pass
- [ ] Manual testing: Preview → Cancel restores previous appearance

## Implementation Priority

**v2.1 Maintenance Milestone:**
1. Phase 1: Add AppState to App (foundation)
2. Phase 2: Wire ThemeViewModel to use AppState (core workflow)

**Future (v2.2+):**
3. Phase 3: Error state UI enhancements
4. Optional: WallpaperViewModel preview workflow
5. Optional: Undo/redo with snapshot stack

## Conclusion

The AppState state machine is fully implemented and tested, but completely unused. Integration requires:

1. **Adding AppState to App** (coordinator role)
2. **Plumbing state transitions via messages** (App validates, children execute)
3. **Snapshot creation/restoration in App** (uses existing state tracking)
4. **Reactive UI updates** (StateChanged message + #[watch] macros)

The integration is straightforward because:
- App already tracks all necessary state (theme_id, wallpapers)
- Relm4's message-passing architecture naturally supports this pattern
- AppState's API is well-designed for the use case

**Primary benefit:** Preview → Cancel will actually restore previous state (currently broken for wallpapers).

**Secondary benefits:**
- Explicit state machine prevents invalid transitions (preview during preview, etc.)
- Error state tracking enables recovery workflows
- Foundation for future features (undo/redo, wallpaper preview)
