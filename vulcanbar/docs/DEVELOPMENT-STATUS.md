# VulcanBar Development Status

## Current State (2024-12-12)

VulcanBar modular mode is functional with the following status:

### Working Features

| Feature | Status | Notes |
|---------|--------|-------|
| DRM/Touch Bar display | ✅ Working | 2008x60 resolution |
| Touch input | ✅ Working | Device: "Apple Inc. Touch Bar Display Touchpad" |
| Module rendering | ✅ Working | Left/center/right layout |
| Clock module | ✅ Working | Configurable format |
| Battery module | ✅ Working | Uses charge_now/charge_full for accuracy |
| Brightness module | ✅ Working | Reads from /sys/class/backlight |
| Workspaces module | ✅ Working | Shows 5 persistent workspaces |
| Hot-reload config | ✅ Working | inotify-based |
| OLED pixel shift | ✅ Working | Burn-in protection |
| Hyprland workspace switching | ✅ Working | Socket found before privilege drop |

### Known Issues / TODO

1. **Volume module** - Not tested yet, uses PulseAudio
2. **FKeys module** - Basic implementation, needs key bindings
3. **Rendering optimization** - Currently renders every ~200ms, could be smarter
4. **libinput warnings** - "event processing lagging behind" messages (cosmetic)

### Key Implementation Details

#### Touch Bar Device Matching
The Touch Bar digitizer must match BOTH "Touch Bar" AND "Touchpad":
```rust
if dev.name().contains("Touch Bar") && dev.name().contains("Touchpad") {
    digitizer = Some(dev);
}
```

#### Battery Reading
Uses charge_now/charge_full ratio (not capacity file) to match Waybar:
```rust
let charge_now = fs::read_to_string(path.join("charge_now"))...
let charge_full = fs::read_to_string(path.join("charge_full"))...
// Returns (charge_now / charge_full) * 100
```

#### Hyprland Socket Discovery
Socket must be found BEFORE privilege drop (runs as nobody user):
```rust
// Find socket while still root
let hyprland_socket = find_hyprland_socket();
// Then drop privileges
PrivDrop::default().user("nobody")...
```

Socket locations searched:
- `/tmp/hypr/`
- `/run/user/1000/hypr/`
- `/run/user/{uid}/hypr/`

#### Workspaces Panic Protection
The hyprland crate panics if Hyprland isn't running. Protected with:
```rust
if !Self::hyprland_available() {
    return Ok(()); // Keep default buttons
}
let result = std::panic::catch_unwind(|| { ... });
```

### Configuration

Default config location: `/etc/vulcanbar/config.toml`

Example working config:
```toml
[general]
font = "JetBrains Mono Nerd Font:bold"
font-size = 28.0
enable-pixel-shift = true
show-button-outlines = true
spacing = 12

[layout]
left = ["workspaces"]
center = ["clock"]
right = ["brightness", "battery"]

[modules.clock]
format = "%H:%M  %a %d"
interval = 60

[modules.battery]
display = "both"
interval = 30

[modules.brightness]
display = "icon"
interval = 5

[modules.workspaces]
persistent-workspaces = 5
active-only = false
```

### CLI Usage

```bash
# Legacy mode (tiny-dfr compatible)
vulcanbar

# Modular mode (Waybar-style)
vulcanbar --modular

# With debug logging
RUST_LOG=vulcanbar=debug vulcanbar --modular
```

### Systemd Service

Located at: `/etc/systemd/system/vulcanbar.service`

To switch between modes, edit the ExecStart line:
```ini
# Legacy mode
ExecStart=/usr/bin/vulcanbar

# Modular mode
ExecStart=/usr/bin/vulcanbar --modular
```

### Files Modified in This Session

- `src/main.rs` - Added modular_main(), CLI parsing, touch device matching fix
- `src/modules/workspaces.rs` - Added Hyprland availability checks
- `src/modules/battery.rs` - Fixed percentage calculation
- `systemd/vulcanbar.service` - Added mode switching comments
- `config.toml` - Working example config

### Next Steps

1. Test volume module with PulseAudio
2. Implement FKeys module key actions
3. Add workspace active indicator sync with Hyprland
4. Consider running as user service for better Hyprland integration
5. Reduce debug logging verbosity in production
