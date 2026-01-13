# vulcan-vault

**Obsidian-based knowledge vault with RAG capabilities for AI agents**

vulcan-vault provides persistent memory and semantic search for Claude Code and other AI agents in the VulcanOS ecosystem. It combines local vector embeddings, structured markdown storage, and agent memory mechanics inspired by human cognition.

## Features

- **Local-first RAG** - Vector search via SQLite-vec, embeddings via Ollama
- **Agent Memory** - Decisions, lessons, preferences with confidence decay
- **Obsidian Compatible** - Valid vault structure, YAML frontmatter, wikilinks
- **Task Integration** - Bidirectional linking with vulcan-todo
- **MCP Server** - 28 tools for AI agent access
- **Zero Cloud Dependencies** - Everything runs locally

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         vulcan-vault                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   ┌──────────┐    ┌──────────┐    ┌────────────────────────┐   │
│   │  Claude  │───▶│   MCP    │───▶│     Store Layer        │   │
│   │   Code   │    │  Server  │    │  (SQLite + sqlite-vec) │   │
│   └──────────┘    └──────────┘    └────────────────────────┘   │
│                         │                    │                  │
│                         ▼                    ▼                  │
│               ┌──────────────┐     ┌──────────────────┐        │
│               │     RAG      │     │     Memory       │        │
│               │   Pipeline   │     │     System       │        │
│               │  ┌─────────┐ │     │ ┌─────────────┐  │        │
│               │  │ Chunker │ │     │ │  Formation  │  │        │
│               │  └────┬────┘ │     │ │  Retrieval  │  │        │
│               │       ▼      │     │ │    Decay    │  │        │
│               │  ┌─────────┐ │     │ └─────────────┘  │        │
│               │  │ Embedder│ │     └──────────────────┘        │
│               │  └─────────┘ │                                  │
│               └──────────────┘                                  │
│                      │                                          │
│                      ▼                                          │
│               ┌──────────────┐                                  │
│               │    Ollama    │                                  │
│               │   (local)    │                                  │
│               └──────────────┘                                  │
│                                                                  │
│   Zones: Projects/ │ Tasks/ │ Learning/ │ Agent-Memories/       │
└─────────────────────────────────────────────────────────────────┘
```

## Quick Start

### Prerequisites

```bash
# Install Ollama and pull embedding model
curl -fsSL https://ollama.com/install.sh | sh
ollama pull nomic-embed-text

# Verify Ollama is running
ollama list
```

### Installation

```bash
cd vulcan-vault
cargo build --release
sudo cp target/release/vulcan-vault /usr/local/bin/
```

### Initialize Vault

```bash
vulcan-vault init
```

This creates the vault structure at `~/.config/vulcan-vault/vault/`:

```
vault/
├── Projects/                    # Project documentation
│   └── {project-name}/
│       ├── _index.md           # Project overview
│       ├── architecture.md     # Technical design
│       └── decisions/          # ADRs
├── Tasks/                       # Task-linked context
│   ├── by-id/                  # Notes linked to vulcan-todo tasks
│   └── templates/              # Task note templates
├── Learning/                    # Study materials
│   ├── courses/
│   ├── topics/
│   └── reading-notes/
├── Agent-Memories/              # AI agent memory
│   ├── decisions/              # Architecture choices
│   ├── lessons/                # Learnings from errors
│   ├── preferences/            # User preferences
│   └── sessions/               # Session summaries
├── Meta/                        # Glossary, tags
├── Templates/                   # Note templates
└── .obsidian/                   # Obsidian config
```

### Basic Usage

```bash
# Record a memory
vulcan-vault remember --type lesson "Always validate input before processing"

# Recall memories
vulcan-vault recall "input validation" --min-confidence 0.3

# List notes
vulcan-vault list --type project

# Search notes
vulcan-vault query "error handling patterns"

# View statistics
vulcan-vault stats
```

### Run as MCP Server

```bash
# Start MCP server (for Claude Code integration)
vulcan-vault --mcp
```

## Data Flow

### Note Processing

```
┌────────────────┐     ┌──────────┐     ┌────────────┐     ┌───────────┐
│   Markdown +   │────▶│  Parser  │────▶│  Chunker   │────▶│  Ollama   │
│ YAML Frontmatter│     │          │     │            │     │ Embeddings│
└────────────────┘     └──────────┘     └────────────┘     └───────────┘
                                              │                   │
                                              ▼                   ▼
                                        ┌──────────┐       ┌───────────┐
                                        │  Chunks  │       │  768-dim  │
                                        │ (text)   │       │  vectors  │
                                        └──────────┘       └───────────┘
                                              │                   │
                                              └─────────┬─────────┘
                                                        ▼
                                               ┌────────────────┐
                                               │   SQLite-vec   │
                                               │    vault.db    │
                                               └────────────────┘
