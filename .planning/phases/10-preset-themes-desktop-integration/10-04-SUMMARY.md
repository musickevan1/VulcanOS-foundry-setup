---
phase: 10-preset-themes-desktop-integration
plan: 04
subsystem: appearance-manager-services
tags: [rust, app-discovery, theme-integration, detection]
requires:
  - "10-01-SUMMARY.md"  # Preset themes to pair with third-party apps
  - "10-02-SUMMARY.md"  # Additional theme variants for recommendations
dependencies:
  graph:
    requires: ["Phase 6 service architecture", "Phase 9 theming infrastructure"]
    provides: ["Third-party app discovery service", "Theme marketplace integration"]
    affects: ["10-05 Discovery UI components", "10-06 Theme recommendations"]
tech-stack:
  added:
    - which: "Binary detection (checks PATH for installed apps)"
    - open: "URL opening (marketplace/docs links)"
  patterns:
    - "Service module pattern (app_discovery.rs in services/)"
    - "Binary detection via which::which()"
    - "XDG config path resolution with HOME fallback"
    - "Configuration file parsing (init.lua, kitty.conf, etc.)"
key-files:
  created:
    - vulcan-appearance-manager/src/services/app_discovery.rs: "Third-party app theming discovery service"
  modified:
    - vulcan-appearance-manager/Cargo.toml: "Added which and open dependencies"
    - vulcan-appearance-manager/src/services/mod.rs: "Export app_discovery module"
decisions:
  - id: binary-detection-strategy
    choice: "Use which crate for PATH lookups"
    rationale: "Standard cross-platform binary detection, respects user's PATH"
    alternatives: ["Direct filesystem checks", "Package manager queries"]
  - id: config-detection-patterns
    choice: "Parse config files for theme indicators (colorscheme keywords, include statements)"
    rationale: "Accurate detection of actual theme configuration, not just config file presence"
    alternatives: ["Check config file existence only", "Query app APIs"]
  - id: app-coverage
    choice: "Focus on 6 common developer apps (Neovim, Kitty, Alacritty, btop, VS Code, Firefox)"
    rationale: "Covers terminal, editor, browser, system monitor - core VulcanOS use cases"
    alternatives: ["Support all possible apps", "Only support GTK apps"]
  - id: url-handling
    choice: "Non-blocking open::that() in separate thread"
    rationale: "Prevents UI freeze when opening URLs, graceful error handling"
    alternatives: ["xdg-open system call", "gtk::show_uri"]
metrics:
  duration: "2 min"
  completed: 2026-01-30
---

# Phase 10 Plan 04: Third-Party App Discovery Service

Third-party app detection service for theme marketplace integration

## One-liner

Rust service detecting 6 themeable apps (Neovim, Kitty, Alacritty, btop, VS Code, Firefox) with installation and configuration status for theme marketplace links.

## What was built

### App Discovery Service Module

**Location:** `vulcan-appearance-manager/src/services/app_discovery.rs`

**Core functionality:**
- `AppTheming` struct containing app name, installed status, configured status, docs URL, icon
- `discover_apps()` function returning detection results for all supported apps
- `open_url()` helper for non-blocking URL opening in default browser

**Supported applications:**

| App       | Binary     | Config Detection                                | Docs URL                                     |
| --------- | ---------- | ----------------------------------------------- | -------------------------------------------- |
| Neovim    | `nvim`     | colorscheme in init.lua/init.vim                | github.com/topics/neovim-colorscheme         |
| Kitty     | `kitty`    | current-theme.conf or theme include in kitty.conf | sw.kovidgoyal.net/kitty/kittens/themes       |
| Alacritty | `alacritty`| [colors] or import in alacritty.toml             | github.com/alacritty/alacritty-theme         |
| btop      | `btop`     | color_theme in btop.conf                        | github.com/aristocratos/btop#themes          |
| VS Code   | `code`*    | workbench.colorTheme in settings.json            | marketplace.visualstudio.com/.../Themes      |
| Firefox   | `firefox`  | chrome/userChrome.css in any profile             | github.com/nicoth-in/nicothin-firefox-theme |

*VS Code detection checks for `code`, `code-oss`, or `codium` binaries.

### Detection Logic

**Installation check:**
- Uses `which::which()` to check if binary exists in PATH
- Returns `true` if found, `false` otherwise

**Configuration check:**
- Resolves XDG_CONFIG_HOME or ~/.config
- Parses relevant config files for theme indicators
- Returns `true` if theme configuration detected, `false` otherwise

