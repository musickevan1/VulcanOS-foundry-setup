---
name: checkpoint
description: Use this skill when the user asks to "save checkpoint", "create checkpoint", "restore checkpoint", "list checkpoints", "checkpoint diff", or wants to manage conversation context snapshots for branching and restoration.
version: 1.0.0
---

# Checkpoint Skill - Context State Management

You are helping the user manage conversation context checkpoints using vulcan-vault. Checkpoints capture the current session state for later restoration, enabling branching workflows and context preservation.

## Commands

### `/checkpoint save <name>`
Save current session state as a named checkpoint.

**Workflow:**
1. Ask user for a checkpoint name if not provided
2. Summarize current context (what we've been working on, key decisions)
3. Get active tasks from vulcan-todo
4. Call `mcp__vulcan-vault__save_checkpoint` with:
   - `name`: The checkpoint name
   - `session_id`: Current session identifier (use date-based if not available)
   - `context_summary`: Your summary of the current work state
   - `active_tasks`: List of active task IDs from vulcan-todo

**Example:**
```
User: /checkpoint save before-refactor
Assistant: I'll save a checkpoint capturing our current progress...
[Calls save_checkpoint with context summary]
Checkpoint saved: before-refactor (id: abc123...)
```

### `/checkpoint list`
Show all available checkpoints.

**Workflow:**
1. Call `mcp__vulcan-vault__list_checkpoints`
2. Format results as a table showing: name, session, created date, parent (if branched)
3. Highlight the most recent checkpoint

**Example Output:**
```
| Name             | Session                    | Created     | Parent |
|------------------|---------------------------|-------------|--------|
| before-refactor  | session-2026-01-15-ctx    | 2 hours ago | -      |
| sprint-1-done    | session-2026-01-15-ctx    | 3 hours ago | -      |
```

### `/checkpoint restore <id>`
Restore context from a checkpoint (informational - doesn't modify state).

**Workflow:**
1. Call `mcp__vulcan-vault__get_checkpoint` with the ID
2. Display the checkpoint's context summary
3. List the active tasks at that checkpoint
4. Suggest: "To continue from this point, focus on: [tasks]"

**Note:** This doesn't actually restore state - it shows what the context was so you can mentally resume.

### `/checkpoint diff <id>`
Compare current context to a checkpoint.

**Workflow:**
1. Get the checkpoint via `mcp__vulcan-vault__get_checkpoint`
2. Get current session context via `mcp__vulcan-vault__get_session_context`
3. Compare and show:
   - Tasks completed since checkpoint
   - New tasks added
   - PRPs created/modified
   - Key decisions made

### `/checkpoint branch <name> --from <parent_id>`
Create a checkpoint that branches from another.

**Workflow:**
1. Get the parent checkpoint
2. Create new checkpoint with `parent_checkpoint` set to parent ID
3. This enables exploring alternative approaches from a decision point

## Context Summary Guidelines

When saving checkpoints, include:
1. **Current task/feature** - What are we working on?
2. **Progress state** - What's done, what's in progress?
3. **Key decisions** - What choices were made and why?
4. **Blockers/notes** - Any important context for resuming?

**Good summary example:**
```
Working on Sprint 1: Context Engineering Foundation. Completed PRP note type
implementation with 5 new MCP tools. Created PRP for Sprint 2 (Ralph Loop).
Database migration added for field persistence. Ready to create /checkpoint
skill next.
```

## Integration with vulcan-todo

When saving checkpoints:
1. Query `mcp__vulcan-todo__get_context` to get active tasks
2. Include in-progress and high-priority pending task IDs
3. This allows checkpoint restoration to show what tasks were active

## When to Suggest Checkpoints

Proactively suggest saving a checkpoint when:
- Completing a significant milestone (sprint, feature)
- Before making risky/experimental changes
- Before major refactoring
- When switching to a different task
- At natural stopping points in a session

**Example prompt:**
> "We've completed the database migration. Would you like me to save a checkpoint before we move on to the next task?"