```

### Memory Retrieval

```
┌─────────┐     ┌────────┐     ┌──────────────┐     ┌─────────────┐
│  Query  │────▶│ Embed  │────▶│ Vector Search│────▶│   Score &   │
│         │     │        │     │ (cosine sim) │     │   Rank      │
└─────────┘     └────────┘     └──────────────┘     └─────────────┘
                                                           │
                                                           ▼
                                           ┌───────────────────────────┐
                                           │ score = 0.7 × similarity  │
                                           │       + 0.3 × confidence  │
                                           └───────────────────────────┘
```

## CLI Reference

| Command | Description |
|---------|-------------|
| `vulcan-vault init` | Initialize vault directory structure |
| `vulcan-vault list [--type TYPE] [--project NAME]` | List notes with filters |
| `vulcan-vault query QUERY [--project NAME]` | Semantic search |
| `vulcan-vault stats` | Show vault statistics |
| `vulcan-vault rebuild [--force]` | Rebuild all embeddings |
| `vulcan-vault task-context TASK_ID` | Get notes linked to task |
| `vulcan-vault remember --type TYPE CONTENT` | Record a memory |
| `vulcan-vault recall CONTEXT [--min-confidence N]` | Search memories |
| `vulcan-vault --mcp` | Run as MCP server |

### Examples

```bash
# List all project notes
vulcan-vault list --type project

# Search for authentication-related content
vulcan-vault query "OAuth implementation"

# Record a decision
vulcan-vault remember --type decision "Using SQLite-vec for vector storage"

# Record a lesson learned
vulcan-vault remember --type lesson "Always check Ollama health before embedding"

# Recall relevant memories
vulcan-vault recall "database design" --min-confidence 0.5 --limit 5

# Rebuild embeddings for all notes
vulcan-vault rebuild --force

# Get context for a vulcan-todo task
vulcan-vault task-context abc123-def456
```

## MCP Tools

vulcan-vault exposes 28 MCP tools for AI agent integration:

### Note Management

| Tool | Description |
|------|-------------|
| `create_note` | Create new note with type, content, tags |
| `get_note` | Retrieve note by ID or path |
| `update_note` | Update note fields |
| `delete_note` | Delete note and associated chunks |
| `list_notes` | List notes with filters |
| `search_notes` | Keyword search |

### Semantic Search

| Tool | Description |
|------|-------------|
| `semantic_search` | Vector similarity search |
| `search_by_context` | Search with context filtering |
| `find_similar` | Find notes similar to a given note |
| `get_backlinks` | Find notes referencing this note |
| `get_related` | Find related via wikilinks |

### Task Integration

| Tool | Description |
|------|-------------|
| `get_task_context` | Get all context for a task |
| `link_note_to_task` | Link note to vulcan-todo task |
| `unlink_note_from_task` | Remove task link |
| `get_notes_by_task` | Get all notes for a task |

### Agent Memory

| Tool | Description |
|------|-------------|
| `record_decision` | Record an architecture decision |
| `record_lesson` | Record a lesson learned |
| `record_preference` | Record a user preference |
| `recall_memories` | Search memories by context |
| `recall_memories_semantic` | Semantic memory search |
| `reinforce_memory` | Increase confidence when applied |

### Memory Maintenance

| Tool | Description |
|------|-------------|
| `apply_decay` | Run confidence decay |
| `cleanup_expired` | Archive low-confidence memories |

### Statistics

| Tool | Description |
|------|-------------|
| `get_vault_stats` | Note counts, types, projects |
| `get_memory_stats` | Memory metrics |

## Memory System

The memory system mimics human memory with formation, retrieval, and decay:

### Memory Types

| Type | Purpose | Initial Confidence |
|------|---------|-------------------|
| `Decision` | Architecture/implementation choices | 0.8 |
| `Lesson` | Learnings from errors or successes | 0.8 |
| `Preference` | Observed user preferences | 0.9 |
| `Session` | Session summaries | 1.0 |

### Confidence Lifecycle

```
    Formation              Reinforcement              Decay
        │                       │                       │
        ▼                       ▼                       ▼
   ┌─────────┐            ┌─────────┐            ┌──────────┐
   │conf=0.8 │───────────▶│  +0.1   │───────────▶│-0.01/day │
   └─────────┘            └─────────┘            └──────────┘
   (new memory)        (successfully            (after 7-day
                         applied)               grace period)
                                                      │
                                                      ▼
                                               ┌──────────┐
                                               │ Archive  │
                                               │ if <0.1  │
                                               └──────────┘
