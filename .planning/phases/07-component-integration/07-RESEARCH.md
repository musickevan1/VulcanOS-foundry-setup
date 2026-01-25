# Phase 7: Component Integration - Research

**Researched:** 2026-01-24
**Domain:** GTK4/libadwaita UI component integration with Relm4
**Confidence:** HIGH

## Summary

This phase merges two existing Relm4 applications (vulcan-theme-manager and vulcan-appearance-manager/wallpaper-manager) into a single tabbed interface using libadwaita's ViewStack pattern. Both applications are already built on the same technology stack (GTK4 0.9, libadwaita 0.7, Relm4 0.9) with identical dependency versions, making integration primarily a structural task rather than an architectural migration.

The research confirms that:
1. **ViewStack + ViewSwitcher is the correct libadwaita pattern** for fixed application views (not dynamic user-managed tabs)
2. **Both codebases use identical Relm4 component patterns** (Controller-based composition, message forwarding, factory patterns for grids)
3. **Existing components can be migrated as-is** with minimal changes to internal logic
4. **Shared CSS module already exists** (brand_css.rs) eliminating styling duplication

**Primary recommendation:** Use adw::ViewStack with adw::ViewSwitcher in the header bar for two fixed views ("Themes" and "Wallpapers"), migrate existing components into these views preserving their internal layouts (horizontal paned for themes, vertical paned for wallpapers), and elevate profile manager to header bar as a shared component.

## Standard Stack

### Core (Already in Use)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| gtk4 | 0.9 (features: v4_16) | Widget toolkit | Official GTK4 Rust bindings, latest stable |
| libadwaita | 0.7 (features: v1_6) | GNOME HIG widgets | Provides ViewStack/ViewSwitcher for modern GNOME apps |
| relm4 | 0.9 (features: libadwaita) | Reactive UI framework | Type-safe message passing, component composition |
| anyhow | 1.0 | Error handling | Ergonomic error propagation, context chaining |
| serde/serde_json/toml | 1.0/1.0/0.8 | Serialization | Standard Rust serialization ecosystem |

**Key insight:** Both apps already use **identical dependency versions**, confirmed by examining Cargo.toml files. This means zero version conflict resolution is needed.

### Supporting (Already in Use)

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| image | 0.25 | Image loading | Wallpaper thumbnail generation |
| dirs | 5 | Cross-platform paths | Finding config/data directories |
| regex | 1 | Pattern matching | Theme file parsing validation |
| lazy_static | 1.4 | Global state | Compiled regex patterns |
| const_format | 0.2 | Compile-time strings | CSS module string concatenation |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| ViewStack | TabView | TabView is for dynamic user-managed tabs (like browser tabs), ViewStack is for fixed app sections - **ViewStack is correct** per [libadwaita docs](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1.5/class.ViewStack.html) |
| ViewSwitcher in header | ViewSwitcherBar | ViewSwitcherBar is for narrow/mobile windows, ViewSwitcher in header is standard for desktop apps |
| Horizontal tabs | Sidebar navigation | Sidebar adds complexity, two tabs don't justify vertical real estate |

**Installation:**
```bash
# Already specified in both Cargo.toml files
[dependencies]
gtk4 = { version = "0.9", package = "gtk4", features = ["v4_16"] }
libadwaita = { version = "0.7", package = "libadwaita", features = ["v1_6"] }
relm4 = { version = "0.9", features = ["libadwaita"] }
```

## Architecture Patterns

### Recommended Project Structure

