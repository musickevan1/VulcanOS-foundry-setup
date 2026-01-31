# Phase 12: UX Polish - Research

**Researched:** 2026-01-30
**Domain:** BindingMode state transitions, wallpaper library management
**Confidence:** HIGH

## Summary

This maintenance phase addresses three UX requirements that wire existing infrastructure rather than build new features. The BindingMode architecture is fully implemented with three states (ThemeBound, CustomOverride, Unbound) but lacks the auto-transition logic when users manually change wallpapers. The wallpaper library structure exists for all 10 themes, but 7 of them are missing actual wallpaper images.

Key findings:
1. **UX-01 (BindingMode auto-transition):** The missing logic is a single condition check in `app.rs` at line 294-307. When `AppMsg::WallpapersChanged` is received and `current_binding_mode == ThemeBound`, it should transition to `CustomOverride`.
2. **UX-02 (Missing wallpapers):** 7 themes need wallpapers: gruvbox-dark, gruvbox-light, nord, tokyonight, rosepine, onedark, vulcan-forge. Only catppuccin-mocha, catppuccin-latte, and dracula have images.
3. **UX-03 (LICENSE files):** All 10 LICENSE files exist but 7 are placeholder text awaiting actual wallpaper downloads.

**Primary recommendation:** Add 3-line BindingMode transition in `app.rs`, download 7 CC0/MIT wallpapers from documented sources, update LICENSE attribution.

## Standard Stack

This phase operates on existing infrastructure - no new dependencies needed.

### Core Architecture (Already Exists)
| Component | Location | Purpose | Status |
|-----------|----------|---------|--------|
| `BindingMode` enum | `src/models/binding.rs:8-16` | ThemeBound/CustomOverride/Unbound | COMPLETE |
| `App.current_binding_mode` | `src/app.rs:57` | Tracks current binding state | COMPLETE |
| `AppMsg::WallpapersChanged` | `src/app.rs:42` | Message when wallpaper changes | COMPLETE |
| `AppMsg::BindingModeChanged` | `src/app.rs:45` | Message to update binding mode | COMPLETE |

### Wallpaper Library Structure (Already Exists)
| Directory | Purpose | Status |
|-----------|---------|--------|
| `dotfiles/wallpapers/{theme}/` | Per-theme wallpaper storage | 10/10 exist |
| `dotfiles/wallpapers/{theme}/LICENSE` | Attribution file | 10/10 exist |
| `dotfiles/wallpapers/{theme}/{theme}.png` | Primary wallpaper | 3/10 have images |

### Missing Wallpapers (7 themes)
| Theme | Directory | LICENSE | Wallpaper |
|-------|-----------|---------|-----------|
| gruvbox-dark | EXISTS | Placeholder | MISSING |
| gruvbox-light | EXISTS | Placeholder | MISSING |
| nord | EXISTS | Placeholder | MISSING |
| tokyonight | EXISTS | Placeholder | MISSING |
| rosepine | EXISTS | Placeholder | MISSING |
| onedark | EXISTS | Placeholder | MISSING |
| vulcan-forge | EXISTS | Placeholder | MISSING |

### Themes with Wallpapers (3 themes)
| Theme | Wallpaper | Source | License |
|-------|-----------|--------|---------|
| catppuccin-mocha | catppuccin-mocha.png (61KB) | zhichaoh/catppuccin-wallpapers | MIT |
| catppuccin-latte | catppuccin-latte.png (61KB) | zhichaoh/catppuccin-wallpapers | MIT |
| dracula | dracula.png (366KB) | draculatheme.com | Unknown (verify) |

## Architecture Patterns

### Pattern: BindingMode State Machine

The BindingMode tracks how wallpaper relates to theme:

```
Theme Applied (with wallpaper) --> User accepts --> ThemeBound
                               --> User declines --> Unbound

ThemeBound + User changes wallpaper manually --> CustomOverride  [UX-01: MISSING]

Unbound + User changes wallpaper manually --> Unbound (no change)

CustomOverride + User reapplies theme wallpaper --> ThemeBound (via BindingDialog)
```

**Current Code Flow (app.rs:294-307):**
```rust
// Source: vulcan-appearance-manager/src/app.rs:294-307
AppMsg::WallpapersChanged(wallpapers) => {
    // Track current wallpapers
    self.current_wallpapers = wallpapers.clone();

    // Notify profile manager of wallpaper changes
    self.profile_manager.emit(ProfileManagerInput::UpdateWallpapers(wallpapers.clone()));

    // Sync state to profile view for saving
    self.profile_view.emit(ProfileViewMsg::UpdateCurrentState {
        theme_id: self.current_theme_id.clone(),
        wallpapers: self.current_wallpapers.clone(),
        binding_mode: self.current_binding_mode.clone(),
    });
}
```

**Missing Logic (UX-01 Fix):**
```rust
// INSERT AFTER line 296 ("self.current_wallpapers = wallpapers.clone();")
// Auto-transition: ThemeBound -> CustomOverride when user manually changes wallpaper
if self.current_binding_mode == BindingMode::ThemeBound {
    self.current_binding_mode = BindingMode::CustomOverride;
}
```

