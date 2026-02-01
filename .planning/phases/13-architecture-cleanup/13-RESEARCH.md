# Phase 13: Architecture Cleanup - Research

**Researched:** 2026-02-01
**Domain:** Rust/GTK4/Relm4 state machine integration with UI components
**Confidence:** HIGH

## Summary

This phase wires an existing AppState state machine (Idle, Previewing, Applying, Error) into a Relm4/GTK4 theme browser UI. The state machine already exists in `state.rs` with full transition validation. The challenge is integrating it into the component architecture where ThemeBrowserModel (child) sends selection events to ThemeViewModel (parent), which coordinates preview/apply operations.

Research focused on three core domains: (1) Relm4's message-passing component architecture for state synchronization, (2) GTK4 Revealer widget for action bar slide animations, and (3) Rust enum-based state machine patterns for maintaining explicit transitions while working within Relm4's SimpleComponent framework.

The standard approach uses message forwarding to transform child outputs into parent state machine transitions, stores the AppState in the parent component model, and uses GTK4 Revealer with SlideUp transition for the action bar appearance. Button sensitivity is controlled via `#[watch]` macros that react to state changes.

**Primary recommendation:** Store AppState in ThemeViewModel, add PreviewSnapshot creation when clicking theme cards, use message forwarding to trigger state transitions, wrap action bar in Revealer with reveal_child bound to `state.is_previewing()`.

## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| relm4 | 0.9 | GTK4 Elm-architecture GUI framework | Official Rust GTK4 framework, message-driven state management |
| gtk4 | 0.9 | GTK4 Rust bindings | Core UI toolkit, provides Revealer for animations |
| libadwaita | 0.7 | GNOME design patterns | Modern UI patterns, smooth animations |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| anyhow | 1.0 | Error handling with context | Already used for state transitions (Result returns) |
| tokio | 1.x | Async runtime | If apply operations need async (currently sync via Command) |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| SimpleComponent | AsyncComponent | More complex, only needed if apply becomes async |
| Revealer | Manual CSS animations | Revealer is simpler, GTK-native, handles timing automatically |
| Enum state machine | Typestate pattern | Current enum approach is correct for runtime transitions |

**Installation:**
Already in Cargo.toml - no new dependencies needed.

## Architecture Patterns

### Recommended Integration Structure

```
ThemeViewModel (parent)
├── model: AppState               # State machine instance
├── model: PreviewSnapshot        # Saved state for Cancel
├── children: ThemeBrowserModel   # Sends ThemeSelected
└── children: ActionBar           # Wrapped in Revealer
```

### Pattern 1: State Machine in Parent Component Model

**What:** Store AppState enum instance in the parent component's model struct, transition it in the `update()` method.

**When to use:** When state machine controls UI behavior across multiple child components.

**Example:**
```rust
// Source: Analyzed from existing codebase + Relm4 component patterns
pub struct ThemeViewModel {
    theme_browser: Controller<ThemeBrowserModel>,
    selected_theme: Option<Theme>,
    // ADD: State machine instance
    app_state: AppState,
    preview_snapshot: Option<PreviewSnapshot>,
}

impl SimpleComponent for ThemeViewModel {
    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            ThemeViewMsg::ThemeSelected(theme) => {
                // Clicking theme card while Idle starts preview
                if self.app_state.is_idle() {
                    let snapshot = self.create_snapshot();
                    match self.app_state.clone().start_preview(snapshot.clone()) {
                        Ok(new_state) => {
                            self.app_state = new_state;
                            self.preview_snapshot = Some(snapshot);
                            // Apply preview
                            theme_applier::preview_theme(&theme.theme_id)?;
                        }
                        Err(e) => {
                            // Invalid transition - log error
                        }
                    }
                }
                // Clicking different theme while Previewing switches preview
                else if self.app_state.is_previewing() {
                    // Keep original snapshot, just change preview
                    theme_applier::preview_theme(&theme.theme_id)?;
                }
            }

            ThemeViewMsg::ApplyTheme => {
                match self.app_state.clone().start_apply() {
                    Ok(new_state) => {
                        self.app_state = new_state;
                        // Perform apply operation
                        theme_applier::apply_theme(&theme_id)?;
                        // Finish transition
                        self.app_state = self.app_state.clone().finish()?;
                        self.preview_snapshot = None;
                    }
                    Err(e) => {
                        self.app_state = self.app_state.clone().fail(e.to_string());
                    }
                }
            }

            ThemeViewMsg::CancelPreview => {
                if let Some(snapshot) = &self.preview_snapshot {
                    // Restore previous state
                    if let Some(theme_id) = &snapshot.theme_id {
                        theme_applier::apply_theme(theme_id)?;
                    }
                    // Restore wallpapers per-monitor
                    for (monitor, path) in &snapshot.wallpapers {
                        // Emit to wallpaper view controller
                    }
                }
                match self.app_state.clone().cancel_preview() {
                    Ok(new_state) => {
                        self.app_state = new_state;
                        self.preview_snapshot = None;
                    }
                    Err(e) => { /* log */ }
                }
            }
        }
    }
}
```