```
vulcan-appearance-manager/src/
├── main.rs                      # Entry point, loads brand CSS
├── brand_css.rs                 # Shared CSS module (already exists)
├── app.rs                       # Root app with ViewStack
├── state.rs                     # Application state (already exists for wallpapers)
├── models/                      # Data structures
│   ├── mod.rs
│   ├── theme.rs                 # Theme model (from theme-manager)
│   ├── wallpaper.rs             # Wallpaper model (already exists)
│   ├── monitor.rs               # Monitor model (already exists)
│   ├── profile.rs               # Profile model (already exists)
│   └── color_group.rs           # Color group model (from theme-manager)
├── components/                  # UI components
│   ├── mod.rs
│   ├── theme_view.rs            # NEW: Themes tab container (horizontal paned)
│   ├── theme_browser.rs         # From theme-manager
│   ├── theme_card.rs            # From theme-manager (FactoryComponent)
│   ├── preview_panel.rs         # From theme-manager
│   ├── theme_editor.rs          # From theme-manager
│   ├── wallpaper_view.rs        # NEW: Wallpapers tab container (vertical paned)
│   ├── wallpaper_picker.rs      # Already exists
│   ├── monitor_layout.rs        # Already exists
│   ├── profile_manager.rs       # ELEVATE: Moves to header bar, shared
│   └── split_dialog.rs          # Already exists
└── services/                    # Backend logic
    ├── mod.rs
    ├── theme_parser.rs          # From theme-manager
    ├── theme_storage.rs         # From theme-manager
    ├── theme_applier.rs         # From theme-manager
    ├── wallpaper_backend.rs     # Already exists (swww/hyprpaper abstraction)
    ├── hyprctl.rs               # Already exists
    ├── thumbnail.rs             # Already exists
    ├── image_splitter.rs        # Already exists
    └── profile_storage.rs       # Already exists
```

### Pattern 1: ViewStack + ViewSwitcher for Fixed Views

**What:** Use adw::ViewStack as the content container with adw::ViewSwitcher in the header bar for navigation between fixed application sections.

**When to use:** Applications with 2-5 main views that users switch between frequently, where views are not dynamically created/destroyed.

**Example:**
```rust
// Source: https://relm4.org/docs/stable/libadwaita/struct.ViewStack.html
// and codebase analysis

view! {
    adw::ApplicationWindow {
        adw::ToolbarView {
            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::ViewSwitcher {
                    set_stack: Some(&view_stack),
                    set_policy: adw::ViewSwitcherPolicy::Wide,
                },

                // Profile manager controls here (shared across views)
                pack_start = model.profile_manager.widget() {},
            },

            #[wrap(Some)]
            #[name = "view_stack"]
            set_content = &adw::ViewStack {
                // Add views with titles and icons
                add_titled_with_icon(
                    model.theme_view.widget(),
                    Some("themes"),
                    "Theme Browser",
                    "preferences-color-symbolic"
                ),

                add_titled_with_icon(
                    model.wallpaper_view.widget(),
                    Some("wallpapers"),
                    "Wallpaper Manager",
                    "preferences-desktop-wallpaper-symbolic"
                ),
            },
        }
    }
}
```

### Pattern 2: Controller-Based Component Composition

**What:** Child components wrapped in `Controller<T>` initialized with builder pattern and message forwarding.

**When to use:** Any reusable component that manages its own state and communicates with parent via messages.

**Example:**
```rust
// Source: https://relm4.org/book/stable/child_components.html

// In App model
pub struct App {
    theme_view: Controller<ThemeViewModel>,
    wallpaper_view: Controller<WallpaperViewModel>,
    profile_manager: Controller<ProfileManagerModel>,
}

// In init()
let theme_view = ThemeViewModel::builder()
    .launch(())
    .forward(sender.input_sender(), |msg| {
        match msg {
            ThemeViewOutput::ThemeApplied(theme) => AppMsg::ThemeApplied(theme),
            ThemeViewOutput::Error(e) => AppMsg::ShowError(e),
        }
    });
```

### Pattern 3: FactoryVecDeque for Dynamic Grids

**What:** Relm4's `FactoryVecDeque` manages collections of widgets (like theme cards in FlowBox) with automatic synchronization.

**When to use:** Displaying dynamic lists/grids where items can be added/removed/reordered.

