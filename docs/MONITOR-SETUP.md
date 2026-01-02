# Monitor Configuration Guide

VulcanOS provides a complete monitor configuration system with both TUI and GUI tools, per-monitor wallpaper support, and profile management.

## Tools Overview

| Tool | Type | Purpose | Access |
|-------|-------|---------|---------|
| **hyprmon** | TUI (Terminal) | Primary tool for quick configuration, profile management, advanced settings | `Super + M` |
| **nwg-displays** | GUI (Graphical) | Visual monitor arrangement and workspace assignments | `Super + Alt + M` |
| **vulcan-wallpapers** | CLI + Keybindings | Per-monitor wallpaper profile management | `Super + Shift + W` |

## Quick Start

### 1. Configure Monitors with hyprmon

1. Press `Super + M` to open HyprMon TUI
2. See your monitors as visual boxes in a spatial map
3. Use arrow keys or mouse to arrange monitors
4. Press `R` to open scale selector (common DPI values: 0.5x to 3.0x)
5. Press `F` to open resolution & refresh rate picker
6. Press `A` to apply changes live to Hyprland
7. Press `P` to save layout as named profile (e.g., "3-monitor", "laptop-only")

### 2. Fine-tune with nwg-displays

1. Press `Super + Alt + M` to open nwg-displays GUI
2. Drag and drop monitor boxes to arrange visually
3. Click monitor boxes to select and configure
4. Configure scale, position, and resolution
5. Apply settings and save to config

### 3. Manage Wallpaper Profiles

1. Set wallpapers per monitor:
   ```bash
   vulcan-wallpapers set eDP-1 ~/Pictures/Wallpapers/laptop.png
   vulcan-wallpapers set DP-4 ~/Pictures/Wallpapers/desktop.png
   vulcan-wallpapers set DP-5 ~/Pictures/Wallpapers/side.png
   ```

2. Save current wallpaper setup as profile:
   ```bash
   vulcan-wallpapers save my-setup
   ```

3. Apply wallpaper profile:
   ```bash
   vulcan-wallpapers apply 3-monitor
   ```

## Keybindings

### Monitor Configuration

| Keybinding | Action |
|------------|--------|
| `Super + M` | Open HyprMon TUI (primary tool) |
| `Super + Alt + M` | Open nwg-displays GUI (for visual tweaking) |
| `Super + F1` | Apply "3-monitor" monitor profile |
| `Super + F2` | Apply "laptop-only" monitor profile |
| `Super + F3` | Open profile selection menu |
| `Super + Shift + W` | Rotate through wallpaper profiles |
| `Super + Alt + W` | Reload current wallpapers (no profile change) |

### Access via Vulcan Menu

Press `Super + Alt + Space` → **Display Settings** → Choose:
1. HyprMon (TUI)
2. nwg-displays (GUI)
3. Wallpaper Profiles

## Per-Monitor Wallpapers

VulcanOS supports different wallpapers for each monitor. Configure them via:

### Method 1: CLI (vulcan-wallpapers)

```bash
# Set wallpaper for specific monitor
vulcan-wallpapers set eDP-1 ~/Pictures/Wallpapers/laptop.png
vulcan-wallpapers set DP-4 ~/Pictures/Wallpapers/desktop.png

# Save current setup as profile
vulcan-wallpapers save my-3-monitor-setup

# Apply profile
vulcan-wallpapers apply my-3-monitor-setup

# List all profiles
vulcan-wallpapers list
```

### Method 2: Edit profiles.toml

Edit `~/.config/hypr/wallpapers/profiles.toml`:

```toml
[profile.3-monitor]
description = "My 3-monitor setup"
eDP-1 = "/home/evan/Pictures/Wallpapers/laptop.png"
DP-4 = "/home/evan/Pictures/Wallpapers/desktop.png"
DP-5 = "/home/evan/Pictures/Wallpapers/side.png"
```

Then apply: `vulcan-wallpapers apply 3-monitor`

### Method 3: Edit hyprpaper.conf directly

Edit `~/.config/hypr/hyprpaper.conf`:

```conf
preload = /path/to/laptop.png
preload = /path/to/desktop.png
preload = /path/to/side.png

wallpaper = eDP-1,/path/to/laptop.png
wallpaper = DP-4,/path/to/desktop.png
wallpaper = DP-5,/path/to/side.png

splash = false
```

Reload: `vulcan-wallpapers reload`

## Monitor Profiles

hyprmon provides monitor layout profiles that save:
- Monitor positions (x, y coordinates)
- Scale factors (DPI scaling)
- Resolutions and refresh rates
- Advanced settings (HDR, color mode, VRR, rotation)
- Mirroring relationships

### Creating Profiles