### Pattern 2: Action Bar with Revealer Animation

**What:** Wrap action buttons in GTK4 Revealer widget, bind `reveal_child` property to state machine's `is_previewing()` check.

**When to use:** For UI elements that appear/disappear based on state (action bars, notifications, floating controls).

**Example:**
```rust
// Source: GTK4 Revealer docs + analyzed pattern
view! {
    gtk::Box {
        set_orientation: gtk::Orientation::Vertical,

        model.theme_browser.widget() {},

        // Action bar with slide-up animation
        #[name = "action_bar_revealer"]
        gtk::Revealer {
            set_transition_type: gtk::RevealerTransitionType::SlideUp,
            set_transition_duration: 200, // milliseconds
            #[watch]
            set_reveal_child: model.app_state.is_previewing(),

            gtk::ActionBar {
                pack_start = &gtk::Label {
                    #[watch]
                    set_markup: &format!(
                        "Applied: {} / Previewing: {}",
                        model.original_theme_id,
                        model.selected_theme.as_ref().map(|t| &t.theme_name).unwrap_or("")
                    ),
                },

                pack_end = &gtk::Box {
                    gtk::Button {
                        set_label: "Cancel",
                        #[watch]
                        set_sensitive: model.app_state.is_previewing(),
                        connect_clicked => ThemeViewMsg::CancelPreview,
                    },

                    gtk::Button {
                        set_label: "Apply",
                        add_css_class: "suggested-action",
                        #[watch]
                        set_sensitive: model.app_state.is_previewing() && !model.app_state.is_applying(),

                        // Spinner shows during Applying state
                        #[wrap(Some)]
                        set_child = &gtk::Box {
                            gtk::Spinner {
                                #[watch]
                                set_spinning: model.app_state.is_applying(),
                                #[watch]
                                set_visible: model.app_state.is_applying(),
                            },
                            gtk::Label {
                                #[watch]
                                set_label: if model.app_state.is_applying() { "Applying..." } else { "Apply" },
                            },
                        },
                        connect_clicked => ThemeViewMsg::ApplyTheme,
                    },
                },
            },
        },
    }
}
```

### Pattern 3: PreviewSnapshot Creation

**What:** Before starting preview, capture current theme ID and per-monitor wallpaper mappings into PreviewSnapshot struct.

**When to use:** Any time you need to restore multi-part state after cancel.

**Example:**
```rust
// Source: Existing state.rs PreviewSnapshot struct
impl ThemeViewModel {
    fn create_snapshot(&self) -> PreviewSnapshot {
        // Get current wallpapers from wallpaper view or hyprctl
        let wallpapers = self.get_current_wallpapers();

        PreviewSnapshot {
            wallpapers,
            theme_id: Some(self.original_theme_id.clone()),
        }
    }

    fn get_current_wallpapers(&self) -> HashMap<String, PathBuf> {
        // Query hyprctl for current wallpaper per monitor
        // OR maintain wallpaper state in App coordinator
        // Return map of monitor_name -> wallpaper_path
    }
}
```

### Pattern 4: Message Forwarding for State Transitions

**What:** Child components emit domain events (ThemeSelected), parent transforms these into state machine transitions.

**When to use:** Always - maintains component independence while coordinating state.

**Example:**
```rust
// Source: Relm4 component docs + existing theme_browser.rs
// In ThemeViewModel::init():
let theme_browser = ThemeBrowserModel::builder()
    .launch(())
    .forward(sender.input_sender(), |msg| {
        match msg {
            ThemeBrowserOutput::ThemeSelected(theme) => ThemeViewMsg::ThemeSelected(theme),
        }
    });

// ThemeBrowserModel remains independent - doesn't know about AppState
// ThemeViewModel handles state machine logic when processing ThemeSelected message
```

