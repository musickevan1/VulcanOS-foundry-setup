# Architecture

**Analysis Date:** 2026-01-23

## Pattern Overview

**Overall:** Multi-layer modular architecture combining:
1. **ISO Build System** - archiso profile for creating bootable Arch Linux distribution
2. **Configuration Layer** - GNU Stow-managed dotfiles for Hyprland/desktop environment
3. **Rust MCP Services** - Two domain-specific servers (task management, knowledge vault) for AI agent integration
4. **Desktop Application Layer** - TUI and CLI interfaces built on common storage backends

**Key Characteristics:**
- Separation between ISO configuration (system-level) and user dotfiles (Stow-symlinked)
- Pluggable storage backends (JSON for todo, SQLite with vector extension for vault)
- MCP (Model Context Protocol) for AI agent integration via `.mcp.json`
- Modular Hyprland configuration using source directives
- Feature-gated Rust code (TUI optional, always supports MCP/CLI)

## Layers

**ISO Build Layer:**
- Purpose: Create bootable Arch Linux distribution with T2 MacBook Pro support
- Location: `archiso/`
- Contains: Package manifests, bootloader config, GRUB themes, airootfs skeleton
- Depends on: archiso build tools, AUR packages from custom repository
- Used by: `scripts/build.sh` to generate `.iso` file

**Configuration Layer (Dotfiles):**
- Purpose: User-facing Hyprland compositor and application configs
- Location: `dotfiles/*/`
- Contains: `.config/` subdirectories for each application (Hyprland, Waybar, Kitty, Neovim, etc.)
- Depends on: GNU Stow for symlink management
- Used by: Live system via symlinks (`~/.config/app -> dotfiles/app/.config/app`)
- **Critical:** `.config/` subdirectories are the actual config sources - never delete

**Storage Layer:**
- Purpose: Persist application data
- Implementations:
  - `vulcan-todo`: JSON file-based store (`JsonStore` in `vulcan-todo/src/store/json_store.rs`)
  - `vulcan-vault`: SQLite with sqlite-vec extension (`SqliteStore` in `vulcan-vault/src/store/sqlite_store.rs`)
- Location: `~/.local/share/vulcan-todo/` and `~/.local/share/vulcan-vault/` (XDG defaults)
- Trait: Both implement `Store` interface for abstracting backend

**Service Layer (MCP Servers):**
- Purpose: Expose domain logic to AI agents via Model Context Protocol
- Implementations:
  - `vulcan-todo` MCP: Task management with projects, sprints, priorities
  - `vulcan-vault` MCP: Semantic search over knowledge base with RAG
- Location: `vulcan-*/src/mcp/server.rs`, `vulcan-*/src/mcp/tools.rs`
- Protocol: stdio-based MCP (Model Context Protocol) in `/gsd:map-codebase`

**CLI/TUI Layer:**
- Purpose: User interaction for development workflows
- Implementations:
  - `vulcan-todo`: Full CLI + optional TUI (feature-gated)
  - `vulcan-vault`: CLI commands with optional TUI
- Entry points: `vulcan-todo --mcp` (server) vs `vulcan-todo list` (CLI) vs `vulcan-todo tui` (TUI)

**Desktop Integration Layer:**
- Purpose: Custom system utilities and keybindings
- Location: `archiso/airootfs/usr/local/bin/`
- Contains: `vulcan-*` scripts for themes, wallpapers, screenshots, S2T, menus
- Integration: Called from Hyprland bindings in `dotfiles/hypr/.config/hypr/bindings.conf`

## Data Flow

**ISO Build Flow:**

1. `scripts/prepare.sh` → Downloads T2 firmware, builds AUR packages, creates custom repo
2. `scripts/build-aur-repo.sh` → Creates local package repository with arch-mact2 packages
3. `scripts/build.sh` → Calls `mkarchiso` with `archiso/profiledef.sh` config
4. archiso reads `packages.x86_64` and overlays `archiso/airootfs/` into live filesystem
5. Output: `out/vulcanos-YYYY.MM.DD-x86_64.iso`

