# vulcan-vault MCP Tools Reference

Complete API reference for all MCP tools exposed by vulcan-vault.

## Overview

vulcan-vault exposes **20 MCP tools** via JSON-RPC 2.0 over stdio. Tools are organized into categories:

| Category | Tools | Purpose |
|----------|-------|---------|
| Note CRUD | 5 | Create, read, update, delete notes |
| Task Integration | 6 | Link notes to vulcan-todo tasks |
| Project Context | 2 | Project-level knowledge |
| Agent Memory | 5 | Record and recall memories |
| Statistics | 1 | Vault metrics |
| Semantic Search | 1 | Vector similarity search |

---

## Note CRUD Operations

### create_note

Create a new note in the vault.

**Input Schema:**
```json
{
  "title": "string (required)",
  "note_type": "project|task|learning|memory|meta (required)",
  "content": "string (required)",
  "project": "string (optional, required for project notes)",
  "task_id": "string (optional, required for task notes)",
  "tags": ["string"]
}
```

**Example:**
```json
{
  "title": "Authentication Architecture",
  "note_type": "project",
  "content": "# Auth System\n\nUsing JWT with refresh tokens...",
  "project": "vulcan-api",
  "tags": ["auth", "security"]
}
```

**Response:**
```json
{
  "content": [{
    "type": "text",
    "text": "Created note: Authentication Architecture (uuid-here)"
  }]
}
```

---

### get_note

Retrieve a note by ID or path.

**Input Schema:**
```json
{
  "id": "string (optional)",
  "path": "string (optional)"
}
```

*One of `id` or `path` is required.*

**Example:**
```json
{
  "id": "abc123-def456-..."
}
```

**Response:**
```json
{
  "content": [{
    "type": "text",
    "text": "{ \"id\": \"...\", \"title\": \"...\", \"content\": \"...\" }"
  }]
}
```

---

### list_notes

List notes with optional filters.

**Input Schema:**
```json
{
  "note_type": "project|task|learning|memory|meta (optional)",
  "project": "string (optional)",
  "limit": "integer (default: 20)"
}
```

**Example:**
```json
{
  "note_type": "project",
  "project": "vulcan-vault",
  "limit": 10
}
```

**Response:**
```json
{
  "content": [{
    "type": "text",
    "text": "[{\"id\":\"...\",\"title\":\"...\",\"type\":\"project\",\"path\":\"...\"}]"
  }]
}
```

---

### search_notes

Keyword search across notes.

**Input Schema:**
```json
{
  "query": "string (required)",
  "limit": "integer (default: 10)"
}
```

**Example:**
```json
{
  "query": "error handling",
  "limit": 5
}
```

---

### delete_note

Delete a note and its associated chunks.

**Input Schema:**
```json
{
  "id": "string (required)"
}
```

---

## Task Integration

### get_task_context

Get context notes linked to a vulcan-todo task.

**Input Schema:**
```json
{
  "task_id": "string (required)"
}
```

**Example:**
```json
{
  "task_id": "abc123-def456-..."
}
```

**Response:**
```json
{
  "content": [{
    "type": "text",
    "text": "[{\"id\":\"...\",\"title\":\"...\",\"context_type\":\"implementation\",\"content\":\"...\"}]"
  }]
}
```

---

### link_note_to_task

Link an existing note to a vulcan-todo task.

**Input Schema:**
```json
{
  "note_id": "string (required)",
  "task_id": "string (required)"
}
```

**Response includes `data` for updating task.context_notes:**
```json
{
  "content": [{"type": "text", "text": "Linked note '...' to task ..."}],
  "data": {
    "note_id": "...",
    "task_id": "...",
    "note_title": "..."
  }
}
```

---

### unlink_note_from_task

Remove task link from a note.

**Input Schema:**
```json
{
  "note_id": "string (required)"
}
```

---

### get_task_notes

Get comprehensive context for a task: linked notes, project context, and relevant memories.

**Input Schema:**
```json
{
  "task_id": "string (required)",
  "project": "string (optional)",
  "include_project": "boolean (default: true)",
  "include_memories": "boolean (default: true)"
}
```

