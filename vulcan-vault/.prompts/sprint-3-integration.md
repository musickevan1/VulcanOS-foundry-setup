# Sprint 3: vulcan-todo Integration

I'm continuing work on vulcan-vault, an Obsidian-based knowledge vault with RAG for AI agents.

## Project Location
/home/evan/VulcanOS/vulcan-vault

## Prerequisites Complete
- Phase 1: Core foundation (models, storage, MCP server)
- Sprint 2: RAG pipeline (chunking, embeddings, search)

## Sprint 3 Goal
Integrate with vulcan-todo for bidirectional task-context linking. Auto-fetch context when tasks start.

## Get Tasks
Use vulcan-todo MCP:
```
get_sprint_tasks(sprint_id="e6d9cc5d-6631-4138-b423-66f1140a6657")
start_sprint(id="e6d9cc5d-6631-4138-b423-66f1140a6657")
```

## Files to Modify

### In vulcan-todo repo (/home/evan/VulcanOS/vulcan-todo/)
- `src/models/task.rs` - Add fields:
  ```rust
  pub context_notes: Vec<String>,  // Note UUIDs from vulcan-vault
  pub auto_fetch_context: bool,
  ```
- `src/mcp/tools.rs` - Hook start_task to fetch context

### In vulcan-vault repo
- `src/mcp/tools.rs` - Add link_note_to_task, get_task_notes tools

## Integration Flow
1. `start_task(id)` in vulcan-todo
2. If `auto_fetch_context == true`, call vulcan-vault
3. `get_task_context(task_id)` returns linked notes + project context + memories
4. Return combined context to agent

## Important
This sprint modifies TWO repos. Be careful with imports and ensure both compile.

## Instructions
Start the sprint, get tasks, work through them in order. Mark each complete before moving to next.
