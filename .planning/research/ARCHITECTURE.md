# Architecture Patterns: VulcanOS Unified Appearance Manager

**Domain:** Desktop appearance management (themes + wallpapers)
**Researched:** 2026-01-24
**Confidence:** HIGH (based on existing codebase analysis)

## Executive Summary

The unified Appearance Manager merges two existing GTK4/Relm4 applications (vulcan-theme-manager and vulcan-wallpaper-manager) into a single cohesive interface while adding theme-wallpaper binding and shared CSS infrastructure. The architecture must preserve the clean separation of concerns already established while creating new integration points.

**Key architectural decisions:**
1. **Tab-based UI merging pattern** - Keep existing components, add tabbed navigation
2. **Shared services layer** - Extract common infrastructure (CSS generation, file watching)
3. **Binding service** - New service layer for theme-wallpaper coordination
4. **Plugin discovery system** - New service for finding third-party application configs

## Recommended Architecture

### High-Level Component Structure

```
vulcan-appearance-manager/
├── src/
│   ├── main.rs                    # App entry, shared CSS, tab orchestration
│   ├── app.rs                     # Main AppModel with TabView
│   ├── models/
│   │   ├── theme.rs               # From theme-manager (existing)
│   │   ├── color_group.rs         # From theme-manager (existing)
│   │   ├── monitor.rs             # From wallpaper-manager (existing)
│   │   ├── wallpaper.rs           # From wallpaper-manager (existing)
│   │   ├── profile.rs             # From wallpaper-manager (existing)
│   │   ├── binding.rs             # NEW: theme-wallpaper binding
│   │   └── mod.rs
│   ├── services/
│   │   ├── theme_parser.rs        # From theme-manager (existing)
│   │   ├── theme_storage.rs       # From theme-manager (existing)
│   │   ├── theme_applier.rs       # From theme-manager (MODIFIED)
│   │   ├── hyprctl.rs             # From wallpaper-manager (existing)
│   │   ├── hyprpaper.rs           # From wallpaper-manager (existing)
│   │   ├── profile_storage.rs     # From wallpaper-manager (existing)
│   │   ├── thumbnail.rs           # From wallpaper-manager (existing)
│   │   ├── image_splitter.rs      # From wallpaper-manager (existing)
│   │   ├── css_generator.rs       # NEW: shared CSS infrastructure
│   │   ├── binding_manager.rs     # NEW: theme-wallpaper coordination
│   │   ├── app_discovery.rs       # NEW: third-party app detection
│   │   └── mod.rs
│   ├── components/
│   │   ├── theme_tab/             # Theme management (existing components)
│   │   │   ├── theme_browser.rs   # From theme-manager
│   │   │   ├── theme_card.rs      # From theme-manager
│   │   │   ├── theme_editor.rs    # From theme-manager
│   │   │   ├── preview_panel.rs   # From theme-manager
│   │   │   └── mod.rs
│   │   ├── wallpaper_tab/         # Wallpaper management (existing components)
│   │   │   ├── monitor_layout.rs  # From wallpaper-manager
│   │   │   ├── wallpaper_picker.rs # From wallpaper-manager
│   │   │   ├── profile_manager.rs  # From wallpaper-manager
│   │   │   ├── split_dialog.rs     # From wallpaper-manager
│   │   │   └── mod.rs
│   │   ├── binding_tab/           # NEW: Theme-wallpaper binding UI
│   │   │   ├── binding_editor.rs  # Create/edit bindings
│   │   │   ├── binding_list.rs    # Display existing bindings
│   │   │   └── mod.rs
│   │   ├── apps_tab/              # NEW: Third-party app theming
│   │   │   ├── app_browser.rs     # List discovered apps
│   │   │   ├── app_config_editor.rs # Configure app theming
│   │   │   └── mod.rs
│   │   └── mod.rs
│   └── shared/                    # NEW: Shared utilities
│       ├── css.rs                 # CSS constant definitions
│       ├── config.rs              # Shared config paths
│       └── mod.rs
└── Cargo.toml
```

### Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|---------------|-------------------|
| **AppModel** | Tab orchestration, message routing | All tab components, binding_manager |
| **theme_tab/** | Theme browsing, editing, preview, apply | theme_storage, theme_applier, css_generator |
| **wallpaper_tab/** | Monitor layout, wallpaper selection, profiles | hyprctl, hyprpaper, profile_storage |
| **binding_tab/** | Theme-wallpaper binding creation/management | binding_manager, theme_storage, profile_storage |
| **apps_tab/** | Third-party app discovery, config generation | app_discovery, css_generator |
| **theme_applier** | Apply themes via vulcan-theme CLI | vulcan-theme script, binding_manager |
| **binding_manager** | Coordinate theme+wallpaper application | theme_applier, hyprpaper, profile_storage |
| **css_generator** | Generate CSS for third-party apps | theme_storage, app_discovery |
| **app_discovery** | Scan for supported third-party apps | File system (~/.config/) |

### Data Flow

#### Theme Application Flow (without binding)
```
User selects theme → theme_tab → AppModel → theme_applier → vulcan-theme CLI →
  → envsubst templates → Config files written → Services reloaded
```

#### Theme Application Flow (with binding)
```
User selects theme → theme_tab → AppModel → binding_manager checks bindings →
  ├─→ theme_applier applies theme
  └─→ hyprpaper applies bound wallpaper profile
```

#### Wallpaper Application Flow
```
User selects wallpaper → wallpaper_tab → AppModel → hyprpaper service →
  → swww command → Wallpaper displayed
```

#### Binding Creation Flow
```
User creates binding → binding_tab → binding_manager →
  → Binding saved to ~/.config/vulcan/bindings.toml
  → Binding activated (theme + wallpaper applied together)
```

#### Third-Party App Theming Flow
```
App discovery scans ~/.config/ → app_discovery identifies supported apps →
  → apps_tab displays list → User enables theming →
  → css_generator creates app-specific CSS from current theme →
  → CSS written to app config directory → App reloaded
```

## Integration Points

### 1. Existing Apps → Unified App

**Migration strategy:**
- **Move, don't rewrite** - Copy existing component files into new directory structure
- **Namespace with tabs** - Place existing components under `theme_tab/` and `wallpaper_tab/`
- **Preserve models** - Keep all existing model structs unchanged
- **Preserve services** - Keep existing service modules (only MODIFY theme_applier for binding hooks)

**File mapping:**
```
vulcan-theme-manager/src/components/*.rs → theme_tab/*.rs
vulcan-wallpaper-manager/src/components/*.rs → wallpaper_tab/*.rs
vulcan-theme-manager/src/models/*.rs → models/*.rs (merge, no conflicts)
vulcan-wallpaper-manager/src/models/*.rs → models/*.rs (merge, no conflicts)
vulcan-theme-manager/src/services/*.rs → services/*.rs
vulcan-wallpaper-manager/src/services/*.rs → services/*.rs
```

### 2. vulcan-theme CLI Integration

**Current integration:** theme_applier.rs calls `vulcan-theme set <id>` via Command::new()

**New integration points:**
```rust
// services/theme_applier.rs
pub fn apply_theme_with_binding(theme_id: &str) -> Result<()> {
    // 1. Check if theme has binding
    if let Some(binding) = binding_manager::get_binding_for_theme(theme_id)? {
        // 2. Apply theme
        apply_theme(theme_id)?;

        // 3. Apply bound wallpaper profile
        if let Some(profile) = binding.wallpaper_profile {
            profile_storage::apply_profile(&profile)?;
        }
    } else {
        // No binding, just apply theme
        apply_theme(theme_id)?;
    }
    Ok(())
}
```

**vulcan-theme remains unchanged** - No modifications to bash script needed. GUI app calls it as external command.

### 3. Shared CSS Infrastructure

**Problem:** Both apps duplicate CSS in main.rs. Third-party apps need generated CSS.

**Solution:** Extract to shared module
```rust
// src/shared/css.rs
pub const VULCAN_BRAND_CSS: &str = include_str!("../../assets/vulcan-brand.css");

pub fn load_brand_css() {
    relm4::set_global_css(VULCAN_BRAND_CSS);
}

// src/services/css_generator.rs
pub fn generate_css_for_app(app: &SupportedApp, theme: &Theme) -> Result<String> {
    match app {
        SupportedApp::Vscode => generate_vscode_css(theme),
        SupportedApp::Firefox => generate_firefox_css(theme),
        SupportedApp::Thunderbird => generate_thunderbird_css(theme),
        // ... discovered apps
    }
}

fn generate_vscode_css(theme: &Theme) -> Result<String> {
    // Generate VS Code settings.json with theme colors
    let json = serde_json::json!({
        "workbench.colorCustomizations": {
            "editor.background": theme.bg_primary,
            "editor.foreground": theme.fg_primary,
            "activityBar.background": theme.bg_secondary,
            // ... full color mapping
        }
    });
    Ok(serde_json::to_string_pretty(&json)?)
}
```

### 4. Theme-Wallpaper Binding Data Model

**New model:**
```rust
// src/models/binding.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeWallpaperBinding {
    pub theme_id: String,
    pub wallpaper_profile: Option<String>, // Profile name
    pub auto_apply: bool, // Auto-apply wallpaper when theme changes
}

impl ThemeWallpaperBinding {
    pub fn save(&self) -> Result<()> {
        binding_manager::save_binding(self)
    }

    pub fn load(theme_id: &str) -> Result<Option<Self>> {
        binding_manager::load_binding(theme_id)
    }
}
```

**Storage format:** TOML in `~/.config/vulcan/bindings.toml`
```toml
[vulcan-forge]
wallpaper_profile = "forge-flames"
auto_apply = true

[catppuccin-mocha]
wallpaper_profile = "mocha-mountains"
auto_apply = true
```

### 5. Third-Party App Discovery

**Discovery service:**
```rust
// src/services/app_discovery.rs
pub enum SupportedApp {
    Vscode,
    Firefox,
    Thunderbird,
    Slack,
    Discord,
    Spotify,
    // ... extensible enum
}

impl SupportedApp {
    pub fn config_path(&self) -> PathBuf {
        match self {
            Self::Vscode => home_dir().join(".config/Code/User/settings.json"),
            Self::Firefox => home_dir().join(".mozilla/firefox/*/chrome/userChrome.css"),
            Self::Thunderbird => home_dir().join(".thunderbird/*/chrome/userChrome.css"),
            // ...
        }
    }

    pub fn is_installed(&self) -> bool {
        self.config_path().exists() || self.binary_exists()
    }

    pub fn supports_css_theming(&self) -> bool {
        true // All supported apps must support CSS theming
    }
}