**User Configuration Flow:**

1. User edits file in `dotfiles/hypr/.config/hypr/` (e.g., `bindings.conf`)
2. GNU Stow maintains symlink: `~/.config/hypr -> dotfiles/hypr/.config/hypr`
3. Change applies immediately to running Hyprland session
4. To sync to ISO: Copy from `dotfiles/*/` to `archiso/airootfs/etc/skel/.config/*/`

**Task Manager Data Flow (vulcan-todo):**

1. User input → CLI parser (`cli.rs`) or MCP tool handler (`mcp/tools.rs`)
2. Store trait dispatch → JSON file operations (`store/json_store.rs`)
3. JSON file updates propagate to:
   - TUI display (if running with `--features tui`)
   - MCP server response (if running `--mcp`)
   - CLI output

**Knowledge Vault RAG Flow (vulcan-vault):**

1. Obsidian `.md` files in vault directory
2. Scanner discovers files (glob pattern matching)
3. Markdown parser extracts frontmatter YAML + content chunks
4. Text chunks sent to embedding service (Ollama or cloud API)
5. Embeddings stored in SQLite with sqlite-vec
6. Query → embedding → vector search → results ranked by similarity + metadata filters

**MCP Integration Flow:**

1. OpenCode/Claude starts MCP server: `vulcan-todo --mcp` or `vulcan-vault --mcp`
2. Server listens on stdin/stdout (stdio transport)
3. Client sends JSON-RPC tool calls via `.mcp.json` registration
4. Server executes tool, returns results to client
5. Tools include: `create_task`, `list_tasks`, `query_vault`, `recall_memories`, etc.

**State Management:**

- `vulcan-todo`: Stateless store abstraction, mutable Task/Sprint structs, file I/O on each operation
- `vulcan-vault`: Stateless search operations, mutable Note structs, connection pooling via Mutex<Connection>
- Both support concurrent read/write through storage backend synchronization
- No in-process caching between operations

## Key Abstractions

**Store Trait (`vulcan-todo` and `vulcan-vault`):**
- Purpose: Abstract storage implementation from business logic
- Examples: `Arc<dyn Store>` passed through CLI/TUI/MCP layers
- Pattern: All data mutations go through Store trait; backend implementation handles persistence
- Implementations:
  - `JsonStore` (vulcan-todo): Reads/writes `~/.local/share/vulcan-todo/tasks.json`
  - `SqliteStore` (vulcan-vault): Direct SQL queries with transaction support

**Task & Sprint Models (vulcan-todo):**
- Purpose: Core domain entities
- Location: `vulcan-todo/src/models/task.rs`, `vulcan-todo/src/models/sprint.rs`
- Methods: `.is_pending()`, `.complete()`, `.belongs_to_project()`, `.priority.level()`
- Serialization: Full serde support for JSON persistence

**Note & Memory Models (vulcan-vault):**
- Purpose: Knowledge base entities with lifecycle
- Location: `vulcan-vault/src/models/note.rs`, `vulcan-vault/src/models/memory.rs`
- Lifecycle: Notes move through `NoteStatus`: Draft → Active → Archived
- Metadata: Title, tags, project, created/updated timestamps

**Hyprland Configuration Modules:**
- Purpose: Modular Hyprland config via source directives
- Location: `dotfiles/hypr/.config/hypr/*.conf`
- Pattern: Main `hyprland.conf` sources: monitors.conf, input.conf, envs.conf, bindings.conf, autostart.conf, looknfeel.conf, windowrules.conf
- Editing: Changes to sourced files take effect on Hyprland reload (`$mainMod SHIFT R`)

**MCP Tool Registration:**
- Purpose: Expose domain operations as callable tools for AI agents
- Location: `vulcan-*/src/mcp/tools.rs`
- Pattern: Each tool implements `ToolContext` → operation → `ToolResult` (success/error)
- Tools become available as `mcp__vulcan-{todo|vault}__{tool_name}` in Claude Code

