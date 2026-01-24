# External Integrations

**Analysis Date:** 2026-01-23

## APIs & External Services

**Ollama (Local Embedding Service):**
- Service: Ollama HTTP API for embedding generation
- What it's used for: Vectorization of notes and text chunks for semantic search in vulcan-vault
- SDK/Client: Reqwest HTTP client
- Endpoint: `http://localhost:11434/api/embeddings` (default, configurable)
- Auth: None (local service)
- Models: `nomic-embed-text` (768-dimensional embeddings)
- Documentation: Embeddings handled in `vulcan-vault/src/rag/embeddings.rs`

**OpenCode AI Assistant:**
- Service: AI code assistant for VulcanOS development
- What it's used for: MCP server integration for vulcan-todo and vulcan-vault tasks/memory
- Installation: Post-install curl script (package not in ISO, user installs: `curl -fsSL https://opencode.ai/install | bash`)
- MCP Registration: `.mcp.json` configuration (created after installation)
- Environment variable: `OPENCODE_SESSION_ID` (vulcan-todo detects session scope)
- Integration points:
  - `vulcan-todo/src/mcp/tools.rs` - MCP tools for task management
  - `vulcan-vault/src/mcp/tools.rs` - MCP tools for memory/note access
  - Session-scoped tasks created when `OPENCODE_SESSION_ID` is set

## Data Storage

**Databases:**

*SQLite (Local File)* - Primary data store
- Type: File-based relational database with vector extension
- Purpose: Note storage, chunks, memories, metadata
- Client: `rusqlite` (0.31) Rust binding
- Extension: `sqlite-vec` (0.1) for vector operations
- Location: `~/.config/vulcan-vault/vault.db`
- Schema: Managed by `vulcan-vault/src/store/sqlite_store.rs`
- Tables:
  - `notes` - Note metadata with YAML frontmatter
  - `chunks` - Text segments with embeddings for semantic search
  - `memories` - Agent memory (decisions, lessons, preferences)
  - `links` - Bidirectional note references (Obsidian-style)
- Vector dimension: 768 (nomic-embed-text output)

*File Storage:* Obsidian-compatible markdown vault
- Type: Directory-based note storage
- Location: `~/.config/vulcan-vault/vault/`
- Format: Markdown with YAML frontmatter
- Zones:
  - `Projects/` - Project documentation
  - `Tasks/` - Task notes linked to vulcan-todo
  - `Learning/` - Reference material and lessons
  - `Agent-Memories/` - AI agent decision log and context

**Caching:**
- In-memory Tokio runtime (async operations in MCP servers)
- File-watching with `notify` crate (real-time vault sync)
- No external cache service (Redis not used)

## Authentication & Identity

**Auth Provider:**
- None (all services are local)
- D-Bus session: Implicit authentication via systemd user session
  - Zbus detects running session for vulcan-todo session-scoping

**Vault Access:**
- File-system based (standard Unix permissions)
- User owns `~/.config/vulcan-vault/` directory
- Database: SQLite file permissions (user-readable)

## Monitoring & Observability

**Error Tracking:**
- Not configured for production
- Development: File-based logging
- MCP servers use `tracing` crate for structured logging

**Logs:**
- Rust services: `tracing-subscriber` with env-filter configuration
- systemd user session: `journalctl --user -u <service>`
- ISO build: Shell script output to stdout/stderr (captured by build.sh)
- Hyprland: `~/.local/share/hyprland/` logs (if configured)
- Ollama: Ollama service logs (depends on how service is started)

**Speech-to-Text Service (hyprwhspr):**
- systemd user service: `hyprwhspr.service`
- Logs: `journalctl --user -u hyprwhspr.service -f`
- Local Whisper/Parakeet models (no external API)

## CI/CD & Deployment

**Hosting:**
- Not a web application
- Arch Linux distribution (self-hosted on T2 MacBook Pro)
- Custom repository: `arch-mact2` for T2-specific packages
  - Server: `https://mirror.funami.tech/arch-mact2/os/x86_64`

