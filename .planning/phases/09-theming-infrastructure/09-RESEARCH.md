# Phase 9: Theming Infrastructure - Research

**Researched:** 2026-01-25
**Domain:** Theme propagation systems, GTK4 self-theming, component reload mechanisms
**Confidence:** HIGH

## Summary

Phase 9 implements automatic theme propagation to all desktop components and self-theming for the Appearance Manager GUI. Research reveals that **most of the heavy lifting is already done** — the `vulcan-theme` CLI script already handles template processing and component reloading for waybar, wofi, swaync, hyprlock, kitty, and alacritty.

The primary work for this phase is:
1. **Verifying and fixing the existing propagation chain** — ensuring vulcan-theme correctly processes all templates and reloads components
2. **Implementing self-theming for the Appearance Manager GUI** — making the app reflect the active theme colors using GTK4 CssProvider

The standard approach is well-established: bash envsubst for template processing, component-specific IPC/signals for hot-reloading, and GTK4 CssProvider for runtime CSS updates.

**Primary recommendation:** Use GTK4 CssProvider to load theme-generated CSS at runtime, updating the @define-color variables that the app's brand_css.rs module references. This creates a clean dependency: brand_css.rs defines the default "Vulcan Forge" look, while runtime CSS can override those colors when a different theme is active.

## Standard Stack

### Core

| Library/Tool | Version | Purpose | Why Standard |
|--------------|---------|---------|--------------|
| envsubst | GNU gettext | Template variable substitution | Industry standard for shell-style variable expansion, handles ${VAR} and $VAR patterns |
| GTK4 CssProvider | 4.x | Runtime CSS loading | Official GTK4 mechanism for dynamic styling, built-in and well-documented |
| std::process::Command | Rust std | Process execution | Rust standard library, no external dependencies needed |
| anyhow | Latest | Error handling | Already used project-wide (from STATE.md decisions) |

### Supporting

| Library/Tool | Version | Purpose | When to Use |
|--------------|---------|---------|-------------|
| relm4::set_global_css | 0.10+ | Initial CSS loading | App startup only (not for runtime updates) |
| hyprctl | Hyprland | Compositor reload | When Hyprland config changes (already in vulcan-theme) |
| pkill | coreutils | Signal-based reload | For waybar SIGUSR2 reload |
| swaync-client | swaync | Notification daemon IPC | For swaync style reload (-R, -rs flags) |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| envsubst | Custom template engine | envsubst is simpler, battle-tested, and handles exactly our use case (no need for logic/conditionals) |
| CssProvider | Recompiling brand_css.rs | CssProvider allows runtime changes without rebuild; recompiling requires app restart |
| Component-specific IPC | Kill and restart all components | Hot-reload preserves state and is faster; full restart loses window positions, history, etc. |

**Installation:**

All tools already available:
- envsubst: Part of GNU gettext (already used in vulcan-theme script)
- GTK4: Already in use (vulcan-appearance-manager uses gtk4-rs)
- Standard tools: pkill, hyprctl, swaync-client already in dotfiles scripts

## Architecture Patterns

### Recommended Project Structure

Already exists:
```
dotfiles/
├── themes/
│   ├── colors/              # Theme color definitions (.sh files)
│   │   ├── vulcan-forge.sh
│   │   ├── catppuccin-mocha.sh
│   │   └── ...
│   └── templates/           # Component templates (.tpl files)
│       ├── waybar-style.css.tpl
│       ├── swaync-style.css.tpl
│       ├── hyprlock.conf.tpl
│       ├── kitty.conf.tpl
│       ├── alacritty.toml.tpl
│       └── wofi-style.css.tpl
├── scripts/.local/bin/
│   └── vulcan-theme         # Main theme application script
vulcan-appearance-manager/
└── src/
    ├── brand_css.rs         # Default Vulcan Forge colors
    ├── services/
    │   └── theme_applier.rs # Calls vulcan-theme CLI
    └── main.rs              # App entry point
```

