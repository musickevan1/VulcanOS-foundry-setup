---
phase: 05-vulcanos-wallpaper-manager
plan: 08
subsystem: verification
tags: [human-verification, testing, user-acceptance]
requires: ["05-07"]
provides: ["verified-wallpaper-manager"]
affects: ["phase-completion"]
tech-stack:
  added: []
  patterns: ["user-acceptance-testing"]
key-files:
  created: []
  modified:
    - "vulcan-wallpaper-manager/src/services/hyprpaper.rs"
    - "dotfiles/scripts/.local/bin/vulcan-menu"
decisions:
  - id: "wallpaper-backend"
    choice: "swww instead of hyprpaper"
    rationale: "VulcanOS uses swww for wallpaper management, not hyprpaper"
    alternatives: ["Install hyprpaper", "Support both backends"]
metrics:
  duration: "~15 minutes"
  completed: "2026-01-24"
---

# Phase 05 Plan 08: Human Verification Summary

**One-liner:** Human verification of VulcanOS Wallpaper Manager with bug fixes for swww backend and menu integration

## What Was Verified

Complete end-to-end testing of the VulcanOS Wallpaper Manager GUI application on actual hardware.

### Verification Results

| Check | Status | Notes |
|-------|--------|-------|
| GUI launches | ✓ | Adwaita styling, proper window |
| Monitor layout | ✓ | Displays all connected monitors |
| Wallpaper selection | ✓ | Thumbnails load correctly |
| Apply wallpaper | ✓ | After swww fix |
| Profile management | ✓ | Save/load works |
| Menu integration | ✓ | After menu fix |

## Issues Found and Fixed

### Issue 1: Wallpaper Backend Mismatch

**Problem:** Application was built for hyprpaper but VulcanOS uses swww for wallpaper management.

**Symptoms:**
- "hyprpaper preload failed" errors
- "hyprpaper set wallpaper failed" errors
- Wallpapers not applying to monitors

**Solution:** Rewrote `src/services/hyprpaper.rs` to use swww commands instead:
- `swww img <path> --outputs <monitor>` replaces hyprpaper preload+wallpaper
- Added smooth fade transition (0.5s)
- Simplified API (swww doesn't need separate preload)

**Commit:** `e99bf4e`

### Issue 2: Menu Entries Not Showing

**Problem:** "Wallpaper Manager" and "Wallpaper Profiles" entries missing from vulcan-menu.

**Root Cause:** Old version of vulcan-menu at `/usr/local/bin/vulcan-menu` was being used instead of the updated dotfiles version.

**Solution:**
1. Identified duplicate script locations
2. Simplified menu entries to plain text (icon rendering issues)
3. Removed sed icon-stripping that broke plain text matching
4. User synced script to `/usr/local/bin/`

**Commit:** `e99bf4e`

## Verification Steps Completed

1. **Build and Launch** ✓
   - `cargo build --release` succeeded
   - GUI window opens with Adwaita styling

2. **Monitor Layout** ✓
   - All 5 monitors displayed (eDP-1, DP-4, DP-9, DP-10, DP-12)
   - Clicking monitor highlights in blue

3. **Wallpaper Selection** ✓
   - Thumbnails load from ~/Pictures/Wallpapers/
   - Selection works correctly

4. **Apply Wallpaper** ✓
   - After swww fix, wallpapers apply immediately
   - Smooth fade transition

5. **Profile Management** ✓
   - Profiles save and load correctly
   - Persists across app restarts

6. **Menu Integration** ✓
   - After menu fix, entry appears in Style → Wallpaper
   - Launches GUI correctly

## Technical Discoveries

### swww vs hyprpaper

VulcanOS autostart.conf uses:
```bash
exec-once = swww-daemon && sleep 0.3 && swww restore
```

Key differences:
- **swww:** Single command `swww img <path> --outputs <monitor>`, built-in transitions
- **hyprpaper:** Requires `preload` then `wallpaper` commands via hyprctl

### Menu Script Locations

Two locations exist:
- `~/.local/bin/vulcan-menu` (symlink to dotfiles)
- `/usr/local/bin/vulcan-menu` (system-wide, may be outdated)

Keybindings call `vulcan-menu` without path, so PATH order determines which runs.

## Commits

| Hash | Type | Description |
|------|------|-------------|
| e99bf4e | fix | Switch wallpaper backend to swww and fix menu integration |

## Success Criteria Met

- ✅ User confirms all verification steps pass
- ✅ Application is stable without crashes
- ✅ Performance is acceptable (thumbnails load smoothly)
- ✅ UI matches VulcanOS design language (Adwaita)

## Phase 05 Status

**Plan 08 of 8** - Human verification complete

All plans in Phase 5 are now complete:
- 05-01: Project scaffold ✓
- 05-02: Main app window ✓
- 05-03: Wallpaper picker ✓
- 05-04: Component integration ✓
- 05-05: Profile management ✓
- 05-06: Panoramic splitting ✓
- 05-07: Desktop integration ✓
- 05-08: Human verification ✓

Phase 5 ready for goal verification.
