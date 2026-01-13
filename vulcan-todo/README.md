# Vulcan-Todo

A keyboard-driven task manager for VulcanOS with TUI, CLI, and MCP server modes.

## Features

- **TUI Mode**: Interactive terminal UI with keyboard navigation
- **MCP Server**: OpenCode agent integration for autonomous task management
- **CLI Mode**: Scriptable task operations from the command line
- **Project Organization**: Group tasks by project for better organization
- **Agent Tools**: Specialized tools for AI agent workflows
- **JSON Storage**: Plain text data storage, easy to version control

## Installation

### Build from Source

```bash
cd vulcan-todo
cargo build --release
cargo install --path .
```

### With TUI support

```bash
cargo build --release --features tui
```

## Usage

### TUI Mode (Interactive)

```bash
vulcan-todo
# or with MCP disabled for faster startup
vulcan-todo --tui
```

### MCP Server Mode (for OpenCode)

```bash
vulcan-todo --mcp
```

### CLI Mode

```bash
# List tasks
vulcan-todo list
vulcan-todo list --status pending
vulcan-todo list --priority high
vulcan-todo list --project vulcan-os

# Add a task
vulcan-todo add "Buy groceries" --priority high --tags shopping food --project personal

# Add a task with project from tag
vulcan-todo add "Implement feature" --tags "project:vulcan-os" --priority high

# List all projects with stats
vulcan-todo projects

# Show tasks in a specific project
vulcan-todo project vulcan-os

# Complete a task
vulcan-todo done <task-id>

# Search tasks
vulcan-todo search "groceries"

# Show statistics
vulcan-todo stats
```

## Keybindings (TUI Mode)

### Navigation
| Key | Action |
|-----|--------|
| `j` / `↓` | Next task |
| `k` / `↑` | Previous task |
| `g` | Jump to first task |
| `G` | Jump to last task |

### Task Actions
| Key | Action |
|-----|--------|
| `n` | New task (opens input dialog) |
| `e` | Edit task title |
| `x` / `Space` | Toggle complete |
| `d` | Delete task (with confirmation) |
| `D` (Shift) | Delete immediately |
| `p` | Cycle priority |
| `P` (Shift) | Open project selector |

### Filtering & Search
| Key | Action |
|-----|--------|
| `/` | Search tasks |
| `o` | Cycle sort order |
| `c` | Clear all filters |
| `r` | Refresh task list |

### Other
| Key | Action |
|-----|--------|
| `?` | Show help overlay |
| `q` | Quit |
| `Esc` | Clear filters, or quit if no filters |

### Input Mode (when editing/creating tasks)
| Key | Action |
|-----|--------|
| `Enter` | Submit input |
| `Esc` | Cancel |
| `Ctrl+U` | Clear input |
| `Ctrl+W` | Delete word |
| `←` / `→` | Move cursor |
| `Home` / `End` | Jump to start/end |

## Project Organization

Tasks can be organized by projects for better management:

### Creating Tasks with Projects

```bash
# Via CLI
vulcan-todo add "Implement feature X" --project vulcan-os --priority high

# Via MCP
{
  "name": "create_task",
  "arguments": {
    "title": "Implement feature X",
    "project": "vulcan-os",
    "priority": "high"
  }
}

# Auto-assign from tags (format: project:tagname)
vulcan-todo add "Task for project" --tags "project:rust" --priority medium
```

### Managing Projects

```bash
# List all projects with task counts
vulcan-todo projects

# Show tasks in a specific project
vulcan-todo project vulcan-os

# Filter tasks by project
vulcan-todo list --project vulcan-os --status pending

# Update task project
vulcan-todo edit <task-id> --project new-project

# Remove project from task
vulcan-todo edit <task-id> -P ""
```

### Auto-Migration

Migrate existing tasks to use projects:

```bash
# Via CLI
vulcan-todo migrate-projects

# Via MCP
{
  "name": "migrate_projects",
  "arguments": {}
}
```

This will extract project names from `project:tagname` tags and set the project field.

## MCP Tools

When running in MCP mode, the following tools are available:

### Task Management

| Tool | Description |
|------|-------------|
| `list_tasks` | List tasks with optional filtering by status, priority, project, or search |
| `get_task` | Get a single task by ID |
| `create_task` | Create a new task with title, description, priority, tags, project, and due date |
| `update_task` | Update an existing task with new values for any field |
| `complete_task` | Mark a task as done |
| `uncomplete_task` | Reopen a completed task |
| `delete_task` | Delete a task |
| `search_tasks` | Search tasks by title, description, or tags |

### Project Management

| Tool | Description |
|------|-------------|
| `list_projects` | List all projects with task counts (pending, done, total) |
| `get_project` | Get all tasks in a specific project |
| `migrate_projects` | Auto-assign projects from `project:tagname` tags |

### Agent Tools

Specialized tools for AI agent workflows:

| Tool | Description |
|------|-------------|
| `get_next_task` | Get the highest priority pending task. Ideal for agents picking up the next important item |
| `complete_and_get_next` | Complete the current task and get the next highest priority task automatically |
| `suggest_project` | Suggest appropriate project based on task title using keyword matching |

## MCP Usage Examples

### Basic Task Management

```json
// Create a task
{
  "name": "create_task",
  "arguments": {
    "title": "Implement user authentication",
    "description": "Add OAuth2 login support",
    "priority": "high",
    "tags": ["backend", "security"],
    "project": "vulcan-os"
  }
}

// List pending tasks in a project
{
  "name": "list_tasks",
  "arguments": {
    "status": "pending",
    "project": "vulcan-os",
    "limit": 10
  }
}

// Get project overview
{
  "name": "list_projects",
  "arguments": {}
}
```

### Agent Workflow

```json
// Get the most important task to work on
{
  "name": "get_next_task",
  "arguments": {
    "project": "vulcan-os"
  }
}

// Complete current task and get next
{
  "name": "complete_and_get_next",
  "arguments": {
    "completed_id": "task-uuid-here",
    "project": "vulcan-os"
  }
}

// Get project suggestions when creating tasks
{
  "name": "suggest_project",
  "arguments": {
    "title": "Implement REST API endpoints"
  }
}
```

### Migration

```json
// Migrate existing tasks to use project field
{
  "name": "migrate_projects",
  "arguments": {}
}
```

## OpenCode Integration

Add to `~/.config/opencode/opencode.json`:

```json
{
  "mcp": {
    "vulcan-todo": {
      "type": "local",
      "command": ["/path/to/vulcan-todo", "--mcp"]
    }
  }
}
```

Or use the absolute path for development:

```json
{
  "mcp": {
    "vulcan-todo": {
      "type": "local",
      "command": ["/home/evan/VulcanOS/vulcan-todo/target/release/vulcan-todo", "--mcp"]
    }
  }
}
```

Then OpenCode agents can manage tasks:

> "Create a follow-up task to review this PR for the vulcan-os project"
> "What are my highest priority tasks in the vulcan-os project?"
> "Complete the authentication task and get my next task"

## File Locations

- Tasks: `~/.config/vulcan-todo/tasks.json`
- Logs: `~/.config/vulcan-todo/logs/` (when logging enabled)

## Hyprland Integration

Super+T launches vulcan-todo:

```bash
bind = $mainMod, T, exec, vulcan-todo
```

## License

MIT - VulcanOS Project