### Pattern: Theme-Wallpaper Resolution

When applying a theme, the wallpaper path is resolved:

```rust
// Source: vulcan-appearance-manager/src/models/binding.rs:72-92
pub fn resolve_theme_wallpaper(theme: &Theme) -> Option<PathBuf> {
    let wallpaper_rel = theme.theme_wallpaper.as_ref()?;  // e.g., "tokyonight.png"
    let theme_dir = theme.source_path.as_ref()?.parent()?;
    let abs_path = theme_dir.join(wallpaper_rel);
    if abs_path.exists() {
        Some(abs_path)
    } else {
        eprintln!("Warning: Theme '{}' suggests wallpaper '{}' but file not found", ...);
        None
    }
}
```

**Implication:** If wallpaper file doesn't exist, theme applies but binding dialog doesn't appear. Adding wallpapers will automatically enable the binding feature for those themes.

### Anti-Patterns to Avoid
- **Changing BindingMode in multiple places:** Only `app.rs` should manage BindingMode state
- **Hardcoding wallpaper paths:** Use relative paths in theme files, resolved at runtime
- **Skipping LICENSE updates:** Every downloaded wallpaper needs attribution

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Wallpaper state tracking | Custom state | Existing `current_binding_mode` | Already wired to profile system |
| Theme-wallpaper lookup | Manual path building | Existing `resolve_theme_wallpaper()` | Handles all edge cases |
| User notification | Custom dialogs | Existing `BindingDialogModel` | Already shows wallpaper preview |
| Wallpaper attribution | Inline comments | LICENSE file per theme | Established pattern, easier to audit |

**Key insight:** The BindingMode architecture is complete. UX-01 is a 3-line fix, not a new feature.

## Common Pitfalls

### Pitfall 1: Forgetting Source Distinction for WallpapersChanged
**What goes wrong:** Transitioning to CustomOverride even when wallpaper was set BY the theme (via ApplyThemeWallpaper)
**Why it happens:** `WallpapersChanged` fires for both manual and theme-initiated wallpaper changes
**How to avoid:** Currently both paths emit `WallpapersChanged`. The theme path also emits `BindingModeChanged(ThemeBound)` AFTER the wallpaper change. Order matters: wallpaper change sets CustomOverride, then BindingModeChanged resets to ThemeBound.
**Warning signs:** Applying theme + wallpaper results in CustomOverride instead of ThemeBound

**Analysis of message flow:**
```
Theme Apply with wallpaper (user accepts):
1. theme_view -> ApplyBoth -> BindingModeChanged(ThemeBound) emitted FIRST
2. theme_view -> ApplyWallpaper -> app handles -> wallpaper_view -> WallpapersChanged

Manual wallpaper change:
1. wallpaper_view -> ApplyWallpaper -> WallpapersChanged
2. (BindingModeChanged not emitted)
```

**Solution:** Check `current_binding_mode` at time of WallpapersChanged. If ThemeBound, it means user is overriding.

### Pitfall 2: Wrong Wallpaper Resolution Path
**What goes wrong:** Wallpaper not found even though file exists
**Why it happens:** `theme_wallpaper` in .sh file should be filename only, not full path
**How to avoid:** Use format: `export THEME_WALLPAPER="tokyonight.png"` (just filename)
**Warning signs:** "file not found" warnings when theme has wallpaper

**Correct path resolution:**
- Theme file: `dotfiles/themes/colors/tokyonight.sh`
- Theme's source_path: `dotfiles/themes/colors/tokyonight.sh`
- Theme's parent: `dotfiles/themes/colors/`
- THEME_WALLPAPER value: `tokyonight.png`
- Resolved path: `dotfiles/themes/colors/tokyonight.png` (WRONG - wallpapers are elsewhere!)

**Actually, looking at theme files:**
```bash
# In tokyonight.sh
export THEME_WALLPAPER="tokyonight.png"
```

The wallpaper is expected in the SAME directory as the theme file, but wallpapers are stored in `dotfiles/wallpapers/{theme}/`. This appears to be a path mismatch that needs investigation.

### Pitfall 3: License Non-Compliance
**What goes wrong:** Using wallpapers without proper attribution or wrong license
**Why it happens:** Grabbing images from search results without checking source
**How to avoid:** Only use CC0 or GPL-compatible (MIT, Apache, BSD) wallpapers. Document in LICENSE.
**Warning signs:** LICENSE file says "placeholder" or has no attribution

### Pitfall 4: Inconsistent Wallpaper Naming
**What goes wrong:** THEME_WALLPAPER doesn't match actual filename
**Why it happens:** Renaming files after download
**How to avoid:** Use consistent naming: `{theme-id}.png` (e.g., `tokyonight.png`, `nord.png`)
**Warning signs:** Theme applies but no wallpaper binding dialog appears

## Code Examples

