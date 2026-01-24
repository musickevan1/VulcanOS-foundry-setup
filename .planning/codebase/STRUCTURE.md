# Codebase Structure

**Analysis Date:** 2026-01-23

## Directory Layout

```
/home/evan/VulcanOS/
├── archiso/                          # ISO build system (archiso profile)
│   ├── profiledef.sh                 # archiso profile metadata
│   ├── packages.x86_64               # Package manifest (600+ packages)
│   ├── pacman.conf                   # Package manager config
│   ├── airootfs/                     # Root filesystem overlay for live system
│   │   ├── etc/                      # System configuration
│   │   │   ├── skel/.config/         # Default user skeleton configs
│   │   │   ├── pacman.d/             # Pacman hooks
│   │   │   ├── systemd/              # Systemd service symlinks
│   │   │   ├── locale.conf, hostname, vconsole.conf
│   │   ├── usr/local/bin/            # Custom system scripts (vulcan-*, docker-mcp-gateway)
│   │   └── root/                     # Root user home (install scripts)
│   ├── grub/                         # GRUB bootloader configuration + theme
│   ├── syslinux/                     # BIOS boot config
│   └── efiboot/                      # UEFI boot configuration
│
├── dotfiles/                         # GNU Stow-managed user configurations
│   │                                 # ⚠️ CRITICAL: Each subdirectory has .config/ structure
│   ├── hypr/                         # Hyprland compositor configuration
│   │   └── .config/hypr/             # SOURCE: Actual config files
│   │       ├── hyprland.conf         # Main config (sources modular files)
│   │       ├── monitors.conf         # Display configuration
│   │       ├── input.conf            # Keyboard/mouse settings
│   │       ├── bindings.conf         # Keybindings (hotkeys to apps/scripts)
│   │       ├── envs.conf             # Environment variables
│   │       ├── looknfeel.conf        # Visual appearance
│   │       ├── autostart.conf        # Startup applications
│   │       ├── windowrules.conf      # Window behavior rules
│   │       ├── workspaces.conf       # Workspace configuration
│   │       ├── hypridle.conf         # Idle management
│   │       └── hyprlock.conf         # Lock screen appearance
│   ├── waybar/                       # Status bar configuration
│   │   └── .config/waybar/
│   │       ├── config.jsonc          # Bar layout and modules
│   │       ├── style.css             # Bar styling
│   │       ├── workspace-icons.json  # Custom workspace icons
│   │       └── workspaces.json       # Workspace layout
│   ├── kitty/                        # Terminal emulator config
│   │   └── .config/kitty/
│   │       ├── kitty.conf            # Terminal settings
│   │       └── colors.conf           # Color scheme
│   ├── nvim/                         # Neovim configuration
│   │   └── .config/nvim/
│   │       ├── init.lua              # Main config
│   │       ├── lua/vulcan/           # Plugin configs
│   │       └── after/                # Post-initialization hooks
│   ├── wofi/                         # Application launcher config
│   │   └── .config/wofi/
│   │       ├── config                # Launcher behavior
│   │       └── style.css             # Launcher styling
│   ├── swaync/                       # Notification center config
│   │   └── .config/swaync/
│   │       ├── config.json           # Notification behavior
│   │       └── style.css             # Notification styling
│   ├── opencode/                     # OpenCode AI assistant config
│   │   └── .config/opencode/
│   │       ├── opencode.json         # Assistant configuration
│   │       ├── agent/                # Agent templates
│   │       └── knowledge/            # Knowledge base snapshots
│   ├── starship/                     # Shell prompt configuration
│   │   └── .config/
│   │       └── starship.toml         # Prompt settings
│   ├── bash/                         # Bash shell configuration
│   │   └── .bashrc, .bash_profile
│   ├── git/                          # Git configuration
│   │   └── .gitconfig, .gitignore
│   ├── scripts/                      # User scripts and utilities
│   │   └── .local/bin/
│   │       ├── vulcan-*              # Custom scripts for development
│   │       ├── workspace-switch
│   │       ├── configure-workspaces
│   │       ├── opencode-picker
│   │       └── vulcan-menu
│   ├── systemd/                      # User systemd services
│   │   └── .config/systemd/user/
│   │       └── *.service             # User-level services
│   ├── goimapnotify/                 # Mail notifications
│   │   └── .config/goimapnotify/
│   ├── nwg-dock-hyprland/            # App dock configuration
│   │   └── .config/nwg-dock-hyprland/
│   ├── themes/                       # GTK/Qt theme configurations
│   │   ├── gtk-3.0/, gtk-4.0/
│   │   ├── qt5ct/, qt6ct/
│   │   └── kvantum/
│   └── .stow-local-ignore            # Files to ignore during stow
│
├── vulcan-todo/                      # Task manager MCP server (Rust)
│   ├── Cargo.toml                    # Dependencies: clap, serde, tokio, tracing
│   ├── src/
│   │   ├── main.rs                   # Entry point (CLI/TUI/MCP mode dispatch)
│   │   ├── cli.rs                    # Argument parsing (clap derive)
│   │   ├── models/
│   │   │   ├── mod.rs                # Task, Sprint, Priority, Status enums
│   │   │   ├── task.rs               # Task struct with serialization
│   │   │   └── sprint.rs             # Sprint lifecycle management
│   │   ├── store/
│   │   │   ├── mod.rs                # Store trait definition (abstraction)
│   │   │   └── json_store.rs         # JsonStore implementation (file I/O)
│   │   ├── mcp/
│   │   │   ├── mod.rs                # MCP module exports
│   │   │   ├── server.rs             # MCP stdio server loop
│   │   │   ├── tools.rs              # Tool handlers (create_task, list_tasks, etc)
│   │   │   └── protocol.rs           # JSON-RPC protocol definitions
│   │   └── ui/
│   │       ├── mod.rs                # TUI module (feature-gated)
│   │       ├── tui.rs                # Ratatui-based terminal UI
│   │       └── app.rs                # TUI state management
│   └── tests/
│       └── *.rs                      # Integration tests
│
├── vulcan-vault/                     # Knowledge vault RAG server (Rust)
│   ├── Cargo.toml                    # Dependencies: rusqlite, sqlite-vec, reqwest, pulldown-cmark
│   ├── src/
│   │   ├── lib.rs                    # Public API exports
│   │   ├── main.rs                   # Entry point (CLI/MCP mode dispatch)
│   │   ├── models/
│   │   │   ├── mod.rs                # Note, Memory, Chunk types
│   │   │   ├── note.rs               # Note entity with metadata
│   │   │   ├── memory.rs             # Memory with decay/retrieval logic
│   │   │   └── chunk.rs              # Text chunk for embeddings
│   │   ├── store/
│   │   │   ├── mod.rs                # Store trait definition
│   │   │   ├── sqlite_store.rs       # SqliteStore with sqlite-vec vectors
│   │   │   └── error.rs              # StoreError types
│   │   ├── rag/
│   │   │   ├── mod.rs                # RAG pipeline orchestration
│   │   │   ├── chunker.rs            # Text chunking strategies
│   │   │   └── embeddings.rs         # Embedding service integration (Ollama/cloud)
│   │   ├── memory/
│   │   │   ├── mod.rs                # Memory subsystem
│   │   │   ├── formation.rs          # Memory creation
│   │   │   ├── retrieval.rs          # Similarity-based retrieval
│   │   │   └── decay.rs              # Time-based decay algorithm
│   │   ├── mcp/
│   │   │   ├── mod.rs                # MCP module
│   │   │   ├── server.rs             # MCP server implementation
│   │   │   ├── tools.rs              # Tools: query, remember, recall
│   │   │   └── protocol.rs           # Protocol definitions
│   │   └── ui/
│   │       ├── mod.rs                # TUI module (optional)
│   │       ├── tui.rs                # Optional terminal UI
│   │       └── app.rs                # TUI state (if enabled)
│   └── tests/
│       └── rag_integration.rs        # RAG pipeline tests
│
├── scripts/                          # Build and utility scripts
│   ├── build.sh                      # Main ISO build script (calls mkarchiso)
│   ├── prepare.sh                    # Pre-build: downloads firmware, builds AUR packages
│   ├── build-aur-repo.sh             # Builds custom package repository from AUR
│   ├── test-iso.sh                   # QEMU testing (BIOS/UEFI modes)
│   ├── backup-vulcan-config.sh       # Backup dotfiles and vaults
│   ├── auto-backup-wrapper.sh        # Scheduled backup automation
│   ├── verify-backups.sh             # Backup integrity checks
│   ├── split-wallpaper.sh            # Wallpaper processing utility
│   └── test-s2t.sh                   # Speech-to-text testing
│
├── branding/                         # Visual identity and assets
│   ├── BRAND-GUIDELINES.md           # Design guidelines
│   ├── vulcan-palette.css            # Color palette (CSS variables)
│   ├── logos/                        # Logo assets (SVG, PNG)
│   ├── wallpapers/                   # Desktop wallpapers
│   ├── icons/                        # Custom icon sets
│   └── themes/                       # GTK/Qt theme packages
│
├── customrepo/                       # Custom package repository
│   └── x86_64/                       # Built packages and repo database
│       ├── *.pkg.tar.zst             # Pre-built packages
│       └── custom.db*                # Package database
│
├── docs/                             # Documentation
│   ├── INSTALL.md                    # Installation guide
│   ├── KEYBINDINGS.md                # Keyboard shortcuts reference
│   ├── OPENCODE.md                   # OpenCode AI setup guide
│   └── (other technical docs)
│
├── .planning/                        # GSD execution planning
│   └── codebase/
│       ├── STACK.md                  # Technology stack analysis
│       ├── CONVENTIONS.md            # Coding conventions
│       ├── ARCHITECTURE.md           # (This document - architecture patterns)
│       ├── STRUCTURE.md              # (This document - directory organization)
│       ├── CONCERNS.md               # Technical debt and issues
│       └── TESTING.md                # Testing patterns
│
├── CLAUDE.md                         # Project instructions and reference
├── .mcp.json                         # MCP server registration (OpenCode integration)
├── .gitignore                        # Git ignore patterns
├── VERSION                           # Version information (MAJOR.MINOR.PATCH)
├── out/                              # Build output (ISO files)
└── pacman.conf                       # Package manager configuration

```