### Pattern 5: Error State Display

**What:** When state machine enters Error state, display inline error message in action bar, keep preview active so user can retry or cancel.

**When to use:** Apply failures that should allow retry without losing preview state.

**Example:**
```rust
// In view macro:
gtk::ActionBar {
    // Error message row
    #[watch]
    set_visible: model.app_state.is_error(),

    gtk::Box {
        gtk::Image {
            set_icon_name: Some("dialog-error-symbolic"),
        },
        gtk::Label {
            #[watch]
            set_markup: &if let AppState::Error { message, .. } = &model.app_state {
                format!("<span color='red'>{}</span>", message)
            } else {
                String::new()
            },
        },
    },
}

// In update():
ThemeViewMsg::ApplyTheme => {
    match self.app_state.clone().start_apply() {
        Ok(new_state) => {
            self.app_state = new_state;
            if let Err(e) = theme_applier::apply_theme(&theme_id) {
                // Apply failed - enter error state
                self.app_state = self.app_state.clone().fail(e.to_string());
                // Stay in preview mode - user can retry or cancel
            } else {
                // Success - finish and return to idle
                self.app_state = self.app_state.clone().finish()?;
            }
        }
        Err(e) => { /* invalid transition */ }
    }
}
```

### Anti-Patterns to Avoid

- **Storing state in multiple places:** AppState is single source of truth, don't duplicate is_previewing flags in model
- **Skipping state machine transitions:** Always use AppState methods (start_preview, cancel_preview), never manually set AppState::Idle
- **Blocking UI during apply:** Current theme_applier uses sync Command which is fast enough, but if it becomes slow, move to AsyncComponent with commands
- **Forgetting to clone AppState:** Transition methods consume self, must clone before calling: `self.app_state.clone().start_preview()`

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Slide-in animation | Custom CSS transitions | gtk::Revealer with SlideUp | GTK-native, handles timing/easing, accessibility-aware, respects gtk-enable-animations setting |
| Loading spinner | Custom spinning icon | gtk::Spinner | Native widget, proper ARIA labels, theme-aware styling |
| State validation | Runtime checks with bool flags | AppState enum with transition methods | Compile-time transition validation, exhaustive match checking, explicit error handling |
| Component state sync | Global mutable state | Message forwarding with Controller | Maintains component independence, type-safe message passing, prevents action-at-a-distance bugs |
| Async operations | Manual threading | Relm4 AsyncComponent / commands | Integrated with GTK main loop, proper shutdown handling, sender lifetime management |

**Key insight:** GTK4 and Relm4 provide high-level abstractions for common UI patterns. Using them results in more maintainable code with better accessibility and theme integration than custom implementations.

## Common Pitfalls

### Pitfall 1: AppState Clone Required for Transitions

**What goes wrong:** Calling `self.app_state.start_preview()` fails because transition methods consume `self`.

**Why it happens:** AppState transition methods are designed to consume the old state and return the new state, following Rust ownership patterns. This prevents accidental use of stale state.

**How to avoid:** Always clone before transitioning: `self.app_state.clone().start_preview(snapshot)`.

**Warning signs:** Compiler error "cannot move out of `self.app_state` which is behind a mutable reference"

### Pitfall 2: Revealer Reveal Not Updating

**What goes wrong:** Action bar doesn't slide in when state changes to Previewing.

**Why it happens:** The `#[watch]` macro in Relm4 only triggers on component update cycles. If you change state without triggering an update, the view won't refresh.

**How to avoid:** State changes should always happen in the `update()` method, which automatically triggers view updates. Never mutate model fields outside of `update()`.

**Warning signs:** State machine shows Previewing in logs, but UI doesn't change.

### Pitfall 3: Forgetting to Clear Preview Snapshot

**What goes wrong:** After apply completes, clicking a new theme crashes because preview_snapshot is still set from previous session.

**Why it happens:** PreviewSnapshot is used for cancel restore. If not cleared after apply, stale wallpaper paths may be restored.

**How to avoid:** Set `self.preview_snapshot = None` in both finish transitions (after Apply succeeds and after Cancel completes).

**Warning signs:** Wallpapers revert to old preview state when clicking new theme after applying.

