# VulcanOS Technology Stack

## Operating System
- **Base**: Arch Linux (rolling release)
- **Kernel**: linux-t2 (patched for T2 MacBook support)
- **Init**: systemd

## Desktop Environment
- **Compositor**: Hyprland (Wayland)
- **Display Manager**: greetd + tuigreet
- **Status Bar**: Waybar
- **Launcher**: wofi
- **Notifications**: swaync
- **Lock Screen**: hyprlock
- **Idle Manager**: hypridle

## Development Tools
- **Terminal**: kitty (GPU accelerated)
- **Shell**: bash + starship prompt
- **Editor**: neovim (with LSP) + VS Code
- **VCS**: git + lazygit + gh CLI
- **Containers**: Docker + podman

## Languages & Runtimes
- **Rust**: rustup + rust-analyzer
- **Go**: go + gopls
- **Python**: pyenv + pyright
- **Node.js**: nvm + pnpm
- **Lua**: lua-language-server

## T2 MacBook Hardware
- **WiFi**: apple-bcm-firmware + brcmfmac
- **Audio**: apple-t2-audio-config + PipeWire
- **Keyboard/Trackpad**: apple-bce
- **Touch Bar**: tiny-dfr
- **Fan Control**: t2fanrd

## Package Management
- **Official**: pacman
- **AUR**: yay
- **Dotfiles**: GNU Stow

## Key Repositories
- arch-mact2: T2 kernel and hardware packages
- AUR: Community packages