### UX-01: BindingMode Auto-Transition Fix
```rust
// Source: vulcan-appearance-manager/src/app.rs
// Location: Inside AppMsg::WallpapersChanged handler, after line 296

AppMsg::WallpapersChanged(wallpapers) => {
    // Track current wallpapers
    self.current_wallpapers = wallpapers.clone();

    // UX-01: Auto-transition ThemeBound -> CustomOverride on manual wallpaper change
    if self.current_binding_mode == BindingMode::ThemeBound {
        self.current_binding_mode = BindingMode::CustomOverride;
    }

    // ... rest of existing code ...
}
```

### UX-02: Theme Wallpaper Reference Pattern
```bash
# Source: dotfiles/themes/colors/tokyonight.sh (line 100-101)
# Wallpaper (relative path from theme directory)
export THEME_WALLPAPER="tokyonight.png"
```

### UX-03: LICENSE File Format
```markdown
# Tokyo Night Wallpapers

## Current Wallpapers

### tokyonight.png
- **Source:** [URL where downloaded from]
- **License:** [CC0/MIT/etc]
- **Resolution:** [e.g., 3840x2160]
- **Downloaded:** [date]
- **Author:** [creator name or username]

## License Requirements

All wallpapers added to this directory must be:
- CC0 (Public Domain) OR
- GPL-compatible (MIT, GPL, Apache, BSD) OR
- Properly licensed for redistribution
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Separate theme/wallpaper apps | Unified vulcan-appearance-manager | v2.0 | Single app, unified profiles |
| Manual wallpaper management | BindingMode state tracking | v2.0 | Automatic coordination |
| No theme wallpapers | THEME_WALLPAPER in theme files | v2.0 | Theme can suggest wallpaper |

**Status:** Architecture complete. This phase wires the auto-transition and adds missing content.

## Open Questions

### 1. Wallpaper Path Resolution
**What we know:** Themes reference wallpapers as `{theme}.png`, resolution looks in theme's parent directory
**What's unclear:** Wallpapers are stored in `dotfiles/wallpapers/{theme}/` but themes are in `dotfiles/themes/colors/`
**Recommendation:** Verify actual path resolution in `resolve_theme_wallpaper()` - may need to update theme files to use relative paths like `../wallpapers/tokyonight/tokyonight.png` or symlink wallpapers into theme directories

### 2. Dracula License
**What we know:** dracula.png exists (366KB) from draculatheme.com
**What's unclear:** Exact license terms - draculatheme.com uses MIT for code, wallpapers may differ
**Recommendation:** Verify license on draculatheme.com/wallpaper before shipping

## Sources

### Primary (HIGH confidence)
- `vulcan-appearance-manager/src/app.rs` - Full source review, lines 244-347
- `vulcan-appearance-manager/src/models/binding.rs` - BindingMode enum and resolve_theme_wallpaper
- `vulcan-appearance-manager/src/components/theme_view.rs` - Theme apply flow
- `vulcan-appearance-manager/src/components/wallpaper_view.rs` - Wallpaper change flow
- `dotfiles/wallpapers/` - Directory listing, LICENSE file review
- `dotfiles/themes/colors/*.sh` - All 10 theme files reviewed

### Secondary (MEDIUM confidence)
- `dotfiles/wallpapers/README.md` - Documents expected structure and sources

## Metadata

**Confidence breakdown:**
- UX-01 fix location: HIGH - Exact line identified in app.rs
- Wallpaper status: HIGH - File system verified
- Path resolution: MEDIUM - May need verification during implementation

**Research date:** 2026-01-30
**Valid until:** Indefinite (internal codebase knowledge)

---

## Implementation Checklist for Planner

### UX-01: BindingMode Auto-Transition
- [ ] Add 3-line condition in `app.rs` after line 296
- [ ] Verify message ordering doesn't break theme+wallpaper apply
- [ ] Test: Apply theme with wallpaper (should be ThemeBound), then manually change wallpaper (should become CustomOverride)

### UX-02: Missing Wallpapers (7 themes)
Download from documented sources (check LICENSE first):
- [ ] gruvbox-dark.png - Try: github.com/morhetz/gruvbox/wiki or gruvbox-community repos
- [ ] gruvbox-light.png - Same sources as dark
- [ ] nord.png - Try: github.com/arcticicestudio/nord or nordtheme.com
- [ ] tokyonight.png - Try: github.com/enkia/tokyo-night-vscode-theme or folke/tokyonight.nvim
- [ ] rosepine.png - Try: github.com/rose-pine or rosepinetheme.com
- [ ] onedark.png - Try: github.com/joshdick/onedark.vim or atom themes
- [ ] vulcan-forge.png - Create original or use CC0 source (volcano/forge aesthetic)

### UX-03: LICENSE Updates
For each downloaded wallpaper, update LICENSE with:
- [ ] Source URL
- [ ] License type (must be CC0/MIT/GPL-compatible)
- [ ] Resolution
- [ ] Download date
- [ ] Author attribution

### Path Resolution Verification
- [ ] Test that resolve_theme_wallpaper() finds wallpapers correctly
- [ ] If broken, update THEME_WALLPAPER paths in theme files to use relative path to wallpapers directory
