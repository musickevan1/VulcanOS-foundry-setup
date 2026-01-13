# Sprint 2: RAG Pipeline

I'm continuing work on vulcan-vault, an Obsidian-based knowledge vault with RAG for AI agents.

## Project Location
/home/evan/VulcanOS/vulcan-vault

## Phase 1 Complete
- Rust project with Cargo.toml, models (Note, Chunk, Memory)
- SQLite-vec storage layer (v0.1.6 working)
- Basic MCP server with 15 tools
- CLI with init, stats, remember, recall

## Sprint 2 Goal
Implement RAG pipeline: markdown chunking, embedding generation, semantic search via sqlite-vec

## Get Tasks
Use vulcan-todo MCP:
```
get_sprint_tasks(sprint_id="2266eec1-3451-4119-bbf2-25146a4fcaf1")
start_sprint(id="2266eec1-3451-4119-bbf2-25146a4fcaf1")
```

## Files to Create
- `src/rag/mod.rs` - Module exports
- `src/rag/chunker.rs` - Markdown parsing, split by headings, respect code blocks
- `src/rag/embeddings.rs` - Ollama client (nomic-embed-text) with OpenAI fallback
- `src/rag/search.rs` - Vector similarity search via sqlite-vec

## Key Specs
- Chunk size: 1000 chars max, 100 char overlap
- Embedding dims: 768 (nomic-embed-text)
- Split on ## headings, keep code blocks intact

## Reference
- vulcan-todo patterns: /home/evan/VulcanOS/vulcan-todo/
- Existing store: src/store/sqlite_store.rs (has vector_search method)

## Instructions
Start the sprint, get tasks, work through them in order. Mark each complete before moving to next.