1. Open hyprmon: `Super + M`
2. Arrange your monitors
3. Configure scale, resolution, etc.
4. Press `P` to save as profile
5. Enter profile name (e.g., "work", "home", "presentation")

### Applying Profiles

**Keybindings:**
- `Super + F1` → Apply "3-monitor" profile
- `Super + F2` → Apply "laptop-only" profile
- `Super + F3` → Show profile selection menu

**CLI:**
```bash
# Apply specific profile
hyprmon --profile work

# Show profile menu
hyprmon profiles
```

### Default Profiles

VulcanOS includes these default profiles:

| Profile | Description |
|---------|-------------|
| `3-monitor` | Full 3-monitor docked setup |
| `laptop-only` | Laptop only, external displays disabled |
| `presentation` | Mirrored to external for presentations |
| `dark-mode` | Dark wallpapers for nighttime use |
| `minimal` | Minimalist single wallpaper across all monitors |

## Wallpaper Profiles

vulcan-wallpapers manages wallpaper profiles that can be linked to monitor profiles.

### Profile Workflow

1. Configure monitors with hyprmon: `Super + M`
2. Set wallpapers with vulcan-wallpapers:
   ```bash
   vulcan-wallpapers set eDP-1 ~/Pictures/Wallpapers/laptop.png
   vulcan-wallpapers set DP-4 ~/Pictures/Wallpapers/desktop.png
   ```
3. Save wallpaper profile: `vulcan-wallpapers save 3-monitor`
4. Switch profiles with keybinding: `Super + Shift + W`

### Profile Synchronization

Monitor profiles (hyprmon) and wallpaper profiles (vulcan-wallpapers) are separate:

- **Monitor profiles**: Save display layout, scale, resolution, etc.
- **Wallpaper profiles**: Save wallpaper assignments per monitor

For seamless switching, use same profile names for both:
```bash
# Apply monitor layout
hyprmon --profile work

# Apply matching wallpapers
vulcan-wallpapers apply work
```

## Advanced Monitor Settings

### hyprmon Advanced Dialog (Press `C` or `D`)

Access advanced display settings for each monitor:

#### Color Settings
- **Color Depth**: 8-bit or 10-bit
- **Color Mode**: Auto, sRGB, Wide, HDR, or HDR-EDID
- **SDR Controls**: Brightness (0.5-2.0) and Saturation (0.5-1.5) for HDR

#### Display Features
- **VRR** (Variable Refresh Rate): Off, On, or Fullscreen-only
- **Transform**: Normal, 90°, 180°, or 270° rotation

#### Mirroring

Press `M` in hyprmon to configure monitor mirroring:
- Select a source monitor to mirror from
- Visual indicators show mirror relationships
- Automatic circular dependency prevention

### Keyboard Controls (hyprmon TUI)

| Key | Action |
|-----|--------|
| `↑↓←→` or `hjkl` | Move selected monitor |
| `Shift+↑↓←→` | Move by 10× grid size |
| `Tab` / `Shift+Tab` | Cycle through monitors |
| `G` | Change grid size (1, 8, 16, 32, 64 px) |
| `L` | Toggle snap mode (Off, Edges, Centers, Both) |
| `R` | Open scale selector with common DPI values |
| `F` | Open resolution & refresh rate mode picker |
| `[` / `]` | Decrease/Increase scale by 0.05 |
| `Enter` or `Space` | Toggle monitor active/inactive |
| `C` or `D` | Open advanced display settings dialog |
| `M` | Open monitor mirroring configuration |
| `A` | Apply changes live to Hyprland |
| `S` | Save changes to configuration file |
| `P` | Save current layout as named profile |
| `Z` | Revert to previous configuration |
| `Q` or `Ctrl+C` | Quit |

## Workspace Assignment

### nwg-displays Workspace Assignment

nwg-displays can save workspace→output assignments to `~/.config/hypr/workspaces.conf`:

```conf
workspace 1 output eDP-1
workspace 2 output eDP-1
workspace 3 output eDP-1
workspace 4 output DP-4
workspace 5 output DP-4
workspace 6 output DP-5
workspace 7 output DP-5
workspace 8 output eDP-1
workspace 9 output DP-4
workspace 10 output DP-5
```

**Source this in hyprland.conf:**
```conf
source = ~/.config/hypr/workspaces.conf
```

### Manual Workspace Assignment

Add to `~/.config/hypr/bindings.conf`:

```conf
# Assign workspaces 1-3 to laptop (eDP-1)
workspace 1, monitor:eDP-1
workspace 2, monitor:eDP-1
workspace 3, monitor:eDP-1

# Assign workspaces 4-5 to desktop (DP-4)
workspace 4, monitor:DP-4
workspace 5, monitor:DP-4

# Assign workspaces 6-7 to side monitor (DP-5)
workspace 6, monitor:DP-5
workspace 7, monitor:DP-5
```

