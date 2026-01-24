# Domain Pitfalls: Unified Appearance Management

**Domain:** Merging theme-manager + wallpaper-manager into unified appearance system
**Researched:** 2026-01-24
**Context:** VulcanOS milestone to consolidate vulcan-theme-manager and vulcan-wallpaper-manager

## Executive Summary

This research identifies pitfalls specific to:
1. Merging two existing GTK4/Relm4 applications with overlapping state
2. Bridging shell-script theme system with GUI management
3. Coordinating wallpaper-theme bindings across multiple backends
4. Maintaining backward compatibility with existing CLI tools

**Critical finding:** The most dangerous pitfall is **state synchronization drift** between the GUI app, CLI tool (vulcan-theme), and live system configuration. The current architecture has THREE sources of truth that can diverge.

---

## Critical Pitfalls

Mistakes that cause rewrites or major issues.

### Pitfall 1: State Synchronization Drift (Three Sources of Truth)

**What goes wrong:**
The system currently has three potential sources of truth:
1. **vulcan-theme CLI state** - tracks "current theme" via internal state
2. **GUI application state** - each app tracks selected_theme/selected_wallpaper
3. **Live system state** - actual applied configs (swww query, GTK settings, etc.)

When merging apps, these can diverge catastrophically:
- User applies theme via CLI → GUI shows wrong theme as current
- User previews in GUI → CLI thinks different theme is active
- System state changes externally → both GUI and CLI unaware
- App restarts → loses preview state, applies wrong theme

**Why it happens:**
- Each component was designed independently with its own state tracking
- No shared state store or event bus
- Preview/apply distinction creates temporary state that's not persisted
- Relm4 component lifecycle doesn't guarantee cleanup on app crash

**Consequences:**
- User applies Theme A via GUI, later runs `vulcan-theme current` → says Theme B
- Preview gets stuck: user cancels preview but system stays in preview state
- Wallpaper profile applies but theme manager shows different theme
- System restore after logout applies wrong theme+wallpaper combination

**Prevention:**
1. **Single source of truth:** Make live system state the authority
   - Query actual state on app startup (swww query, vulcan-theme current)
   - Don't cache "current theme" in app state - query when needed
   - Use CLI as write-through layer, not separate state store

2. **Explicit state transitions:**
   ```rust
   enum AppearanceState {
       Idle(CurrentConfig),           // Normal state, config applied
       Previewing(PreviewConfig, Original), // Temporary preview, revertible
       Applying(NewConfig),           // In-flight transition
   }
   ```

3. **Stateless preview implementation:**
   - Preview should be `apply(new) + store(original)` not `set_preview_flag()`
   - Cancel preview = `apply(original)` not `clear_preview_flag()`
   - This prevents stuck preview state

4. **Startup state reconciliation:**
   ```rust
   fn init() -> AppState {
       let cli_theme = query_vulcan_theme_current()?;
       let live_wallpapers = query_swww_state()?;
       let gtk_theme = query_gsettings()?;

       // Detect inconsistencies early
       if cli_theme != gtk_theme {
           warn!("State drift detected!");
           // Show user reconciliation dialog
       }

       AppState { /* use live state as truth */ }
   }
   ```

**Detection:**
- App shows Theme A as current, but `vulcan-theme current` says Theme B
- Preview cancel button enabled when no preview is active
- Wallpaper doesn't match selected theme's theme_wallpaper field
- Toast notifications about "applied theme X" but UI still shows theme Y selected

**Phase to address:** Phase 1 (Foundation architecture)
Must establish single source of truth pattern before building unified UI.

---

### Pitfall 2: Shell Script Parsing Fragility

**What goes wrong:**
Theme files are bash `.sh` scripts that export variables. The parser in `theme_parser.rs` uses regex/string matching, which is fragile:

```bash
# This works
export THEME_NAME="Tokyo Night"

# This breaks the parser
export THEME_NAME="He said \"hello\""  # Nested quotes

# This breaks the parser
export THEME_WALLPAPER="~/Pictures/My Wallpapers/space.png"  # Spaces in path

# This breaks the parser silently
export ACCENT="#ff6600"
export ACCENT_ALT = "#ffaa00"  # Space before = breaks bash, parser might miss it
```

**Why it happens:**
- Parser uses simplified regex instead of full bash parser
- No validation that .sh file is actually valid bash
- Shell variable expansion (`~`, `$HOME`) handled inconsistently
- Comments, multiline strings, escaped characters not properly handled

**Consequences:**
- User creates custom theme via editor → parser silently drops values
- Imported theme file has slightly different syntax → corrupts theme
- Theme with spaces in wallpaper path → file not found error at apply time
- Round-trip edit loses comments, formatting, special characters

**Prevention:**
1. **Validate before parse:** Run `bash -n theme.sh` to check syntax validity
2. **Normalize on save:** When serializing, use consistent safe format:
   ```bash
   # Always single-quoted for values, double-quoted for paths
   export THEME_NAME='Tokyo Night'
   export THEME_WALLPAPER="/full/absolute/path/no/spaces"
   ```
3. **Expand variables immediately:** Convert `~` to `/home/user` on import
4. **Whitelist allowed syntax:** Only support simple `export VAR="value"` format
5. **Add parser tests:** Test suite with malformed theme files

**Detection:**
- Theme import succeeds but preview shows wrong colors
- Saved theme can't be re-imported
- Theme editor shows empty fields for valid theme file
- Bash syntax errors when running `vulcan-theme set <custom-theme>`

**Phase to address:** Phase 1 (Foundation)
Parser must be hardened before exposing theme editing to users.

---

### Pitfall 3: Wallpaper Backend Assumption Mismatch

**What goes wrong:**
Code currently assumes `swww` backend (as seen in autostart.conf and service code), but documentation/comments reference `hyprpaper`, and old config files use `hyprpaper.conf`. This creates four failure modes:

1. **Old config loaded:** App finds hyprpaper.conf, tries to parse, but system runs swww
2. **Backend detection fails:** App assumes swww is running, but user manually killed daemon
3. **Transition period:** User switching from hyprpaper to swww, both configs exist
4. **Future backend change:** Code is swww-specific, hard to swap backends later

**Why it happens:**
- VulcanOS transitioned from hyprpaper to swww mid-development
- Hard-coded backend calls in `hyprpaper.rs` service layer (misnamed file!)
- No abstraction layer for wallpaper backends
- Config file discovery doesn't check which daemon is actually running

**Consequences:**
- App launches, can't set wallpaper (swww not running)
- Wallpaper appears to apply but doesn't persist (wrong config file updated)
- Profile restore fails silently (queries swww when hyprpaper is running)
- Future: Hyprland switches recommended backend → requires rewrite

**Prevention:**
1. **Runtime backend detection:**
   ```rust
   fn detect_wallpaper_backend() -> Result<WallpaperBackend> {
       if is_process_running("swww-daemon") {
           Ok(WallpaperBackend::Swww)
       } else if is_process_running("hyprpaper") {
           Ok(WallpaperBackend::Hyprpaper)
       } else {
           Err("No wallpaper backend running")
       }
   }
   ```

2. **Backend abstraction trait:**
   ```rust
   trait WallpaperBackend {
       fn apply(&self, monitor: &str, path: &Path) -> Result<()>;
       fn query_active(&self) -> Result<HashMap<String, PathBuf>>;
       fn preload(&self, path: &Path) -> Result<()>;
   }

   impl WallpaperBackend for SwwwBackend { /* ... */ }
   impl WallpaperBackend for HyprpaperBackend { /* ... */ }
   ```

3. **Explicit backend selection in UI:**
   Show detected backend in status bar, allow user override in settings

4. **Config file versioning:**
   Don't auto-migrate hyprpaper.conf to swww - warn user and offer migration