pub fn discover_installed_apps() -> Vec<SupportedApp> {
    SupportedApp::all()
        .into_iter()
        .filter(|app| app.is_installed())
        .collect()
}
```

**App theming state:** Stored in `~/.config/vulcan/app-theming.toml`
```toml
[apps]
vscode = { enabled = true, last_applied = "vulcan-forge" }
firefox = { enabled = true, last_applied = "vulcan-forge" }
thunderbird = { enabled = false }
```

## Patterns to Follow

### Pattern 1: Tab-Based App Merging
**What:** Use libadwaita TabView to combine two apps into one window
**When:** Merging related functionality without complete rewrite
**Example:**
```rust
// src/app.rs
pub struct App {
    theme_tab: Controller<ThemeTabModel>,
    wallpaper_tab: Controller<WallpaperTabModel>,
    binding_tab: Controller<BindingTabModel>,
    apps_tab: Controller<AppsTabModel>,
}

view! {
    adw::ApplicationWindow {
        adw::TabView {
            append = &adw::TabPage {
                set_title: "Themes",
                set_child: model.theme_tab.widget(),
            },
            append = &adw::TabPage {
                set_title: "Wallpapers",
                set_child: model.wallpaper_tab.widget(),
            },
            append = &adw::TabPage {
                set_title: "Bindings",
                set_child: model.binding_tab.widget(),
            },
            append = &adw::TabPage {
                set_title: "Applications",
                set_child: model.apps_tab.widget(),
            },
        }
    }
}
```

### Pattern 2: Service Message Routing
**What:** Central app model routes messages to appropriate service layer
**When:** Multiple tabs need to coordinate through shared services
**Example:**
```rust
// src/app.rs
fn update(&mut self, msg: AppMsg, sender: ComponentSender<Self>) {
    match msg {
        AppMsg::ApplyTheme(theme_id) => {
            // Check for binding before applying
            if let Ok(binding) = binding_manager::get_binding(&theme_id) {
                // Apply theme + wallpaper together
                theme_applier::apply_theme(&theme_id)?;
                if let Some(profile) = binding.wallpaper_profile {
                    profile_storage::apply_profile(&profile)?;
                    self.wallpaper_tab.emit(WallpaperTabMsg::ProfileApplied(profile));
                }
            } else {
                // Just apply theme
                theme_applier::apply_theme(&theme_id)?;
            }
        }
        // ... route to appropriate tab
    }
}
```

### Pattern 3: Shared CSS Generation
**What:** Generate application-specific CSS/config from theme model
**When:** Propagating theme to third-party applications
**Example:**
```rust
// src/services/css_generator.rs
pub struct CssTemplate {
    template: String,
    output_path: PathBuf,
}