### Pitfall 4: Action Bar Always Visible

**What goes wrong:** Action bar never disappears when returning to Idle state.

**Why it happens:** Revealer's `reveal_child` is not properly bound to state machine. Common mistake is setting it once in init instead of using `#[watch]`.

**How to avoid:** Use `#[watch]` macro on `set_reveal_child` property to make it reactive to state changes.

**Warning signs:** Action bar shows even when no theme is selected or after apply completes.

### Pitfall 5: Multi-Preview Snapshot Loss

**What goes wrong:** Clicking theme A (preview), then theme B (preview), then Cancel restores theme B instead of original theme.

**Why it happens:** Creating new snapshot on every ThemeSelected instead of only on Idle -> Previewing transition.

**Example of wrong approach:**
```rust
// WRONG - creates new snapshot every time
ThemeViewMsg::ThemeSelected(theme) => {
    let snapshot = self.create_snapshot(); // Captures theme B
    self.app_state = self.app_state.clone().start_preview(snapshot)?;
}
```

**How to avoid:** Only create snapshot when transitioning from Idle, keep it unchanged during multi-preview:
```rust
// CORRECT - snapshot only when entering preview mode
ThemeViewMsg::ThemeSelected(theme) => {
    if self.app_state.is_idle() {
        let snapshot = self.create_snapshot(); // Captures original
        self.app_state = self.app_state.clone().start_preview(snapshot)?;
    } else if self.app_state.is_previewing() {
        // Just switch preview, don't create new snapshot
        theme_applier::preview_theme(&theme.theme_id)?;
    }
}
```

**Warning signs:** Cancel doesn't restore original theme, restores previous preview instead.

### Pitfall 6: Button Sensitivity Race Conditions

**What goes wrong:** Apply button is clickable during Applying state, allowing double-apply.

**Why it happens:** Button sensitivity is only checking `is_previewing()` instead of `is_previewing() && !is_applying()`.

**How to avoid:** Combine state checks for button sensitivity:
```rust
gtk::Button {
    #[watch]
    set_sensitive: model.app_state.is_previewing() && !model.app_state.is_applying(),
}
```

**Warning signs:** Multiple "Applying theme..." toasts appear, theme apply errors about concurrent operations.

## Code Examples

Verified patterns from codebase analysis and official sources:

### Complete State Machine Integration

