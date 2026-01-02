# Monitor Configuration - Final Setup

## Summary
Monitor configuration has been standardized across all VulcanOS locations to prevent duplicates and config errors.

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
- Updated by nwg-displays GUI
- Example layout (T2 MacBook Pro with 3 monitors):
  ```
  monitor=eDP-1,3072x1920@60.0,2404x900,1.9
  monitor=DP-6,1920x1080@60.0,2404x0,1.2
  monitor=DP-7,1920x1080@120.0,0x268,0.8
  ```

### 2. VulcanOS Dotfiles
**Paths:**
- `~/VulcanOS/dotfiles/hypr/monitors.conf`
- `~/VulcanOS/dotfiles/hypr/.config/hypr/monitors.conf`

Contains auto-detect template with example commented out for new users.

### 3. ISO Skeleton
**Path:** `~/VulcanOS/archiso/airootfs/etc/skel/.config/hypr/monitors.conf`

Ships with auto-detect defaults. Users customize after installation.

## Layout Example (Current Working Setup)

```
[DP-7: Sceptre]  [DP-6: Float2 Pro]
 0.8x scale       1.2x scale
                 [eDP-1: MacBook]
                  1.9x scale
```

**Physical positions:**
- DP-7: (0, 268) - Left side
- DP-6: (2404, 0) - Top right
- eDP-1: (2404, 900) - Bottom right

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

1. ✅ Use `nwg-displays` GUI for easy configuration
2. ✅ Monitors auto-configured on first boot via template
3. ✅ User-specific configs stay in `~/.config/hypr/monitors.conf`
4. ✅ Never commit personal monitor positions to VulcanOS repo
5. ✅ Template in ISO uses auto-detection for portability

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

---
**Last Updated:** 2026-01-02  
**Status:** ✅ Standardized across all VulcanOS locations