## Directory Purposes

**archiso/**
- Purpose: ISO build configuration and live system overlay
- Contains: Build profile, package lists, bootloader configs, root filesystem skeleton
- Key files: `profiledef.sh` (archiso metadata), `packages.x86_64` (600+ packages)
- Important: `airootfs/etc/skel/` provides default user home files; `airootfs/etc/` system configs

**dotfiles/**
- Purpose: Version-controlled user configurations using GNU Stow
- Contains: Application config directories with `.config/` subdirectory structure
- **Critical:** Each `dotfiles/app/.config/app/` is the actual config source - Stow symlinks to `~/.config/app/`
- Never delete `.config/` subdirectories - they are the config file storage, not templates
- Key apps: hypr (Hyprland), waybar (status bar), nvim (editor), kitty (terminal), opencode (AI)

**vulcan-todo/** and **vulcan-vault/**
- Purpose: Rust-based MCP servers for AI agent integration
- Contains: Cargo projects with CLI/MCP modes, modular architecture
- Key pattern: Each has Store trait (abstraction), models (domain entities), mcp/ (agent interface), ui/ (optional TUI)
- Storage: vulcan-todo uses JSON files; vulcan-vault uses SQLite with vector search
- MCP tools become available in Claude Code as `mcp__vulcan-{todo|vault}__{tool_name}`

**scripts/**
- Purpose: Build pipeline and system utilities
- Contains: Bash scripts for ISO creation, AUR package building, testing, backups
- Key script: `build.sh` (main orchestrator), `prepare.sh` (environment setup)
- Pattern: All scripts source from project root and use colored output

**branding/**
- Purpose: VulcanOS visual identity
- Contains: CSS palette, logo assets, wallpapers, icon themes, GTK/Qt configurations
- Key file: `vulcan-palette.css` (primary color scheme)
- Used by: Hyprland looknfeel.conf references palette colors

**customrepo/**
- Purpose: Pre-built custom packages for fast ISO builds
- Contains: `.pkg.tar.zst` archives and `repo.db` index
- Used by: `scripts/build-aur-repo.sh` populates; `pacman.conf` references as custom repository

## Key File Locations

**Entry Points:**
- `archiso/profiledef.sh`: ISO build entry point
- `vulcan-todo/src/main.rs`: Task manager entry point (CLI/MCP/TUI dispatch)
- `vulcan-vault/src/main.rs`: Knowledge vault entry point (CLI/MCP dispatch)
- `dotfiles/hypr/.config/hypr/hyprland.conf`: Desktop environment entry point (sources modular configs)
- `scripts/build.sh`: Manual ISO rebuild trigger

**Configuration:**
- `archiso/packages.x86_64`: System packages to install
- `archiso/pacman.conf`: Package manager mirrors and repositories
- `dotfiles/hypr/.config/hypr/*.conf`: All Hyprland settings (monitors, bindings, appearance)
- `branding/vulcan-palette.css`: Primary color definitions
- `.mcp.json`: MCP server registration for AI agents

**Core Logic:**
- `vulcan-todo/src/store/json_store.rs`: Task persistence (JSON file I/O)
- `vulcan-vault/src/store/sqlite_store.rs`: Knowledge vault storage (SQLite + sqlite-vec)
- `vulcan-todo/src/mcp/tools.rs`: Task manager AI tools
- `vulcan-vault/src/mcp/tools.rs`: Knowledge vault AI tools
- `vulcan-vault/src/rag/embeddings.rs`: Embedding service integration

**Testing:**
- `vulcan-todo/tests/`: Integration tests for task manager
- `vulcan-vault/tests/rag_integration.rs`: RAG pipeline tests
- `scripts/test-iso.sh`: QEMU-based ISO validation
- `scripts/test-s2t.sh`: Speech-to-text testing

## Naming Conventions

**Files:**
- ISO build configs: `*.conf` (hyprland.conf, monitors.conf, pacman.conf)
- Rust source: `*.rs` with snake_case module names (main.rs, cli.rs, json_store.rs)
- Shell scripts: `*.sh` with kebab-case names (build.sh, test-iso.sh, vulcan-menu)
- Markdown docs: `*.md` with UPPERCASE names in docs/ and .planning/codebase/
- Config formats: `.json` for JSON-based configs (opencode.json, config.json), `.jsonc` for JSONC (waybar)
- Stylesheets: `.css` for Waybar/Swaync/Wofi styling
- Markdown: `*.md` for vault notes (auto-discovered by vulcan-vault glob)

**Directories:**
- Rust modules: `src/models/`, `src/store/`, `src/mcp/`, `src/ui/` (functional domains)
- Dotfiles apps: `dotfiles/{app}/` with `.config/{app}/` structure (mirrors home directory structure)
- System configs: `archiso/airootfs/etc/skel/` (user skeleton) vs `archiso/airootfs/etc/` (system)
- Build output: `out/` (ISO files), `customrepo/x86_64/` (packages)
- Assets: `branding/logos/`, `branding/wallpapers/`, `branding/icons/`

**Naming Examples:**
- Task file: `~/.local/share/vulcan-todo/tasks.json`
- Vault database: `~/.local/share/vulcan-vault/vault.db`
- Symlinks: `~/.config/hypr -> dotfiles/hypr/.config/hypr`
- Scripts: `opencode-picker`, `vulcan-menu`, `workspace-switch`
- Configs: `hyprland.conf`, `config.jsonc`, `style.css`

## Where to Add New Code

**New Feature in vulcan-todo:**
- Primary code: `vulcan-todo/src/models/` (if new domain entity) or `vulcan-todo/src/cli.rs` (if new command)
- Store methods: Add to `vulcan-todo/src/store/mod.rs` trait, implement in `json_store.rs`
- MCP tool: Add to `vulcan-todo/src/mcp/tools.rs`
- Tests: `vulcan-todo/tests/integration_tests.rs`

**New Feature in vulcan-vault:**
- Primary code: `vulcan-vault/src/models/` (if new domain entity)
- Search logic: `vulcan-vault/src/rag/` (if new search/indexing strategy)
- MCP tool: Add to `vulcan-vault/src/mcp/tools.rs`
- Memory system: `vulcan-vault/src/memory/` (if new memory type)
- Tests: `vulcan-vault/tests/rag_integration.rs`

**New Component/Module:**
- Implementation: Create `src/{new_module}/mod.rs` and subdirectories by responsibility
- Pattern: Keep functionality cohesive; use trait abstractions for different backends

**Desktop Integration:**
- Custom scripts: `dotfiles/scripts/.local/bin/` (symlinked to `~/.local/bin/`)
- Keybindings: Add to `dotfiles/hypr/.config/hypr/bindings.conf` with `bind = ...` directive
- Systemd services: `dotfiles/systemd/.config/systemd/user/` for user-level services

**Utilities:**
- Shared helpers: `vulcan-{todo|vault}/src/{domain}/` (e.g., error types in `store/error.rs`)
- System scripts: `scripts/` (build, test, maintenance utilities)
- Desktop utilities: `archiso/airootfs/usr/local/bin/` (system-wide scripts in live system)

**Theme/Branding:**
- Colors: Defined in `branding/vulcan-palette.css` as CSS variables
- Applied via: `dotfiles/hypr/.config/hypr/looknfeel.conf` and GTK/Qt theme configs
- Wallpapers: Add to `branding/wallpapers/`, reference in `autostart.conf` or `hyprpaper.conf`

## Special Directories

**archiso/airootfs/**
- Purpose: Overlay for live system root filesystem
- Generated: No (hand-maintained)
- Committed: Yes (version controlled)
- Structure mirrors `/` - files here appear as `/` in live system
- Example: `airootfs/usr/local/bin/vulcan-menu` → `/usr/local/bin/vulcan-menu` in ISO

**dotfiles/*/.config/**
- Purpose: GNU Stow source directories (actual config files)
- Generated: No (hand-maintained)
- Committed: Yes (critical - DO NOT DELETE)
- These are NOT templates; they are the actual config sources
- Stow creates symlinks: `~/.config/app -> dotfiles/app/.config/app`
- Changes here apply immediately to running system via symlinks

**customrepo/x86_64/**
- Purpose: Pre-built package cache
- Generated: Yes (by `scripts/build-aur-repo.sh`)
- Committed: No (.gitignore'd)
- Populated by building AUR packages once, reused in ISO builds
- Structure: `*.pkg.tar.zst` (packages) and `custom.db*` (index)

**.planning/codebase/**
- Purpose: GSD execution planning documents
- Generated: Yes (by `/gsd:map-codebase` and `/gsd:plan-phase`)
- Committed: Yes (tracked in repo)
- Contains: STACK.md, CONVENTIONS.md, ARCHITECTURE.md, STRUCTURE.md, TESTING.md, CONCERNS.md
- Used by: `/gsd:plan-phase` to create implementation plans, `/gsd:execute-phase` to follow patterns

**out/**
- Purpose: ISO build output
- Generated: Yes (by `scripts/build.sh`)
- Committed: No (ignored, too large)
- Contains: Bootable ISO files with versioned names (vulcanos-YYYY.MM.DD-x86_64.iso)

---

*Structure analysis: 2026-01-23*
