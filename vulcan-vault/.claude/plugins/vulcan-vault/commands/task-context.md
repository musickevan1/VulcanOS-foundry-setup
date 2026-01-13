---
description: Fetch knowledge context for current or specified task from vulcan-vault
argument-hint: [task_id]
allowed-tools: Read, Glob, Grep
---

# /task-context - Task Context Retrieval

Fetch comprehensive context for the current or specified task from vulcan-vault.

## Usage

```
/task-context               # Get context for current in-progress task
/task-context <task_id>     # Get context for specific task
```

## Workflow

### 1. Identify Target Task

If no task_id provided:
1. Call `mcp__vulcan-todo__get_context` to find in-progress tasks
2. Use the first in-progress task, or prompt user to specify

If task_id provided:
1. Validate task exists with `mcp__vulcan-todo__get_task`

### 2. Fetch Vault Context

Call `mcp__vulcan_vault__get_task_notes` with the task_id to retrieve:
- **Linked notes** - Notes directly linked to this task
- **Project context** - Notes from the task's project
- **Relevant memories** - Decisions, lessons, preferences related to this work

### 3. Format and Display

Present context in a structured format:

```markdown
## Task: [Task Title]
**Project:** [Project name]
**Priority:** [Priority]

### Linked Notes
- [Note title] - [Brief excerpt]
...

### Project Context
- [Project note title] - [Brief excerpt]
...

### Relevant Memories
- [Memory type]: [Memory title] - [Content summary]
...

### Key Insights
[Synthesize 2-3 key points from the context that may help with the task]
```

## Fallback Behavior

If vulcan-vault MCP is unavailable:
- Inform user that vault context cannot be fetched
- Suggest checking if vulcan-vault is running (`vulcan-vault --mcp`)

If no context found for task:
- Report "No vault context linked to this task"
- Suggest using `/remember` to record context as work progresses

## Ask the User

If multiple in-progress tasks found:
- "Multiple tasks in progress. Which one do you want context for?"
- List task titles with IDs for selection
