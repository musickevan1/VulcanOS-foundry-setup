# VulcanBar

A Waybar-like Touch Bar daemon for T2 MacBook Pro running Linux.

## Overview

VulcanBar provides a modular, configurable Touch Bar experience similar to Waybar's approach. It's designed specifically for T2 MacBook Pro hardware with the `appletbdrm` DRM driver.

Based on [tiny-dfr](https://github.com/AsahiLinux/tiny-dfr) by AsahiLinux.

## Features

- **Modular Architecture**: Plug-and-play modules for different functionality
- **Waybar-style Config**: Familiar TOML configuration format
- **Hot Reload**: Configuration changes apply without restart
- **OLED Protection**: Pixel shifting to prevent burn-in
- **Hyprland Integration**: Native workspace switching support

## Modules

| Module | Description | Type |
|--------|-------------|------|
| `fkeys` | Function keys (F1-F12) and media keys | Static |
| `clock` | Current time with strftime format | Polling |
| `battery` | Battery percentage and status | Polling |
| `brightness` | Screen brightness level | Polling |
| `volume` | Audio volume via PulseAudio | Event |
| `workspaces` | Hyprland workspace switcher | Event |

## Installation

### From Source

```bash
# Build
cargo build --release

# Install binary
sudo install -Dm755 target/release/vulcanbar /usr/bin/vulcanbar

# Install default config
sudo mkdir -p /usr/share/vulcanbar /etc/vulcanbar
sudo install -Dm644 share/vulcanbar/vulcanbar.toml /usr/share/vulcanbar/config.toml

# Install systemd service
sudo install -Dm644 systemd/vulcanbar.service /etc/systemd/system/vulcanbar.service

# Enable service (legacy mode by default)
sudo systemctl enable --now vulcanbar

# To use modular mode, edit the service:
sudo sed -i 's|ExecStart=/usr/bin/vulcanbar|ExecStart=/usr/bin/vulcanbar --modular|' /etc/systemd/system/vulcanbar.service
sudo systemctl daemon-reload
sudo systemctl restart vulcanbar
```

### Quick Update (after code changes)

```bash
cargo build --release
sudo systemctl stop vulcanbar
sudo cp target/release/vulcanbar /usr/bin/vulcanbar
sudo systemctl start vulcanbar
```

### Configuration

Create `/etc/vulcanbar/config.toml` or `~/.config/vulcanbar/config.toml`:

```toml
[general]
font = "JetBrains Mono Nerd Font:bold"
font-size = 32.0
enable-pixel-shift = true
show-button-outlines = true
adaptive-brightness = true
active-brightness = 128
spacing = 16

[layout]
left = ["workspaces"]
center = ["clock"]
right = ["volume", "brightness", "battery"]

[modules.clock]
format = "%H:%M"
interval = 60

[modules.battery]
display = "both"  # "icon", "percentage", or "both"
interval = 30
low-threshold = 20
critical-threshold = 10

[modules.volume]
display = "icon"
on-click = "toggle-mute"

[modules.brightness]
display = "icon"
interval = 5

[modules.workspaces]
persistent-workspaces = 5
active-only = false
```

## Hardware Requirements

- T2 MacBook Pro (2018-2020)
- Linux kernel with `apple-bce` and `appletbdrm` drivers
- Touch Bar hardware active

## Dependencies

- cairo
- libinput
- freetype
- fontconfig
- librsvg 2.59+
- libpulse (for volume module)
- uinput enabled in kernel config

## Architecture

```
vulcanbar/
├── src/
│   ├── main.rs           # Entry point (legacy tiny-dfr)
│   ├── lib.rs            # Library root
│   ├── config/           # Configuration system
│   │   ├── schema.rs     # Config structs
│   │   └── new_loader.rs # TOML loader + hot-reload
│   ├── display/          # Rendering
│   │   ├── drm_backend.rs    # DRM framebuffer
│   │   ├── compositor.rs     # Module layout
│   │   └── pixel_shift.rs    # OLED protection
│   ├── events/           # Event handling
│   │   └── epoll.rs      # Event multiplexer
│   └── modules/          # Touch Bar modules
│       ├── mod.rs        # Module trait + registry
│       ├── fkeys.rs
│       ├── clock.rs
│       ├── battery.rs
│       ├── brightness.rs
│       ├── volume.rs
│       └── workspaces.rs
├── share/vulcanbar/      # Default assets
└── systemd/              # Service files
```

## Development

```bash
# Check
cargo check

# Build debug
cargo build

# Build release
cargo build --release

# Run with logging
RUST_LOG=vulcanbar=debug cargo run
```

## License

MIT AND Apache-2.0

### Credits

- Copyright The Asahi Linux Contributors (tiny-dfr)
- Icons from Google's [material-design-icons](https://github.com/google/material-design-icons) (Apache 2.0)
- Part of [VulcanOS](https://github.com/VulcanOS/VulcanOS)
