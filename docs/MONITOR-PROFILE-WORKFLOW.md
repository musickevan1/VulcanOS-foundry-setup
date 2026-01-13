# Monitor Profile Workflow - Daily Usage Guide

This guide explains the stable, reboot-proof monitor profile system for VulcanOS.

## Quick Reference

| Keybinding | Profile | Monitors | Use Case |
|------------|---------|----------|----------|
| `Super + F1` | **Desktop** | 3 monitors | Home setup (all monitors) |
| `Super + F2` | **Campus** | 2 monitors | On-the-go (Float2 Pro + Laptop) |
| `Super + F3` | **Laptop** | 1 monitor | Portable (Laptop only) |
| `Super + Shift + M` | **Emergency Restore** | - | Fix broken config instantly |

## Daily Workflow

### At Home (Desktop Mode)
```bash
# Monitors auto-configured on boot with last-used profile
# If needed, apply desktop profile:
Super + F1
```

**Active monitors:**
- Sceptre M27 (left, 60Hz, 1.0x scale)
- Float2 Pro (top right, 120Hz, 1.0x scale)
- MacBook Laptop (bottom right, 60Hz, 1.6x scale)

### Going to Campus (Hybrid Mode)
```bash
1. Unplug laptop from Sceptre
2. Press Super + F2 (campus profile)
3. Pack up and go!
```

**Active monitors:**
- Float2 Pro (top, 120Hz, 1.0x scale)
- MacBook Laptop (bottom, 60Hz, 1.6x scale)
- Layout: Vertical stack

### At Campus (Laptop Only)
```bash
# If not using Float2 Pro at campus:
Super + F3
```

**Active monitors:**
- MacBook Laptop only (60Hz, 1.6x scale)

### Emergency: Monitors Messed Up
```bash
# If something breaks (GUI tool overwrites config, etc.):
Super + Shift + M

# Instantly restores desktop profile
```

## Profile Details

### Desktop Profile
**File:** `~/.config/hyprmon-desc/profiles/desktop.conf`

```
Layout:
[Sceptre M27]    [Float2 Pro]
  1.0x scale       1.0x scale
  Left side        Top right
                  [Laptop]
                   1.6x scale
                  Bottom right
```

**Scales:**
- All external monitors: **1.0x** (consistent zoom)
- Laptop: **1.6x** (comfortable HiDPI viewing)

**Positions:**
- Sceptre: (0, 0)
- Float2 Pro: (1920, 0)
- Laptop: (1920, 1080)

---

### Campus Profile
**File:** `~/.config/hyprmon-desc/profiles/campus.conf`

```
Layout:
  [Float2 Pro]
    1.0x scale
      Top
  [Laptop]
   1.6x scale
    Bottom
```

**Scales:**
- Float2 Pro: **1.0x**
- Laptop: **1.6x**

**Positions:**
- Float2 Pro: (0, 0)
- Laptop: (0, 1080) - directly below Float2 Pro
- Perfect alignment (both 1920px wide)

---

### Laptop Profile
**File:** `~/.config/hyprmon-desc/profiles/laptop.conf`

```
Layout:
  [Laptop]
   1.6x scale
   Center only
```

**Scales:**
- Laptop: **1.6x**

**Position:**
- Laptop: (0, 0) - centered

**Note:** All external monitors explicitly disabled

## Why This System Works

### Reboot-Stable
- Uses **EDID descriptions** instead of connector names (DP-6, DP-7)
- Kernel can assign different DP-X numbers on each boot
- Monitors stay in correct positions regardless

### Before (Broken):
```conf
monitor=DP-6,1920x1080@60,0x0,1.0  # Might become DP-7 on next boot!
monitor=DP-7,1920x1080@120,1920x0,1.0  # Might become DP-6 on next boot!
```

### After (Stable):
```conf
monitor=desc:Sceptre Tech Inc Sceptre M27 0000000000000,1920x1080@60,0x0,1.0
monitor=desc:Invalid Vendor Codename - RTK Float2 Pro 0x01010130,1920x1080@120,1920x0,1.0
```

**Result:** Monitors ALWAYS in correct positions, regardless of DP-X assignment!

## Tools & Commands

### Profile Management

```bash
# Apply specific profile
hyprmon-desc --profile desktop
hyprmon-desc --profile campus
hyprmon-desc --profile laptop

# Show profile menu
hyprmon-desc profiles

# Show help
hyprmon-desc --help
```

### Emergency Restore

```bash
# Restore desktop profile (most common)
restore-monitor-config

# Restore specific profile
restore-monitor-config campus
restore-monitor-config laptop
```

### Monitor Information

```bash
# List all monitors with details
hyprctl monitors

# Show just monitor names and descriptions
hyprctl monitors | grep -E "(Monitor|description:)"

# Check current scales
hyprctl monitors | grep scale:
```

### Manual Configuration

```bash
# Edit profile directly
nano ~/.config/hyprmon-desc/profiles/desktop.conf

# Apply manually
cp ~/.config/hyprmon-desc/profiles/desktop.conf ~/.config/hypr/monitors.conf
hyprctl reload
```