**Example:**
```rust
// Source: https://github.com/Relm4/Relm4/blob/main/examples/factory.rs
// and vulcan-theme-manager/src/components/theme_browser.rs

#[derive(Debug)]
pub struct ThemeItem {
    theme: Theme,
    is_current: bool,
}

#[relm4::factory(pub)]
impl FactoryComponent for ThemeItem {
    type Init = (Theme, bool);
    type Input = ();
    type Output = ThemeBrowserOutput;
    type ParentWidget = gtk::FlowBox;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            add_css_class: "theme-card",
            // Color preview grid + theme name + badges
        }
    }
}

// In browser component
pub struct ThemeBrowserModel {
    themes: FactoryVecDeque<ThemeItem>,
    current_theme_id: String,
}

// Update themes
fn refresh_themes(&mut self) {
    let mut guard = self.themes.guard();
    guard.clear();
    for theme in load_themes() {
        let is_current = theme.theme_id == self.current_theme_id;
        guard.push_back((theme, is_current));
    }
}
```

### Pattern 4: Nested Paned Layouts

**What:** GTK4 Paned widgets create resizable split views. Each tab can have its own orientation.

**When to use:** When two related components need to share space with user-adjustable sizing.

**Example:**
```rust
// Source: https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.Paned.html
// and existing app.rs files

// Themes tab: Horizontal paned (browser left, preview right)
gtk::Paned {
    set_orientation: gtk::Orientation::Horizontal,
    set_position: 550,

    #[wrap(Some)]
    set_start_child = &gtk::Frame {
        model.theme_browser.widget() {},
    },

    #[wrap(Some)]
    set_end_child = &gtk::Frame {
        model.preview_panel.widget() {},
    },
}

// Wallpapers tab: Vertical paned (layout top, picker bottom)
gtk::Paned {
    set_orientation: gtk::Orientation::Vertical,
    set_position: 350,

    #[wrap(Some)]
    set_start_child = &gtk::Frame {
        model.monitor_layout.widget() {},
    },

    #[wrap(Some)]
    set_end_child = &gtk::Frame {
        model.wallpaper_picker.widget() {},
    },
}
```

### Pattern 5: Modal Dialogs for Editors

**What:** Complex editors (theme editor, split dialog) open as separate modal windows.

**When to use:** When editing requires full user focus and separate save/cancel workflow.

**Example:**
```rust
// Source: vulcan-theme-manager/src/app.rs

fn open_editor(&mut self, theme: Option<Theme>, is_new: bool, sender: ComponentSender<Self>) {
    let editor = ThemeEditorModel::builder()
        .launch((theme, is_new))
        .forward(sender.input_sender(), |msg| {
            match msg {
                ThemeEditorOutput::Saved(theme) => AppMsg::ThemeSaved(theme),
                ThemeEditorOutput::Cancelled => AppMsg::EditorCancelled,
            }
        });

    let window = gtk::Window::builder()
        .title("Edit Theme")
        .modal(true)
        .default_width(550)
        .default_height(600)
        .child(editor.widget())
        .build();

    window.present();
    self.editor_dialog = Some(editor);
    self.editor_window = Some(window);
}
```

### Anti-Patterns to Avoid

- **Don't mix message types:** Each component should have distinct Input/Output enums. Use `forward()` closure to convert between incompatible types.
- **Don't bypass Controller:** Never access child component internals directly. Always communicate via `.emit(msg)`.
- **Don't mutate FactoryVecDeque without guard:** All mutations require `.guard()` call for automatic widget sync. Forgetting this leads to stale UI.
- **Don't add/remove FlowBox children directly when bound to model:** FlowBox bound to FactoryVecDeque manages children automatically. Manual manipulation causes crashes.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Tab navigation | Custom tab bar with buttons | adw::ViewStack + adw::ViewSwitcher | ViewSwitcher handles keyboard shortcuts (Ctrl+Tab), accessibility, adaptive width, and matches GNOME HIG |
| Dynamic widget lists | Manual widget insertion/removal | relm4::factory::FactoryVecDeque | Automatic synchronization, RAII guards prevent forgetting updates, efficient minimal re-renders |
| Color parsing | String manipulation for hex | gtk4::gdk::RGBA with to_string()/parse() | Handles alpha channel, validation, and conversion to CSS format |
| Modal state management | Boolean flags + conditionals | Dedicated component with Controller | Encapsulates state, prevents memory leaks from unclosed dialogs |
| Cairo drawing contexts | Manual surface allocation | gtk::DrawingArea with set_draw_func | GTK4 provides context automatically, handles resize events |

