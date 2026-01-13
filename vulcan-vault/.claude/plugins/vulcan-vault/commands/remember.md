---
description: Record a memory (decision, lesson, or preference) to vulcan-vault
argument-hint: --type <type> <content>
allowed-tools: Read
---

# /remember - Memory Recording

Record memories from the current conversation to vulcan-vault for future recall.

## Usage

```
/remember --type decision "Chose SQLite over PostgreSQL for local storage"
/remember --type lesson "Always validate user input before database operations"
/remember --type preference "User prefers concise error messages"
```

## Memory Types

| Type | Purpose | Example |
|------|---------|---------|
| `decision` | Architecture/implementation choices | "Using REST over GraphQL for API" |
| `lesson` | Learnings from errors or successes | "Cache invalidation needed after writes" |
| `preference` | User preferences observed | "Prefers verbose logging in dev" |

## Workflow

### 1. Parse Arguments

Extract from command:
- `--type` (required): decision, lesson, or preference
- Content: Everything after the type flag

If type not specified, ask user which type applies.

### 2. Auto-Detect Context

From the current conversation, identify:
- **Project** - Which project this relates to (if apparent)
- **Tags** - Relevant keywords for later retrieval
- **Related task** - Current task if working on one

### 3. Record Memory

Call the appropriate MCP tool based on type:
- `decision` → `mcp__vulcan_vault__record_decision`
- `lesson` → `mcp__vulcan_vault__record_lesson`
- `preference` → `mcp__vulcan_vault__record_preference`

Pass:
- `title` - Brief summary (auto-generated from content)
- `content` - Full memory content
- `context` - Project/task context
- `tags` - Extracted keywords

### 4. Confirm to User

After recording:
```
Memory recorded to vault:
- Type: [type]
- Title: [auto-generated title]
- Tags: [tags]
- ID: [memory_id]

This memory will be recalled when working on similar tasks.
```

## Examples

### Recording a Decision
```
/remember --type decision "Using Rust for the CLI tool because of performance requirements and type safety"
```

Creates memory:
- Title: "CLI tool: Rust for performance and type safety"
- Context: Current project
- Tags: ["rust", "cli", "performance", "architecture"]

### Recording a Lesson
```
/remember --type lesson "The MCP server needs to handle empty arrays gracefully - found bug when no results returned"
```

Creates memory:
- Title: "MCP server: Handle empty array responses"
- Context: vulcan-vault project
- Tags: ["mcp", "error-handling", "edge-case"]

### Recording a Preference
```
/remember --type preference "User wants compact JSON output without pretty printing in production"
```

Creates memory:
- Title: "JSON output: Compact in production"
- Context: User preferences
- Tags: ["json", "formatting", "production"]

## Ask the User

If type is ambiguous:
- "Is this a decision (choice made), lesson (learning), or preference (user preference)?"

If content is too brief:
- "Could you provide more context? Brief memories are harder to recall later."