## Entry Points

**ISO Build:**
- Location: `archiso/profiledef.sh`
- Triggers: Manual `./scripts/build.sh` execution
- Responsibilities: Defines archiso profile metadata (arch, boot modes, compression)

**Dotfiles Installation:**
- Location: `dotfiles/` (all subdirectories with `.config/` structure)
- Triggers: `stow <app>` from dotfiles/ to create symlinks
- Responsibilities: Configuration for each desktop/development application

**vulcan-todo Entry Points:**
- Location: `vulcan-todo/src/main.rs`
- CLI Mode: `vulcan-todo list` → handle_command() → store operations
- TUI Mode: `vulcan-todo tui` → ui::run_tui() with crossterm backend
- MCP Mode: `vulcan-todo --mcp` → mcp::run_mcp_server() listening on stdio
- Responsibilities: Argument parsing, mode selection, store initialization

**vulcan-vault Entry Points:**
- Location: `vulcan-vault/src/main.rs`
- Query Mode: `vulcan-vault query "search term"` → SQLite vector search
- Init Mode: `vulcan-vault init` → Create schema and index existing .md files
- MCP Mode: `vulcan-vault --mcp` → MCP server for AI agent integration
- Responsibilities: CLI dispatch, vault directory scanning, embedding coordination

**Hyprland Autostart:**
- Location: `dotfiles/hypr/.config/hypr/autostart.conf`
- Triggers: Session start (loaded by Hyprland compositor)
- Responsibilities: Launch waybar, swaync, hyprpaper, hypridle, hyprwhspr, custom scripts

**Desktop Integration Scripts:**
- Location: `archiso/airootfs/usr/local/bin/vulcan-*` (installed to live system)
- Triggers: Hyprland keybindings in `bindings.conf`
- Responsibilities: Theme switching, wallpaper selection, screenshots, S2T, menu UI

## Error Handling

**Strategy:** Result-based error propagation with context

**Patterns:**
- Rust: `anyhow::Result<T>` for recoverable errors, `?` operator for propagation
- Storage errors: Custom `StoreError` enum with `Display` impl
- MCP errors: Tool returns `ToolResult::Error` with message
- CLI: Errors print to stderr with contextual message, exit code 1
- TUI: Errors display in status bar, user can retry

**Examples from codebase:**
- Task not found: "Task not found: {id}" to stderr
- Store lock poisoned: "Database lock lost" (vulcan-vault)
- Embedding failure: Tool returns error message to MCP client
- Invalid JSON: Store deserialization error with line number

## Cross-Cutting Concerns

**Logging:**
- Framework: `tracing` crate with `tracing-subscriber` initialization
- Pattern: `tracing::info!()`, `tracing::error!()` throughout MCP servers
- Control: `RUST_LOG` env var filters output (e.g., `RUST_LOG=debug vulcan-vault --mcp`)

**Validation:**
- Task creation: Title required, priority auto-defaults to Normal
- Project assignment: Via tags or explicit project field
- Sprint assignment: Task must exist before assigning to sprint
- Note creation: Content required, metadata optional

**Authentication:**
- Model: None for local CLI/TUI (filesystem permissions)
- MCP: Inherited from Claude/OpenCode session (no auth within protocol)
- T2 Hardware: Handled at boot level (GRUB params), not application layer

**Configuration:**
- Hyprland: Modular .conf files sourced at startup
- vulcan-todo: JSON file path via `VULCAN_TODO_PATH` env var
- vulcan-vault: Vault path via `VULCAN_VAULT_PATH` env var, defaults to `~/.local/share/vulcan-vault/`
- Theme/Branding: Palette defined in `branding/vulcan-palette.css`, applied via GTK/Qt themes

---

*Architecture analysis: 2026-01-23*