**Example:**
```json
{
  "task_id": "abc123...",
  "project": "vulcan-vault",
  "include_memories": true
}
```

**Response:**
```json
{
  "task_notes": [{"id": "...", "title": "...", "content": "..."}],
  "project_notes": [{"id": "...", "title": "..."}],
  "memories": [{"id": "...", "type": "decision", "title": "...", "confidence": 0.8}],
  "summary": {
    "task_note_count": 2,
    "project_note_count": 5,
    "memory_count": 3
  }
}
```

---

### get_notes_by_ids

Batch retrieve notes by UUIDs. Useful for validating `context_notes` from vulcan-todo.

**Input Schema:**
```json
{
  "note_ids": ["string"]
}
```

**Response:**
```json
{
  "found": [{"id": "...", "title": "...", "path": "..."}],
  "missing": ["uuid-not-found"],
  "summary": {
    "found_count": 3,
    "missing_count": 1
  }
}
```

---

### create_task_context

Create a context note linked to a task.

**Input Schema:**
```json
{
  "task_id": "string (required)",
  "title": "string (required)",
  "content": "string (required)",
  "context_type": "implementation|research|blockers|notes (optional)"
}
```

---

## Project Context

### get_project_context

Get all notes for a project.

**Input Schema:**
```json
{
  "project": "string (required)"
}
```

---

### list_projects

List all projects in the vault.

**Input Schema:**
```json
{}
```

**Response:**
```json
{
  "content": [{
    "type": "text",
    "text": "[\"vulcan-vault\", \"vulcan-todo\", \"vulcanos\"]"
  }]
}
```

---

## Agent Memory

### record_lesson

Record a lesson learned from experience.

**Input Schema:**
```json
{
  "title": "string (required)",
  "content": "string (required)",
  "context": "string (required) - When this lesson applies",
  "tags": ["string"]
}
```

**Example:**
```json
{
  "title": "Always validate config before startup",
  "content": "Missing config fields caused silent failures. Now we validate all required fields on startup and provide clear error messages.",
  "context": "configuration rust startup",
  "tags": ["rust", "error-handling"]
}
```

**Initial confidence:** 0.8

---

### record_decision

Record an architecture or implementation decision.

**Input Schema:**
```json
{
  "title": "string (required)",
  "content": "string (required)",
  "context": "string (required)",
  "tags": ["string"]
}
```

**Example:**
```json
{
  "title": "Using SQLite-vec for vector storage",
  "content": "Chose SQLite with sqlite-vec extension over PostgreSQL+pgvector for:\n- Single-file deployment\n- No daemon required\n- Matches vulcan-todo pattern",
  "context": "database storage vector-search",
  "tags": ["architecture", "database"]
}
```

**Initial confidence:** 0.8

---

### record_preference

Record an observed user preference.

**Input Schema:**
```json
{
  "title": "string (required)",
  "content": "string (required)",
  "context": "string (required)"
}
```

**Example:**
```json
{
  "title": "Prefers verbose error messages",
  "content": "User prefers detailed error messages with suggestions for fixes rather than terse error codes.",
  "context": "error-handling user-experience"
}
```

**Initial confidence:** 0.9

---

### recall_memories

Search for relevant memories by context.

**Input Schema:**
```json
{
  "context": "string (required)",
  "memory_type": "decision|lesson|preference|session (optional)",
  "min_confidence": "number (default: 0.3)",
  "limit": "integer (default: 10)"
}
```

**Example:**
```json
{
  "context": "database design",
  "memory_type": "decision",
  "min_confidence": 0.5,
  "limit": 5
}
```

**Response:**
```json
[
  {
    "id": "...",
    "type": "decision",
    "title": "Using SQLite-vec for vector storage",
    "content": "...",
    "context": "database storage",
    "confidence": 0.8,
    "times_applied": 3
  }
]
```

---

### reinforce_memory

Increase memory confidence after successful application.

**Input Schema:**
```json
{
  "id": "string (required)"
}
```

