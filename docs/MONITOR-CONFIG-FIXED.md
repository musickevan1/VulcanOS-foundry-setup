# Monitor Configuration - Duplicate Resolution

## Problem Identified
Multiple monitor configuration locations were causing conflicts and Hyprland config errors.

## Locations Found

### 1. Live User Config (`~/.config/hypr/`)
**Status:** ✅ FIXED
- `hyprland.conf` (lines 59-61): Contains working user-specific monitor config
- `monitors.conf`: Cleared to prevent duplicates

### 2. VulcanOS Dotfiles - Root Level (`~/VulcanOS/dotfiles/hypr/`)
**Status:** ✅ CLEAN (Templates)
- `hyprland.conf`: No hardcoded monitors (sources from monitors.conf)
- `monitors.conf`: Auto-detect template for new installations

### 3. VulcanOS Dotfiles - Nested (`~/VulcanOS/dotfiles/hypr/.config/hypr/`)
**Status:** ✅ FIXED
- `hyprland.conf` (lines 59-61): Synced with user config
- `monitors.conf`: Cleared to prevent duplicates

### 4. ISO Skeleton (`~/VulcanOS/archiso/airootfs/etc/skel/.config/hypr/`)
**Status:** ✅ CLEAN (Templates)
- `hyprland.conf`: No hardcoded monitors
- `monitors.conf`: Auto-detect template

## Solution Applied

### For User Config (Active System)
Monitor definitions kept in `~/.config/hypr/hyprland.conf`:
```
monitor=eDP-1,3072x1920@60.00,1984x896,2.67
monitor=DP-6,1920x1080@60.00,64x384,1.05
monitor=DP-7,1920x1080@120.00,1984x192,1.67
```

### For VulcanOS Distribution
- ISO skeleton uses auto-detect template
- Users configure monitors via `nwg-displays` or `hyprctl`
- User-specific configs are NOT synced to the repo

## Best Practices Going Forward

1. **User-specific monitor configs** go in `~/.config/hypr/hyprland.conf`
2. **VulcanOS templates** use auto-detect in `monitors.conf`
3. **Never commit** personal monitor positions to the repo
4. Use `nwg-displays` for GUI configuration when needed
5. Keep only ONE definition per monitor to avoid conflicts

## Verification
```bash
# Check for duplicates
grep -r "monitor.*eDP-1" ~/VulcanOS/dotfiles/hypr/
grep -r "monitor.*eDP-1" ~/.config/hypr/

# Should only appear once in hyprland.conf
```

---
Fixed: 2026-01-02
