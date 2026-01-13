# Vulcan-Vault: Knowledge Management for AI Agents

Vulcan-vault is an Obsidian-based knowledge vault with RAG (Retrieval-Augmented Generation) capabilities designed for AI agents.

## Overview

Vulcan-vault provides:
- **Persistent knowledge base** - Notes organized by project, task, and topic
- **Semantic search** - Find information by meaning, not just keywords
- **Agent memory** - Record and recall decisions, lessons, and preferences
- **Task integration** - Link context to vulcan-todo tasks

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Claude Code Plugin                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   /context   │  │  /remember   │  │ @vault-search│      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└────────────────────────────┬────────────────────────────────┘
                             │ MCP Protocol
┌────────────────────────────▼────────────────────────────────┐
│                    Vulcan-Vault MCP Server                   │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐        │
│  │  Notes  │  │ Search  │  │ Memory  │  │  Tasks  │        │
│  │  CRUD   │  │  (RAG)  │  │ System  │  │  Links  │        │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘        │
└────────────────────────────┬────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────┐
│                        Storage Layer                         │
│  ┌──────────────────────┐  ┌──────────────────────┐        │
│  │   SQLite + vec       │  │   Markdown Files     │        │
│  │   (embeddings)       │  │   (Obsidian vault)   │        │
│  └──────────────────────┘  └──────────────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

## Installation

### Prerequisites
- Rust toolchain (for building)
- Ollama with `nomic-embed-text` model (for embeddings)

### Build
```bash
cd /home/evan/VulcanOS/vulcan-vault
cargo build --release
```

### Install
```bash
# Add to PATH or copy to ~/.local/bin
cp target/release/vulcan-vault ~/.local/bin/
```

### Enable Plugin
The Claude Code plugin at `.claude/plugins/vulcan-vault/` auto-configures the MCP server.

## Usage

### CLI Commands

```bash
# Start MCP server (used by Claude Code)
vulcan-vault --mcp

# Initialize vault
vulcan-vault init

# Quick note creation
vulcan-vault note "Title" --type learning --content "..."
```

### Claude Code Commands

```bash
# Get context for current task
/context

# Record a memory
/remember --type decision "Chose X over Y because..."

# Search the vault
@vault-search find information about error handling
```

## MCP Tools Reference

### Note Operations
| Tool | Description |
|------|-------------|
| `create_note` | Create new note with title, type, content |
| `get_note` | Get note by ID or path |
| `list_notes` | List notes with filters |
| `search_notes` | Keyword search |
| `delete_note` | Remove a note |

### Task Integration
| Tool | Description |
|------|-------------|
| `get_task_context` | Get context notes for a task |
| `link_note_to_task` | Link note to vulcan-todo task |
| `unlink_note_from_task` | Remove task link |
| `get_task_notes` | Comprehensive task context |
| `create_task_context` | Create and link context note |

### Agent Memory
| Tool | Description |
|------|-------------|
| `record_lesson` | Record a lesson learned |
| `record_decision` | Record a decision |
| `record_preference` | Record user preference |
| `recall_memories` | Search memories by context |
| `reinforce_memory` | Increase memory confidence |

### Search
| Tool | Description |
|------|-------------|
| `semantic_search` | Vector similarity search |
| `search_notes` | Keyword search |

### Utility
| Tool | Description |
|------|-------------|
| `get_stats` | Vault statistics |
| `get_project_context` | Get project notes |
| `list_projects` | List all projects |

## Vault Structure

```
~/.config/vulcan-vault/vault/
├── Projects/                   # Project documentation
│   └── vulcan-vault/
│       └── architecture.md
├── Tasks/
│   └── by-id/                  # Task context notes
│       └── abc-123.md
├── Learning/
│   ├── courses/                # Course notes
│   ├── topics/                 # Topic collections
│   └── reading-notes/          # Article summaries
├── Agent-Memories/
│   ├── decisions/              # Decision records
│   ├── lessons/                # Lesson records
│   ├── preferences/            # Preference records
│   └── sessions/               # Session summaries
├── Meta/                       # Cross-cutting metadata
├── Templates/                  # Note templates
└── .obsidian/                  # Obsidian vault config
```

## Note Types

| Type | Purpose | Location |
|------|---------|----------|
| `project` | Project documentation | Projects/ |
| `task` | Task context | Tasks/by-id/ |
| `learning` | Knowledge capture | Learning/ |
| `memory` | Agent memories | Agent-Memories/ |
| `meta` | Metadata and indices | Meta/ |

## Integration with vulcan-todo

Vulcan-vault integrates bidirectionally with vulcan-todo:

1. **Notes link to tasks** - Each note can have a `task_id` field
2. **Tasks reference notes** - `get_task_notes` fetches all linked context
3. **Project alignment** - Notes and tasks share project names

Example workflow:
```bash
# 1. Get current task
mcp__vulcan-todo__get_context

# 2. Fetch vault context for task
mcp__vulcan_vault__get_task_notes(task_id="...")

# 3. Work on task with context...

# 4. Record insights
/remember --type lesson "Discovered X when implementing Y"
```

## Memory System

The agent memory system captures knowledge for future recall:

### Memory Types
- **Decision** - Why a choice was made (architectural, design)
- **Lesson** - What was learned from success or failure
- **Preference** - User preferences observed over time

### Memory Lifecycle
1. **Formation** - Memory recorded with context and tags
2. **Retrieval** - Memories surfaced when context matches
3. **Reinforcement** - Confidence increases on successful application
4. **Decay** - Unused memories decay over time

### Recall Algorithm
Memories are retrieved based on:
- Context similarity (keyword matching)
- Memory type filter
- Confidence threshold
- Recency

## Semantic Search

Vulcan-vault uses Ollama for embeddings:

### Setup
```bash
# Install Ollama
# Pull embedding model
ollama pull nomic-embed-text
```

### How It Works
1. Notes are chunked by headers
2. Each chunk gets a 768-dim embedding
3. Search queries are embedded
4. Cosine similarity finds relevant chunks

### Fallback
If Ollama is unavailable, keyword search is used instead.

## Configuration

Vault location: `~/.config/vulcan-vault/`
- `vault/` - Markdown files (Obsidian vault)
- `vault.db` - SQLite database with embeddings

## Troubleshooting

### MCP server won't start
```bash
# Check binary exists
which vulcan-vault

# Test MCP mode
vulcan-vault --mcp
# Should output JSON-RPC capabilities
```

### Semantic search not working
```bash
# Check Ollama
ollama list
# Should show nomic-embed-text

# If missing
ollama pull nomic-embed-text
```

### No context for tasks
- Ensure task_id matches between vault notes and vulcan-todo
- Use `link_note_to_task` to create links
- Use `/remember` to record context as you work

## Development

Source: `/home/evan/VulcanOS/vulcan-vault/`

```bash
# Build
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug vulcan-vault --mcp
```

## Related Documentation

- [vulcan-todo](./VULCAN-TODO.md) - Task management system
- [CLAUDE.md](../CLAUDE.md) - VulcanOS development guide