## Troubleshooting

### Monitors Not Detected

**hyprmon:**
- Ensure `hyprctl` is available: `which hyprctl`
- Verify Hyprland is running
- Install `wlr-randr` for additional detection: `pacman -S wlr-randr`

**nwg-displays:**
- Check if running in Wayland session
- Verify dependencies installed: `pacman -S gtk-layer-shell gtk3 python-gobject python-i3ipc`

### Changes Not Persisting

**Monitor settings:**
- Check write permissions for `~/.config/hypr/monitors.conf`
- Verify config path: `echo $HYPRLAND_CONFIG`
- Check if symlinks are correct

**Wallpaper settings:**
- Check if `hyprpaper` is running: `pgrep -x hyprpaper`
- Reload wallpapers: `vulcan-wallpapers reload`
- Check wallpaper paths exist

### Wallpaper Images Not Found

```bash
# Check profile file
cat ~/.config/hypr/wallpapers/profiles.toml

# Test paths
ls -la ~/Pictures/Wallpapers/

# Rebuild profile if needed
vulcan-wallpapers init
```

### Profile Switching Issues

**hyprmon:**
```bash
# List profiles
hyprmon profiles

# Apply directly
hyprmon --profile 3-monitor

# View logs
journalctl --user -u hyprland
```

**vulcan-wallpapers:**
```bash
# List profiles
vulcan-wallpapers list

# Reinitialize from current setup
vulcan-wallpapers init
```

### hyprpaper Issues

**Reload hyprpaper:**
```bash
# Kill existing instance
pkill -x hyprpaper

# Start manually
hyprpaper &

# Or use script
vulcan-wallpapers reload
```

**Check configuration:**
```bash
# View current config
cat ~/.config/hypr/hyprpaper.conf

# Test specific wallpaper
hyprctl hyprpaper wallpaper "eDP-1,/path/to/wallpaper.png"
```

## Configuration Files

| File | Purpose | Managed By |
|-------|---------|-------------|
| `~/.config/hypr/monitors.conf` | Monitor positions, scales, resolutions | hyprmon |
| `~/.config/hypr/hyprpaper.conf` | Wallpaper preloads and assignments | vulcan-wallpapers |
| `~/.config/hypr/workspaces.conf` | Workspace → monitor assignments | nwg-displays |
| `~/.config/hypr/wallpapers/profiles.toml` | Wallpaper profile definitions | vulcan-wallpapers |
| `~/.config/hypr/wallpapers/current-profile.txt` | Current wallpaper profile | vulcan-wallpapers rotate |
| `~/.config/nwg-displays/config` | nwg-displays settings | nwg-displays |
| `~/.config/hyprland.conf` | Main Hyprland config (sources above) | User/HyprMon |

## Integration with Other Tools

### Theme System

VulcanOS theme system includes wallpaper rotation:
```bash
# Cycle through wallpaper themes
vulcan-theme rotate

# Note: This rotates wallpaper themes, not monitor profiles
```

### Screenshot Tool

Wallpaper directory is used by screenshot tool:
```bash
# Screenshots saved to ~/Pictures/Screenshots
vulcan-screenshot region
vulcan-screenshot screen
```

### Idle/Lock System

Hypridle manages screen locking based on idle time:
```bash
# Configuration: ~/.config/hypr/hypridle.conf
# Toggle auto-lock: Super + Ctrl + I
```

## Best Practices

1. **Name Profiles Consistently**: Use same names for monitor and wallpaper profiles (e.g., "work", "home")

2. **Test Before Saving**: Apply changes live with `A` in hyprmon before saving

3. **Use Snap Mode**: Enable edge snapping (press `L`) for easy monitor alignment

4. **Backup Configs**: hyprmon creates timestamped backups automatically

5. **Rollback Safe**: Press `Z` in hyprmon to revert if something breaks

6. **Per-Monitor Scaling**: Set appropriate scale for each monitor (laptop 2.0x, external 1.0-1.6x)

7. **Workspace Organization**: Assign workspaces logically (1-3 laptop, 4-5 desktop, 6-7 side)

## Resources

- **HyprMon**: https://github.com/erans/hyprmon
- **nwg-displays**: https://github.com/nwg-piotr/nwg-displays
- **Hyprland Monitors**: https://wiki.hypr.land/Configuring/Monitors/
- **Hyprland Wiki**: https://wiki.hypr.land/
- **VulcanOS Docs**: https://github.com/yourusername/VulcanOS/tree/main/docs

## Getting Help

```bash
# HyprMon help
hyprmon --help

# vulcan-wallpapers help
vulcan-wallpapers help

# Check Hyprland status
hyprctl monitors

# Check active wallpaper config
hyprctl hyprpaper listloaded
```
