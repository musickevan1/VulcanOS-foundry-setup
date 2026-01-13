# vulcan-vault Session Prompts

Copy the relevant prompt when starting a new Claude Code session.

---

## Sprint 2: RAG Pipeline

```
I'm continuing work on vulcan-vault, an Obsidian-based knowledge vault with RAG for AI agents.

**Project location:** /home/evan/VulcanOS/vulcan-vault

**Phase 1 is complete:**
- Rust project with Cargo.toml, models (Note, Chunk, Memory)
- SQLite-vec storage layer (v0.1.6 working)
- Basic MCP server with 15 tools
- CLI with init, stats, remember, recall

**Sprint 2 Goal:** Implement RAG pipeline (chunking, embeddings, semantic search)

**Get my tasks:**
Use vulcan-todo MCP to get sprint tasks:
- Sprint ID: 2266eec1-3451-4119-bbf2-25146a4fcaf1
- Call: get_sprint_tasks(sprint_id="2266eec1-3451-4119-bbf2-25146a4fcaf1")

**Key files to create:**
- src/rag/chunker.rs - Markdown parsing, split by headings
- src/rag/embeddings.rs - Ollama client with OpenAI fallback
- src/rag/search.rs - Vector similarity search

**Reference:** vulcan-todo patterns at /home/evan/VulcanOS/vulcan-todo/

Start the sprint and work through tasks in order.
```

---

## Sprint 3: vulcan-todo Integration

```
I'm continuing work on vulcan-vault, an Obsidian-based knowledge vault with RAG for AI agents.

**Project location:** /home/evan/VulcanOS/vulcan-vault

**Prerequisites complete:**
- Phase 1: Core foundation (models, storage, MCP server)
- Sprint 2: RAG pipeline (chunking, embeddings, search)

**Sprint 3 Goal:** Integrate with vulcan-todo for task-context linking

**Get my tasks:**
Use vulcan-todo MCP to get sprint tasks:
- Sprint ID: e6d9cc5d-6631-4138-b423-66f1140a6657
- Call: get_sprint_tasks(sprint_id="e6d9cc5d-6631-4138-b423-66f1140a6657")

**Key changes:**
- vulcan-todo/src/models/task.rs - Add context_notes, auto_fetch_context fields
- vulcan-todo/src/mcp/tools.rs - Hook into start_task workflow
- vulcan-vault/src/mcp/tools.rs - Add link_note_to_task, get_task_notes tools

**This sprint modifies TWO repos:**
- /home/evan/VulcanOS/vulcan-vault
- /home/evan/VulcanOS/vulcan-todo

Start the sprint and work through tasks in order.
```

---

## Sprint 4: Agent Memory System

```
I'm continuing work on vulcan-vault, an Obsidian-based knowledge vault with RAG for AI agents.

**Project location:** /home/evan/VulcanOS/vulcan-vault

**Prerequisites complete:**
- Phase 1: Core foundation (models, storage, MCP server)
- Sprint 2: RAG pipeline (chunking, embeddings, search)

**Sprint 4 Goal:** Build the agent memory system (formation, retrieval, decay)

**Get my tasks:**
Use vulcan-todo MCP to get sprint tasks:
- Sprint ID: 9660fe3c-56d3-4e16-9ce9-bc50c2b947a9
- Call: get_sprint_tasks(sprint_id="9660fe3c-56d3-4e16-9ce9-bc50c2b947a9")

**Key files to create:**
- src/memory/formation.rs - Create memories from agent observations
- src/memory/retrieval.rs - Semantic search for memories
- src/memory/decay.rs - Confidence decay and reinforcement

**Memory types:** decision, lesson, preference, session

**Existing memory model:** src/models/memory.rs (already has reinforce(), decay() methods)

Start the sprint and work through tasks in order.
```

---

## Sprint 5: Plugin & Polish

```
I'm continuing work on vulcan-vault, an Obsidian-based knowledge vault with RAG for AI agents.

**Project location:** /home/evan/VulcanOS/vulcan-vault

**All core features complete:**
- Phase 1: Core foundation
- Sprint 2: RAG pipeline
- Sprint 3: vulcan-todo integration
- Sprint 4: Agent memory system

**Sprint 5 Goal:** Create Claude Code plugin and documentation

**Get my tasks:**
Use vulcan-todo MCP to get sprint tasks:
- Sprint ID: 1bf12d8e-eba6-46db-b52e-cb49944e1c9f
- Call: get_sprint_tasks(sprint_id="1bf12d8e-eba6-46db-b52e-cb49944e1c9f")

**Key files to create:**
- .claude/plugins/vulcan-vault/
  - .claude-plugin/plugin.json
  - agents/vault-search.md
  - commands/context.md
  - commands/remember.md

**Reference plugin:** /home/evan/VulcanOS/.claude/plugins/vulcanos/

**Also update:**
- .mcp.json for Docker gateway
- docs/VULCAN-VAULT.md for documentation

Start the sprint and work through tasks in order.
```

---

## Execution Order

| Order | Sprint | Can Parallelize? |
|-------|--------|------------------|
| 1 | Sprint 2: RAG Pipeline | No - must be first |
| 2a | Sprint 3: vulcan-todo Integration | Yes - parallel with 4 |
| 2b | Sprint 4: Agent Memory System | Yes - parallel with 3 |
| 3 | Sprint 5: Plugin & Polish | No - must be last |

**Recommended:** Run sequentially for simpler context management. Parallel only if you want to work on both simultaneously.