impl CssTemplate {
    pub fn apply(&self, theme: &Theme) -> Result<()> {
        let css = self.template
            .replace("{{bg_primary}}", &theme.bg_primary)
            .replace("{{fg_primary}}", &theme.fg_primary)
            .replace("{{accent}}", &theme.accent);
            // ... full substitution

        fs::write(&self.output_path, css)?;
        Ok(())
    }
}
```

### Pattern 4: CLI Tool Delegation
**What:** Delegate complex theme application to existing bash script
**When:** Bash script handles envsubst templates and service reloading
**Why:** Don't duplicate bash logic in Rust - call existing tooling
**Example:**
```rust
// services/theme_applier.rs
pub fn apply_theme(theme_id: &str) -> Result<()> {
    let vulcan_theme = find_vulcan_theme()?;
    Command::new(&vulcan_theme)
        .arg("set")
        .arg(theme_id)
        .status()?;
    Ok(())
}
```

## Anti-Patterns to Avoid

### Anti-Pattern 1: Complete Rewrite
**What:** Rewriting existing components from scratch
**Why bad:** Wastes working code, introduces new bugs, delays delivery
**Instead:** Move existing component files into new directory structure, add namespacing

### Anti-Pattern 2: Duplicating vulcan-theme Logic
**What:** Reimplementing envsubst template processing in Rust
**Why bad:**
- Duplicates 490 lines of working bash code
- Must maintain two implementations
- Risk of divergence between CLI and GUI behavior
**Instead:** Call vulcan-theme as subprocess, let it handle templating

### Anti-Pattern 3: Tight Coupling Between Tabs
**What:** Theme tab directly calling wallpaper tab methods
**Why bad:** Creates circular dependencies, hard to test, brittle
**Instead:** Route all cross-tab communication through AppModel, use binding_manager service

### Anti-Pattern 4: Hardcoded App List
**What:** Match statement with fixed list of supported apps
**Why bad:** Requires code change to add new app support
**Instead:**
```rust
// GOOD: Extensible discovery
pub fn discover_apps() -> Vec<DiscoveredApp> {
    scan_config_dir()
        .filter(|app| has_css_support(app))
        .collect()
}

// BAD: Hardcoded
pub fn supported_apps() -> Vec<&'static str> {
    vec!["vscode", "firefox", "thunderbird"] // Can't extend without code change
}
```

### Anti-Pattern 5: Inline CSS in Components
**What:** Hardcoding CSS strings in component view! macros
**Why bad:** Duplicated across components, hard to theme, inconsistent
**Instead:** Use shared CSS module, load once in main.rs

## Build Order Recommendations

**Phase 1: Foundation** (Week 1)
1. Create new vulcan-appearance-manager crate
2. Copy all models/ from both apps (no conflicts, merge mod.rs)
3. Copy all services/ from both apps (merge mod.rs)
4. Add shared/css.rs module
5. Verify cargo build succeeds

**Phase 2: Tab Integration** (Week 1-2)
1. Create app.rs with TabView skeleton
2. Copy theme_tab/ components (rename imports)
3. Copy wallpaper_tab/ components (rename imports)
4. Wire up existing tabs to AppModel
5. Test existing functionality works in tabs

**Phase 3: Binding System** (Week 2)
1. Implement models/binding.rs
2. Implement services/binding_manager.rs
3. Add binding hooks to theme_applier.rs
4. Create binding_tab/ components
5. Test theme+wallpaper binding

**Phase 4: App Discovery** (Week 3)
1. Implement services/app_discovery.rs
2. Implement services/css_generator.rs (start with VS Code)
3. Create apps_tab/ components
4. Test VS Code theming
5. Add more apps incrementally

**Parallel track:** Update dotfiles/archiso throughout to include new binary

## Deployment Strategy

**Binary replacement:**
```bash
# Old
vulcan-theme-manager
vulcan-wallpaper-manager