**Example detection flow (Neovim):**
```rust
1. Check if nvim binary exists → installed: bool
2. Look for ~/.config/nvim/init.lua or init.vim
3. Parse for "colorscheme" or "vim.cmd.colorscheme" keywords → configured: bool
4. Return AppTheming struct with status and docs URL
```

## Key Design Decisions

### Why these 6 apps?

**Coverage rationale:**
- **Terminals:** Kitty (primary), Alacritty (alternate) - most used in VulcanOS
- **Editor:** Neovim - modal editing, highly themeable
- **IDE:** VS Code - GUI development, extension marketplace
- **Browser:** Firefox - userChrome.css theming, privacy-focused
- **System:** btop - system monitoring, built-in themes

These apps represent core VulcanOS workflows (development, browsing, monitoring) and have active theming communities.

### Discovery-only approach

**Decision:** Provide URLs to theme marketplaces, don't implement theme installation
**Rationale:**
- Each app has its own plugin/theme management system
- Users already familiar with their app's theme installation method
- Theme formats are app-specific (Vim scripts, JSON, CSS, TOML)
- Marketplace integration requires API keys and authentication

**Service role:** Detection + guidance, not installation automation

### Non-blocking URL opening

**Implementation:** `open::that()` in spawned thread
**Rationale:**
- Prevents UI freeze during browser launch
- Graceful error handling with stderr logging
- Returns immediately, user sees instant response

## Tasks Completed

| Task | Commit  | Description                                   |
| ---- | ------- | --------------------------------------------- |
| 1    | 08ad43a | App discovery service with 6 app detectors    |

## Technical Implementation

### Dependencies Added

```toml
which = "6"      # Binary detection via PATH lookup
open = "5"       # URL opening in default browser
```

### Module Structure

```
services/
├── app_discovery.rs    # NEW: Third-party app detection
├── mod.rs              # Export app_discovery
└── (other services...)
```

### Public API

```rust
pub struct AppTheming {
    pub name: String,
    pub installed: bool,
    pub configured: bool,
    pub docs_url: String,
    pub icon: String,
}

pub fn discover_apps() -> Vec<AppTheming>
pub fn open_url(url: &str)
```

### Detection Patterns

**Binary detection:**
```rust
let installed = which::which("nvim").is_ok();
```

**Config path resolution:**
```rust
fn config_dir() -> Option<PathBuf> {
    std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .ok()
        .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
}
```

**Config parsing (Neovim example):**
```rust
let configured = config_dir()
    .map(|d| d.join("nvim"))
    .filter(|p| p.exists())
    .and_then(|nvim_dir| {
        let init_lua = nvim_dir.join("init.lua");
        if init_lua.exists() {
            std::fs::read_to_string(init_lua).ok()
                .map(|s| s.contains("colorscheme"))
        } else {
            Some(false)
        }
    })
    .unwrap_or(false);
```

## Verification

**Build check:**
```bash
$ cd vulcan-appearance-manager
$ cargo check
   Compiling vulcan-appearance-manager v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s)
```

**Module export:**
```bash
$ grep "pub mod app_discovery" src/services/mod.rs
pub mod app_discovery;
```

**Success criteria met:**
- ✅ App discovery service implemented with 6 app detectors
- ✅ Each detector checks installation and configuration status
- ✅ Each app has a marketplace/docs URL
- ✅ Service compiles and integrates with existing codebase

## Integration Points

**Service layer:**
- Part of `services/` module alongside theme_parser, wallpaper_backend, etc.
- No external API calls (all local detection)
- Synchronous operations (fast enough for UI responsiveness)

**Future UI integration (10-05):**
```rust
use vulcan_appearance_manager::services::app_discovery::{discover_apps, open_url};

let apps = discover_apps();
for app in apps {
    if app.installed && !app.configured {
        // Show "Configure theme" suggestion
        // Button opens app.docs_url via open_url()
    }
}
```

## Next Phase Readiness

**Enables:**
- Plan 10-05: Discovery UI components (theme tab section for third-party apps)
- Plan 10-06: Theme recommendation system (suggest themes based on installed apps)

**Ready for:**
- UI integration (AppTheming struct ready for GTK4/Relm4 components)
- Marketplace link buttons (open_url() helper ready)
- Status badges (installed/configured booleans for UI state)

**No blockers.** Service is self-contained and ready for UI layer integration.

## Deviations from Plan

None - plan executed exactly as written.

## What's next

**Plan 10-05:** Discovery UI components
- Create GTK4 widgets for app discovery section
- Display app cards with install/config status badges
- Add "View themes" buttons linking to marketplaces

**Plan 10-06:** Theme recommendations
- Suggest themes based on detected app configurations
- Show complementary themes for installed apps
- Integration with preset theme library (10-01)
