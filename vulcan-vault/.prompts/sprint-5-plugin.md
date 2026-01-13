# Sprint 5: Plugin & Polish

I'm continuing work on vulcan-vault, an Obsidian-based knowledge vault with RAG for AI agents.

## Project Location
/home/evan/VulcanOS/vulcan-vault

## All Core Features Complete
- Phase 1: Core foundation (models, storage, MCP server)
- Sprint 2: RAG pipeline (chunking, embeddings, search)
- Sprint 3: vulcan-todo integration (bidirectional linking)
- Sprint 4: Agent memory system (formation, retrieval, decay)

## Sprint 5 Goal
Create Claude Code plugin with agents, commands, hooks. Add Docker Gateway entry. Write documentation.

## Get Tasks
Use vulcan-todo MCP:
```
get_sprint_tasks(sprint_id="1bf12d8e-eba6-46db-b52e-cb49944e1c9f")
start_sprint(id="1bf12d8e-eba6-46db-b52e-cb49944e1c9f")
```

## Plugin Structure to Create
```
.claude/plugins/vulcan-vault/
├── .claude-plugin/
│   └── plugin.json
├── agents/
│   └── vault-search.md      # Read-only search agent
├── commands/
│   ├── context.md           # /context - fetch task context
│   └── remember.md          # /remember - record memory
└── hooks/
    └── (optional hooks)
```

## Reference Plugin
/home/evan/VulcanOS/.claude/plugins/vulcanos/

## Additional Tasks
- Update .mcp.json for Docker Gateway catalog
- Write docs/VULCAN-VAULT.md
- Create README.md with usage examples

## Command Specs

### /context [task_id]
- Fetch context for current or specified task
- Return linked notes, project context, relevant memories
- Format for easy reading

### /remember --type <type> <content>
- Record a memory (decision/lesson/preference)
- Auto-detect context from conversation

## Instructions
Start the sprint, get tasks, work through them in order. Mark each complete before moving to next.
