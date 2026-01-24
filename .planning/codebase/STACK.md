# Technology Stack

**Analysis Date:** 2026-01-23

## Languages

**Primary:**
- Bash - Build scripts, installation, system automation
- Rust - MCP server implementations (vulcan-todo, vulcan-vault), vector storage pipelines
- TOML - Configuration for Hyprland, dotfiles, and Rust projects

**Secondary:**
- Markdown - Documentation, notes storage (Obsidian vault format)
- YAML - Frontmatter in notes, configuration data
- JSONC - Waybar configuration (`archiso/airootfs/etc/skel/.config/waybar/config.jsonc`)
- Shell Scripts - Custom binaries in dotfiles

## Runtime

**Environment:**
- Linux kernel (linux-t2 with T2 MacBook Pro patches from arch-mact2 repository)
- GNU userspace (base Arch Linux)
- systemd (init system)

**Package Manager:**
- pacman (Arch Linux native)
- yay (AUR helper, included in ISO)
- Cargo (Rust package manager)
- Lockfiles: `vulcan-todo/Cargo.lock`, `vulcan-vault/Cargo.lock`

## Frameworks

**Desktop Compositor:**
- Hyprland - Wayland compositor, config at `dotfiles/hypr/.config/hypr/`
- systemd (service management for desktop services)

**Display Manager:**
- SDDM - Login screen, replaces greetd from original design

**Status Bar:**
- Waybar - System status bar, config at `dotfiles/waybar/.config/waybar/`

**Application Launcher:**
- Wofi - Wayland-native launcher, config at `dotfiles/wofi/.config/wofi/`

**Notification System:**
- Swaync - Notification center, config at `dotfiles/swaync/.config/swaync/`

**Build/ISO System:**
- archiso - Arch Linux ISO creation framework
- mkarchiso - ISO builder invoked by `scripts/build.sh`
- squashfs-tools - Filesystem compression for initramfs
- xorriso - ISO 9660 creation

**Rust Web/Network:**
- Tokio - Async runtime for MCP servers and vault operations
- Reqwest - HTTP client for Ollama embeddings API
- Zbus - D-Bus communication (session detection in vulcan-todo)

**Terminal/TUI:**
- Ratatui - Terminal UI framework (optional feature in vulcan-todo, vulcan-vault)
- Crossterm - Terminal input handling (optional feature)
- Kitty - Terminal emulator, config at `dotfiles/kitty/.config/kitty/`

**Audio Pipeline:**
- PipeWire - Modern audio server
- WirePlumber - PipeWire session manager
- pipewire-pulse - PulseAudio compatibility layer
- pipewire-alsa - ALSA compatibility layer

**Text Processing & Parsing:**
- pulldown-cmark - Markdown parser for note processing
- gray_matter - YAML frontmatter extraction from notes
- regex - Link and pattern extraction in vault

**Testing:**
- Tempfile - Temporary file creation for tests
- assert_fs - Filesystem assertion testing

## Key Dependencies

**Critical (Runtime Behavior):**
- sqlite-vec (0.1) - Vector search extension for SQLite in vulcan-vault
  - Enables semantic search of notes via embeddings
  - Located in `vulcan-vault/Cargo.toml`
- rusqlite (0.31) - SQLite database binding for Rust
  - Manages note storage, chunks, memories
- clap (4.4) - CLI argument parsing for both MCP servers
- serde + serde_json (1.0) - Serialization for JSON/configuration

**Infrastructure:**
- Ollama (external HTTP service at `http://localhost:11434`)
  - Embedding generation via `nomic-embed-text` model (768-dim vectors)
  - HTTP requests via Reqwest
  - Connection handled in `vulcan-vault/src/rag/embeddings.rs`
- Docker & docker-compose - Containerization (included in packages but not used for build)

**Development Environment:**
- rust-analyzer - Rust LSP server
- typescript-language-server - TypeScript/JavaScript support
- pyright - Python type checking
- lua-language-server - Lua support
- bash-language-server - Shell script support
- gopls - Go support (for future projects)

**Serialization & Data:**
- toml (0.8) - TOML parsing for vault configuration
- serde_yaml (0.9) - YAML parsing for frontmatter
- uuid (1.6) - Unique IDs for notes, chunks, memories
- chrono (0.4) - Timestamp handling for creation/modification dates

**File Watching & UI:**
- notify (6.1) - Real-time file system monitoring (optional TUI feature)
- anyhow + thiserror - Error handling utilities
- tracing + tracing-subscriber - Logging framework

## Configuration

**Environment:**
- Wayland-specific variables in `archiso/airootfs/etc/skel/.config/hypr/envs.conf`:
  - `XDG_SESSION_TYPE=wayland`
  - `XDG_CURRENT_DESKTOP=Hyprland`
  - Qt/GTK backend selection: `QT_QPA_PLATFORM=wayland;xcb`
  - Audio latency: `PIPEWIRE_LATENCY=128/48000`
  - Editor/browser defaults: `EDITOR=nvim`, `BROWSER=firefox`
- Build environment via `archiso/pacman.conf`:
  - Package repositories: core, extra, multilib, arch-mact2 (T2 support)
  - Parallel downloads: 5

**Build Configuration:**
- ISO metadata: `archiso/profiledef.sh`
  - ISO label: `VULCANOS_YYYYMM`
  - Version: `YYYY.MM.DD` (from build date)
  - Boot modes: BIOS (syslinux), UEFI-32/64 (GRUB), BIOS El Torito
- Squashfs compression: `xz` with BCJ x86 filter for size optimization

**Package Manifest:**
- `archiso/packages.x86_64` - Complete package list (185+ packages)
  - Includes: base-devel, docker, neovim, git, stow, language servers
  - T2 packages: linux-t2, apple-bcm-firmware, apple-t2-audio-config, t2fanrd, tiny-dfr
  - Desktop: hyprland, waybar, wofi, swaync, hyprlock, hypridle, sddm

## Platform Requirements

**Development:**
- Rust 1.70+ (via rustup recommended)
- Arch Linux host for ISO builds (archiso required)
- 20GB free disk space for build artifacts
- Build tools: base-devel, git, mkcpio, mkinitcpio-archiso
- Root access for `mkarchiso`

**Production:**
- T2 MacBook Pro (2019+) with 50GB+ partition
- Secure Boot disabled (required for linux-t2)
- UEFI firmware (recommended) or BIOS with MBR
- Apple BCM4355 WiFi chip (required for arch-mact2 WiFi)
- 8GB+ RAM recommended

**Runtime Services:**
- Ollama service running locally (for embedding generation)
  - Default endpoint: `http://localhost:11434`
  - Required model: `nomic-embed-text` (768-dim embeddings)
- D-Bus session (for Zbus integration in vulcan-todo)

---

*Stack analysis: 2026-01-23*
