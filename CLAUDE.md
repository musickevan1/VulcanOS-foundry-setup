# VulcanOS - Custom Arch Linux Distribution

A development-focused, opinionated Arch Linux distribution designed for T2 MacBook Pro (2019) hardware with Hyprland compositor.

## Project Overview

VulcanOS is a custom Arch Linux-based distribution built using archiso, targeting developers who need:
- Full T2 MacBook Pro hardware support (WiFi, audio, keyboard, trackpad, Touch Bar)
- Modern Wayland compositor (Hyprland) with tiling window management
- Pre-configured development environment for web, systems, Python/ML, and design work
- Minimal, fast, and keyboard-driven workflow

## Directory Structure

```
VulcanOS/
├── CLAUDE.md                   # This file - project reference
├── archiso/                    # ISO build system (archiso profile)
│   ├── airootfs/              # Root filesystem overlay
│   │   ├── etc/               # System configuration
│   │   │   ├── pacman.d/      # Pacman hooks and mirrors
│   │   │   ├── systemd/       # Systemd service links
│   │   │   ├── skel/          # Default user skeleton
│   │   │   ├── modprobe.d/    # Kernel module configs
│   │   │   ├── mkinitcpio.conf
│   │   │   ├── locale.gen
│   │   │   ├── locale.conf
│   │   │   ├── vconsole.conf
│   │   │   └── hostname
│   │   ├── root/              # Root user home (install scripts)
│   │   └── usr/local/bin/     # Custom scripts
│   ├── efiboot/               # UEFI boot configuration
│   ├── grub/                  # GRUB bootloader config
│   ├── syslinux/              # BIOS boot configuration
│   ├── packages.x86_64        # Package manifest
│   ├── pacman.conf            # Package manager configuration
│   └── profiledef.sh          # Build profile metadata
├── dotfiles/                   # Pre-configured user dotfiles
│   ├── hypr/                  # Hyprland configuration
│   ├── waybar/                # Status bar configuration
│   ├── alacritty/             # Terminal configuration
│   ├── wofi/                  # Launcher configuration
│   ├── swaync/                # Notification center
│   ├── nvim/                  # Neovim configuration
│   ├── git/                   # Git configuration
│   └── bash/                  # Shell configuration
├── scripts/                    # Build and utility scripts
│   ├── build.sh               # Main build script
│   ├── build-aur-repo.sh      # AUR package builder
│   ├── prepare.sh             # Pre-build preparation
│   ├── test-iso.sh            # ISO testing with QEMU
│   └── install.sh             # Post-install configuration
├── customrepo/                 # Custom/AUR package repository
│   └── x86_64/                # Built packages and database
├── docs/                       # Documentation
│   ├── INSTALL.md             # Installation guide
│   ├── KEYBINDINGS.md         # Keyboard shortcuts reference
│   ├── T2-SUPPORT.md          # T2-specific documentation
│   └── DEVELOPMENT.md         # Development environment guide
├── branding/                   # Custom branding assets
│   ├── logos/
│   ├── wallpapers/
│   └── themes/
└── VERSION                     # Version information
```

## Building the ISO

### Prerequisites

```bash
# On Arch Linux host system
sudo pacman -S archiso git squashfs-tools

# For AUR package building
sudo pacman -S base-devel
```

### Quick Build

```bash
cd /home/evan/NewOS
./scripts/prepare.sh          # Download resources, build AUR packages
./scripts/build.sh            # Build the ISO
```

### Build Process Details

1. **prepare.sh** - Prepares build environment
   - Downloads T2 firmware files
   - Builds AUR packages (yay, tiny-dfr, etc.)
   - Creates local package repository
   - Downloads/caches necessary resources

2. **build-aur-repo.sh** - Builds AUR packages
   - Clones AUR repositories
   - Builds packages with makepkg
   - Creates repo-add database

3. **build.sh** - Main ISO builder
   - Invokes mkarchiso with custom profile
   - Uses work directory at /tmp/archiso-work
   - Outputs ISO to ./out/

4. **test-iso.sh** - Validates ISO
   - Boots ISO in QEMU (BIOS and UEFI modes)
   - Basic functionality checks

### Build Output

```
out/
├── newos-YYYY.MM.DD-x86_64.iso
├── SHA256SUMS
└── MD5SUMS
```

## Package Categories

### Core System (~25 packages)
- Base: base, linux-t2, linux-t2-headers, linux-firmware
- Boot: grub, efibootmgr, os-prober
- Init: base-devel, sudo, which
- Network: networkmanager, iwd, ufw
- T2: apple-bcm-firmware, apple-t2-audio-config, t2fanrd, tiny-dfr