```rust
// Source: Synthesized from state.rs + Relm4 component patterns + theme_view.rs

pub struct ThemeViewModel {
    theme_browser: Controller<ThemeBrowserModel>,
    preview_panel: Controller<PreviewPanelModel>,
    selected_theme: Option<Theme>,
    original_theme_id: String,

    // State machine integration
    app_state: AppState,
    preview_snapshot: Option<PreviewSnapshot>,
}

#[derive(Debug)]
pub enum ThemeViewMsg {
    ThemeSelected(Theme),
    ApplyTheme,
    CancelPreview,
    // ... other messages
}

impl SimpleComponent for ThemeViewModel {
    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let original_theme_id = theme_applier::get_current_theme()
            .unwrap_or_else(|_| "tokyonight".to_string());

        let theme_browser = ThemeBrowserModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| {
                match msg {
                    ThemeBrowserOutput::ThemeSelected(theme) =>
                        ThemeViewMsg::ThemeSelected(theme),
                }
            });

        let model = ThemeViewModel {
            theme_browser,
            preview_panel,
            selected_theme: None,
            original_theme_id,
            app_state: AppState::Idle, // Start in Idle state
            preview_snapshot: None,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            ThemeViewMsg::ThemeSelected(theme) => {
                self.selected_theme = Some(theme.clone());

                // State machine transition: Idle -> Previewing
                if self.app_state.is_idle() {
                    let snapshot = self.create_snapshot();
                    match self.app_state.clone().start_preview(snapshot.clone()) {
                        Ok(new_state) => {
                            self.app_state = new_state;
                            self.preview_snapshot = Some(snapshot);

                            // Apply preview
                            if let Err(e) = theme_applier::preview_theme(&theme.theme_id) {
                                self.app_state = self.app_state.clone().fail(e.to_string());
                            }
                        }
                        Err(e) => {
                            eprintln!("Invalid state transition: {}", e);
                        }
                    }
                }
                // Multi-preview: switch preview without new snapshot
                else if self.app_state.is_previewing() {
                    if let Err(e) = theme_applier::preview_theme(&theme.theme_id) {
                        self.app_state = self.app_state.clone().fail(e.to_string());
                    }
                }
            }

            ThemeViewMsg::ApplyTheme => {
                if let Some(ref theme) = self.selected_theme {
                    // Transition to Applying state
                    match self.app_state.clone().start_apply() {
                        Ok(new_state) => {
                            self.app_state = new_state;

                            // Perform apply (sync operation)
                            match theme_applier::apply_theme(&theme.theme_id) {
                                Ok(_) => {
                                    // Success - finish and return to Idle
                                    match self.app_state.clone().finish() {
                                        Ok(new_state) => {
                                            self.app_state = new_state;
                                            self.preview_snapshot = None;
                                            self.original_theme_id = theme.theme_id.clone();

                                            sender.output(ThemeViewOutput::ThemeApplied(
                                                theme.theme_id.clone()
                                            )).ok();
                                        }
                                        Err(e) => {
                                            eprintln!("Invalid finish transition: {}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    // Apply failed - enter error state
                                    self.app_state = self.app_state.clone().fail(
                                        format!("Failed to apply theme: {}", e)
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Cannot apply from current state: {}", e);
                        }
                    }
                }
            }

            ThemeViewMsg::CancelPreview => {
                // Restore previous state
                if let Some(snapshot) = &self.preview_snapshot {
                    // Restore theme
                    if let Some(theme_id) = &snapshot.theme_id {
                        if let Err(e) = theme_applier::apply_theme(theme_id) {
                            eprintln!("Failed to restore theme: {}", e);
                        }
                    }

                    // Restore wallpapers (emit to wallpaper controller)
                    // This would be coordinated through App parent
                    sender.output(ThemeViewOutput::RestoreWallpapers(
                        snapshot.wallpapers.clone()
                    )).ok();
                }

                // Transition back to Idle
                match self.app_state.clone().cancel_preview() {
                    Ok(new_state) => {
                        self.app_state = new_state;
                        self.preview_snapshot = None;
                    }
                    Err(e) => {
                        eprintln!("Invalid cancel transition: {}", e);
                    }
                }
            }
        }
    }
}

impl ThemeViewModel {
    fn create_snapshot(&self) -> PreviewSnapshot {
        // Get current wallpapers from system
        // This would query hyprctl or use cached state from App
        let wallpapers = HashMap::new(); // Placeholder

        PreviewSnapshot {
            wallpapers,
            theme_id: Some(self.original_theme_id.clone()),
        }
    }
}
```

### Action Bar UI with Revealer

