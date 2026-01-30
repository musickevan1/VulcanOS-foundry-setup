# BindingMode Analysis

**Research Date:** 2026-01-30
**Purpose:** Understand BindingMode implementation and identify where CustomOverride detection should be triggered
**Context:** v2.1 Maintenance Milestone - Auto-detecting CustomOverride on manual wallpaper change

## Executive Summary

BindingMode tracks the relationship between theme and wallpaper in unified profiles. Currently, the transition from `ThemeBound` to `CustomOverride` when a user manually changes wallpaper is **NOT automatic**. The infrastructure exists but the detection logic is missing. This analysis identifies the exact integration point where auto-detection should be added.

**Key Finding:** The `WallpaperViewOutput::WallpapersChanged` event already propagates wallpaper changes to the App level, but the App doesn't check if this breaks an existing theme binding. Adding this check would enable automatic CustomOverride detection.

---

## 1. Current Implementation

### 1.1 BindingMode Enum

**Location:** `src/models/binding.rs:9-33`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BindingMode {
    /// Theme's suggested wallpaper is active
    ThemeBound,
    /// User has overridden theme's suggestion with custom wallpaper
    CustomOverride,
    /// Theme has no wallpaper suggestion (default)
    Unbound,
}
```

**Three States:**

| State | Meaning | When Set |
|-------|---------|----------|
| `ThemeBound` | Theme's wallpaper is active | User applies theme+wallpaper via binding dialog |
| `CustomOverride` | User manually changed wallpaper after applying theme | **NEVER (Missing Implementation)** |
| `Unbound` | No theme-wallpaper relationship | Default, or user applies theme without wallpaper |

**Display Names:**
- `ThemeBound` → "Theme Wallpaper"
- `CustomOverride` → "Custom Override"
- `Unbound` → "No Theme Wallpaper"

### 1.2 State Tracking in App

**Location:** `src/app.rs:49-60`

```rust
pub struct App {
    view_stack: adw::ViewStack,
    theme_view: Controller<ThemeViewModel>,
    wallpaper_view: Controller<WallpaperViewModel>,
    profile_view: Controller<ProfileViewModel>,
    profile_manager: Controller<ProfileManagerModel>,
    toast_overlay: adw::ToastOverlay,
    // Current appearance state for profile saving
    current_binding_mode: BindingMode,           // Line 57
    current_theme_id: Option<String>,             // Line 58
    current_wallpapers: HashMap<String, PathBuf>, // Line 59
}
```

The App maintains three pieces of state:
- **current_binding_mode** - Current relationship between theme and wallpaper
- **current_theme_id** - Active theme ID (if any)
- **current_wallpapers** - Per-monitor wallpaper paths

**Initial State (Line 231-234):**
```rust
current_binding_mode: BindingMode::Unbound,
current_theme_id: None,
current_wallpapers: HashMap::new(),
```

### 1.3 How BindingMode Currently Changes

**Location:** `src/app.rs:244-348`

| Event | Source | New Mode | Code Path |
|-------|--------|----------|-----------|
| Apply theme+wallpaper | Binding dialog → `BindingDialogOutput::ApplyBoth` | `ThemeBound` | `theme_view.rs:332-334` |
| Apply theme only | Binding dialog → `BindingDialogOutput::ApplyThemeOnly` | `Unbound` | `theme_view.rs:321-323` |
| Load profile | Profile view → `AppMsg::ProfileLoad` | `profile.binding_mode` | `app.rs:319` |
| Direct mode change | Theme view → `AppMsg::BindingModeChanged` | Provided mode | `app.rs:330-338` |

**Manual Wallpaper Change:** Currently does **NOT** change binding mode.

---

## 2. Wallpaper Change Points

### 2.1 WallpaperView Component

**Location:** `src/components/wallpaper_view.rs`

WallpaperView is responsible for displaying wallpapers and handling user interactions.

**Key Messages:**

```rust
pub enum WallpaperViewMsg {
    MonitorSelected(String),           // User selects monitor
    WallpaperSelected(PathBuf),        // User selects wallpaper from picker
    ApplyWallpaper,                    // User clicks "Apply" button
    SetWallpaperForAll(PathBuf),       // Apply same wallpaper to all monitors
    ApplyProfile(HashMap<String, PathBuf>), // Apply profile wallpapers
    // ... split dialog, refresh, etc.
}
```

**User-Initiated Wallpaper Changes:**

1. **Single Monitor Change** (`ApplyWallpaper`, Line 203-232)
   - User selects monitor + wallpaper, clicks "Apply"
   - Backend applies wallpaper
   - Emits `WallpaperViewOutput::WallpapersChanged(monitor_wallpapers)`

2. **All Monitors Change** (`SetWallpaperForAll`, Line 370-398)
   - User applies same wallpaper to all monitors
   - Iterates through monitors and applies
   - Emits `WallpaperViewOutput::WallpapersChanged(monitor_wallpapers)`

3. **Split Dialog** (`SplitGenerated`, Line 296-325)
   - User imports panoramic image and auto-splits to monitors
   - Applies all generated wallpapers
   - Emits `WallpaperViewOutput::WallpapersChanged(monitor_wallpapers)`

**Output Events:**

```rust
pub enum WallpaperViewOutput {
    ShowToast(String),
    WallpapersChanged(HashMap<String, PathBuf>),  // ← KEY EVENT
}
```

### 2.2 Event Propagation to App

**Location:** `src/app.rs:175-182`

WallpaperView output is forwarded to App:

```rust
let wallpaper_view = WallpaperViewModel::builder()
    .launch(())
    .forward(sender.input_sender(), |msg| {
        match msg {
            WallpaperViewOutput::ShowToast(text) => AppMsg::ShowToast(text),
            WallpaperViewOutput::WallpapersChanged(wps) => AppMsg::WallpapersChanged(wps),
        }
    });