### Pattern 1: Template Processing with envsubst

**What:** Use bash envsubst to substitute theme variables in template files
**When to use:** Processing any config file that needs theme colors injected
**Example:**

```bash
# From vulcan-theme script (lines 160-182)
process_template() {
    local template="$1"
    local output="$2"

    mkdir -p "$(dirname "${output}")"

    # Only substitute theme-specific variables
    local vars='$THEME_NAME $THEME_ID $BG_PRIMARY $BG_SECONDARY'
    vars+=' $FG_PRIMARY $ACCENT $RED $GREEN $YELLOW $BLUE'
    # ... (full list in script)

    envsubst "${vars}" < "${template}" > "${output}"
}
```

**Key points:**
- Explicitly list variables to substitute (prevents accidental substitution of component-specific variables like starship's `$path`)
- Template files use `${VARIABLE}` syntax for clarity
- RAW variants (e.g., `$ACCENT_RAW`) strip the `#` prefix for Hyprland rgba() format

### Pattern 2: Component Hot-Reload

**What:** Use component-specific IPC mechanisms to reload configuration without restart
**When to use:** After updating component config files
**Example:**

```bash
# Waybar: SIGUSR2 signal reloads style
pkill -SIGUSR2 waybar

# SwayNC: IPC client reload
swaync-client -R    # Reload config
swaync-client -rs   # Reload style

# Hyprland: IPC reload
hyprctl reload
```

**Key points:**
- Waybar: SIGUSR1 exists but SIGUSR2 is for style reload specifically
- SwayNC: Separate flags for config vs style reload
- Hyprland: Auto-reloads on file save, but explicit reload ensures immediate effect
- Kitty/Alacritty: Config file rewrite is sufficient (they watch for changes)

### Pattern 3: GTK4 Runtime CSS Loading

**What:** Use CssProvider to load CSS at runtime and update theme colors
**When to use:** Implementing self-theming in GTK4 applications
**Example:**

```rust
// Source: https://gtk-rs.org/gtk4-rs/stable/latest/book/css.html
use gtk::prelude::*;
use gtk::{CssProvider, gdk::Display};

fn load_theme_css(css_content: &str) {
    let provider = CssProvider::new();
    provider.load_from_string(css_content);

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
```

**Key points:**
- `load_from_string()` replaces any previously loaded CSS from that provider
- Use `STYLE_PROVIDER_PRIORITY_APPLICATION` (400) for app-level theming
- Call this function whenever theme changes to update colors immediately
- The CSS uses `@define-color` syntax (GTK4) not CSS custom properties

### Pattern 4: Delegating to Existing CLI

**What:** Call vulcan-theme CLI from Rust instead of reimplementing logic
**When to use:** When a battle-tested script already exists
**Example:**

```rust
// From theme_applier.rs (lines 99-113)
pub fn apply_theme(theme_id: &str) -> Result<()> {
    let vulcan_theme = find_vulcan_theme()?;

    let status = Command::new(&vulcan_theme)
        .arg("set")
        .arg(theme_id)
        .status()
        .context("Failed to run vulcan-theme set")?;

    if !status.success() {
        anyhow::bail!("Failed to apply theme: {}", theme_id);
    }

    Ok(())
}
```

**Key points:**
- Don't duplicate logic — delegate to proven script
- Use `.status()` when you don't need stdout/stderr
- Check exit status explicitly (Command doesn't auto-fail on non-zero)
- Provide context with anyhow for error messages

### Anti-Patterns to Avoid

- **Don't restart components unnecessarily:** Use hot-reload mechanisms instead of `pkill -9` + restart (loses state, slower)
- **Don't substitute all environment variables:** Specify exact variables to envsubst to avoid breaking component configs (e.g., starship uses `$path`)
- **Don't ignore exit codes:** Command::status() doesn't propagate errors — check `.success()` explicitly
- **Don't reload before write completes:** Add small delays (`sleep 0.2`) after config writes to ensure filesystem flush

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Template variable substitution | Custom regex replacer | envsubst | Handles edge cases (escaped dollars, variable scoping, missing vars), GNU-tested |
| Color format conversion | Manual string manipulation | Export both `$ACCENT` and `$ACCENT_RAW` in theme file | Theme files already export RAW variants for Hyprland, no need to parse/convert |
| Component reload ordering | Complex dependency graph | Simple sequential reload | Components are independent; waybar doesn't care if swaync reloads first |
| Theme color extraction | Parse .sh files in Rust | Source the file in bash subshell | Bash can source its own files safely; parsing bash in Rust is error-prone |
| CSS string building | Template literals in Rust | Load from string with envsubst | CSS is already templated; just substitute variables |

**Key insight:** Theme propagation is a "glue" problem, not a "logic" problem. The vulcan-theme script already does the gluing — our job is to call it correctly and handle the self-theming edge case.

## Common Pitfalls

### Pitfall 1: Relm4 set_global_css Cannot Be Called Twice

**What goes wrong:** Calling `relm4::set_global_css()` more than once panics at runtime
**Why it happens:** Relm4 enforces single initialization — it's designed for app startup, not runtime updates
**How to avoid:** Use GTK4 CssProvider directly for runtime CSS updates, bypassing relm4's wrapper
**Warning signs:** Panic message: "set_global_css can only be called once" or similar

**Solution:**
```rust
// WRONG: This panics on second call
relm4::set_global_css(new_theme_css);

// RIGHT: Use CssProvider directly
let provider = CssProvider::new();
provider.load_from_string(new_theme_css);
gtk::style_context_add_provider_for_display(
    &Display::default().unwrap(),
    &provider,
    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
);
```

### Pitfall 2: Component Reload Race Conditions

**What goes wrong:** Config file updated, reload triggered immediately, component reads old file
**Why it happens:** Filesystem writes aren't always synchronous; component reads file before flush completes
**How to avoid:** Add small delays (0.2-0.5s) after config writes before triggering reload
**Warning signs:** Component still shows old theme colors despite successful apply

**Solution:**
```bash
# From vulcan-theme (lines 338-342)
pkill -9 waybar 2>/dev/null || true
sleep 0.5  # Wait for process cleanup
hyprctl dispatch exec waybar &> /dev/null
sleep 0.3  # Wait for startup
```

### Pitfall 3: GTK @define-color vs CSS Custom Properties

**What goes wrong:** Using CSS custom properties (`--accent-color: #f97316`) doesn't work in GTK4
**Why it happens:** GTK4's CSS parser uses `@define-color` syntax, not CSS standard custom properties
**How to avoid:** Use `@define-color accent_color #f97316;` format (underscores, no hyphens in variable names)
**Warning signs:** Colors not applying; GTK CSS parsing errors in console

**Solution:**
```css
/* WRONG: CSS custom properties don't work in GTK4 */
:root {
    --accent-color: #f97316;
}
button { background: var(--accent-color); }

/* RIGHT: GTK @define-color syntax */
@define-color accent_color #f97316;
button { background-color: @accent_color; }
```

### Pitfall 4: Command Exit Status Not Checked

**What goes wrong:** `Command::status()` returns Ok even if command fails; error goes undetected
**Why it happens:** Rust Command doesn't automatically propagate non-zero exit codes as errors
**How to avoid:** Always check `.status()?.success()` and bail if false
**Warning signs:** Silent failures; theme says "applied" but components unchanged

**Solution:**
```rust
// WRONG: This succeeds even if command fails
Command::new("vulcan-theme").arg("set").arg(theme_id).status()?;

// RIGHT: Check exit code explicitly
let status = Command::new("vulcan-theme")
    .arg("set")
    .arg(theme_id)
    .status()?;
if !status.success() {
    anyhow::bail!("Theme application failed");
}
```

## Code Examples

Verified patterns from official sources:

### Reading Current Theme from vulcan-theme CLI

```rust
// From theme_applier.rs (lines 56-79)
pub fn get_current_theme() -> Result<String> {
    let vulcan_theme = find_vulcan_theme()?;

    let output = Command::new(&vulcan_theme)
        .arg("current")
        .output()
        .context("Failed to run vulcan-theme current")?;

    if !output.status.success() {
        anyhow::bail!("vulcan-theme current failed");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse "Current theme: Name (id)" format
    if let Some(start) = stdout.rfind('(') {
        if let Some(end) = stdout.rfind(')') {
            return Ok(stdout[start + 1..end].to_string());
        }
    }

    // Fallback: return trimmed output
    Ok(stdout.trim().to_string())
}
```

### Processing Template with envsubst

```bash
# From vulcan-theme (lines 160-182)
process_template() {
    local template="$1"
    local output="$2"

    mkdir -p "$(dirname "${output}")"

    # Only substitute theme-specific variables
    local vars='$THEME_NAME $THEME_ID $BG_PRIMARY $BG_SECONDARY $BG_TERTIARY'
    vars+=' $FG_PRIMARY $FG_SECONDARY $FG_MUTED $ACCENT $ACCENT_ALT'
    vars+=' $RED $GREEN $YELLOW $BLUE $PURPLE $CYAN $ORANGE $PINK'
    vars+=' $BORDER_ACTIVE $BORDER_INACTIVE $SELECTION $CURSOR'
    vars+=' $GRADIENT_START $GRADIENT_END'
    vars+=' $GRADIENT_START_RAW $GRADIENT_END_RAW $ACCENT_RAW'

    envsubst "${vars}" < "${template}" > "${output}"
}
```

### Loading GTK4 CSS at Runtime

```rust
// Source: https://gtk-rs.org/gtk4-rs/stable/latest/book/css.html
use gtk::prelude::*;
use gtk::{CssProvider, gdk::Display};

fn apply_theme_css(theme_css: &str) -> Result<()> {
    let provider = CssProvider::new();
    provider.load_from_string(theme_css);

    let display = Display::default()
        .context("Could not connect to display")?;

    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    Ok(())
}
```

### Component Reload Sequence

```bash
# From vulcan-theme (lines 331-348)
print_info "Reloading services..."

# Reload Hyprland
if command -v hyprctl &> /dev/null && pgrep -x Hyprland &> /dev/null; then
    timeout 2 hyprctl reload &> /dev/null || true
fi

# Restart Waybar (use hyprctl to launch in user session context)
pkill -9 waybar 2>/dev/null || true
sleep 0.5
hyprctl dispatch exec waybar &> /dev/null
sleep 0.3

# Reload SwayNC
if command -v swaync-client &> /dev/null && pgrep -x swaync &> /dev/null; then
    timeout 2 swaync-client -R &> /dev/null || true
    timeout 2 swaync-client -rs &> /dev/null || true
fi
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual theme file editing | GUI theme manager with live preview | Phase 7 (v2.0) | Users can see colors before applying |
| Separate theme/wallpaper apps | Unified Appearance Manager | Phase 6-7 (v2.0) | Single source of truth for appearance |
| Hard-coded component configs | Template-based theming | Pre-v1.0 (2024) | Themes propagate to all components |
| Config file rewrite + restart | Hot-reload via IPC/signals | Current (2026) | No lost state, faster theme switching |
| CSS custom properties (web) | GTK @define-color | GTK4 migration | GTK4 uses its own CSS dialect |

**Deprecated/outdated:**
- **hyprpaper for wallpapers**: Replaced by swww (better multi-monitor support, animations)
- **Manual Waybar restart**: Now use `pkill -SIGUSR2 waybar` for style-only reload (but vulcan-theme uses full restart for reliability)
- **Alacritty YAML config**: Migrated to TOML format (alacritty.toml)

## Open Questions

Things that couldn't be fully resolved:

1. **Self-theming CSS generation strategy**
   - What we know: Need to generate CSS with @define-color overrides matching current theme
   - What's unclear: Where to generate it (Rust or bash), when to regenerate (on theme change only or on startup too)
   - Recommendation: Generate in bash (already has theme color access), write to `~/.config/vulcan/current-theme.css`, load in Rust on startup and theme change

2. **Component reload error handling**
   - What we know: vulcan-theme uses `|| true` to ignore errors, some reloads have timeouts
   - What's unclear: Should we surface errors to user or silently continue (current behavior)
   - Recommendation: Keep silent for now (per CONTEXT.md: "Trust the propagation works; only surface errors if something fails"). Future phase could add error detection.

3. **CssProvider instance lifecycle**
   - What we know: Creating new CssProvider for each theme change
   - What's unclear: Does GTK garbage-collect old providers or should we cache one instance
   - Recommendation: Create new provider each time (simpler, GTK handles cleanup, no measurable performance impact)

## Sources

### Primary (HIGH confidence)

- [GTK4 CssProvider Documentation](https://docs.gtk.org/gtk4/class.CssProvider.html) - Official GTK4 CssProvider API
- [GTK4 Rust bindings CSS guide](https://gtk-rs.org/gtk4-rs/stable/latest/book/css.html) - Official gtk4-rs CSS examples
- [GTK4 style_context_add_provider_for_display](https://docs.gtk.org/gtk4/type_func.StyleContext.add_provider_for_display.html) - Runtime CSS loading API
- [Relm4 set_global_css](https://relm4.org/docs/stable/relm4/fn.set_global_css.html) - Relm4 CSS initialization
- [GNU envsubst documentation](https://www.gnu.org/software/gettext/manual/html_node/envsubst-Invocation.html) - Official envsubst manual
- [Kitty remote control (set-colors)](https://sw.kovidgoyal.net/kitty/kittens/themes/) - Kitty theme kitten documentation
- [Alacritty live config reload](https://alacritty.org/config-alacritty.html) - Official Alacritty config documentation
- [Waybar man page](https://man.archlinux.org/man/waybar.5.en) - Waybar configuration reference
- [Rust std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html) - Rust command execution API

### Secondary (MEDIUM confidence)

- [Linux Handbook envsubst guide](https://linuxhandbook.com/envsubst-command/) - Practical envsubst examples
- [Baeldung envsubst tutorial](https://www.baeldung.com/linux/envsubst-command) - Template processing patterns
- [Hyprland Status Bars Wiki](https://wiki.hypr.land/Useful-Utilities/Status-Bars/) - Waybar reload mechanisms
- [Rust Command error handling discussion](https://users.rust-lang.org/t/best-error-handing-practices-when-using-std-command/42259) - Community best practices

### Tertiary (LOW confidence)

- [Gradience GTK4 theming tool](https://fostips.com/customize-gtk4-app-window-colors/) - Third-party GTK4 theming approach
- [GTK4 theming discussions](https://discuss.kde.org/t/simple-hack-to-tinting-theming-libadwaita-gtk4-apps-in-kde-plasma/29444) - Community workarounds

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All tools documented, in active use, official APIs
- Architecture patterns: HIGH - Verified from existing codebase (vulcan-theme script, theme_applier.rs)
- Pitfalls: HIGH - Derived from GTK4/Rust documentation and existing code patterns
- Self-theming implementation: MEDIUM - Multiple valid approaches, needs design decision

**Research date:** 2026-01-25
**Valid until:** 30 days (stable technologies, unlikely to change)

**Key finding:** Most infrastructure already exists. Phase 9 is verification + one new feature (self-theming), not building from scratch.