**Effect:**
- `confidence += 0.1` (max 1.0)
- `times_applied += 1`
- `last_applied = now`

---

## Statistics

### get_stats

Get vault statistics.

**Input Schema:**
```json
{}
```

**Response:**
```json
{
  "total_notes": 42,
  "notes_by_type": {
    "project": 10,
    "task": 15,
    "learning": 8,
    "memory": 7,
    "meta": 2
  },
  "total_chunks": 156,
  "total_links": 23,
  "total_memories": 12,
  "projects": ["vulcan-vault", "vulcan-todo"]
}
```

---

## Semantic Search

### semantic_search

Vector similarity search using Ollama embeddings.

**Requirements:** Ollama running with `nomic-embed-text` model.

**Input Schema:**
```json
{
  "query": "string (required)",
  "limit": "integer (default: 10)",
  "note_types": ["project|task|learning|memory|meta"],
  "project": "string (optional)",
  "tags": ["string"],
  "min_similarity": "number 0-1 (default: 0)"
}
```

**Example:**
```json
{
  "query": "error handling patterns in rust",
  "limit": 5,
  "note_types": ["project", "learning"],
  "min_similarity": 0.5
}
```

**Response:**
```json
[
  {
    "note_id": "...",
    "note_title": "Error Handling Best Practices",
    "note_path": "Learning/topics/rust-errors.md",
    "note_type": "learning",
    "project": null,
    "heading": "Using Result and Option",
    "content": "The ? operator provides ergonomic error propagation...",
    "similarity": "0.87"
  }
]
```

**Similarity Calculation:**
```
similarity = 1 - (cosine_distance / 2)
```
- `0.0` = completely different
- `1.0` = identical

---

## Error Handling

All tools return errors in MCP format:

```json
{
  "content": [{
    "type": "text",
    "text": "Error message here"
  }],
  "isError": true
}
```

Common errors:
- `"Missing <field>"` - Required parameter not provided
- `"Note not found: <id>"` - Note doesn't exist
- `"Memory not found: <id>"` - Memory doesn't exist
- `"Embedding error: ... Is Ollama running?"` - Ollama unavailable

---

## MCP Protocol

vulcan-vault uses JSON-RPC 2.0 over stdio.

### List Tools

**Request:**
```json
{"jsonrpc":"2.0","method":"tools/list","id":1}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "tools": [
      {
        "name": "create_note",
        "description": "Create a new note in the vault",
        "inputSchema": {...}
      }
    ]
  }
}
```

### Call Tool

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "create_note",
    "arguments": {
      "title": "My Note",
      "note_type": "learning",
      "content": "..."
    }
  },
  "id": 2
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "content": [{
      "type": "text",
      "text": "Created note: My Note (uuid)"
    }]
  }
}
```

---

## Usage Examples

### Claude Code Integration

Add to `.mcp.json`:
```json
{
  "vulcan-vault": {
    "command": "vulcan-vault",
    "args": ["--mcp"]
  }
}
```

### Direct Testing

```bash
# List tools
echo '{"jsonrpc":"2.0","method":"tools/list","id":1}' | vulcan-vault --mcp

# Create a note
echo '{"jsonrpc":"2.0","method":"tools/call","params":{"name":"create_note","arguments":{"title":"Test","note_type":"meta","content":"Hello"}},"id":2}' | vulcan-vault --mcp

# Search semantically
echo '{"jsonrpc":"2.0","method":"tools/call","params":{"name":"semantic_search","arguments":{"query":"error handling"}},"id":3}' | vulcan-vault --mcp
```

### Task Context Workflow

```
1. Start task in vulcan-todo
   └─> get task.context_notes

2. Validate notes exist
   └─> get_notes_by_ids(context_notes)

3. Get comprehensive context
   └─> get_task_notes(task_id, project)

4. Work on task, record learnings
   └─> record_lesson(...) or record_decision(...)

5. Create context for future
   └─> create_task_context(task_id, ...)

6. Complete task, reinforce useful memories
   └─> reinforce_memory(memory_id)
```
