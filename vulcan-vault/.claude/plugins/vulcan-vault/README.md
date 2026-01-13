# Vulcan-Vault Plugin for Claude Code

Knowledge vault integration providing RAG search and agent memory for Claude Code.

## Quick Start

1. **Enable the plugin** - The plugin auto-starts vulcan-vault MCP server
2. **Use commands** - `/context` for task context, `/remember` to record memories
3. **Search** - `@vault-search` for knowledge retrieval

## Commands

### /context [task_id]

Fetch knowledge context for the current or specified task.

```bash
/context                    # Context for current in-progress task
/context abc-123-def        # Context for specific task
```

Returns:
- Linked notes from the vault
- Project context
- Relevant memories (decisions, lessons)

### /remember --type <type> <content>

Record a memory to the vault for future recall.

```bash
/remember --type decision "Using SQLite for local storage"
/remember --type lesson "Always validate config before startup"
/remember --type preference "User prefers verbose error messages"
```

Memory types:
- `decision` - Architecture or implementation choices
- `lesson` - Learnings from errors or successes
- `preference` - Observed user preferences

## Agents

### @vault-search

Read-only agent for searching the knowledge vault.

```bash
@vault-search find notes about error handling
@vault-search what decisions were made about the API design?
@vault-search recall memories about user preferences
```

Capabilities:
- Semantic search (concepts and meaning)
- Keyword search (specific terms)
- Memory recall (decisions, lessons, preferences)
- Task and project context

## MCP Tools

When the plugin is enabled, these vulcan-vault MCP tools are available:

| Tool | Purpose |
|------|---------|
| `semantic_search` | Vector similarity search |
| `search_notes` | Keyword search |
| `recall_memories` | Memory retrieval |
| `get_task_notes` | Task context |
| `get_project_context` | Project notes |
| `record_decision` | Record a decision |
| `record_lesson` | Record a lesson |
| `record_preference` | Record a preference |
| `create_note` | Create vault note |
| `list_notes` | Browse notes |

Full list: 28 tools available via `mcp__vulcan_vault__*`

## Requirements

- vulcan-vault binary installed and in PATH
- Ollama running locally (for semantic search)
- vulcan-todo MCP (for task integration)

## Integration with vulcan-todo

The vault integrates bidirectionally with vulcan-todo:
- Notes can link to tasks via `task_id`
- `/context` fetches vault context for tasks
- Memories can be associated with projects

## Vault Location

Default: `~/.config/vulcan-vault/vault/`

Structure:
```
vault/
├── Projects/           # Project documentation
├── Tasks/by-id/        # Task context notes
├── Learning/           # Courses, topics, reading notes
├── Agent-Memories/     # Decisions, lessons, preferences
└── .obsidian/          # Obsidian vault config
```

## Troubleshooting

**MCP tools not available:**
- Check if vulcan-vault is in PATH: `which vulcan-vault`
- Verify MCP server works: `vulcan-vault --mcp` (should show JSON-RPC)

**Semantic search not working:**
- Ensure Ollama is running: `ollama list`
- Check model available: `ollama pull nomic-embed-text`

**No context for tasks:**
- Use `/remember` to record context as you work
- Link notes to tasks via `link_note_to_task` tool