**CI Pipeline:**
- None configured (manual build via `scripts/build.sh`)
- Local build tools: archiso, pacman, cargo
- No GitHub Actions or external CI

**Build Artifacts:**
- Output directory: `./out/`
- ISO file: `vulcanos-YYYY.MM.DD-x86_64.iso`
- Checksums: SHA256SUMS, MD5SUMS
- Custom AUR repository: `customrepo/x86_64/`

## Environment Configuration

**Required environment variables (runtime):**
- `XDG_SESSION_TYPE=wayland` - Tell apps to use Wayland
- `XDG_CURRENT_DESKTOP=Hyprland` - Identify desktop environment
- `OLLAMA_URL` (optional) - Custom Ollama endpoint (default: http://localhost:11434)
- `OPENCODE_SESSION_ID` (optional, set by OpenCode) - Scope vulcan-todo to session

**Build environment variables:**
- `SOURCE_DATE_EPOCH` - ISO version timestamp (set by mkarchiso)
- `PYTHONHOME`, `RUSTUP_HOME` - Optional dev environment overrides

**Secrets location:**
- No secrets managed in this codebase
- User credentials stored in system keyring via:
  - NetworkManager (WiFi/Bluetooth)
  - Standard Linux password system
  - Application-specific configs in `~/.config/`

## Webhooks & Callbacks

**Incoming:**
- None configured (not a web server)
- Hyprland IPC socket: `~/.config/hypr/hyprland.sock` (local only)
- D-Bus signals: Used by systemd services internally

**Outgoing:**
- None configured

**Real-time Sync:**
- File-system watching: `notify` crate monitors vault directory
- MCP tools expose vault changes to connected OpenCode sessions
- No external webhooks

## External Dependencies by Component

**vulcan-todo (MCP Task Server):**
- MCP protocol: JSON-RPC over stdio
- Storage: File-based task store (JSON)
- D-Bus: Session detection via Zbus
- No external APIs

**vulcan-vault (MCP Knowledge Server):**
- Ollama API: POST to `/api/embeddings` endpoint
- Markdown parser: pulldown-cmark (local, no network)
- File system: Obsidian-compatible vault on disk
- SQLite database: Local file with sqlite-vec extension

**Hyprland Desktop:**
- Wayland server: Local display protocol
- XDG Desktop Portal: System integration (from `xdg-desktop-portal-hyprland` package)
- PipeWire: Local audio server
- Systemd: Service/session management
- D-Bus: Desktop integration (notifications, file dialogs)

**Build System:**
- arch-mact2 repository: GitHub-hosted AUR repository
- Arch Linux mirrors: Pacman repositories (configurable in `archiso/pacman.conf`)
- Custom AUR packages: Built locally in `customrepo/`

## Network Access

**Required (online install):**
- Arch Linux package servers (for initial pacman sync)
- arch-mact2 mirror (for T2-specific packages)
- GitHub (clone Rust dependencies via Cargo)

**Optional (local):**
- Ollama service (must be running on same machine)
- OpenCode (communicates via MCP protocol)

**Not accessed:**
- Cloud storage
- Remote databases
- Public APIs
- Telemetry services

## API Contracts

**Ollama Embeddings API:**
```json
POST /api/embeddings
{
  "model": "nomic-embed-text",
  "prompt": "text to embed"
}
→
{
  "embedding": [f32; 768]
}
```
Error handling: `EmbeddingError::ServiceUnavailable`, `EmbeddingError::ModelNotFound`

**MCP Server Protocol:**
- Stdin/Stdout JSON-RPC 2.0
- Tools exposed: task creation, listing, completion, sprint management (vulcan-todo)
- Tools exposed: note search, memory formation, retrieval (vulcan-vault)
- Implemented in `*/src/mcp/tools.rs`

**Vault File Format:**
- Markdown with YAML frontmatter (Obsidian compatible)
- Metadata: type, status, tags, aliases, project, task_id
- Automatic parsing: gray_matter extracts frontmatter, pulldown-cmark parses markdown

---

*Integration audit: 2026-01-23*
