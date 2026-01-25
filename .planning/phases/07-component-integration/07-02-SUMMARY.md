---
phase: 07
plan: 02
subsystem: ui-components
tags: [gtk4, relm4, theme-ui, factory-pattern, migration]

requires:
  - 06-01  # Unified crate foundation
  - 06-02  # Theme model and parser

provides:
  - Theme card factory component (FactoryComponent for FlowBox)
  - Theme browser grid with FactoryVecDeque pattern
  - Cairo-based theme preview panel
  - Theme editor with ExpanderRow color groups

affects:
  - 07-03  # Theme view container will compose these components
  - 07-04  # App integration will use theme browser

tech-stack:
  added: []
  patterns:
    - relm4::factory::FactoryComponent for theme cards
    - FactoryVecDeque with message forwarding
    - Cairo DrawingArea custom rendering
    - adw::ExpanderRow for collapsible groups

key-files:
  created:
    - vulcan-appearance-manager/src/components/theme_card.rs
    - vulcan-appearance-manager/src/components/theme_browser.rs
    - vulcan-appearance-manager/src/components/preview_panel.rs
    - vulcan-appearance-manager/src/components/theme_editor.rs
  modified:
    - vulcan-appearance-manager/src/components/mod.rs

decisions:
  - decision: Extract ThemeItem into separate theme_card.rs file
    rationale: Better organization than embedding in theme_browser.rs
    timestamp: 2026-01-25

  - decision: Use gtk::ColorButton despite deprecation
    rationale: Works correctly, migration to ColorDialogButton deferred to future polish phase
    timestamp: 2026-01-25

  - decision: Message forwarding from ThemeCardOutput to ThemeBrowserOutput
    rationale: Converts factory output to browser output, maintains proper component boundaries
    timestamp: 2026-01-25

metrics:
  duration: 5 minutes
  completed: 2026-01-25
---

# Phase 7 Plan 2: Theme UI Component Migration Summary

Migrated theme UI components from vulcan-theme-manager to vulcan-appearance-manager

## What Was Built

Migrated four proven theme UI components from the standalone vulcan-theme-manager application into the unified vulcan-appearance-manager crate, preserving their functionality while updating imports to use the unified crate's models and services.

### Components Migrated

1. **theme_card.rs** - Factory component for individual theme cards
   - 8-color palette preview (2 rows of 4 DrawingArea widgets)
   - Theme name with ellipsize
   - Current and Built-in badges
   - Clickable selection via GestureClick
   - Implements FactoryComponent for FlowBox parent

2. **theme_browser.rs** - Grid container for theme cards
   - FactoryVecDeque manages theme card collection
   - ScrolledWindow with FlowBox (2-6 columns adaptive)
   - Message forwarding: ThemeCardOutput → ThemeBrowserOutput
   - Refresh and SetCurrentTheme input messages
   - Loads themes from theme_storage service

3. **preview_panel.rs** - Cairo-based theme preview
   - Mock desktop rendering with theme colors
   - Mock Waybar top bar with accent highlight
   - Mock terminal window with ANSI color output
   - Empty state rendering for no theme selected
   - Theme info display (name and ID)

4. **theme_editor.rs** - Color group editor dialog
   - ExpanderRow-based organization (8 groups)
   - ColorButton widgets for color fields
   - Entry widgets for text fields (system themes)
   - Theme name/ID editing
   - Save/Cancel actions with output messages

### Technical Details

**FactoryVecDeque Pattern:**
```rust
let mut themes = FactoryVecDeque::builder()
    .launch(gtk::FlowBox::default())
    .forward(sender.output_sender(), |msg| {
        match msg {
            ThemeCardOutput::Selected(theme) => ThemeBrowserOutput::ThemeSelected(theme),
        }
    });
```

**Cairo Preview Rendering:**
```rust
area.set_draw_func(move |_, cr, width, height| {
    // Background
    cr.set_source_rgb(bg_primary.0, bg_primary.1, bg_primary.2);
    cr.rectangle(0.0, 0.0, width as f64, height as f64);
    let _ = cr.fill();
    // ... mock window, waybar, terminal content
});
```

**Color Group Organization:**
```rust
for group in ColorGroup::all_groups() {
    let expander = adw::ExpanderRow::builder()
        .title(group.name)
        .expanded(group.name == "Accents" || group.name == "Backgrounds")
        .build();
    // Add color rows...
}
```

## Deviations from Plan

None - plan executed exactly as written.

## Decisions Made

### 1. Separate theme_card.rs file
**Context:** vulcan-theme-manager had ThemeItem embedded within theme_browser.rs
**Decision:** Extract into separate theme_card.rs file
**Rationale:** Better organization, clearer module boundaries, easier to maintain
**Impact:** One additional file, but clearer responsibility separation

### 2. Keep gtk::ColorButton despite deprecation
**Context:** GTK4 deprecated ColorButton in favor of ColorDialogButton
**Decision:** Continue using ColorButton for theme editor
**Rationale:**
- Works correctly and is functional
- Migration to ColorDialogButton is non-trivial (50+ color fields)
- Can be addressed in future polish phase
- Deprecation warnings are acceptable for now
**Impact:** Deprecation warnings in build, but no functional issues

### 3. Message forwarding closure for type conversion
**Context:** ThemeCard outputs ThemeCardOutput, ThemeBrowser needs ThemeBrowserOutput
**Decision:** Use forward() closure to convert message types
**Rationale:** Proper Relm4 pattern for component composition, maintains type safety
**Impact:** Clean separation of component concerns, proper message boundaries

## Verification Results

1. `cargo check` passes with only unused import warnings (expected)
2. All four components created and exported in mod.rs
3. Components use unified crate's models (crate::models::Theme, ColorGroup)
4. Components use unified crate's services (crate::services::theme_storage, theme_applier)
5. FactoryVecDeque pattern correctly implemented with message forwarding
6. Cairo drawing functions properly clone theme colors for closure capture

## Testing Notes

Components compile successfully but are not yet integrated into the app UI. This is expected - the components are building blocks that will be composed into a ThemeView container in plan 07-03.

**Expected warnings:**
- Unused imports in components (will be used when integrated)
- Unused ColorGroup/ColorField in models/mod.rs (used by theme_editor)
- gtk::ColorButton deprecation (acknowledged, deferred to future polish)

## Next Phase Readiness

**Blockers:** None

**Recommended next steps:**
1. Plan 07-03: Create ThemeView container that composes browser + preview panel
2. Plan 07-04: Integrate ThemeView into app.rs with ViewStack
3. Future polish: Migrate ColorButton → ColorDialogButton with hex Entry fallback

**Component state:**
- ✅ theme_card.rs - Ready to use in FactoryVecDeque
- ✅ theme_browser.rs - Ready to use in ThemeView
- ✅ preview_panel.rs - Ready to use in ThemeView
- ✅ theme_editor.rs - Ready to use as modal dialog from ThemeView

**Dependencies available:**
- Theme model with preview_colors() method
- ColorGroup with all_groups() for editor
- theme_storage::load_all_themes()
- theme_applier::get_current_theme()

All components are isolated, tested via cargo check, and ready for composition in the next plan.