**Key insight:** GTK4 and Relm4 provide comprehensive solutions for UI patterns. Custom implementations are almost always worse due to missing edge cases (accessibility, keyboard navigation, memory management, HiDPI scaling).

## Common Pitfalls

### Pitfall 1: ViewStack Pages Not Showing

**What goes wrong:** Adding child widgets to ViewStack but they don't appear or switcher is empty.

**Why it happens:** ViewStack requires explicit page configuration with title and name via `add_titled()` or `add_titled_with_icon()`. Plain `add()` doesn't create visible pages.

**How to avoid:** Always use `add_titled_with_icon()` for pages with proper name, title, and icon:
```rust
view_stack.add_titled_with_icon(
    &child_widget,
    Some("unique-name"),  // Must be unique and non-empty
    "Display Title",      // User-visible name
    "icon-name-symbolic"  // Standard icon name
);
```

**Warning signs:** ViewSwitcher appears empty or has no buttons, pages exist but switcher shows nothing.

### Pitfall 2: Controller Message Forwarding Type Mismatches

**What goes wrong:** Compilation errors when child component Output type doesn't match parent Input type.

**Why it happens:** Child components are reusable and don't know parent's message types. Direct forwarding fails when types differ.

**How to avoid:** Use `forward()` closure to explicitly convert:
```rust
// WRONG - Output and Input types differ
.forward(sender.input_sender(), identity)

// CORRECT - Explicit conversion
.forward(sender.input_sender(), |msg| {
    match msg {
        ChildOutput::Action1 => ParentMsg::HandleAction1,
        ChildOutput::Action2(data) => ParentMsg::HandleAction2(data),
    }
})
```

**Warning signs:** Compiler error "expected `AppMsg`, found `ChildOutput`" or similar type mismatches.

### Pitfall 3: FactoryVecDeque Mutations Without Guard

**What goes wrong:** Modifying FactoryVecDeque (push, remove, reorder) but UI doesn't update or updates incorrectly.

**Why it happens:** FactoryVecDeque uses RAII guard pattern. Mutations only sync to widgets when guard drops. Forgetting `.guard()` means changes never render.

**How to avoid:** Always wrap mutations in guard scope:
```rust
// WRONG - Won't update UI
self.items.push_back(new_item);

// CORRECT - Guard ensures update
{
    let mut guard = self.items.guard();
    guard.push_back(new_item);
    guard.remove(old_index);
} // Guard drops here, UI updates automatically
```

**Warning signs:** Data model changes but UI is frozen, items appear in wrong order, items disappear without being removed.

### Pitfall 4: Cairo DrawingArea Memory Leaks in GTK4

**What goes wrong:** Creating multiple DrawingArea widgets with custom draw functions causes memory to grow unbounded, especially during repeated previews.

**Why it happens:** GTK4 DrawingArea holds references to draw function closures. If closures capture heavy data (images, buffers) or aren't properly released, memory accumulates. This is a known concern per [GTK4 memory management docs](https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_memory_management.html).

**How to avoid:**
- Keep draw function closures lightweight
- Don't capture large data structures; pass via widget data or separate model
- For preview panels, reuse single DrawingArea and update its draw function rather than creating new widgets
- Explicitly clear/reset when switching between previews

**Warning signs:** Memory usage climbs with each preview, `htop` shows GTK process growing, sluggish UI after multiple operations.

### Pitfall 5: ColorButton Deprecation in GTK4

**What goes wrong:** Using `gtk::ColorButton` generates deprecation warnings or doesn't build with newer GTK4 versions.

