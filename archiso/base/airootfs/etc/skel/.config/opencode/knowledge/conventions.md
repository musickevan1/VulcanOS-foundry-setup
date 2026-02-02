# VulcanOS Conventions

## Script Naming
- All VulcanOS scripts: `vulcan-*` prefix
- Location: `~/.local/bin/` or `/usr/local/bin/`
- Must be executable: `chmod +x`

## Directory Structure

### Dotfiles (GNU Stow)
```
dotfiles/
├── hypr/.config/hypr/       # Hyprland config
├── waybar/.config/waybar/   # Waybar config
├── kitty/.config/kitty/     # Terminal config
├── opencode/.config/opencode/ # OpenCode config
├── scripts/                  # User scripts (symlink to ~/.local/bin)
└── themes/                   # Theme system
```

### ISO Build
```
archiso/
├── airootfs/etc/skel/       # Default user home
├── airootfs/usr/local/bin/  # System scripts
└── packages.x86_64          # Package manifest
```

## Configuration Patterns

### Hyprland Modular Config
- `hyprland.conf` - Main file, sources others
- `bindings.conf` - Keybindings
- `looknfeel.conf` - Visual appearance
- `autostart.conf` - Startup applications

### Keybinding Conventions
- `Super + Space` - App launcher
- `Super + Return` - Terminal
- `Super + Shift + Letter` - Launch specific apps
- `Super + Number` - Workspaces
- `Super + Arrow` - Window focus

## Shell Script Template
```bash
#!/bin/bash
# Description of script
# Usage: script-name [options]

set -euo pipefail

main() {
    # Implementation
}

main "$@"
```

## Git Workflow
- Conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`
- Test changes locally before committing
- Keep commits atomic and focused