```

### 2.3 App Handling of WallpapersChanged

**Location:** `src/app.rs:294-307`

```rust
AppMsg::WallpapersChanged(wallpapers) => {
    // Track current wallpapers
    self.current_wallpapers = wallpapers.clone();

    // Notify profile manager of wallpaper changes
    self.profile_manager.emit(ProfileManagerInput::UpdateWallpapers(wallpapers.clone()));

    // Sync state to profile view for saving
    self.profile_view.emit(ProfileViewMsg::UpdateCurrentState {
        theme_id: self.current_theme_id.clone(),
        wallpapers: self.current_wallpapers.clone(),
        binding_mode: self.current_binding_mode.clone(),  // ← USES EXISTING MODE
    });
}
```

**PROBLEM:** The App updates `current_wallpapers` but **does NOT check if this breaks an existing `ThemeBound` state**. The binding mode is propagated as-is without validation.

---

## 3. Missing Detection Logic

### 3.1 What's Missing

When `AppMsg::WallpapersChanged` is received, the App needs to:

1. **Check if currently ThemeBound**
   - If `current_binding_mode == BindingMode::ThemeBound`
   - And `current_theme_id` exists

2. **Resolve theme's wallpaper**
   - Load theme from `theme_storage`
   - Use `resolve_theme_wallpaper()` to get expected path

3. **Compare wallpapers**
   - Compare new wallpapers with theme's wallpaper
   - If they differ → user has overridden

4. **Transition to CustomOverride**
   - Set `current_binding_mode = BindingMode::CustomOverride`
   - Emit `BindingModeChanged` to sync state
   - Optionally show toast notification

### 3.2 Edge Cases to Handle

| Scenario | Expected Behavior |
|----------|-------------------|
| Already `CustomOverride` | No change needed (already in override state) |
| Already `Unbound` | No change needed (no theme binding exists) |
| No theme loaded (`current_theme_id == None`) | No change needed |
| Wallpaper change matches theme wallpaper | Remain `ThemeBound` (user re-applied same) |
| Profile load event | Don't trigger detection (profile sets mode explicitly) |
| Theme wallpaper application | Don't trigger detection (sets `ThemeBound` via binding dialog) |

### 3.3 Required Helper Function

**Location:** `src/models/binding.rs:65-92`

This function already exists:

```rust
pub fn resolve_theme_wallpaper(theme: &Theme) -> Option<PathBuf> {
    let wallpaper_rel = theme.theme_wallpaper.as_ref()?;
    let theme_dir = theme.source_path.as_ref()?.parent()?;
    let abs_path = theme_dir.join(wallpaper_rel);

    if abs_path.exists() {
        Some(abs_path)
    } else {
        None
    }
}
```

---

## 4. Implementation Approach

### 4.1 Integration Point

**Target:** `src/app.rs:294-307` - `AppMsg::WallpapersChanged` handler

**Current Code:**
```rust
AppMsg::WallpapersChanged(wallpapers) => {
    self.current_wallpapers = wallpapers.clone();
    self.profile_manager.emit(ProfileManagerInput::UpdateWallpapers(wallpapers.clone()));
    self.profile_view.emit(ProfileViewMsg::UpdateCurrentState { ... });
}
```

**Enhanced Code (Pseudo):**
```rust
AppMsg::WallpapersChanged(wallpapers) => {
    self.current_wallpapers = wallpapers.clone();

    // AUTO-DETECT CustomOverride
    if self.current_binding_mode == BindingMode::ThemeBound {
        if let Some(ref theme_id) = self.current_theme_id {
            if let Ok(theme) = theme_storage::load_theme(theme_id) {
                if let Some(theme_wallpaper) = resolve_theme_wallpaper(&theme) {
                    // Check if any monitor has different wallpaper
                    let has_override = wallpapers.values().any(|wp| wp != &theme_wallpaper);

                    if has_override {
                        // Transition to CustomOverride
                        self.current_binding_mode = BindingMode::CustomOverride;

                        // Sync state
                        self.profile_view.emit(ProfileViewMsg::UpdateCurrentState {
                            theme_id: self.current_theme_id.clone(),
                            wallpapers: wallpapers.clone(),
                            binding_mode: BindingMode::CustomOverride,
                        });

                        // Optional: Notify user
                        let toast = adw::Toast::new("Wallpaper customized (keeping theme colors)");
                        self.toast_overlay.add_toast(toast);
                    }
                }
            }
        }
    }

    // Continue with existing notifications
    self.profile_manager.emit(ProfileManagerInput::UpdateWallpapers(wallpapers.clone()));
    self.profile_view.emit(ProfileViewMsg::UpdateCurrentState { ... });
}
```

### 4.2 Required Imports

Add to `src/app.rs`:
```rust
use crate::models::resolve_theme_wallpaper;
use crate::services::theme_storage;
```

### 4.3 Dependencies

**No new dependencies needed.** All required functions exist:
- `BindingMode::ThemeBound` - Already used
- `resolve_theme_wallpaper()` - Exists in `models/binding.rs`
- `theme_storage::load_theme()` - Exists in `services/theme_storage.rs`

### 4.4 Testing Strategy

**Manual Testing:**

1. **Setup:**
   - Apply theme with wallpaper (→ `ThemeBound`)
   - Verify profile view shows "Theme Wallpaper"

2. **Trigger Override:**
   - Go to Wallpapers tab
   - Select different wallpaper
   - Click "Apply"

3. **Verify:**
   - Profile view should now show "Custom Override"
   - Theme colors should remain active
   - Save profile and reload - mode should persist

**Edge Case Testing:**

1. **Re-apply theme wallpaper:**
   - Start in `CustomOverride`
   - Manually apply theme's original wallpaper
   - Should remain `CustomOverride` (no auto-revert to ThemeBound)

2. **Profile load:**
   - Save profile with `ThemeBound`
   - Change wallpaper (→ `CustomOverride`)
   - Load profile
   - Should restore `ThemeBound` from profile (not auto-override)

3. **No theme loaded:**
   - Start with no theme
   - Change wallpaper
   - Should remain `Unbound`

---

## 5. Related Code Paths

### 5.1 Theme Application with Wallpaper

**Flow:** User applies theme via binding dialog

1. `theme_view.rs:225-236` - User clicks "Apply Theme"
2. `theme_view.rs:228-230` - Check if theme has wallpaper
3. `theme_view.rs:405-419` - Show binding dialog
4. `theme_view.rs:326-335` - Handle `ApplyBoth` choice
   - Sets `BindingMode::ThemeBound`
   - Emits `ThemeViewOutput::ApplyWallpaper(wallpaper_path)`
5. `app.rs:341-346` - Handles `ApplyThemeWallpaper`
   - Emits `WallpaperViewMsg::SetWallpaperForAll(wallpaper_path)`
6. `wallpaper_view.rs:370-398` - Applies wallpaper
   - Emits `WallpaperViewOutput::WallpapersChanged`
7. `app.rs:294-307` - Handles `WallpapersChanged`
   - **CURRENT:** Doesn't detect override
   - **PROPOSED:** Would check and transition to CustomOverride if user later changes

### 5.2 Profile Saving

**Location:** `src/components/profile_view.rs` (inferred from app.rs usage)

The App pushes current state to ProfileView via:
```rust
self.profile_view.emit(ProfileViewMsg::UpdateCurrentState {
    theme_id: self.current_theme_id.clone(),
    wallpapers: self.current_wallpapers.clone(),
    binding_mode: self.current_binding_mode.clone(),
});
```

When user saves profile, ProfileView uses this state to create `UnifiedProfile` with correct `binding_mode`.

---

## 6. Recommendations

### 6.1 Implementation Priority

**HIGH** - This is a core UX issue. Users expect the UI to reflect reality:
- If they manually change wallpaper after applying a theme, the binding state should update automatically
- Current behavior silently keeps "Theme Wallpaper" label even though user has customized

### 6.2 Implementation Scope

**Minimal:** Single function enhancement in `app.rs:294-307`
- ~15-20 lines of code
- No new dependencies
- No UI changes
- Uses existing infrastructure

### 6.3 Future Enhancements

Consider adding:

1. **Revert to Theme Wallpaper Action**
   - If in `CustomOverride`, show button to restore theme's wallpaper
   - Would transition back to `ThemeBound`

2. **Wallpaper Comparison UI**
   - Show current wallpaper vs theme's suggested wallpaper
   - Visual diff in profile view

3. **Auto-detection Settings**
   - User preference to enable/disable auto-detection
   - Some users may prefer manual control

---

## 7. Open Questions

### 7.1 Multi-Monitor Scenarios

**Question:** If theme has one wallpaper but user has multiple monitors, what's the expected behavior?

**Current Behavior:**
- `SetWallpaperForAll` applies theme wallpaper to all monitors when using binding dialog
- Each monitor gets same wallpaper path

**Detection Logic:**
- If ANY monitor has different wallpaper than theme → `CustomOverride`
- This seems correct (user has customized at least one monitor)

### 7.2 Partial Overrides

**Question:** If user changes wallpaper on Monitor A but not Monitor B, should it be `CustomOverride`?

**Answer:** Yes. The binding is at profile level, not per-monitor. Any customization breaks the theme binding.

### 7.3 Toast Notification

**Question:** Should auto-detection show a toast notification?

**Recommendation:** Yes, but subtle:
- "Wallpaper customized (keeping theme colors)"
- Informs user that mode changed
- Confirms theme colors are still active

---

## 8. Files to Modify

| File | Changes | Lines |
|------|---------|-------|
| `src/app.rs` | Add CustomOverride detection in `WallpapersChanged` handler | ~20 lines added |
| `src/app.rs` | Add imports for `resolve_theme_wallpaper`, `theme_storage` | 2 lines |

**Total estimated changes:** ~25 lines across 1 file

---

## 9. Verification Checklist

After implementation:

- [ ] Apply theme+wallpaper → mode is `ThemeBound`
- [ ] Change wallpaper manually → mode transitions to `CustomOverride`
- [ ] Save profile in `CustomOverride` → profile stores correct mode
- [ ] Load profile → mode restored correctly
- [ ] Profile view shows correct binding mode label
- [ ] Theme colors remain active after override
- [ ] Toast notification appears on auto-transition
- [ ] Edge case: No theme loaded → no crash
- [ ] Edge case: Theme with no wallpaper → no false positives
- [ ] Edge case: Re-apply same wallpaper → no spurious transitions

---

## Conclusion

The BindingMode system is well-architected with clear state transitions. The missing piece is **automatic detection when user manually changes wallpaper**. This can be implemented with minimal changes in the `AppMsg::WallpapersChanged` handler by:

1. Checking if currently `ThemeBound`
2. Resolving theme's wallpaper
3. Comparing with new wallpaper paths
4. Transitioning to `CustomOverride` if different

All required infrastructure exists. Implementation is straightforward and low-risk.