# New (replaces both)
vulcan-appearance-manager
```

**Desktop file:**
```ini
[Desktop Entry]
Name=Appearance Manager
Comment=Manage themes, wallpapers, and application appearance
Exec=vulcan-appearance-manager
Icon=preferences-desktop-theme
Categories=Settings;DesktopSettings;GTK;
```

**Menu integration:** Update vulcan-menu to launch unified app instead of separate apps

## Performance Considerations

| Concern | Solution |
|---------|----------|
| **Tab switching lag** | Lazy-load wallpaper thumbnails only when wallpaper tab visible |
| **Theme preview slowness** | Keep vulcan-theme CLI fast (already optimized with envsubst) |
| **App discovery scan** | Cache discovered apps, only re-scan on demand |
| **CSS generation** | Generate on-demand when user enables app, not on every theme change |
| **Multiple wallpaper preloads** | Use swww (already does this efficiently) |

## Configuration Files

**New configuration locations:**
```
~/.config/vulcan/
├── current-theme              # Existing (from vulcan-theme)
├── bindings.toml              # NEW: theme-wallpaper bindings
├── app-theming.toml           # NEW: third-party app enablement
└── appearance-manager.toml    # NEW: GUI app settings (window size, etc.)

~/.config/vulcan-wallpaper-manager/
└── profiles/                  # Existing (from wallpaper-manager)
    └── *.toml

~/.config/themes/              # Existing (from vulcan-theme)
├── colors/
│   └── *.sh
└── templates/
    └── *.tpl
```

## Testing Strategy

**Component isolation:**
- Theme tab tests: Can apply themes without wallpaper tab
- Wallpaper tab tests: Can apply wallpapers without theme tab
- Binding tests: Mock both theme_applier and hyprpaper services

**Integration tests:**
- Binding application: Theme change triggers wallpaper change
- Profile coordination: Wallpaper profile respects theme binding
- CLI coordination: GUI changes reflected in CLI tool output

**Manual testing checklist:**
- [ ] Theme tab works identically to old app
- [ ] Wallpaper tab works identically to old app
- [ ] Theme binding creates wallpaper change
- [ ] Wallpaper profile manual change doesn't break binding
- [ ] Third-party app CSS generates correctly
- [ ] vulcan-theme CLI still works independently

## Migration Path

**For users:**
1. Update system: `yay -S vulcan-appearance-manager`
2. Old apps automatically replaced by new unified app
3. Existing themes and wallpaper profiles preserved (same config locations)
4. vulcan-theme CLI continues to work (unchanged)

**For developers:**
1. Archive old repos: vulcan-theme-manager, vulcan-wallpaper-manager
2. All future development in vulcan-appearance-manager
3. Keep vulcan-theme bash script independent (still used by both GUI and CLI users)

## Sources

- [GitHub - Relm4/Relm4: Build truly native applications with ease!](https://github.com/Relm4/Relm4)
- [Introduction - GUI development with Relm4](https://relm4.org/book/stable/)
- [Themeing and GTK4 · linuxmint · Discussion #182](https://github.com/orgs/linuxmint/discussions/182)
- [Gradience - Change Colors & Custom CSS to GTK4 + LibAwaita Apps](https://fostips.com/customize-gtk4-app-window-colors/)
- Existing VulcanOS codebase analysis (vulcan-theme-manager, vulcan-wallpaper-manager)