### Desktop Environment (~35 packages)
- Compositor: hyprland, xdg-desktop-portal-hyprland
- Session: greetd, greetd-tuigreet, polkit-gnome
- Bar: waybar
- Launcher: wofi
- Notifications: swaync
- Lock: hyprlock, hypridle
- Terminal: alacritty
- File Manager: thunar, thunar-archive-plugin, tumbler
- Utils: wl-clipboard, cliphist, grim, slurp, wf-recorder

### Audio/Video (~10 packages)
- PipeWire stack: pipewire, wireplumber, pipewire-pulse, pipewire-alsa
- Controls: pamixer, pavucontrol
- Recording: obs-studio

### Development (~60 packages)
- VCS: git, git-delta, github-cli, lazygit, stow
- Editors: neovim, neovim-lspconfig, code (VS Code OSS)
- Web: nodejs, npm, nvm, pnpm, yarn, typescript-language-server
- Systems: rustup, go, gcc, clang, cmake, meson, ninja, gdb, lldb
- Python: python, pyenv, python-virtualenv, jupyterlab, python-lsp-server
- Containers: docker, docker-compose, docker-buildx, podman, podman-compose
- DBs: postgresql, redis, sqlite, dbeaver, pgcli
- LSPs: rust-analyzer, gopls, lua-language-server, clangd

### CLI Utilities (~30 packages)
- Modern replacements: ripgrep, fd, fzf, bat, eza, zoxide
- Processing: jq, yq, shellcheck, shfmt
- Monitoring: btop, htop, bandwhich
- Networking: curl, httpie, wget
- Shell: bash-completion, starship, tmux, direnv

### Design (~15 packages)
- Graphics: gimp, inkscape, blender, krita
- Theming: nwg-look, kvantum, qt5ct, qt6ct, font-manager

### Fonts (~15 packages)
- Developer: ttf-jetbrains-mono-nerd, ttf-fira-code, adobe-source-code-pro-fonts
- UI: inter-font, noto-fonts, ttf-liberation, adobe-source-sans-fonts
- Emoji: noto-fonts-emoji

## Custom Repository

The arch-mact2 repository provides T2-specific packages:

```ini
# In pacman.conf
[arch-mact2]
Server = https://mirror.funami.tech/arch-mact2/os/x86_64
SigLevel = Never
```

Key packages from arch-mact2:
- linux-t2 (patched kernel)
- linux-t2-headers
- apple-bcm-firmware
- apple-t2-audio-config
- t2fanrd
- tiny-dfr

## Configuration Structure

### Hyprland Configuration

Located in `dotfiles/hypr/`:
- `hyprland.conf` - Main config, sources modular files
- `monitors.conf` - Display setup
- `input.conf` - Keyboard/mouse settings
- `bindings.conf` - Keybindings
- `envs.conf` - Environment variables
- `looknfeel.conf` - Visual appearance
- `autostart.conf` - Startup applications
- `windowrules.conf` - Window behavior rules
- `hypridle.conf` - Idle management
- `hyprlock.conf` - Lock screen appearance

### Key Environment Variables

```bash
# Wayland
XDG_SESSION_TYPE=wayland
XDG_CURRENT_DESKTOP=Hyprland
GDK_BACKEND=wayland,x11,*
QT_QPA_PLATFORM=wayland;xcb
SDL_VIDEODRIVER=wayland
MOZ_ENABLE_WAYLAND=1
ELECTRON_OZONE_PLATFORM_HINT=wayland

# Qt theming
QT_STYLE_OVERRIDE=kvantum
QT_QPA_PLATFORMTHEME=qt5ct

# Cursor
XCURSOR_SIZE=24
HYPRCURSOR_SIZE=24
```

## T2 MacBook Pro Support

### Required Kernel Parameters

```
intel_iommu=on iommu=pt pcie_ports=compat
```

These must be added to GRUB configuration.

### Pre-Installation (macOS Recovery)

1. Boot to Recovery (Cmd+R)
2. Open Startup Security Utility
3. Set Secure Boot to "No Security"
4. Enable "Allow booting from external media"
5. Create partition for Linux (keep EFI partition intact)

### Hardware Status

| Component | Status | Driver/Package |
|-----------|--------|----------------|
| WiFi | ✓ Works | apple-bcm-firmware + brcmfmac |
| Bluetooth | ✓ Works | Built into brcmfmac |
| Audio Out | ✓ Works | apple-t2-audio-config + PipeWire |
| Audio In | ⚠ Partial | May need external mic |
| Keyboard | ✓ Works | apple-bce (VHCI) |
| Trackpad | ✓ Works | apple-bce + libinput |
| Keyboard Backlight | ✓ Works | apple-magic-backlight |
| Touch Bar | ✓ Works | tiny-dfr |
| Fan Control | ✓ Works | t2fanrd |
| Touch ID | ✗ No | T2 Secure Enclave not accessible |
| Suspend | ⚠ Partial | Requires systemd workaround |

