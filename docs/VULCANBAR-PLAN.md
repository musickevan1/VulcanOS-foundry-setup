# VulcanBar: Touch Bar Status Daemon Implementation Plan

## Session Prompt

Use this to start a new Claude Code session:

---

I want to implement VulcanBar, a Waybar-like Touch Bar daemon for my T2 MacBook Pro. This is a complete implementation plan - please follow it phase by phase.

**Project:** `/home/evan/VulcanOS/vulcanbar/`

**Technical Context:**
- Touch Bar: 2008x60 pixels via `appletbdrm` DRM driver
- Reference: tiny-dfr (https://github.com/AsahiLinux/tiny-dfr) - fork this as base
- Language: Rust
- Config: TOML with hot-reload

Please start with Phase 1: Clone tiny-dfr and restructure into the new directory layout.

---

## Overview

Build a Waybar-like Touch Bar daemon for T2 MacBooks by forking tiny-dfr and refactoring it into a modular architecture.

**Decisions:**
- Language: Rust (fork tiny-dfr)
- Config: TOML
- Modules: Clock, Battery, Volume, Brightness, Workspaces (all basic)

## Project Location

```
/home/evan/VulcanOS/vulcanbar/
```

## Directory Structure

```
vulcanbar/
├── Cargo.toml
├── src/
│   ├── main.rs                 # Entry point, event loop
│   ├── lib.rs                  # Library root
│   ├── display/
│   │   ├── mod.rs
│   │   ├── drm_backend.rs      # DRM/KMS framebuffer (from tiny-dfr)
│   │   ├── renderer.rs         # Cairo rendering abstraction
│   │   └── compositor.rs       # Module layout compositor
│   ├── input/
│   │   ├── mod.rs
│   │   ├── touch.rs            # Touch event handling
│   │   ├── uinput.rs           # Virtual keyboard emission
│   │   └── actions.rs          # Action execution
│   ├── modules/
│   │   ├── mod.rs              # Module trait + registry
│   │   ├── base.rs             # BaseModule helper
│   │   ├── clock.rs            # Clock (polling)
│   │   ├── battery.rs          # Battery (polling)
│   │   ├── volume.rs           # Volume (PulseAudio events)
│   │   ├── brightness.rs       # Brightness (polling)
│   │   ├── workspaces.rs       # Hyprland IPC
│   │   └── fkeys.rs            # Static F-keys
│   ├── config/
│   │   ├── mod.rs
│   │   ├── schema.rs           # Config structs
│   │   └── loader.rs           # TOML + hot-reload
│   └── events/
│       ├── mod.rs
│       └── epoll.rs            # Event multiplexer
├── share/vulcanbar/
│   └── config.toml             # Default config
└── systemd/
    └── vulcanbar.service
```

## Implementation Phases

### Phase 1: Project Setup & Fork
1. Clone tiny-dfr to `/home/evan/VulcanOS/vulcanbar/`
2. Restructure into new directory layout
3. Update `Cargo.toml` with dependencies:
   - Keep: `drm`, `cairo-rs`, `librsvg-rebind`, `input`, `nix`, `toml`, `serde`, `chrono`
   - Add: `crossbeam-channel`, `libpulse-binding`, `hyprland`, `anyhow`, `thiserror`, `tracing`
4. Verify it compiles with existing tiny-dfr code

### Phase 2: Module Trait Architecture
1. Create `Module` trait in `src/modules/mod.rs`:
   ```rust
   pub trait Module: Send {
       fn name(&self) -> &str;
       fn width(&self) -> i32;
       fn render(&self, ctx: &RenderContext) -> Result<()>;
       fn on_touch(&mut self, event: TouchEvent) -> Option<Action>;
       fn update_interval(&self) -> Option<Duration>;
       fn update(&mut self) -> Result<bool>;
       fn start_listener(&mut self, tx: Sender<ModuleEvent>) -> Result<Option<i32>>;
   }
   ```
2. Create `ModuleRegistry` with factory pattern
3. Create `BaseModule` helper with common rendering utilities
4. Port tiny-dfr's F-key functionality to `FKeysModule`

### Phase 3: Configuration System
1. Define `Config` schema in `src/config/schema.rs`:
   - `GeneralConfig`: font, pixel_shift, brightness, spacing
   - `LayoutConfig`: left/center/right module arrays
   - Per-module configs: `ClockConfig`, `BatteryConfig`, etc.
2. Implement TOML loader with system/user config merge
3. Add inotify hot-reload via `ConfigManager`

**Example config:**
```toml
[general]
font = "JetBrains Mono Nerd Font"
enable-pixel-shift = true

[layout]
left = ["workspaces"]
center = ["clock"]
right = ["volume", "brightness", "battery"]

[modules.clock]
format = "%H:%M"
interval = 60

[modules.workspaces]
persistent-workspaces = 5
```

### Phase 4: Core Modules (Polling)
1. **ClockModule** (`src/modules/clock.rs`)
   - Polling interval: configurable (default 60s)
   - Format: strftime string
   - Simplest module - implement first

2. **BatteryModule** (`src/modules/battery.rs`)
   - Read from `/sys/class/power_supply/BAT*/`
   - Polling interval: 30s
   - Color coding: green (charging), red (<20%), white (normal)

3. **BrightnessModule** (`src/modules/brightness.rs`)
   - Read from `/sys/class/backlight/*/brightness`
   - Polling interval: 5s

### Phase 5: Event-Driven Modules
1. **VolumeModule** (`src/modules/volume.rs`)
   - Use `libpulse-binding` for PulseAudio/PipeWire events
   - Subscribe to sink changes
   - Touch action: toggle mute

2. **WorkspacesModule** (`src/modules/workspaces.rs`)
   - Use `hyprland` crate for IPC
   - Listen to workspace change events via `EventListener`
   - Touch action: switch workspace

### Phase 6: Compositor & Main Loop
1. **Compositor** (`src/display/compositor.rs`)
   - Layout modules: left (fixed), center (stretch), right (fixed)
   - Calculate module positions and widths
   - Map touch coordinates to modules

2. **Main Event Loop** (`src/main.rs`)
   - epoll-based async I/O (from tiny-dfr)
   - Module update channel (`crossbeam-channel`)
   - Rate-limited rendering (~60fps max)
   - Pixel shift for OLED longevity

### Phase 7: Integration & Polish
1. Create systemd service file
2. Add to VulcanOS autostart (`dotfiles/hypr/.config/hypr/autostart.conf`)
3. Error handling and logging (`tracing`)
4. Documentation

## Key Dependencies (Cargo.toml)

```toml
[dependencies]
drm = "0.14"
cairo-rs = { version = "0.20", features = ["freetype", "png"] }
librsvg-rebind = "0.1"
input = "0.8"
nix = { version = "0.29", features = ["event", "signal", "inotify", "socket"] }
serde = { version = "1", features = ["derive"] }
toml = "0.8"
chrono = { version = "0.4", features = ["unstable-locales"] }
crossbeam-channel = "0.5"
libpulse-binding = "2.0"
hyprland = { version = "0.4", default-features = false, features = ["sync"] }
anyhow = "1"
tracing = "0.1"
```

## Critical Reference Files

| Purpose | File |
|---------|------|
| DRM rendering reference | tiny-dfr `src/display.rs` |
| Event loop reference | tiny-dfr `src/main.rs` |
| Touch handling reference | tiny-dfr `src/main.rs:675-952` |
| Config pattern reference | tiny-dfr `src/config.rs` |
| Waybar config example | `dotfiles/waybar/.config/waybar/config.jsonc` |
| Autostart integration | `dotfiles/hypr/.config/hypr/autostart.conf` |

## Testing Strategy

1. Test on actual T2 hardware (not QEMU - Touch Bar requires real hardware)
2. Start with FKeysModule to verify DRM rendering works
3. Add modules incrementally, testing each
4. Test hot-reload by editing config while running

## Deliverables

1. Working `vulcanbar` binary
2. Default configuration file
3. Systemd service file
4. Integration with VulcanOS dotfiles
5. README with usage instructions

---

## tiny-dfr Architecture Reference

### Key Structs (from analysis)

**DrmBackend** (`display.rs`):
- `Card` - DRM device file handle
- `DumbBuffer` - Memory-mapped framebuffer
- `Mode` - Display resolution (2008x60)
- Uses atomic KMS for mode setting
- `dirty()` sends ClipRects for partial updates

**Rendering Pipeline**:
1. Cairo `ImageSurface` (ARGB32) with 90° rotation
2. `ButtonImage` enum: Text, Svg, Bitmap, Time, Battery
3. `Button::render()` draws to Cairo context
4. `FunctionLayer::draw()` orchestrates render cycle
5. Copy surface data to DRM framebuffer
6. Call `dirty()` for changed regions

**Touch Handling**:
- libinput on seat "seat-touchbar"
- `x_transformed()` / `y_transformed()` for coords
- `FunctionLayer::hit()` maps coords to button index
- uinput virtual keyboard for key emission

**Event Loop** (epoll-based):
- Config file watching (inotify)
- Touch input events
- Main keyboard events (Fn key)
- Timeout-based updates (time display, pixel shift)

### Key Code Locations in tiny-dfr

- DRM init: `display.rs:180` (`open_card()`)
- Framebuffer setup: `display.rs:96-99`
- Button rendering: `main.rs:338-437` (`Button::render()`)
- Touch events: `main.rs:909-946`
- Hit detection: `main.rs:634-673` (`FunctionLayer::hit()`)
- Main loop: `main.rs:825-951`
- Config loading: `config.rs:66-117`