```rust
// Source: GTK4 Revealer docs + ActionBar patterns

view! {
    gtk::Box {
        set_orientation: gtk::Orientation::Vertical,
        set_spacing: 0,

        // Main content
        gtk::ScrolledWindow {
            set_vexpand: true,
            model.theme_browser.widget() {},
        },

        // Action bar with slide-up animation
        gtk::Revealer {
            set_transition_type: gtk::RevealerTransitionType::SlideUp,
            set_transition_duration: 200,
            #[watch]
            set_reveal_child: model.app_state.is_previewing() || model.app_state.is_error(),

            gtk::ActionBar {
                set_center_widget: Some(&gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 12,

                    // Status indicator
                    gtk::Label {
                        #[watch]
                        set_markup: &{
                            if model.app_state.is_previewing() {
                                format!(
                                    "Applied: <b>{}</b> / Previewing: <b>{}</b>",
                                    model.original_theme_id,
                                    model.selected_theme.as_ref()
                                        .map(|t| t.theme_name.as_str())
                                        .unwrap_or("None")
                                )
                            } else if let AppState::Error { message, .. } = &model.app_state {
                                format!("<span color='red'>Error: {}</span>", message)
                            } else {
                                String::new()
                            }
                        },
                    },
                }),

                // Action buttons
                pack_end = &gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,

                    gtk::Button {
                        set_label: "Cancel",
                        #[watch]
                        set_sensitive: model.app_state.is_previewing() && !model.app_state.is_applying(),
                        connect_clicked => ThemeViewMsg::CancelPreview,
                    },

                    gtk::Button {
                        add_css_class: "suggested-action",
                        #[watch]
                        set_sensitive: model.app_state.is_previewing() && !model.app_state.is_applying(),

                        #[wrap(Some)]
                        set_child = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,

                            gtk::Spinner {
                                #[watch]
                                set_spinning: model.app_state.is_applying(),
                                #[watch]
                                set_visible: model.app_state.is_applying(),
                            },

                            gtk::Label {
                                #[watch]
                                set_label: if model.app_state.is_applying() {
                                    "Applying..."
                                } else {
                                    "Apply"
                                },
                            },
                        },

                        connect_clicked => ThemeViewMsg::ApplyTheme,
                    },
                },
            },
        },
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual state tracking with bool flags | Explicit enum state machine with transitions | Phase 13 (v2.1) | Compile-time validation of transitions, prevents invalid states |
| Always-visible action buttons | Revealer-wrapped action bar | Phase 13 (v2.1) | Cleaner UI, only shows during preview |
| Single preview snapshot | Multi-preview with original snapshot | Phase 13 (v2.1) | Cancel restores original, not previous preview |
| Sync theme operations | Sync (current is fast enough) | N/A | If apply becomes slow, migrate to AsyncComponent |

**Deprecated/outdated:**
- Manual state validation: Don't use `if is_previewing && !is_applying` - use AppState transition methods
- Global state in App: State machine lives in ThemeViewModel, App only coordinates cross-tab communication

## Open Questions

1. **Wallpaper state coordination**
   - What we know: PreviewSnapshot needs HashMap<String, PathBuf> of current wallpapers
   - What's unclear: Should ThemeViewModel query hyprctl directly, or should App maintain wallpaper cache?
   - Recommendation: Query hyprctl in `create_snapshot()` - simpler, no cache sync issues. If performance becomes issue, add cache later.

2. **Re-clicking same theme while previewing**
   - What we know: CONTEXT.md marks as "Claude's Discretion"
   - What's unclear: No-op vs double-click-to-apply behavior
   - Recommendation: Make it no-op (ignore click). Rationale: Prevents accidental double-click from applying without confirmation, Apply button is explicit action.

3. **Async apply operations**
   - What we know: Current theme_applier uses sync Command, works fast
   - What's unclear: Whether future operations (wallpaper download, network themes) need async
   - Recommendation: Keep sync for now using SimpleComponent. If async needed, migrate to AsyncComponent with commands later (pattern well-documented in Relm4).

4. **Closing app while previewing**
   - What we know: CONTEXT.md says "implicit apply" (keep new look)
   - What's unclear: How to detect app close and trigger apply
   - Recommendation: Implement in App::drop() or window delete-event handler - call apply_theme with currently previewed theme.

## Sources

### Primary (HIGH confidence)
- Existing codebase state.rs - AppState state machine implementation with full test coverage
- [Relm4 Components Documentation](https://relm4.org/book/next/components.html) - Component message passing patterns
- [GTK4 Revealer API](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.Revealer.html) - Animation widget API
- [GTK4 RevealerTransitionType Enum](https://docs.gtk.org/gtk4/enum.RevealerTransitionType.html) - Available transition types
- [GTK4 ActionBar](https://docs.gtk.org/gtk4/class.ActionBar.html) - Action bar widget reference
- [GTK4 Spinner](https://gtk-rs.org/gtk4-rs/stable/0.3/docs/gtk4/struct.Spinner.html) - Loading indicator API

### Secondary (MEDIUM confidence)
- [Rust State Machine Pattern (Hoverbear)](https://hoverbear.org/blog/rust-state-machine-pattern/) - Generic state holder pattern
- [Enum-based State Machines (Yoshua Wuyts)](https://blog.yoshuawuyts.com/state-machines-2/) - Variant-specific transitions
- [Relm4 Async Components Discussion](https://github.com/Relm4/Relm4/issues/342) - AsyncComponent patterns
- [Relm4 Global State Discussion](https://github.com/orgs/Relm4/discussions/552) - Component state sharing patterns

### Tertiary (LOW confidence)
- WebSearch results for GTK4 animation patterns - general ecosystem knowledge, not specific to v0.9

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Cargo.toml confirmed, versions match, no new dependencies needed
- Architecture: HIGH - Patterns verified against existing codebase structure and official Relm4/GTK4 docs
- Pitfalls: HIGH - Derived from state.rs transition methods and Relm4 component lifecycle

**Research date:** 2026-02-01
**Valid until:** 2026-03-01 (30 days - stable stack, not fast-moving)
