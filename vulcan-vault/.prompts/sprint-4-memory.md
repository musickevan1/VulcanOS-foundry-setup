# Sprint 4: Agent Memory System

I'm continuing work on vulcan-vault, an Obsidian-based knowledge vault with RAG for AI agents.

## Project Location
/home/evan/VulcanOS/vulcan-vault

## Prerequisites Complete
- Phase 1: Core foundation (models, storage, MCP server)
- Sprint 2: RAG pipeline (chunking, embeddings, search)

## Sprint 4 Goal
Build the agent memory system: formation, semantic retrieval, confidence decay, reinforcement.

## Get Tasks
Use vulcan-todo MCP:
```
get_sprint_tasks(sprint_id="9660fe3c-56d3-4e16-9ce9-bc50c2b947a9")
start_sprint(id="9660fe3c-56d3-4e16-9ce9-bc50c2b947a9")
```

## Files to Create
- `src/memory/mod.rs` - Module exports (stub exists)
- `src/memory/formation.rs` - Create memories from agent observations
- `src/memory/retrieval.rs` - Semantic search for memories using embeddings
- `src/memory/decay.rs` - Confidence decay and reinforcement logic

## Memory Types
- `decision` - Choices made with context and outcome
- `lesson` - Learnings from errors or corrections
- `preference` - Observed user preferences
- `session` - Session summaries

## Existing Code
- `src/models/memory.rs` - Memory struct with reinforce(), decay() methods
- `src/store/sqlite_store.rs` - save_memory(), search_memories() already implemented
- CLI already has `remember` and `recall` commands

## Key Features to Add
1. Semantic memory retrieval (embed query, search by similarity)
2. Time-based decay (configurable rate, 7-day grace period)
3. Reinforcement on successful application
4. Session summary auto-generation

## Instructions
Start the sprint, get tasks, work through them in order. Mark each complete before moving to next.