### Known Issues

1. **WiFi stability**: Use iwd as NetworkManager backend
2. **Suspend**: Touch Bar may not work after resume
3. **Dual GPU**: Force Intel iGPU as default on hybrid models
4. **Force Touch**: Not supported (hardware limitation)

## Customization Guidelines

### Adding Packages

1. Edit `archiso/packages.x86_64`
2. Add package name (one per line)
3. Rebuild ISO

### Adding AUR Packages

1. Add to `AUR_PACKAGES` array in `scripts/build-aur-repo.sh`
2. Run `./scripts/build-aur-repo.sh`
3. Add package name to `archiso/packages.x86_64`
4. Rebuild ISO

### Customizing Dotfiles

1. Modify files in `dotfiles/` directory
2. Test locally before including in ISO
3. Ensure files will be deployed to `/etc/skel/` in airootfs

### Changing Theming

1. Edit `dotfiles/hypr/looknfeel.conf` for Hyprland appearance
2. Modify `dotfiles/waybar/style.css` for bar styling
3. Configure GTK/Qt themes via respective configs
4. Update wallpapers in `branding/wallpapers/`

## Development Workflow

### Local Testing

```bash
# Test ISO in QEMU
./scripts/test-iso.sh

# Test specific component
# (modify dotfiles, test locally, then include in ISO)
```

### Version Management

```bash
# VERSION file format
MAJOR=0
MINOR=1
PATCH=0
CODENAME="Genesis"
```

### Release Process

1. Update VERSION file
2. Update CHANGELOG.md
3. Build and test ISO
4. Tag release in git
5. Generate checksums
6. Create GitHub release (optional)

## Key Design Decisions

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Init System | systemd | Arch standard, service management |
| Bootloader | GRUB | T2 compatibility with --removable |
| Display Manager | greetd + tuigreet | Minimal, fast, TUI-based |
| Compositor | Hyprland | Modern Wayland, excellent tiling |
| Audio | PipeWire | Low latency, modern, T2 compatible |
| Shell | Bash | Maximum compatibility |
| AUR Helper | yay | Popular, feature-rich, Go-based |
| Editor | Neovim + VS Code | Terminal + GUI options |
| Dotfile Management | git + stow | Version controlled symlinks |
| Firewall | UFW | Simple, effective |
| Network | NetworkManager + iwd | GUI integration + T2 WiFi stability |

## Maintenance

### Regular Updates

```bash
# Update package list to latest versions
# Review arch-mact2 for kernel updates
# Rebuild AUR packages for compatibility
# Test on actual T2 hardware
```

### Monitoring Upstream

- Arch Linux news and package changes
- t2linux project updates (kernel patches)
- Hyprland releases
- archiso changes

## Troubleshooting

### Build Issues

- Ensure archiso is latest version
- Check disk space (need ~20GB for build)
- Verify AUR packages built successfully
- Check pacman.conf syntax

### T2 Hardware Issues

- Verify kernel parameters are set
- Check apple-bce module loaded: `lsmod | grep apple`
- Verify firmware exists: `ls /lib/firmware/brcm/`
- Check audio: `aplay -l` should show AppleT2

### Hyprland Issues

- Check logs: `journalctl --user -u hyprland`
- Verify environment variables set
- Test with minimal config first

## Resources

- Arch Linux Wiki: https://wiki.archlinux.org/
- t2linux Wiki: https://wiki.t2linux.org/
- Hyprland Wiki: https://wiki.hyprland.org/
- archiso Guide: https://wiki.archlinux.org/title/Archiso
- arch-mact2 Repo: https://github.com/NoaHimesaka1873/arch-mact2

## File Quick Reference

| Task | File(s) |
|------|---------|
| Add package | `archiso/packages.x86_64` |
| Kernel params | `archiso/grub/grub.cfg` |
| Build metadata | `archiso/profiledef.sh` |
| User configs | `dotfiles/*` → `archiso/airootfs/etc/skel/` |
| System configs | `archiso/airootfs/etc/*` |
| Custom scripts | `archiso/airootfs/usr/local/bin/` |
| Enable service | `archiso/airootfs/etc/systemd/system/*.wants/` |
| Post-install | `scripts/install.sh` |

## Contributing

1. Fork repository
2. Create feature branch
3. Make changes with clear commit messages
4. Test build and functionality
5. Submit pull request

## License

This project configuration is open source. Individual packages retain their original licenses.