**Detection:**
- App works on dev machine (swww), fails on user's install (hyprpaper)
- Wallpaper changes don't persist across compositor restart
- `list_active()` returns empty when wallpapers are clearly set
- Error logs mention swww when user is running hyprpaper

**Phase to address:** Phase 1 (Foundation)
Backend abstraction must exist before building profile/theme binding system.

---

### Pitfall 4: Component Lifecycle Memory Leaks

**What goes wrong:**
Relm4 Controllers for child components (theme_browser, preview_panel, editor_dialog) are stored in parent App struct. When opening/closing dialogs repeatedly or switching between themes rapidly, child components accumulate without proper cleanup, causing:

- Memory usage grows with each theme preview
- Widget count increases without bound
- Event handlers remain connected to destroyed widgets
- GTK4's cairo renderer has known ~70kb leak per window (compounding issue)

**Why it happens:**
- Child Controllers stored as `Option<Controller<T>>` but never properly dropped
- Dialog windows closed via `window.close()` but Controller reference kept alive
- Preview panel receives theme updates but doesn't clean up previous theme's resources
- GTK4 itself has known memory leaks in cairo/NGL renderers

**Consequences:**
- App uses 50MB after launch, 500MB after 100 theme previews
- Lag when opening theme editor after browsing many themes
- Eventual OOM crash on systems with limited RAM (ISO's target: 4GB machines)
- System swapping during appearance customization (terrible UX)

**Prevention:**
1. **Explicit cleanup in dialog pattern:**
   ```rust
   // Bad: Controller kept alive
   self.editor_dialog = Some(editor);
   self.editor_window = Some(window);

   // Good: Drop Controller when done
   if let Some(old_editor) = self.editor_dialog.take() {
       old_editor.shutdown(); // If available
       drop(old_editor);
   }
   ```

2. **Detach read-only child components:**
   Preview panel doesn't need bi-directional messaging, use `.detach()`

3. **Limit preview panel updates:**
   Don't recreate entire preview on every theme change, update colors only:
   ```rust
   // Bad: Recreates widget tree
   self.preview_panel = create_new_preview(theme);

   // Good: Update existing widgets
   self.preview_panel.emit(PreviewPanelInput::UpdateColors(theme.colors()));
   ```

4. **Resource pooling for thumbnails:**
   Wallpaper picker loads all thumbnails into memory - implement LRU cache with max size

5. **Monitor with instrumentation:**
   ```rust
   #[cfg(debug_assertions)]
   fn log_memory_usage() {
       let mem = current_memory_mb();
       if mem > 200 { warn!("High memory usage: {}MB", mem); }
   }
   ```

**Detection:**
- Run app, open 20 themes, check `ps aux` - memory should stay flat
- Open/close theme editor 50 times - memory should not grow linearly
- Browse wallpaper grid with 1000 images - should not OOM
- Valgrind shows increasing "definitely lost" bytes over time

**Phase to address:** Phase 2 (Component integration)
Must profile during component merge, before shipping unified app.

---

### Pitfall 5: Theme-Wallpaper Binding Coherence

**What goes wrong:**
Theme files have optional `THEME_WALLPAPER` field. Wallpaper profiles have per-monitor paths. When both exist, conflicts arise:

1. **Ambiguous authority:** Theme says "use wallpaper A", profile says "use wallpaper B"
2. **Single vs multi-monitor:** Theme specifies one wallpaper, profile specifies 5 (one per monitor)
3. **Profile switching:** User switches monitor profile (laptop→desktop), theme wallpaper now wrong
4. **Missing wallpaper:** Theme references `/path/to/cool.png` which doesn't exist
5. **Circular dependency:** Applying theme changes wallpaper, changing wallpaper invalidates theme

**Why it happens:**
- Theme and wallpaper systems designed independently
- No clear precedence rules (theme wins? profile wins? user's last action wins?)
- VulcanOS has dynamic monitor setups (hyprmon-desc profiles: laptop, desktop, console, campus, presentation)
- Theme wallpaper is optional, so sometimes exists, sometimes doesn't

**Consequences:**
- User applies "Tokyo Night" theme → wallpaper changes to theme's default
- User carefully arranges per-monitor wallpapers → theme application wipes them out
- User switches to laptop profile → theme's 3840x2160 wallpaper stretched on 1920x1200 screen
- "Undo theme" button doesn't restore wallpapers (only theme colors)
- Theme preview shows wallpaper A, theme application shows wallpaper B

**Prevention:**
1. **Explicit binding modes:**
   ```rust
   enum ThemeWallpaperBinding {
       ThemeControlled,     // Theme THEME_WALLPAPER always applied
       ProfileControlled,   // Wallpaper profile independent of theme
       ThemeDefaulted,      // Use theme wallpaper if no profile set
       UserOverride,        // User manually picked, ignore both
   }
   ```

2. **Per-monitor theme wallpaper support:**
   Extend theme format to support monitor-specific wallpapers:
   ```bash
   export THEME_WALLPAPER_DEFAULT="~/Wallpapers/tokyo-night.png"
   export THEME_WALLPAPER_PRIMARY="~/Wallpapers/tokyo-night-ultrawide.png"
   export THEME_WALLPAPER_SECONDARY="~/Wallpapers/tokyo-night-vertical.png"
   ```

3. **Binding state in profile:**
   Save binding mode in wallpaper profile:
   ```toml
   [profile.desktop]
   binding_mode = "profile_controlled"
   monitor_wallpapers = { "DP-1" = "...", "DP-2" = "..." }
   ```

4. **Clear UI indicator:**
   Show icon/badge when wallpaper is "locked to theme" vs "independent"

5. **Smart application order:**
   ```rust
   fn apply_appearance(theme: Theme, profile: WallpaperProfile, mode: BindingMode) {
       match mode {
           ThemeControlled => {
               apply_theme_colors(theme);
               apply_theme_wallpaper(theme); // Theme wins
           }
           ProfileControlled => {
               apply_theme_colors(theme);
               apply_wallpaper_profile(profile); // Profile wins
           }
           ThemeDefaulted => {
               apply_theme_colors(theme);
               if !profile_has_wallpapers() && theme.has_wallpaper() {
                   apply_theme_wallpaper(theme);
               } else {
                   apply_wallpaper_profile(profile);
               }
           }
       }
   }
   ```

**Detection:**
- Apply theme, wallpapers change unexpectedly
- Set wallpapers carefully, apply theme, wallpapers reset
- Switch monitor profile, theme looks wrong (wallpaper doesn't match)
- Preview theme shows one wallpaper, apply shows different wallpaper
- Undo/cancel doesn't restore original wallpaper state

**Phase to address:** Phase 3 (Theme-Wallpaper binding)
This is the core integration challenge. Requires clear design decision before implementation.

---

## Moderate Pitfalls

Mistakes that cause delays or technical debt.

### Pitfall 6: GTK CSS Cascading Conflicts

**What goes wrong:**
Unified app tries to theme itself using same CSS system it's managing for desktop. Conflicts arise:

- App loads custom CSS to style its own GTK widgets
- User applies theme that changes GTK CSS globally
- App's custom styles get overridden by theme CSS
- App UI becomes unreadable (dark text on dark background)

**Why it happens:**
GTK CSS has cascading issues where global CSS sends a giant string into the app that may override certain things based on random string matching. No strong isolation between "app-specific CSS" and "system theme CSS".

**Prevention:**
1. **Namespace app CSS with high-specificity selectors:**
   ```css
   /* Bad: Can be overridden by theme */
   .theme-preview { background: #fff; }

   /* Good: App-specific class unlikely to collide */
   .vulcan-appearance-manager .theme-preview-card {
       background: #fff;
   }
   ```

2. **Use inline styles for critical UI:**
   For elements that MUST be readable regardless of theme, set style directly in code

3. **Test with extreme themes:**
   Test app with all-black theme, all-white theme, high-contrast theme

4. **Libadwaita as foundation:**
   Use Adwaita widgets which are designed to respect system theme properly

**Detection:**
- Apply theme, app's own UI becomes unreadable
- Preview panel colors don't show correctly
- Button text disappears on certain themes

**Phase to address:** Phase 2 (Component integration)
Test each theme against app's own UI.

---

### Pitfall 7: Config File Format Fragmentation

**What goes wrong:**
System has multiple overlapping config formats:
- Themes: `.sh` bash scripts in `~/.config/themes/colors/`
- Wallpaper profiles: `.toml` in `~/.config/vulcan-wallpaper/profiles/`
- GTK: gsettings database
- Hyprland: `.conf` files
- Waybar/other tools: `.jsonc`, `.css`, `.toml`

When unified app tries to manage all of these:
- Parsing errors in one format shouldn't crash app
- Format updates require updating multiple parsers
- Backup/restore must handle all formats
- Export feature: which format to use?

**Prevention:**
1. **Format abstraction layer:**
   ```rust
   trait ConfigFormat {
       fn parse(&self, path: &Path) -> Result<Config>;
       fn serialize(&self, config: &Config) -> String;
       fn validate(&self, path: &Path) -> Result<()>;
   }
   ```

2. **Fail gracefully on parse errors:**
   Show toast "Skipped malformed theme: X" instead of crashing

3. **Unified export format option:**
   Let user export as "VulcanOS Appearance Profile" (single TOML with all settings)

4. **Version config files:**
   Add version header to generated configs for future migration

**Detection:**
- One malformed theme file prevents app from loading all themes
- Import fails silently because format wasn't detected
- Export produces invalid config file

**Phase to address:** Phase 2 (Component integration)

---

### Pitfall 8: Live Reload Race Conditions

**What goes wrong:**
When applying theme, multiple config files are updated and apps reloaded:

```bash
# Theme application sequence
1. Write ~/.config/gtk-3.0/settings.ini
2. Write ~/.config/gtk-4.0/settings.ini
3. Write ~/.config/Kvantum/kvantum.kvconfig
4. Run gsettings set org.gnome.desktop.interface gtk-theme ...
5. Reload waybar (killall -SIGUSR2 waybar)
6. Reload hyprland (hyprctl reload)
```

Race condition windows:
- File written but not flushed when app reads it
- App reloads before all files written (sees half-applied theme)
- Multiple apps reloading simultaneously contend for resources

**Prevention:**
1. **Synchronous write-then-reload:**
   ```rust
   fn apply_theme(theme: &Theme) -> Result<()> {
       // Write ALL configs first
       write_gtk3_config(theme)?;
       write_gtk4_config(theme)?;
       write_kvantum_config(theme)?;

       // Flush to disk
       std::io::stdout().flush()?;

       // THEN reload apps
       reload_waybar()?;
       reload_hyprland()?;
   }
   ```

2. **Add delays between reloads:**
   ```rust
   reload_waybar()?;
   std::thread::sleep(Duration::from_millis(100));
   reload_hyprland()?;
   ```

3. **Check reload success:**
   Don't just fire-and-forget `killall -SIGUSR2`, check if process exists first

4. **Atomic config updates:**
   Write to temp file, then atomic rename (prevents partial reads)

**Detection:**
- Theme applies but waybar shows old colors (reload happened too early)
- Sometimes theme applies correctly, sometimes partially
- Config file corruption after theme application

**Phase to address:** Phase 3 (Application)

---

### Pitfall 9: Thumbnail Generation Performance

**What goes wrong:**
Wallpaper picker loads directory with 1000+ high-res images. Generating thumbnails on-the-fly:

- Blocks UI thread (app freezes)
- Re-generates same thumbnail multiple times (no cache)
- Loads full 4K image into memory just to shrink to 200px thumbnail
- File watcher re-scans directory on every thumbnail write

**Prevention:**
1. **Thumbnail cache directory:**
   `~/.cache/vulcan-appearance-manager/thumbnails/` with hash-based names

2. **Background thumbnail generation:**
   Use tokio task to generate thumbnails asynchronously

3. **Progressive loading:**
   Show placeholder, load thumbnails in batches of 20

4. **Use system thumbnailer:**
   Check for existing thumbnails in `~/.cache/thumbnails/` (FreeDesktop spec)

5. **Debounce file watcher:**
   Don't re-scan on every change, batch updates every 500ms

**Detection:**
- App freezes when opening wallpaper picker
- High CPU usage when browsing wallpapers
- Thousands of thumbnail files regenerated every launch

**Phase to address:** Phase 4 (Polish)

---

### Pitfall 10: Undo/History Stack Complexity

**What goes wrong:**
User wants "undo" for appearance changes. Naive approach:

```rust
// Stores entire appearance state per change
struct UndoStack {
    history: Vec<AppearanceState>,  // 10 entries * 50KB each = 500KB
}
```

But appearance state includes:
- Theme (40+ color values)
- Wallpaper paths (5 monitors * path)
- GTK settings
- Kvantum settings
- Waybar config

Problems:
- Memory usage grows unbounded
- Undo of theme change also undoes wallpaper change (user wanted independent undo)
- Preview + apply creates duplicate history entries
- Can't undo just wallpaper, must undo entire appearance

**Prevention:**
1. **Separate undo stacks:**
   ```rust
   struct UndoManager {
       theme_history: VecDeque<ThemeSnapshot>,
       wallpaper_history: VecDeque<WallpaperSnapshot>,
       max_entries: usize,
   }
   ```

2. **Diff-based history:**
   Store only changed values, not entire state

3. **Clear history on explicit apply:**
   Preview changes are temporary (don't add to history), only apply adds entry

4. **Limit history depth:**
   Keep last 10 entries, drop oldest

**Detection:**
- Memory grows with number of theme changes
- Undo button reverts more than expected
- Can't undo wallpaper without undoing theme

**Phase to address:** Phase 4 (Polish)

---

## Minor Pitfalls

Mistakes that cause annoyance but are fixable.

### Pitfall 11: File Picker Default Directory

**What goes wrong:**
Import theme/wallpaper dialogs open to random directory (last used by any GTK app).

**Prevention:**
Set default directory to known locations:
- Theme import: `~/.config/themes/colors/`
- Wallpaper import: `~/Pictures/Wallpapers/`

**Detection:**
Dialog opens to `/usr/share/` or `~/Downloads/`

**Phase to address:** Phase 4 (Polish)

---

### Pitfall 12: No Validation on Theme Import

**What goes wrong:**
User imports `.sh` file that's actually a bash script virus, or just garbage data.

**Prevention:**
1. Run `bash -n theme.sh` to validate syntax
2. Check for required exports (THEME_NAME, THEME_ID, etc.)
3. Sandbox bash execution (don't actually source untrusted .sh)
4. Show preview before importing

**Detection:**
Importing garbage file crashes app or corrupts theme list

**Phase to address:** Phase 2 (Component integration)

---

### Pitfall 13: Hardcoded Paths

**What goes wrong:**
Code has hardcoded `/home/evan/` paths instead of using `$HOME` or `dirs::home_dir()`.

**Prevention:**
Audit codebase for hardcoded paths, replace with proper home dir detection.

**Detection:**
App works for user "evan", fails for everyone else.

**Phase to address:** Phase 1 (Foundation) - must fix before merge

---

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| Foundation architecture | State synchronization drift (#1) | Establish single source of truth pattern |
| Foundation architecture | Backend assumption mismatch (#3) | Implement backend abstraction trait |
| Foundation architecture | Shell script parsing fragility (#2) | Harden parser, add validation |
| Component integration | Memory leaks (#4) | Profile memory usage, implement cleanup |
| Component integration | GTK CSS conflicts (#6) | Namespace app CSS, test with extreme themes |
| Theme-Wallpaper binding | Binding coherence (#5) | Design binding mode system first |
| Application logic | Live reload races (#8) | Atomic writes, synchronous reload sequence |
| Polish & UX | Thumbnail performance (#9) | Implement cache, async loading |
| Polish & UX | Undo complexity (#10) | Separate undo stacks per concern |

---

## Sources

### Relm4 and GTK4 Component Architecture
- [Relm4 GitHub Repository](https://github.com/Relm4/Relm4)
- [Relm4 Documentation - Introduction](https://relm4.org/book/stable/)
- [Announcing Relm4 v0.5 beta](https://relm4.org/blog/posts/announcing_relm4_v0.5_beta/)

### GTK4 Memory Leaks and Lifecycle
- [GTK4 memory leak in cairo renderer - Issue #6404](https://gitlab.gnome.org/GNOME/gtk/-/issues/6404)
- [Suspected memory leak in GTK 4's NGL renderer - Issue #7045](https://gitlab.gnome.org/GNOME/gtk/-/issues/7045)
- [Valgrind reports memory leak with gtk4](https://discourse.gnome.org/t/valgrind-reports-memory-leak-with-gtk4/25598)

### GTK CSS Theming Cross-Application Issues
- [GTK CSS Theming - ArchWiki](https://wiki.archlinux.org/title/GTK)
- [How to apply CSS per application? - GNOME Discourse](https://discourse.gnome.org/t/how-to-apply-css-per-application/11382)
- [GTK CSS Overview Documentation](https://docs.gtk.org/gtk3/css-overview.html)

### Design System Architecture Pitfalls
- [The Dark Side of Design Systems - Mistakes and Lessons](https://sakalim.com/content/the-dark-side-of-design-systems-mistakes-missteps-and-lessons-learned)
- [System Architecture Mistakes: 10 Deadly Sins](https://medium.com/@hemanthkumar.v/system-architecture-mistakes-10-deadly-sins-how-to-fix-them-b5329cc7dccc)
- [Frontend Design Patterns That Actually Work in 2026](https://www.netguru.com/blog/frontend-design-patterns)

### Wayland Wallpaper Management (swww)
- [swww GitHub - A Solution to your Wayland Wallpaper Woes](https://github.com/LGFae/swww)
- [swww - Hyprland Wiki](https://wiki.hypr.land/Useful-Utilities/Wallpapers/)
- [Can I finally start using Wayland in 2026?](https://michael.stapelberg.ch/posts/2026-01-04-wayland-sway-in-2026/)

### Shell Script Configuration and Security
- [Avoid Race Conditions in Secure Programs](https://tldp.org/HOWTO/Secure-Programs-HOWTO/avoid-race.html)
- [Race Conditions and Secure File Operations](https://developer.apple.com/library/archive/documentation/Security/Conceptual/SecureCodingGuide/Articles/RaceConditions.html)
- [Parsing config files with Bash](https://opensource.com/article/21/6/bash-config)
- [ShellCheck Parser Errors](https://www.shellcheck.net/wiki/Parser-error)

---

## Research Methodology

**Confidence level:** MEDIUM

**Reasoning:**
- **HIGH confidence:** State synchronization issues, shell parsing fragility, wallpaper backend mismatch - directly observed in codebase
- **MEDIUM confidence:** GTK4 memory leaks - confirmed by upstream bug reports, but mitigation strategies are general best practices
- **LOW confidence:** Specific numeric thresholds (thumbnail batch size, history depth) - no hard data, based on general performance wisdom

**Verification approach:**
1. Examined actual codebase (both apps) for state management patterns
2. Verified wallpaper backend confusion (hyprpaper.conf exists, swww in autostart)
3. Confirmed theme format (.sh scripts) and wallpaper format (.toml)
4. Found GTK4/Relm4 memory leak issues in upstream bug trackers
5. Researched general theming system pitfalls via design system literature

**Gaps not covered:**
- Actual memory usage profiling (requires running merged app)
- Performance benchmarks for thumbnail generation
- Real-world testing of theme-wallpaper binding modes
- User testing of undo/history UX

These gaps should be addressed during implementation with instrumentation and user testing.