```

### Retrieval Scoring

Memories are scored by combining semantic similarity with confidence:

```
score = (0.7 × similarity) + (0.3 × confidence)
```

- **Similarity**: How semantically close the memory is to the query (0.0-1.0)
- **Confidence**: How reliable/relevant the memory is (0.0-1.0, decays over time)

## Integration

### vulcan-todo

Bidirectional task-context linking:

```bash
# Link a note to a task
vulcan-vault link-note NOTE_ID TASK_ID

# Get all context when starting a task
vulcan-vault task-context TASK_ID
```

Notes can have `task_id` field in frontmatter:

```yaml
---
type: task
task_id: abc123-def456
auto_fetch: true
context_type: implementation
---
```

### Ollama

Local embeddings via Ollama:

```bash
# Required model
ollama pull nomic-embed-text

# Verify
ollama list | grep nomic
```

Configuration:
- **Model**: `nomic-embed-text` (768 dimensions)
- **Endpoint**: `http://localhost:11434`
- **Timeout**: 30 seconds

### Obsidian

The vault is a valid Obsidian vault:

1. Open Obsidian
2. "Open folder as vault"
3. Select `~/.config/vulcan-vault/vault/`

Features supported:
- YAML frontmatter properties
- Wikilinks (`[[Note Name]]`)
- Tags
- Folders as organization

## Configuration

### Default Paths

| Path | Purpose |
|------|---------|
| `~/.config/vulcan-vault/` | Config directory |
| `~/.config/vulcan-vault/vault/` | Obsidian vault root |
| `~/.config/vulcan-vault/vault.db` | SQLite database |

### Environment Variables

```bash
# Custom vault path (optional)
VULCAN_VAULT_PATH=/path/to/vault
```

## Claude Code Plugin

A Claude Code plugin is included at `.claude/plugins/vulcan-vault/`:

### Commands

- `/context [task_id]` - Fetch context for current or specified task
- `/remember --type TYPE CONTENT` - Record a memory

### Agent

- `@vault-search` - Read-only knowledge retrieval agent

### Setup

Add to `.mcp.json`:

```json
{
  "vulcan-vault": {
    "command": "vulcan-vault",
    "args": ["--mcp"]
  }
}
```

## Troubleshooting

### Ollama not available

```
Error: Ollama is not available. Start it with: ollama serve
```

**Solution:**
```bash
ollama serve &
ollama pull nomic-embed-text
```

### Embeddings not working

```bash
# Check Ollama status
curl http://localhost:11434/api/tags

# Verify model
ollama list | grep nomic-embed-text
```

### MCP server not responding

```bash
# Test MCP protocol
echo '{"jsonrpc":"2.0","method":"tools/list","id":1}' | vulcan-vault --mcp
```

### Database locked

If you see "database is locked" errors, ensure only one process is accessing the vault.

## Development

### Build

```bash
cargo build --release
```

### Test

```bash
# Run all tests
cargo test

# Run with Ollama integration tests
cargo test -- --ignored
```

### Project Structure

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Public API
├── models/              # Data structures
│   ├── note.rs         # Note, NoteType, NoteStatus
│   ├── chunk.rs        # Chunk, ChunkConfig
│   └── memory.rs       # Memory, MemoryType
├── store/               # Persistence
│   ├── mod.rs          # Store trait
│   ├── sqlite_store.rs # SQLite implementation
│   └── error.rs        # Error types
├── rag/                 # RAG pipeline
│   ├── mod.rs          # Pipeline orchestration
│   ├── chunker.rs      # Markdown chunking
│   └── embeddings.rs   # Ollama integration
├── memory/              # Memory system
│   ├── formation.rs    # Recording memories
│   ├── retrieval.rs    # Semantic search
│   └── decay.rs        # Confidence decay
└── mcp/                 # MCP server
    ├── server.rs       # JSON-RPC handler
    ├── protocol.rs     # Protocol types
    └── tools.rs        # Tool implementations
```

## License

Part of VulcanOS. See repository root for license.