## Adjusting Scales

If you want to tweak zoom levels for any profile:

1. **Option A: Edit profile file directly**
   ```bash
   nano ~/.config/hyprmon-desc/profiles/desktop.conf
   # Change scale values (e.g., 1.0 → 1.2 for larger UI)
   hyprmon-desc --profile desktop  # Apply changes
   ```

2. **Option B: Use hyprmon TUI**
   ```bash
   Super + M  # Opens hyprmon visual editor
   # Adjust scales with 'R' key or [ ] keys
   # Press 'A' to apply live
   # Then manually update profile file with new values
   ```

### Scale Guidelines

- **0.8x** - More screen space, smaller UI (good for large monitors)
- **1.0x** - Normal size (current for Sceptre & Float2 Pro)
- **1.2x** - Larger UI, less space (good for small/high-res monitors)
- **1.6x** - Much larger UI (current for laptop's HiDPI display)

**Current scales (optimized for consistency):**
- External monitors (Sceptre, Float2 Pro): **1.0x**
- Laptop: **1.6x**

All monitors have same effective UI zoom level!

## Troubleshooting

### Monitors in Wrong Positions After Reboot
```bash
# This shouldn't happen with desc-based configs
# But if it does:
Super + Shift + M  # Emergency restore

# Then check config:
cat ~/.config/hypr/monitors.conf
# Should see "monitor = desc:..." lines, not "monitor=DP-6,..."
```

### Profile Switching Not Working
```bash
# Check if scripts are in PATH
which hyprmon-desc restore-monitor-config

# Check profiles exist
ls ~/.config/hyprmon-desc/profiles/

# Test manually
~/.local/bin/hyprmon-desc --profile desktop
```

### Wrong Scale After Profile Switch
```bash
# Check profile file has correct scale values
cat ~/.config/hyprmon-desc/profiles/desktop.conf | grep "monitor = desc:"

# Verify what was applied
hyprctl monitors | grep scale:

# If mismatch, edit profile and reapply
nano ~/.config/hyprmon-desc/profiles/desktop.conf
hyprmon-desc --profile desktop
```

### Config Gets Overwritten
**Cause:** Using nwg-displays or other GUI tools that don't support desc-based configs

**Solution:**
- Use `Super + M` (hyprmon) instead of nwg-displays
- Emergency restore: `Super + Shift + M`
- Keybinding for nwg-displays is disabled to prevent this

**If you accidentally open nwg-displays:**
- Don't save changes
- Press `Super + Shift + M` to restore desc-based config

## Advanced: Creating Custom Profiles

```bash
# 1. Configure monitors visually with hyprmon
Super + M
# Arrange monitors, set scales, etc.
# Press 'A' to apply

# 2. Create new profile file
cp ~/.config/hyprmon-desc/profiles/desktop.conf \
   ~/.config/hyprmon-desc/profiles/mycustom.conf

# 3. Edit to use desc-based identifiers
nano ~/.config/hyprmon-desc/profiles/mycustom.conf
# Replace DP-6/DP-7 with desc:... from hyprctl monitors

# 4. Apply
hyprmon-desc --profile mycustom

# 5. (Optional) Add keybinding
# Edit ~/.config/hypr/bindings.conf:
bind = $mainMod, F4, exec, ~/.local/bin/hyprmon-desc --profile mycustom
```

## Monitor Identifiers Reference

```bash
# Find your monitor EDID descriptions:
hyprctl monitors | grep description:

# Current monitors (for reference):
Sceptre M27:
  desc:Sceptre Tech Inc Sceptre M27 0000000000000

Float2 Pro:
  desc:Invalid Vendor Codename - RTK Float2 Pro 0x01010130

MacBook Laptop:
  desc:Apple Computer Inc Color LCD
```

## Files Reference

| File | Purpose |
|------|---------|
| `~/.config/hypr/monitors.conf` | Active monitor config (loaded by Hyprland) |
| `~/.config/hyprmon-desc/profiles/desktop.conf` | Desktop profile (3 monitors) |
| `~/.config/hyprmon-desc/profiles/campus.conf` | Campus profile (2 monitors) |
| `~/.config/hyprmon-desc/profiles/laptop.conf` | Laptop profile (1 monitor) |
| `~/.local/bin/hyprmon-desc` | Profile switching script |
| `~/.local/bin/restore-monitor-config` | Emergency restore script |
| `~/.config/hypr/bindings.conf` | Keybindings configuration |

## Best Practices

1. ✅ **Use profiles for switching** - Don't manually edit monitors.conf
2. ✅ **Avoid nwg-displays** - It overwrites desc-based configs
3. ✅ **Use hyprmon TUI** for adjustments - Then convert to desc-based
4. ✅ **Test before saving** - Press 'A' in hyprmon to apply live first
5. ✅ **Keep consistent scales** - All external monitors at 1.0x recommended
6. ✅ **Emergency restore ready** - Remember Super+Shift+M if something breaks

---

**Last Updated:** 2026-01-04  
**Status:** ✅ Fully implemented and tested
