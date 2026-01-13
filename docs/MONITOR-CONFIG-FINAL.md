# Monitor Configuration - Final Setup

## Summary
Monitor configuration has been standardized across all VulcanOS locations to prevent duplicates and config errors.

**Latest Update (2026-01-03):** Implemented EDID description-based configuration to prevent DP connector swapping on reboot. Configuration is now reboot-stable.

## DP Connector Swapping Issue (SOLVED)

### Problem
When using USB-C hubs or DisplayPort MST (Multi-Stream Transport), the Linux kernel assigns connector names (DP-6, DP-7) **non-deterministically** on boot. This causes monitors to swap positions across reboots:
- Sceptre might be DP-6 on one boot, DP-7 on the next
- Float2 Pro might be DP-7 on one boot, DP-6 on the next

Using connector names in monitor configuration:
```conf
monitor=DP-6,1920x1080@60.0,0x552,0.6      # Unstable - may swap!
monitor=DP-7,1920x1080@120.0,3200x352,1.0  # Unstable - may swap!
```

### Solution: EDID Description-Based Configuration
Use Hyprland's `desc:` syntax to identify monitors by their **EDID description** (manufacturer + model name), which is hardware-based and stable:

```conf
# Stable - survives reboots regardless of DP-X connector assignment
monitor = desc:Sceptre Tech Inc Sceptre M27 0000000000000, 1920x1080@60, 0x552, 0.6
monitor = desc:Invalid Vendor Codename - RTK Float2 Pro 0x01010130, 1920x1080@120, 3200x352, 1.0
monitor = desc:Apple Computer Inc Color LCD, 3072x1920@60, 3200x1438, 1.6
```

### How to Find Monitor Descriptions
```bash
hyprctl monitors | grep -A1 "^Monitor"
# Or for just descriptions:
hyprctl monitors | grep "description:"
```

**Result:** Monitors now stay in consistent positions across reboots, regardless of which DP-X connector the kernel assigns.

## Configuration Strategy

### Single Source of Truth
All monitor definitions are in `monitors.conf` **ONLY**. The `hyprland.conf` file sources this via:
```
source = ~/.config/hypr/monitors.conf
```

### No Duplicates
- ✅ `monitors.conf` - Contains ALL monitor definitions
- ✅ `hyprland.conf` - NO monitor definitions (only sources monitors.conf)

## File Locations

### 1. Live User Config
**Path:** `~/.config/hypr/monitors.conf`
- Current user-specific configuration
- Uses EDID description-based identifiers (stable across reboots)
- Example layout (T2 MacBook Pro with 3 monitors):
  ```
  monitor = desc:Sceptre Tech Inc Sceptre M27 0000000000000, 1920x1080@60, 0x552, 0.6
  monitor = desc:Invalid Vendor Codename - RTK Float2 Pro 0x01010130, 1920x1080@120, 3200x352, 1.0
  monitor = desc:Apple Computer Inc Color LCD, 3072x1920@60, 3200x1438, 1.6
  monitor = , preferred, auto, 1
  ```

### 2. VulcanOS Dotfiles
**Path:** `~/VulcanOS/dotfiles/hypr/.config/hypr/monitors.conf`

Contains auto-detect template with desc-based example (commented out) for new users. Includes instructions on finding monitor descriptions with `hyprctl monitors`.

### 3. ISO Skeleton
**Path:** `~/VulcanOS/archiso/airootfs/etc/skel/.config/hypr/monitors.conf`

Ships with auto-detect defaults. Users customize after installation.

## Layout Example (Current Working Setup)

```
[Sceptre M27]              [Float2 Pro]
 60Hz, 0.6x scale          120Hz, 1.0x scale
 Position: (0, 552)        Position: (3200, 352)
                           [MacBook Laptop]
                            60Hz, 1.6x scale
                            Position: (3200, 1438)
```

**Physical arrangement:**
- Sceptre M27: Left side (main monitor)
- Float2 Pro: Top right
- MacBook Laptop: Bottom right

**Monitor Identifiers (Stable):**
- Sceptre: `desc:Sceptre Tech Inc Sceptre M27 0000000000000`
- Float2 Pro: `desc:Invalid Vendor Codename - RTK Float2 Pro 0x01010130`
- MacBook: `desc:Apple Computer Inc Color LCD`

**Note:** Connector names (DP-6, DP-7) may vary on each boot, but monitors stay in correct positions due to desc-based configuration.

## User Instructions

### Configure Monitors (GUI)
```bash
# Launch nwg-displays
Super+Alt+M
# or
nwg-displays
```

### Configure Monitors (CLI)
```bash
# List current monitors
hyprctl monitors

# Set monitor manually
hyprctl keyword monitor "DP-6,1920x1080@60,1920x0,1.2"

# Reload config
hyprctl reload
```

### Backup Current Config
```bash
~/monitor-config-backup.sh backup
```

## Best Practices

1. ✅ Use `desc:` syntax for multi-monitor setups to prevent DP-X swapping
2. ✅ Use `nwg-displays` GUI for easy configuration (generates connector-based config, convert to desc: afterward)
3. ✅ Monitors auto-configured on first boot via template
4. ✅ User-specific configs stay in `~/.config/hypr/monitors.conf`
5. ✅ Never commit personal monitor positions to VulcanOS repo
6. ✅ Template in ISO uses auto-detection for portability
7. ✅ Find monitor descriptions with: `hyprctl monitors | grep description:`

## Verification

```bash
# Check for duplicate definitions (should be NONE)
grep -r "^monitor=eDP-1" ~/VulcanOS/dotfiles/hypr/*.conf
grep -r "^monitor=eDP-1" ~/.config/hypr/hyprland.conf

# Should only appear in monitors.conf files
```

## Troubleshooting

### Config Errors
- Ensure monitors are ONLY defined in `monitors.conf`
- Check that `hyprland.conf` does NOT have monitor lines at the end
- Run `hyprctl reload` after changes

### GUI Tools Not Working
- Ensure `monitors.conf` exists and is writable
- nwg-displays writes to `monitors.conf` automatically
- Check that file is not marked read-only

### Monitors Not Detected
- Use `hyprctl monitors` to list available outputs
- Check cables and connections
- Try `monitor=,preferred,auto,1` as fallback

### Monitors Swap Positions After Reboot (DP-X Connector Swapping)

**Symptoms:**
- Monitors are in correct positions initially
- After reboot, monitors swap places
- DP-6 and DP-7 connector assignments change

**Cause:**
USB-C hubs and DisplayPort MST cause kernel to assign DP-X connector names non-deterministically.

**Solution:**
Convert to EDID description-based configuration:

1. **Find monitor descriptions:**
   ```bash
   hyprctl monitors | grep description:
   ```

2. **Update `~/.config/hypr/monitors.conf`** to use `desc:` syntax:
   ```conf
   # Instead of:
   monitor=DP-6,1920x1080@60,0x552,0.6
   
   # Use:
   monitor = desc:Sceptre Tech Inc Sceptre M27 0000000000000, 1920x1080@60, 0x552, 0.6
   ```

3. **Apply changes:**
   ```bash
   hyprctl reload
   ```

4. **Test with reboot:**
   Monitors should stay in same positions across multiple reboots.

**Verification:**
```bash
# Before reboot - note connector assignments
hyprctl monitors | grep -E "(Monitor DP|description:)"

# After reboot - connector names may differ, but positions should match
hyprctl monitors | grep -E "(Monitor DP|description:)"
```

---
**Last Updated:** 2026-01-03  
**Status:** ✅ Standardized across all VulcanOS locations | ✅ DP swapping issue resolved with desc-based config