**Why it happens:** GTK4 deprecated ColorButton in favor of ColorDialogButton per [GTK4 docs](https://docs.gtk.org/gtk4/class.ColorButton.html). The old API is legacy.

**How to avoid:** Use `gtk::ColorDialogButton` for new code. For color input with hex strings:
```rust
// Modern approach
let color_btn = gtk::ColorDialogButton::new(gtk::ColorDialog::new());
color_btn.connect_rgba_notify(|btn| {
    let rgba = btn.rgba();
    let hex = format!("#{:02x}{:02x}{:02x}",
        (rgba.red() * 255.0) as u8,
        (rgba.green() * 255.0) as u8,
        (rgba.blue() * 255.0) as u8
    );
    // Use hex string
});
```

**Warning signs:** Deprecation warnings during build, API not available in latest GTK4 versions.

### Pitfall 6: FlowBox Bound to Model - Manual Child Management

**What goes wrong:** Adding/removing widgets directly to FlowBox that's bound to a FactoryVecDeque causes crashes or undefined behavior.

**Why it happens:** Per [GTK4 FlowBox docs](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.FlowBox.html), "It is undefined to add or remove widgets directly while FlowBox is bound to a model."

**How to avoid:** Manage all children via the FactoryVecDeque, never call `flowbox.insert()` or `flowbox.remove()` directly:
```rust
// WRONG - Manual manipulation
flowbox.insert(&widget, position);

// CORRECT - Via factory model
let mut guard = self.factory_items.guard();
guard.insert(position, item_data);
```

**Warning signs:** Segfaults, "widget has no parent" errors, items appearing multiple times, crashes on removal.

## Code Examples

Verified patterns from official sources and existing codebase:

### ViewStack Initialization with Two Views

```rust
// Source: Codebase structure analysis + libadwaita patterns
use adw::prelude::*;
use relm4::prelude::*;

pub struct App {
    view_stack: adw::ViewStack,
    theme_view: Controller<ThemeViewModel>,
    wallpaper_view: Controller<WallpaperViewModel>,
    profile_manager: Controller<ProfileManagerModel>,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("VulcanOS Appearance Manager"),
            set_default_size: (1000, 700),

            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::ViewSwitcher {
                        #[watch]
                        set_stack: Some(&model.view_stack),
                        set_policy: adw::ViewSwitcherPolicy::Wide,
                    },

                    // Profile manager shared across both tabs
                    pack_start = model.profile_manager.widget() {},

                    // Refresh button
                    pack_end = &gtk::Button {
                        set_icon_name: "view-refresh-symbolic",
                        connect_clicked => AppMsg::Refresh,
                    },
                },

                #[wrap(Some)]
                #[name = "view_stack"]
                set_content = &adw::ViewStack {
                    // Initialization below in init()
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let theme_view = ThemeViewModel::builder()
            .launch(())
            .forward(sender.input_sender(), convert_theme_output);

        let wallpaper_view = WallpaperViewModel::builder()
            .launch(())
            .forward(sender.input_sender(), convert_wallpaper_output);

        let profile_manager = ProfileManagerModel::builder()
            .launch(())
            .forward(sender.input_sender(), convert_profile_output);

        let model = App {
            view_stack: adw::ViewStack::new(),
            theme_view,
            wallpaper_view,
            profile_manager,
        };

        let widgets = view_output!();

        // Add pages to ViewStack AFTER widgets initialized
        widgets.view_stack.add_titled_with_icon(
            model.theme_view.widget(),
            Some("themes"),
            "Themes",
            "preferences-color-symbolic"
        );

        widgets.view_stack.add_titled_with_icon(
            model.wallpaper_view.widget(),
            Some("wallpapers"),
            "Wallpapers",
            "preferences-desktop-wallpaper-symbolic"
        );

        ComponentParts { model, widgets }
    }
}
```

### FactoryVecDeque Theme Card Grid

```rust
// Source: vulcan-theme-manager/src/components/theme_browser.rs
use relm4::factory::{FactoryVecDeque, FactoryComponent};

#[derive(Debug)]
pub struct ThemeItem {
    theme: Theme,
    is_current: bool,
}

#[relm4::factory(pub)]
impl FactoryComponent for ThemeItem {
    type Init = (Theme, bool);
    type Input = ();
    type Output = ThemeBrowserOutput;
    type ParentWidget = gtk::FlowBox;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 8,
            set_width_request: 180,
            add_css_class: "theme-card",

            // 8-color palette preview
            gtk::Frame {
                add_css_class: "color-preview-frame",
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 2,
                    // Two rows of 4 colors each
                }
            },

            // Theme name
            gtk::Label {
                set_label: &self.theme.theme_name,
                add_css_class: "theme-name",
            },

            // Badges
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 4,

                gtk::Label {
                    #[watch]
                    set_visible: self.is_current,
                    set_label: "Current",
                    add_css_class: "current-badge",
                },
            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        ThemeItem {
            theme: init.0,
            is_current: init.1,
        }
    }
}

pub struct ThemeBrowserModel {
    themes: FactoryVecDeque<ThemeItem>,
    current_theme_id: String,
}

impl ThemeBrowserModel {
    fn refresh(&mut self) {
        let themes = theme_storage::load_all_themes().unwrap_or_default();

        // Use guard for atomic update
        let mut guard = self.themes.guard();
        guard.clear();

        for theme in themes {
            let is_current = theme.theme_id == self.current_theme_id;
            guard.push_back((theme, is_current));
        }
        // Guard drops here, UI updates automatically
    }
}
```

### Cairo DrawingArea Preview Rendering

```rust
// Source: Pattern from GTK4 DrawingArea docs + codebase theme preview
use gtk::prelude::*;

pub fn setup_preview_drawing_area(area: &gtk::DrawingArea, theme: &Theme) {
    // Clone data needed for draw function to avoid lifetime issues
    let bg = theme.bg_primary.clone();
    let fg = theme.fg_primary.clone();
    let accent = theme.accent.clone();

    area.set_draw_func(move |_area, cr, width, height| {
        // Parse colors
        let bg_rgba = parse_hex_color(&bg).unwrap_or_default();
        let fg_rgba = parse_hex_color(&fg).unwrap_or_default();
        let accent_rgba = parse_hex_color(&accent).unwrap_or_default();

        // Draw mock desktop
        cr.set_source_rgba(bg_rgba.0, bg_rgba.1, bg_rgba.2, 1.0);
        cr.rectangle(0.0, 0.0, width as f64, height as f64);
        cr.fill().ok();

        // Draw mock window
        cr.set_source_rgba(fg_rgba.0, fg_rgba.1, fg_rgba.2, 0.1);
        cr.rectangle(20.0, 20.0, (width - 40) as f64, (height - 40) as f64);
        cr.fill().ok();

        // Draw accent elements
        cr.set_source_rgba(accent_rgba.0, accent_rgba.1, accent_rgba.2, 1.0);
        cr.rectangle(30.0, 30.0, 100.0, 30.0);
        cr.fill().ok();
    });
}

fn parse_hex_color(hex: &str) -> Option<(f64, f64, f64)> {
    if !hex.starts_with('#') || hex.len() != 7 {
        return None;
    }

    let r = u8::from_str_radix(&hex[1..3], 16).ok()? as f64 / 255.0;
    let g = u8::from_str_radix(&hex[3..5], 16).ok()? as f64 / 255.0;
    let b = u8::from_str_radix(&hex[5..7], 16).ok()? as f64 / 255.0;

    Some((r, g, b))
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| gtk::Notebook for tabs | adw::ViewStack + adw::ViewSwitcher | libadwaita 1.0 (2021) | Better GNOME HIG compliance, adaptive width, keyboard shortcuts |
| gtk::ColorButton | gtk::ColorDialogButton | GTK4 4.10 (2023) | Modern color picker dialog, better accessibility |
| Manual factory patterns | relm4::factory::FactoryVecDeque | Relm4 0.5 (2022) | RAII guards prevent missed updates, cleaner API |
| Separate wallpaper/theme apps | Unified appearance manager | This phase (2026) | Single window, shared profiles, cohesive UX |

**Deprecated/outdated:**
- **gtk::ColorButton**: Use ColorDialogButton instead (deprecated in GTK4)
- **adw::TabView for static views**: TabView is for dynamic tabs, ViewStack is for fixed application views
- **Manual child management of model-bound widgets**: FlowBox/ListView with models manage children automatically

## Open Questions

1. **Profile storage format evolution**
   - What we know: Current profiles store wallpaper assignments (HashMap<String, PathBuf>). Theme manager has separate theme profiles (TOML files).
   - What's unclear: Should Phase 7 unify profile formats immediately or wait for Phase 8 (theme-wallpaper binding)?
   - Recommendation: Keep separate for Phase 7. Unified profiles are Phase 8 scope. This phase just elevates profile manager UI to header bar.

2. **Theme editor color input widget choice**
   - What we know: gtk::ColorButton is deprecated, ColorDialogButton is modern but may be heavier for 50+ color fields
   - What's unclear: Is ColorDialogButton too heavyweight for theme editor with many color inputs? Should we use Entry with hex validation instead?
   - Recommendation: Start with Entry + hex validation (lighter, matches theme file format). Can upgrade to ColorDialogButton in later polish phase if UX testing shows users want visual picker.

3. **ViewStack page persistence**
   - What we know: ViewStack shows one page at a time
   - What's unclear: Does ViewStack destroy/recreate hidden pages or keep them in memory? Impact on preview state when switching tabs.
   - Recommendation: Test empirically. If pages persist (likely), preview state survives tab switches. If not, may need explicit state preservation.

## Sources

### Primary (HIGH confidence)

- [libadwaita ViewStack API](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.ViewStack.html) - Official API docs for ViewStack widget
- [libadwaita ViewStack vs TabView](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1.5/class.ViewStack.html) - GNOME docs explaining when to use ViewStack (fixed views) vs TabView (dynamic tabs)
- [Relm4 Child Components](https://relm4.org/book/stable/child_components.html) - Official guide to Controller pattern and message forwarding
- [Relm4 Factories](https://relm4.org/book/stable/efficient_ui/factory.html) - Official guide to FactoryVecDeque with RAII guards
- [Relm4 Factory Example](https://github.com/Relm4/Relm4/blob/main/examples/factory.rs) - Working code example of FactoryVecDeque with reordering
- [GTK4 Paned API](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.Paned.html) - API docs for split pane widget
- [GTK4 FlowBox API](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.FlowBox.html) - API docs noting "undefined to add/remove widgets directly while bound to model"
- Codebase files: vulcan-theme-manager/src/app.rs, vulcan-appearance-manager/src/app.rs, both Cargo.toml files (confirmed identical versions)

### Secondary (MEDIUM confidence)

- [GTK4 Memory Management](https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_memory_management.html) - General GTK memory patterns
- [GTK4 DrawingArea](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.DrawingArea.html) - API docs for custom drawing
- [GTK4 ColorButton deprecation](https://docs.gtk.org/gtk4/class.ColorButton.html) - Official GTK docs showing deprecation warning
- [libadwaita ExpanderRow](https://aaronerhardt.github.io/docs/relm4/libadwaita/struct.ExpanderRow.html) - API docs for collapsible rows

### Tertiary (LOW confidence)

- WebSearch results for "GTK4 memory leaks FlowBox" - Community reports of issues, not official docs

## Metadata

**Confidence breakdown:**
- Standard stack: **HIGH** - Both Cargo.toml files examined, versions confirmed identical, all libraries are mature stable releases
- Architecture: **HIGH** - Official Relm4 book + examples + existing working codebase using these exact patterns
- Pitfalls: **HIGH** - Derived from official API warnings (FlowBox model binding, ColorButton deprecation) and GTK4 memory docs

**Research date:** 2026-01-24
**Valid until:** 30 days (stable ecosystem, GTK4/libadwaita/Relm4 mature with infrequent breaking changes)
