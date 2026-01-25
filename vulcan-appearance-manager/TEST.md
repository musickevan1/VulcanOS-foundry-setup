# Wallpaper Manager Testing

## Prerequisites

1. **Hyprpaper must be running:**
   ```bash
   pgrep hyprpaper || hyprpaper &
   ```

2. **Test wallpapers in ~/Pictures/Wallpapers:**
   ```bash
   ls ~/Pictures/Wallpapers/*.{png,jpg}
   ```

## Running the Application

```bash
cargo run
# or
cargo run --release
```

## Manual Test Checklist

### Window Layout
- [ ] Application window opens (1000x700)
- [ ] Header bar shows "Wallpaper Manager" title
- [ ] Subtitle shows "Select a monitor" initially
- [ ] Split pane: Monitor layout on top, wallpaper picker on bottom
- [ ] Refresh button in header bar (top right)
- [ ] Open folder button in header bar (top left)

### Monitor Layout
- [ ] All monitors render correctly in layout visualization
- [ ] Monitors positioned correctly relative to each other
- [ ] Monitor names displayed
- [ ] Clicking a monitor highlights it in blue
- [ ] Clicking a monitor updates subtitle to show monitor name

### Wallpaper Picker
- [ ] Thumbnails load for wallpapers in ~/Pictures/Wallpapers
- [ ] Grid layout displays 2-4 columns depending on width
- [ ] Scrollable if many wallpapers
- [ ] Clicking a wallpaper selects it

### Apply Functionality
- [ ] Apply button is disabled initially
- [ ] Apply button enables after selecting monitor + wallpaper
- [ ] Apply button has "suggested-action" style (blue)
- [ ] Clicking Apply changes wallpaper on selected monitor
- [ ] Console shows: "Applied {path} to {monitor}"

### Refresh
- [ ] Refresh button reloads monitor list
- [ ] Refresh button rescans wallpaper directory

### Open Directory
- [ ] Clicking folder button opens ~/Pictures/Wallpapers in file manager

## Verification

After clicking Apply, verify wallpaper changed:
```bash
# Visual: Check if wallpaper changed on screen
# IPC: Check hyprctl (if supported by your hyprpaper version)
hyprctl hyprpaper wallpaper {monitor},{path}
```

## Known Limitations

- Hyprpaper must be running before launching the app
- No error dialog if hyprpaper is not available
- Synchronous thumbnail generation may cause brief UI freeze
