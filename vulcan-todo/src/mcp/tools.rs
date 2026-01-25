//! MCP Tools for vulcan-todo
//!
//! Defines all available MCP tools for task management operations.

use crate::models::{Priority, Sprint, SprintStatus, Status, Task};
use crate::store::Store;
use serde_json::{json, Value};
use std::sync::Arc;

/// Context for MCP tool execution
pub struct ToolContext {
    pub store: Arc<dyn Store>,
    pub session_id: Option<String>,
}

impl ToolContext {
    /// Create a new tool context
    pub fn new(store: Arc<dyn Store>) -> Self {
        Self {
            store,
            session_id: None,
        }
    }
}

/// Result of a tool execution
pub struct ToolResult {
    pub success: bool,
    pub message: String,
    pub data: Option<Value>,
}

impl ToolResult {
    /// Create a successful result
    pub fn success(message: String, data: Option<Value>) -> Self {
        Self {
            success: true,
            message,
            data,
        }
    }

    /// Create an error result
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message,
            data: None,
        }
    }

    /// Convert to JSON value
    pub fn to_json(&self) -> Value {
        json!({
            "success": self.success,
            "message": self.message,
            "data": self.data
        })
    }
}

/// Tool function type
type ToolFn = fn(&ToolContext, Value) -> ToolResult;

/// Tool definitions
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    pub function: ToolFn,
}

impl Tool {
    /// Create a new tool definition
    pub fn new(name: String, description: String, input_schema: Value, function: ToolFn) -> Self {
        Self {
            name,
            description,
            input_schema,
            function,
        }
    }
}

/// List all available tools
pub fn get_tools() -> Vec<Tool> {
    vec![
        Tool::new(
            "list_tasks".to_string(),
            "List all tasks with optional filtering by status, priority, project, or search query. \
             Returns sorted list by priority. Use this for: getting overview of pending work, \
             finding tasks by status (pending/done), filtering by priority, \
             filtering by project, searching for specific tasks."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "status": {
                        "type": "string",
                        "enum": ["pending", "in_progress", "done", "all"],
                        "description": "Filter by task status"
                    },
                    "priority": {
                        "type": "string",
                        "enum": ["none", "low", "medium", "high", "urgent"],
                        "description": "Filter by task priority"
                    },
                    "project": {
                        "type": "string",
                        "description": "Filter by project name"
                    },
                    "search": {
                        "type": "string",
                        "description": "Search query to filter tasks"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of tasks to return",
                        "default": 50
                    }
                }
            }),
            list_tasks,
        ),
        Tool::new(
            "get_task".to_string(),
            "Get a single task by its ID".to_string(),
            json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "The unique ID of the task"
                    }
                },
                "required": ["id"]
            }),
            get_task,
        ),
        Tool::new(
            "create_task".to_string(),
            "Create a new task with the given title and optional details".to_string(),
            json!({
                "type": "object",
                "properties": {
                    "title": {
                        "type": "string",
                        "description": "The task title (required)"
                    },
                    "description": {
                        "type": "string",
                        "description": "A detailed description of the task"
                    },
                    "priority": {
                        "type": "string",
                        "enum": ["none", "low", "medium", "high", "urgent"],
                        "description": "Task priority level"
                    },
                    "tags": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Tags to associate with the task"
                    },
                    "project": {
                        "type": "string",
                        "description": "Project name for organization (e.g., 'vulcan-os', 'personal')"
                    },
                    "due_date": {
                        "type": "string",
                        "description": "Due date in ISO format (YYYY-MM-DD)"
                    },
                    "auto_fetch_context": {
                        "type": "boolean",
                        "description": "Auto-fetch context from vulcan-vault when task starts (default: false)"
                    },
                    "context_notes": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "UUIDs of linked vulcan-vault notes"
                    }
                },
                "required": ["title"]
            }),
            create_task,
        ),
        Tool::new(
            "update_task".to_string(),
            "Update an existing task with new values for any field".to_string(),
            json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "The task ID to update"
                    },
                    "title": {
                        "type": "string",
                        "description": "New task title"
                    },
                    "description": {
                        "type": "string",
                        "description": "New task description"
                    },
                    "priority": {
                        "type": "string",
                        "enum": ["none", "low", "medium", "high", "urgent"],
                        "description": "New priority level"
                    },
                    "tags": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "New set of tags"
                    },
                    "project": {
                        "type": "string",
                        "description": "New project name (set to null to remove)"
                    },
                    "auto_fetch_context": {
                        "type": "boolean",
                        "description": "Auto-fetch context from vulcan-vault when task starts"
                    },
                    "context_notes": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "UUIDs of linked vulcan-vault notes"
                    },
                    "ralph_mode": {
                        "type": "boolean",
                        "description": "Enable ralph loop mode for iterative self-correction"
                    },
                    "success_criteria": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Success criteria that must be met (e.g., 'tests pass', 'lint clean')"
                    },
                    "quality_gates": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Quality gates to run: test, lint, typecheck, build, custom:<cmd>"
                    }
                },
                "required": ["id"]
            }),
            update_task,
        ),
        Tool::new(
            "complete_task".to_string(),
            "Mark a task as complete/done".to_string(),
            json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "The task ID to complete"
                    }
                },
                "required": ["id"]
            }),
            complete_task,
        ),
        Tool::new(
            "uncomplete_task".to_string(),
            "Reopen a completed task (mark as pending)".to_string(),
            json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "The task ID to reopen"
                    }
                },
                "required": ["id"]
            }),
            uncomplete_task,
        ),
        Tool::new(
            "delete_task".to_string(),
            "Delete a task permanently".to_string(),
            json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "The task ID to delete"
                    }
                },
                "required": ["id"]
            }),
            delete_task,
        ),
        Tool::new(
            "search_tasks".to_string(),
            "Search tasks by title, description, or tags".to_string(),
            json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum results",
                        "default": 20
                    }
                },
                "required": ["query"]
            }),
            search_tasks,
        ),
        Tool::new(
            "get_stats".to_string(),
            "Get task statistics (pending, completed counts)".to_string(),
            json!({
                "type": "object",
                "properties": {}
            }),
            get_stats,
        ),
        Tool::new(
            "list_projects".to_string(),
            "List all projects with task counts. Use this to get an overview of task organization by project."
                .to_string(),
            json!({
                "type": "object",
                "properties": {}
            }),
            list_projects,
        ),
        Tool::new(
            "get_project".to_string(),
            "Get all tasks in a specific project. Use this when working on a specific project and need to see related tasks."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Project name"
                    }
                },
                "required": ["name"]
            }),
            get_project,
        ),
        Tool::new(
            "migrate_projects".to_string(),
            "Auto-assign projects from 'project:tagname' tags for all tasks. Useful for migrating existing tasks to use the project field."
                .to_string(),
            json!({
                "type": "object",
                "properties": {}
            }),
            migrate_projects,
        ),
        Tool::new(
            "get_next_task".to_string(),
            "Get the highest priority pending task. Ideal for agents that need to pick up the most important next item. Returns task with highest priority level among pending tasks."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "project": {
                        "type": "string",
                        "description": "Optional: get next task from specific project"
                    }
                }
            }),
            get_next_task,
        ),
        Tool::new(
            "complete_and_get_next".to_string(),
            "Complete the current task and get the next highest priority task. Useful for agent workflows where completing one task should automatically retrieve the next item."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "completed_id": {
                        "type": "string",
                        "description": "ID of task to complete"
                    },
                    "project": {
                        "type": "string",
                        "description": "Optional: get next task from specific project"
                    }
                },
                "required": ["completed_id"]
            }),
            complete_and_get_next,
        ),
        Tool::new(
            "suggest_project".to_string(),
            "Suggest appropriate project based on task title. Uses keyword matching to recommend project names. Helpful for agents when creating tasks."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "title": {
                        "type": "string",
                        "description": "Task title to analyze"
                    }
                },
                "required": ["title"]
            }),
            suggest_project,
        ),
        Tool::new(
            "start_task".to_string(),
            "Mark a task as in-progress. Use this when you begin working on a task. \
             This helps track which tasks are actively being worked on. \
             Optionally enable ralph loop mode for iterative self-correction."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "The task ID to mark as in-progress"
                    },
                    "ralph_mode": {
                        "type": "boolean",
                        "description": "Enable ralph loop mode for iterative self-correction"
                    },
                    "success_criteria": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Success criteria that must be met (e.g., 'tests pass')"
                    },
                    "quality_gates": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Quality gates to run: test, lint, typecheck, build, custom:<cmd>"
                    }
                },
                "required": ["id"]
            }),
            start_task,
        ),
        Tool::new(
            "get_context".to_string(),
            "Get relevant tasks for the current work context. Returns in-progress tasks, \
             high-priority pending tasks, and recently modified tasks. Useful for agents \
             to understand current work state and priorities."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "project": {
                        "type": "string",
                        "description": "Optional: limit context to a specific project"
                    },
                    "include_done": {
                        "type": "boolean",
                        "description": "Include recently completed tasks (default: false)",
                        "default": false
                    }
                }
            }),
            get_context,
        ),
        Tool::new(
            "get_ralph_status".to_string(),
            "Get Ralph Loop status for the current in-progress task. Returns the task with \
             ralph_mode enabled, along with its success_criteria and quality_gates. \
             Useful for Stop hooks to check if quality gates should be enforced."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "project": {
                        "type": "string",
                        "description": "Optional: filter by project"
                    }
                }
            }),
            get_ralph_status,
        ),
        Tool::new(
            "bulk_operation".to_string(),
            "Perform bulk operations on multiple tasks at once. Supports completing, \
             deleting, or updating multiple tasks in a single call."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "operation": {
                        "type": "string",
                        "enum": ["complete", "delete", "start", "set_priority", "set_project"],
                        "description": "The operation to perform"
                    },
                    "task_ids": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "List of task IDs to operate on"
                    },
                    "value": {
                        "type": "string",
                        "description": "Value for set_priority (none/low/medium/high/urgent) or set_project (project name)"
                    }
                },
                "required": ["operation", "task_ids"]
            }),
            bulk_operation,
        ),
        // ==================== Sprint Tools ====================
        Tool::new(
            "create_sprint".to_string(),
            "Create a new sprint in a project. Sprints help organize and order tasks within projects."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Sprint name (e.g., 'Sprint 1', 'MVP', 'Q1 Goals')"
                    },
                    "project": {
                        "type": "string",
                        "description": "Project this sprint belongs to (required)"
                    },
                    "goal": {
                        "type": "string",
                        "description": "Sprint goal or description"
                    },
                    "start_date": {
                        "type": "string",
                        "description": "Optional start date in ISO format (YYYY-MM-DD)"
                    },
                    "end_date": {
                        "type": "string",
                        "description": "Optional end date in ISO format (YYYY-MM-DD)"
                    }
                },
                "required": ["name", "project"]
            }),
            create_sprint,
        ),
        Tool::new(
            "list_sprints".to_string(),
            "List sprints, optionally filtered by project or status."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "project": {
                        "type": "string",
                        "description": "Filter by project name"
                    },
                    "status": {
                        "type": "string",
                        "enum": ["planning", "active", "completed", "archived", "all"],
                        "description": "Filter by sprint status"
                    }
                }
            }),
            list_sprints,
        ),
        Tool::new(
            "get_sprint".to_string(),
            "Get sprint details by ID, including tasks in the sprint."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "The sprint ID"
                    }
                },
                "required": ["id"]
            }),
            get_sprint,
        ),
        Tool::new(
            "update_sprint".to_string(),
            "Update sprint details (name, status, dates, goal)."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "The sprint ID to update"
                    },
                    "name": {
                        "type": "string",
                        "description": "New sprint name"
                    },
                    "status": {
                        "type": "string",
                        "enum": ["planning", "active", "completed", "archived"],
                        "description": "New sprint status"
                    },
                    "goal": {
                        "type": "string",
                        "description": "New sprint goal"
                    },
                    "start_date": {
                        "type": "string",
                        "description": "New start date (YYYY-MM-DD)"
                    },
                    "end_date": {
                        "type": "string",
                        "description": "New end date (YYYY-MM-DD)"
                    }
                },
                "required": ["id"]
            }),
            update_sprint,
        ),
        Tool::new(
            "delete_sprint".to_string(),
            "Delete a sprint. Tasks in the sprint will be unassigned (kept but removed from sprint)."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "The sprint ID to delete"
                    }
                },
                "required": ["id"]
            }),
            delete_sprint,
        ),
        Tool::new(
            "start_sprint".to_string(),
            "Mark a sprint as active. Sets status to 'active' and records start date if not set."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "The sprint ID to start"
                    }
                },
                "required": ["id"]
            }),
            start_sprint,
        ),
        Tool::new(
            "complete_sprint".to_string(),
            "Mark a sprint as completed. Sets status to 'completed' and records end date if not set."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "The sprint ID to complete"
                    }
                },
                "required": ["id"]
            }),
            complete_sprint,
        ),
        Tool::new(
            "assign_task_to_sprint".to_string(),
            "Add a task to a sprint. The task will be added at the end of the sprint's task order."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "task_id": {
                        "type": "string",
                        "description": "The task ID to assign"
                    },
                    "sprint_id": {
                        "type": "string",
                        "description": "The sprint ID to assign the task to"
                    }
                },
                "required": ["task_id", "sprint_id"]
            }),
            assign_task_to_sprint,
        ),
        Tool::new(
            "remove_task_from_sprint".to_string(),
            "Remove a task from its current sprint. The task remains but is no longer part of any sprint."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "task_id": {
                        "type": "string",
                        "description": "The task ID to remove from its sprint"
                    }
                },
                "required": ["task_id"]
            }),
            remove_task_from_sprint,
        ),
        Tool::new(
            "reorder_sprint_task".to_string(),
            "Change the position of a task within its sprint. Position is 1-indexed."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "task_id": {
                        "type": "string",
                        "description": "The task ID to reorder"
                    },
                    "position": {
                        "type": "integer",
                        "description": "New position in the sprint (1-indexed)"
                    }
                },
                "required": ["task_id", "position"]
            }),
            reorder_sprint_task,
        ),
        Tool::new(
            "get_sprint_tasks".to_string(),
            "Get tasks in a sprint, ordered by their sprint position."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "sprint_id": {
                        "type": "string",
                        "description": "The sprint ID"
                    },
                    "include_done": {
                        "type": "boolean",
                        "description": "Include completed tasks (default: true)",
                        "default": true
                    }
                },
                "required": ["sprint_id"]
            }),
            get_sprint_tasks,
        ),
        Tool::new(
            "get_backlog".to_string(),
            "Get tasks in a project that are not assigned to any sprint (the backlog)."
                .to_string(),
            json!({
                "type": "object",
                "properties": {
                    "project": {
                        "type": "string",
                        "description": "The project name"
                    }
                },
                "required": ["project"]
            }),
            get_backlog,
        ),
    ]
}

// Tool implementations

fn list_tasks(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let status_filter = args.get("status").and_then(|v| v.as_str());
    let priority_filter = args.get("priority").and_then(|v| v.as_str());
    let project_filter = args.get("project").and_then(|v| v.as_str());
    let search_query = args.get("search").and_then(|v| v.as_str());
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(50) as usize;

    let tasks_result = ctx.store.get_all();

    match tasks_result {
        Ok(all_tasks) => {
            let mut tasks: Vec<Task> = all_tasks;

            // Apply filters
            if let Some(status) = status_filter {
                if status != "all" {
                    let status: Status = status.to_string().into();
                    tasks.retain(|t| t.status == status);
                }
            }

            if let Some(priority) = priority_filter {
                let priority: Priority = priority.to_string().into();
                tasks.retain(|t| t.priority == priority);
            }

            if let Some(project) = project_filter {
                tasks.retain(|t| t.belongs_to_project(project));
            }

            if let Some(query) = search_query {
                tasks = tasks
                    .into_iter()
                    .filter(|t| {
                        t.title.to_lowercase().contains(&query.to_lowercase())
                            || t.description
                                .as_ref()
                                .map(|d| d.to_lowercase().contains(&query.to_lowercase()))
                                .unwrap_or(false)
                            || t.tags
                                .iter()
                                .any(|tag| tag.to_lowercase().contains(&query.to_lowercase()))
                    })
                    .collect();
            }

            // Sort by priority (high first) then by creation date
            tasks.sort_by(|a, b| {
                b.priority
                    .level()
                    .cmp(&a.priority.level())
                    .then_with(|| b.created_at.cmp(&a.created_at))
            });

            // Apply limit
            tasks.truncate(limit);

            // Convert to serializable format
            let task_summaries: Vec<Value> = tasks
                .iter()
                .map(|t| {
                    json!({
                        "id": t.id,
                        "title": t.title,
                        "status": t.status.to_string(),
                        "priority": t.priority.to_string(),
                        "tags": t.tags,
                        "project": t.project,
                        "created_at": t.created_formatted(),
                        "description": t.description
                    })
                })
                .collect();

            ToolResult::success(
                format!("Found {} tasks", task_summaries.len()),
                Some(json!({
                    "tasks": task_summaries,
                    "total": task_summaries.len()
                })),
            )
        }
        Err(e) => ToolResult::error(format!("Failed to list tasks: {}", e)),
    }
}

fn get_task(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);
    let id = match args.get("id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: id".to_string()),
    };

    match ctx.store.get(id) {
        Ok(Some(task)) => ToolResult::success(
            "Task found".to_string(),
            Some(json!({
                "task": {
                    "id": task.id,
                    "title": task.title,
                    "description": task.description,
                    "status": task.status.to_string(),
                    "priority": task.priority.to_string(),
                    "tags": task.tags,
                    "project": task.project,
                    "created_at": task.created_formatted(),
                    "completed_at": task.completed_at.map(|d| d.to_string()),
                    "due_date": task.due_formatted()
                }
            })),
        ),
        Ok(None) => ToolResult::error(format!("Task not found: {}", id)),
        Err(e) => ToolResult::error(format!("Failed to get task: {}", e)),
    }
}

fn create_task(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let title = match args.get("title").and_then(|v| v.as_str()) {
        Some(title) if !title.is_empty() => title.to_string(),
        _ => return ToolResult::error("Missing required parameter: title".to_string()),
    };

    let description = args
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let priority: Priority = args
        .get("priority")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string().into())
        .unwrap_or(Priority::None);

    let tags: Vec<String> = args
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let project = args
        .get("project")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let due_date = args
        .get("due_date")
        .and_then(|v| v.as_str())
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|d| d.with_timezone(&chrono::Utc));

    // vulcan-vault integration fields
    let auto_fetch_context = args
        .get("auto_fetch_context")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let context_notes: Vec<String> = args
        .get("context_notes")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let mut task = Task::new(title);
    task.description = description;
    task.priority = priority;
    task.tags = tags;
    task.project = project;
    task.due_date = due_date;
    task.auto_fetch_context = auto_fetch_context;
    task.context_notes = context_notes;

    match ctx.store.add(&task) {
        Ok(created) => ToolResult::success(
            format!("Task created: {}", created.id),
            Some(json!({
                "id": created.id,
                "title": created.title,
                "status": created.status.to_string(),
                "priority": created.priority.to_string(),
                "project": created.project,
                "tags": created.tags
            })),
        ),
        Err(e) => ToolResult::error(format!("Failed to create task: {}", e)),
    }
}

fn update_task(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let id = match args.get("id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: id".to_string()),
    };

    // Get existing task
    let existing = match ctx.store.get(id) {
        Ok(Some(task)) => task,
        Ok(None) => return ToolResult::error(format!("Task not found: {}", id)),
        Err(e) => return ToolResult::error(format!("Failed to get task: {}", e)),
    };

    // Build updated task
    let mut task = existing;
    if let Some(title) = args.get("title").and_then(|v| v.as_str()) {
        task.title = title.to_string();
    }
    if let Some(desc) = args.get("description").and_then(|v| v.as_str()) {
        task.description = Some(desc.to_string());
    }
    if let Some(priority) = args.get("priority").and_then(|v| v.as_str()) {
        task.priority = priority.to_string().into();
    }
    if let Some(tags) = args.get("tags").and_then(|v| v.as_array()) {
        task.tags = tags
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
    }
    if let Some(project) = args.get("project") {
        if project.is_null() {
            task.project = None;
        } else if let Some(p) = project.as_str() {
            task.project = Some(p.to_string());
        }
    }
    // vulcan-vault integration fields
    if let Some(auto_fetch) = args.get("auto_fetch_context").and_then(|v| v.as_bool()) {
        task.auto_fetch_context = auto_fetch;
    }
    if let Some(notes) = args.get("context_notes").and_then(|v| v.as_array()) {
        task.context_notes = notes
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
    }
    // Ralph loop fields
    if let Some(ralph) = args.get("ralph_mode").and_then(|v| v.as_bool()) {
        task.ralph_mode = ralph;
    }
    if let Some(criteria) = args.get("success_criteria").and_then(|v| v.as_array()) {
        task.success_criteria = criteria
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
    }
    if let Some(gates) = args.get("quality_gates").and_then(|v| v.as_array()) {
        task.quality_gates = gates
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
    }

    match ctx.store.update(&task) {
        Ok(Some(updated)) => ToolResult::success(
            format!("Task updated: {}", updated.id),
            Some(json!({
                "id": updated.id,
                "title": updated.title,
                "status": updated.status.to_string(),
                "priority": updated.priority.to_string(),
                "project": updated.project,
                "tags": updated.tags,
                "ralph_mode": updated.ralph_mode,
                "success_criteria": updated.success_criteria,
                "quality_gates": updated.quality_gates
            })),
        ),
        Ok(None) => ToolResult::error(format!("Task not found: {}", id)),
        Err(e) => ToolResult::error(format!("Failed to update task: {}", e)),
    }
}

fn complete_task(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let id = match args.get("id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: id".to_string()),
    };

    let mut task = match ctx.store.get(id) {
        Ok(Some(task)) => task,
        Ok(None) => return ToolResult::error(format!("Task not found: {}", id)),
        Err(e) => return ToolResult::error(format!("Failed to get task: {}", e)),
    };

    task.complete();

    match ctx.store.update(&task) {
        Ok(_) => ToolResult::success(
            format!("Task completed: {}", task.title),
            Some(json!({
                "id": task.id,
                "title": task.title,
                "status": "done",
                "project": task.project
            })),
        ),
        Err(e) => ToolResult::error(format!("Failed to complete task: {}", e)),
    }
}

fn uncomplete_task(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let id = match args.get("id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: id".to_string()),
    };

    let mut task = match ctx.store.get(id) {
        Ok(Some(task)) => task,
        Ok(None) => return ToolResult::error(format!("Task not found: {}", id)),
        Err(e) => return ToolResult::error(format!("Failed to get task: {}", e)),
    };

    task.uncomplete();

    match ctx.store.update(&task) {
        Ok(_) => ToolResult::success(
            format!("Task reopened: {}", task.title),
            Some(json!({
                "id": task.id,
                "title": task.title,
                "status": "pending",
                "project": task.project
            })),
        ),
        Err(e) => ToolResult::error(format!("Failed to reopen task: {}", e)),
    }
}

fn delete_task(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let id = match args.get("id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: id".to_string()),
    };

    match ctx.store.delete(id) {
        Ok(true) => ToolResult::success(
            format!("Task deleted: {}", id),
            Some(json!({
                "id": id,
                "deleted": true
            })),
        ),
        Ok(false) => ToolResult::error(format!("Task not found: {}", id)),
        Err(e) => ToolResult::error(format!("Failed to delete task: {}", e)),
    }
}

fn search_tasks(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let query = match args.get("query").and_then(|v| v.as_str()) {
        Some(query) if !query.is_empty() => query,
        _ => return ToolResult::error("Missing required parameter: query".to_string()),
    };

    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(20) as usize;

    match ctx.store.search(query) {
        Ok(mut tasks) => {
            tasks.truncate(limit);

            let results: Vec<Value> = tasks
                .iter()
                .map(|t| {
                    json!({
                        "id": t.id,
                        "title": t.title,
                        "status": t.status.to_string(),
                        "priority": t.priority.to_string(),
                        "tags": t.tags,
                        "project": t.project
                    })
                })
                .collect();

            ToolResult::success(
                format!("Found {} tasks matching '{}'", results.len(), query),
                Some(json!({
                    "tasks": results,
                    "query": query,
                    "total": results.len()
                })),
            )
        }
        Err(e) => ToolResult::error(format!("Search failed: {}", e)),
    }
}

fn get_stats(ctx: &ToolContext, _args: Value) -> ToolResult {
    match ctx.store.count() {
        Ok((pending, done)) => {
            let total = pending + done;
            let percent = if total > 0 {
                (done as f64 / total as f64 * 100.0).round()
            } else {
                0.0
            };

            ToolResult::success(
                format!(
                    "Stats: {} pending, {} done ({}% complete)",
                    pending, done, percent
                ),
                Some(json!({
                    "pending": pending,
                    "done": done,
                    "total": total,
                    "percent_complete": percent
                })),
            )
        }
        Err(e) => ToolResult::error(format!("Failed to get stats: {}", e)),
    }
}

fn list_projects(ctx: &ToolContext, _args: Value) -> ToolResult {
    match ctx.store.get_projects() {
        Ok(projects) => {
            let stats = ctx.store.get_project_stats().unwrap_or_default();

            let project_summaries: Vec<Value> = projects
                .iter()
                .map(|p| {
                    let (pending, done) = stats.get(p).copied().unwrap_or((0, 0));
                    json!({
                        "name": p,
                        "pending": pending,
                        "done": done,
                        "total": pending + done
                    })
                })
                .collect();

            ToolResult::success(
                format!("Found {} projects", project_summaries.len()),
                Some(json!({ "projects": project_summaries })),
            )
        }
        Err(e) => ToolResult::error(format!("Failed to list projects: {}", e)),
    }
}

fn get_project(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);
    let name = match args.get("name").and_then(|v| v.as_str()) {
        Some(name) if !name.is_empty() => name,
        _ => return ToolResult::error("Missing required parameter: name".to_string()),
    };

    match ctx.store.get_by_project(name) {
        Ok(tasks) => {
            let task_summaries: Vec<Value> = tasks
                .iter()
                .map(|t| {
                    json!({
                        "id": t.id,
                        "title": t.title,
                        "status": t.status.to_string(),
                        "priority": t.priority.to_string(),
                        "tags": t.tags,
                        "project": t.project,
                        "created_at": t.created_formatted()
                    })
                })
                .collect();

            ToolResult::success(
                format!("Found {} tasks in project '{}'", task_summaries.len(), name),
                Some(json!({
                    "project": name,
                    "tasks": task_summaries,
                    "total": task_summaries.len()
                })),
            )
        }
        Err(e) => ToolResult::error(format!("Failed to get project: {}", e)),
    }
}

fn migrate_projects(ctx: &ToolContext, _args: Value) -> ToolResult {
    match ctx.store.auto_assign_projects_from_tags() {
        Ok(updated) => {
            if updated.is_empty() {
                ToolResult::success(
                    "No tasks needed migration (no 'project:tag' tags found)".to_string(),
                    Some(json!({
                        "migrated": 0,
                        "message": "All tasks already have projects or no project: tags found"
                    })),
                )
            } else {
                ToolResult::success(
                    format!("Migrated {} tasks to use project field", updated.len()),
                    Some(json!({
                        "migrated": updated.len(),
                        "task_ids": updated
                    })),
                )
            }
        }
        Err(e) => ToolResult::error(format!("Failed to migrate projects: {}", e)),
    }
}

fn get_next_task(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);
    let project = args.get("project").and_then(|v| v.as_str());

    match ctx.store.get_all() {
        Ok(all_tasks) => {
            let filtered: Vec<Task> = match project {
                Some(p) => all_tasks
                    .into_iter()
                    .filter(|t| t.is_pending() && t.belongs_to_project(p))
                    .collect(),
                None => all_tasks.into_iter().filter(|t| t.is_pending()).collect(),
            };

            // Get highest priority pending task
            let next = filtered.into_iter().max_by_key(|t| t.priority.level());

            match next {
                Some(task) => ToolResult::success(
                    "Found next task".to_string(),
                    Some(json!({
                        "task": {
                            "id": task.id,
                            "title": task.title,
                            "description": task.description,
                            "status": task.status.to_string(),
                            "priority": task.priority.to_string(),
                            "tags": task.tags,
                            "project": task.project
                        }
                    })),
                ),
                None => ToolResult::error(if project.is_some() {
                    format!("No pending tasks found in project '{}'", project.unwrap())
                } else {
                    "No pending tasks found".to_string()
                }),
            }
        }
        Err(e) => ToolResult::error(format!("Failed to get tasks: {}", e)),
    }
}

fn complete_and_get_next(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);
    let completed_id = match args.get("completed_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: completed_id".to_string()),
    };
    let project = args.get("project").and_then(|v| v.as_str());

    // Complete the task
    let mut task = match ctx.store.get(completed_id) {
        Ok(Some(t)) => t,
        Ok(None) => return ToolResult::error(format!("Task not found: {}", completed_id)),
        Err(e) => return ToolResult::error(format!("Failed to get task: {}", e)),
    };

    task.complete();

    match ctx.store.update(&task) {
        Ok(Some(_)) => {
            // Task completed, now get next
            match ctx.store.get_all() {
                Ok(all_tasks) => {
                    let filtered: Vec<Task> = match project {
                        Some(p) => all_tasks
                            .into_iter()
                            .filter(|t| t.is_pending() && t.belongs_to_project(p))
                            .collect(),
                        None => all_tasks.into_iter().filter(|t| t.is_pending()).collect(),
                    };

                    let next = filtered.into_iter().max_by_key(|t| t.priority.level());

                    ToolResult::success(
                        format!("Task completed: {}. Next task retrieved.", task.title),
                        Some(json!({
                            "completed_id": completed_id,
                            "completed_title": task.title,
                            "next_task": next.as_ref().map(|t| json!({
                                "id": t.id,
                                "title": t.title,
                                "priority": t.priority.to_string(),
                                "project": t.project
                            }))
                        })),
                    )
                }
                Err(e) => ToolResult::error(format!("Failed to get next task: {}", e)),
            }
        }
        Ok(None) => ToolResult::error(format!("Task not found: {}", completed_id)),
        Err(e) => ToolResult::error(format!("Failed to complete task: {}", e)),
    }
}

fn suggest_project(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);
    let title = match args.get("title").and_then(|v| v.as_str()) {
        Some(title) if !title.is_empty() => title,
        _ => return ToolResult::error("Missing required parameter: title".to_string()),
    };

    let title_lower = title.to_lowercase();
    let suggestions: Vec<String> = match title_lower.as_str() {
        _ if title_lower.contains("vulcan") => vec!["vulcan-os".to_string()],
        _ if title_lower.contains("rust") || title_lower.contains("cargo") => {
            vec!["rust".to_string()]
        }
        _ if title_lower.contains("docker") || title_lower.contains("container") => {
            vec!["devops".to_string()]
        }
        _ if title_lower.contains("test") || title_lower.contains("testing") => {
            vec!["testing".to_string()]
        }
        _ if title_lower.contains("docs") || title_lower.contains("documentation") => {
            vec!["documentation".to_string()]
        }
        _ if title_lower.contains("feature") || title_lower.contains("implement") => {
            vec!["features".to_string()]
        }
        _ if title_lower.contains("bug")
            || title_lower.contains("fix")
            || title_lower.contains("issue") =>
        {
            vec!["bugfix".to_string()]
        }
        _ => vec!["general".to_string()],
    };

    ToolResult::success(
        "Project suggestions based on title".to_string(),
        Some(json!({
            "title": title,
            "suggestions": suggestions
        })),
    )
}

fn start_task(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);
    let id = match args.get("id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: id".to_string()),
    };

    match ctx.store.get(id) {
        Ok(Some(mut task)) => {
            if task.is_done() {
                return ToolResult::error(
                    "Cannot start a completed task. Reopen it first.".to_string(),
                );
            }
            if task.is_in_progress() {
                // Include context hints even for already in-progress tasks
                return ToolResult::success(
                    "Task is already in progress".to_string(),
                    Some(json!({
                        "id": id,
                        "title": task.title,
                        "status": "in_progress",
                        "auto_fetch_context": task.auto_fetch_context,
                        "context_notes": task.context_notes,
                        "project": task.project,
                        "ralph_mode": task.ralph_mode,
                        "success_criteria": task.success_criteria,
                        "quality_gates": task.quality_gates
                    })),
                );
            }

            // Set ralph fields if provided
            if let Some(ralph) = args.get("ralph_mode").and_then(|v| v.as_bool()) {
                task.ralph_mode = ralph;
            }
            if let Some(criteria) = args.get("success_criteria").and_then(|v| v.as_array()) {
                task.success_criteria = criteria
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
            }
            if let Some(gates) = args.get("quality_gates").and_then(|v| v.as_array()) {
                task.quality_gates = gates
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
            }

            task.start();

            match ctx.store.update(&task) {
                Ok(Some(updated)) => ToolResult::success(
                    format!("Task started: {}", updated.title),
                    Some(json!({
                        "id": id,
                        "title": updated.title,
                        "status": "in_progress",
                        // Context hints for agent to call vulcan-vault
                        "auto_fetch_context": updated.auto_fetch_context,
                        "context_notes": updated.context_notes,
                        "project": updated.project,
                        // Ralph loop fields
                        "ralph_mode": updated.ralph_mode,
                        "success_criteria": updated.success_criteria,
                        "quality_gates": updated.quality_gates
                    })),
                ),
                Ok(None) => ToolResult::error(format!("Task not found: {}", id)),
                Err(e) => ToolResult::error(format!("Failed to start task: {}", e)),
            }
        }
        Ok(None) => ToolResult::error(format!("Task not found: {}", id)),
        Err(e) => ToolResult::error(format!("Failed to get task: {}", e)),
    }
}

fn get_context(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);
    let project_filter = args.get("project").and_then(|v| v.as_str());
    let include_done = args
        .get("include_done")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    match ctx.store.get_all() {
        Ok(all_tasks) => {
            let mut tasks: Vec<Task> = all_tasks;

            // Filter by project if specified
            if let Some(project) = project_filter {
                tasks.retain(|t| t.belongs_to_project(project));
            }

            // Get in-progress tasks (highest priority - these are being worked on)
            let in_progress: Vec<&Task> = tasks.iter().filter(|t| t.is_in_progress()).collect();

            // Get high-priority pending tasks (urgent and high)
            let mut high_priority: Vec<&Task> = tasks
                .iter()
                .filter(|t| {
                    t.is_pending()
                        && (t.priority == Priority::Urgent || t.priority == Priority::High)
                })
                .collect();
            high_priority.sort_by(|a, b| b.priority.level().cmp(&a.priority.level()));
            high_priority.truncate(5);

            // Get overdue tasks
            let overdue: Vec<&Task> = tasks.iter().filter(|t| t.is_overdue()).collect();

            // Get recently completed (last 24 hours) if requested
            let recent_done: Vec<&Task> = if include_done {
                let cutoff = chrono::Utc::now() - chrono::Duration::hours(24);
                tasks
                    .iter()
                    .filter(|t| t.is_done() && t.completed_at.map(|c| c > cutoff).unwrap_or(false))
                    .collect()
            } else {
                Vec::new()
            };

            // Build summary
            let total_pending = tasks.iter().filter(|t| t.is_pending()).count();
            let total_in_progress = in_progress.len();
            let total_done = tasks.iter().filter(|t| t.is_done()).count();

            let format_task = |t: &Task| -> Value {
                json!({
                    "id": t.id,
                    "title": t.title,
                    "status": t.status.to_string(),
                    "priority": t.priority.to_string(),
                    "project": t.project,
                    "due_date": t.due_formatted()
                })
            };

            ToolResult::success(
                format!(
                    "Context: {} in-progress, {} high-priority, {} overdue",
                    total_in_progress,
                    high_priority.len(),
                    overdue.len()
                ),
                Some(json!({
                    "summary": {
                        "total_pending": total_pending,
                        "total_in_progress": total_in_progress,
                        "total_done": total_done,
                        "project_filter": project_filter
                    },
                    "in_progress": in_progress.iter().map(|t| format_task(t)).collect::<Vec<_>>(),
                    "high_priority": high_priority.iter().map(|t| format_task(t)).collect::<Vec<_>>(),
                    "overdue": overdue.iter().map(|t| format_task(t)).collect::<Vec<_>>(),
                    "recent_completed": recent_done.iter().map(|t| format_task(t)).collect::<Vec<_>>()
                })),
            )
        }
        Err(e) => ToolResult::error(format!("Failed to get context: {}", e)),
    }
}

fn get_ralph_status(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);
    let project_filter = args.get("project").and_then(|v| v.as_str());

    match ctx.store.get_all() {
        Ok(all_tasks) => {
            // Find in-progress tasks with ralph_mode enabled
            let ralph_tasks: Vec<&Task> = all_tasks
                .iter()
                .filter(|t| {
                    t.is_in_progress()
                        && t.ralph_mode
                        && project_filter
                            .map(|p| t.belongs_to_project(p))
                            .unwrap_or(true)
                })
                .collect();

            if ralph_tasks.is_empty() {
                return ToolResult::success(
                    "No Ralph Loop tasks currently active".to_string(),
                    Some(json!({
                        "active": false,
                        "task": null
                    })),
                );
            }

            // Return the first (should typically be only one) ralph-enabled task
            let task = ralph_tasks[0];
            ToolResult::success(
                format!("Ralph Loop active: {}", task.title),
                Some(json!({
                    "active": true,
                    "task": {
                        "id": task.id,
                        "title": task.title,
                        "project": task.project,
                        "ralph_mode": task.ralph_mode,
                        "success_criteria": task.success_criteria,
                        "quality_gates": task.quality_gates
                    }
                })),
            )
        }
        Err(e) => ToolResult::error(format!("Failed to get ralph status: {}", e)),
    }
}

fn bulk_operation(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let operation = match args.get("operation").and_then(|v| v.as_str()) {
        Some(op) => op,
        None => return ToolResult::error("Missing required parameter: operation".to_string()),
    };

    let task_ids: Vec<String> = match args.get("task_ids").and_then(|v| v.as_array()) {
        Some(arr) => arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        None => return ToolResult::error("Missing required parameter: task_ids".to_string()),
    };

    if task_ids.is_empty() {
        return ToolResult::error("task_ids array cannot be empty".to_string());
    }

    let value = args.get("value").and_then(|v| v.as_str());

    let mut success_count = 0;
    let mut failed_ids: Vec<String> = Vec::new();
    let mut results: Vec<Value> = Vec::new();

    for id in &task_ids {
        match ctx.store.get(id) {
            Ok(Some(mut task)) => {
                let result = match operation {
                    "complete" => {
                        task.complete();
                        ctx.store.update(&task)
                    }
                    "delete" => {
                        ctx.store
                            .delete(id)
                            .map(|deleted| if deleted { Some(task.clone()) } else { None })
                    }
                    "start" => {
                        task.start();
                        ctx.store.update(&task)
                    }
                    "set_priority" => {
                        if let Some(pri) = value {
                            task.priority = pri.to_string().into();
                            ctx.store.update(&task)
                        } else {
                            failed_ids.push(id.clone());
                            continue;
                        }
                    }
                    "set_project" => {
                        task.project = value.map(|v| v.to_string());
                        ctx.store.update(&task)
                    }
                    _ => {
                        failed_ids.push(id.clone());
                        continue;
                    }
                };

                match result {
                    Ok(Some(_)) => {
                        success_count += 1;
                        results.push(json!({"id": id, "title": task.title, "success": true}));
                    }
                    Ok(None) => {
                        failed_ids.push(id.clone());
                        results.push(json!({"id": id, "success": false, "error": "Not found"}));
                    }
                    Err(e) => {
                        failed_ids.push(id.clone());
                        results.push(json!({"id": id, "success": false, "error": e.to_string()}));
                    }
                }
            }
            Ok(None) => {
                failed_ids.push(id.clone());
                results.push(json!({"id": id, "success": false, "error": "Task not found"}));
            }
            Err(e) => {
                failed_ids.push(id.clone());
                results.push(json!({"id": id, "success": false, "error": e.to_string()}));
            }
        }
    }

    if failed_ids.is_empty() {
        ToolResult::success(
            format!(
                "Bulk {} completed: {} tasks processed",
                operation, success_count
            ),
            Some(json!({
                "operation": operation,
                "success_count": success_count,
                "results": results
            })),
        )
    } else {
        ToolResult::success(
            format!(
                "Bulk {} partially completed: {} succeeded, {} failed",
                operation,
                success_count,
                failed_ids.len()
            ),
            Some(json!({
                "operation": operation,
                "success_count": success_count,
                "failed_count": failed_ids.len(),
                "failed_ids": failed_ids,
                "results": results
            })),
        )
    }
}

// ==================== Sprint Tool Implementations ====================

fn create_sprint(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let name = match args.get("name").and_then(|v| v.as_str()) {
        Some(n) => n.to_string(),
        None => return ToolResult::error("Missing required parameter: name".to_string()),
    };

    let project = match args.get("project").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => return ToolResult::error("Missing required parameter: project".to_string()),
    };

    let mut sprint = Sprint::new(name, project);

    // Set optional fields
    if let Some(goal) = args.get("goal").and_then(|v| v.as_str()) {
        sprint.goal = Some(goal.to_string());
    }

    if let Some(start_date_str) = args.get("start_date").and_then(|v| v.as_str()) {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d") {
            sprint.start_date = Some(date.and_hms_opt(0, 0, 0).unwrap().and_utc());
        }
    }

    if let Some(end_date_str) = args.get("end_date").and_then(|v| v.as_str()) {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(end_date_str, "%Y-%m-%d") {
            sprint.end_date = Some(date.and_hms_opt(0, 0, 0).unwrap().and_utc());
        }
    }

    match ctx.store.add_sprint(&sprint) {
        Ok(created) => ToolResult::success(
            format!("Sprint created: {}", created.name),
            Some(json!({
                "id": created.id,
                "name": created.name,
                "project": created.project,
                "status": created.status.to_string(),
                "goal": created.goal,
                "start_date": created.start_date.map(|d| d.format("%Y-%m-%d").to_string()),
                "end_date": created.end_date.map(|d| d.format("%Y-%m-%d").to_string())
            })),
        ),
        Err(e) => ToolResult::error(format!("Failed to create sprint: {}", e)),
    }
}

fn list_sprints(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let project_filter = args.get("project").and_then(|v| v.as_str());
    let status_filter = args.get("status").and_then(|v| v.as_str());

    let sprints_result = if let Some(project) = project_filter {
        ctx.store.get_sprints_by_project(project)
    } else {
        ctx.store.get_all_sprints()
    };

    match sprints_result {
        Ok(mut sprints) => {
            // Filter by status if specified
            if let Some(status) = status_filter {
                if status != "all" {
                    let status: SprintStatus = status.to_string().into();
                    sprints.retain(|s| s.status == status);
                }
            }

            let sprint_summaries: Vec<Value> = sprints
                .iter()
                .map(|s| {
                    json!({
                        "id": s.id,
                        "name": s.name,
                        "project": s.project,
                        "status": s.status.to_string(),
                        "goal": s.goal,
                        "start_date": s.start_date.map(|d| d.format("%Y-%m-%d").to_string()),
                        "end_date": s.end_date.map(|d| d.format("%Y-%m-%d").to_string())
                    })
                })
                .collect();

            ToolResult::success(
                format!("Found {} sprints", sprint_summaries.len()),
                Some(json!({
                    "sprints": sprint_summaries,
                    "total": sprint_summaries.len()
                })),
            )
        }
        Err(e) => ToolResult::error(format!("Failed to list sprints: {}", e)),
    }
}

fn get_sprint(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let id = match args.get("id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: id".to_string()),
    };

    match ctx.store.get_sprint(id) {
        Ok(Some(sprint)) => {
            // Also get tasks in this sprint
            let tasks = ctx.store.get_tasks_in_sprint(id).unwrap_or_default();
            let task_summaries: Vec<Value> = tasks
                .iter()
                .map(|t| {
                    json!({
                        "id": t.id,
                        "title": t.title,
                        "status": t.status.to_string(),
                        "priority": t.priority.to_string(),
                        "sprint_order": t.sprint_order
                    })
                })
                .collect();

            ToolResult::success(
                "Sprint found".to_string(),
                Some(json!({
                    "sprint": {
                        "id": sprint.id,
                        "name": sprint.name,
                        "project": sprint.project,
                        "status": sprint.status.to_string(),
                        "goal": sprint.goal,
                        "start_date": sprint.start_date.map(|d| d.format("%Y-%m-%d").to_string()),
                        "end_date": sprint.end_date.map(|d| d.format("%Y-%m-%d").to_string()),
                        "created_at": sprint.created_at.format("%Y-%m-%d %H:%M").to_string()
                    },
                    "tasks": task_summaries,
                    "task_count": task_summaries.len()
                })),
            )
        }
        Ok(None) => ToolResult::error(format!("Sprint not found: {}", id)),
        Err(e) => ToolResult::error(format!("Failed to get sprint: {}", e)),
    }
}

fn update_sprint(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let id = match args.get("id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: id".to_string()),
    };

    match ctx.store.get_sprint(id) {
        Ok(Some(mut sprint)) => {
            // Update fields if provided
            if let Some(name) = args.get("name").and_then(|v| v.as_str()) {
                sprint.name = name.to_string();
            }

            if let Some(status_str) = args.get("status").and_then(|v| v.as_str()) {
                sprint.status = status_str.to_string().into();
            }

            if let Some(goal) = args.get("goal").and_then(|v| v.as_str()) {
                sprint.goal = Some(goal.to_string());
            }

            if let Some(start_date_str) = args.get("start_date").and_then(|v| v.as_str()) {
                if let Ok(date) = chrono::NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d") {
                    sprint.start_date = Some(date.and_hms_opt(0, 0, 0).unwrap().and_utc());
                }
            }

            if let Some(end_date_str) = args.get("end_date").and_then(|v| v.as_str()) {
                if let Ok(date) = chrono::NaiveDate::parse_from_str(end_date_str, "%Y-%m-%d") {
                    sprint.end_date = Some(date.and_hms_opt(0, 0, 0).unwrap().and_utc());
                }
            }

            match ctx.store.update_sprint(&sprint) {
                Ok(Some(updated)) => ToolResult::success(
                    format!("Sprint updated: {}", updated.name),
                    Some(json!({
                        "id": updated.id,
                        "name": updated.name,
                        "project": updated.project,
                        "status": updated.status.to_string(),
                        "goal": updated.goal
                    })),
                ),
                Ok(None) => ToolResult::error(format!("Sprint not found: {}", id)),
                Err(e) => ToolResult::error(format!("Failed to update sprint: {}", e)),
            }
        }
        Ok(None) => ToolResult::error(format!("Sprint not found: {}", id)),
        Err(e) => ToolResult::error(format!("Failed to get sprint: {}", e)),
    }
}

fn delete_sprint(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let id = match args.get("id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: id".to_string()),
    };

    match ctx.store.delete_sprint(id) {
        Ok(true) => ToolResult::success(
            "Sprint deleted (tasks unassigned)".to_string(),
            Some(json!({ "deleted_id": id })),
        ),
        Ok(false) => ToolResult::error(format!("Sprint not found: {}", id)),
        Err(e) => ToolResult::error(format!("Failed to delete sprint: {}", e)),
    }
}

fn start_sprint(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let id = match args.get("id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: id".to_string()),
    };

    match ctx.store.get_sprint(id) {
        Ok(Some(mut sprint)) => {
            sprint.start();
            match ctx.store.update_sprint(&sprint) {
                Ok(Some(updated)) => ToolResult::success(
                    format!("Sprint started: {}", updated.name),
                    Some(json!({
                        "id": updated.id,
                        "name": updated.name,
                        "status": updated.status.to_string(),
                        "start_date": updated.start_date.map(|d| d.format("%Y-%m-%d").to_string())
                    })),
                ),
                Ok(None) => ToolResult::error(format!("Sprint not found: {}", id)),
                Err(e) => ToolResult::error(format!("Failed to start sprint: {}", e)),
            }
        }
        Ok(None) => ToolResult::error(format!("Sprint not found: {}", id)),
        Err(e) => ToolResult::error(format!("Failed to get sprint: {}", e)),
    }
}

fn complete_sprint(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let id = match args.get("id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: id".to_string()),
    };

    match ctx.store.get_sprint(id) {
        Ok(Some(mut sprint)) => {
            sprint.complete();
            match ctx.store.update_sprint(&sprint) {
                Ok(Some(updated)) => ToolResult::success(
                    format!("Sprint completed: {}", updated.name),
                    Some(json!({
                        "id": updated.id,
                        "name": updated.name,
                        "status": updated.status.to_string(),
                        "end_date": updated.end_date.map(|d| d.format("%Y-%m-%d").to_string())
                    })),
                ),
                Ok(None) => ToolResult::error(format!("Sprint not found: {}", id)),
                Err(e) => ToolResult::error(format!("Failed to complete sprint: {}", e)),
            }
        }
        Ok(None) => ToolResult::error(format!("Sprint not found: {}", id)),
        Err(e) => ToolResult::error(format!("Failed to get sprint: {}", e)),
    }
}

fn assign_task_to_sprint(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let task_id = match args.get("task_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: task_id".to_string()),
    };

    let sprint_id = match args.get("sprint_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: sprint_id".to_string()),
    };

    match ctx.store.assign_task_to_sprint(task_id, sprint_id) {
        Ok(Some(task)) => ToolResult::success(
            format!("Task assigned to sprint"),
            Some(json!({
                "task_id": task.id,
                "task_title": task.title,
                "sprint_id": task.sprint_id,
                "sprint_order": task.sprint_order
            })),
        ),
        Ok(None) => ToolResult::error("Task or sprint not found".to_string()),
        Err(e) => ToolResult::error(format!("Failed to assign task to sprint: {}", e)),
    }
}

fn remove_task_from_sprint(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let task_id = match args.get("task_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: task_id".to_string()),
    };

    match ctx.store.remove_task_from_sprint(task_id) {
        Ok(Some(task)) => ToolResult::success(
            format!("Task removed from sprint"),
            Some(json!({
                "task_id": task.id,
                "task_title": task.title
            })),
        ),
        Ok(None) => ToolResult::error("Task not found".to_string()),
        Err(e) => ToolResult::error(format!("Failed to remove task from sprint: {}", e)),
    }
}

fn reorder_sprint_task(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let task_id = match args.get("task_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: task_id".to_string()),
    };

    let position = match args.get("position").and_then(|v| v.as_i64()) {
        Some(pos) => pos as i32,
        None => return ToolResult::error("Missing required parameter: position".to_string()),
    };

    if position < 1 {
        return ToolResult::error("Position must be at least 1".to_string());
    }

    match ctx.store.reorder_task_in_sprint(task_id, position) {
        Ok(Some(task)) => ToolResult::success(
            format!(
                "Task reordered to position {}",
                task.sprint_order.unwrap_or(position)
            ),
            Some(json!({
                "task_id": task.id,
                "task_title": task.title,
                "sprint_order": task.sprint_order
            })),
        ),
        Ok(None) => ToolResult::error("Task not found or not in a sprint".to_string()),
        Err(e) => ToolResult::error(format!("Failed to reorder task: {}", e)),
    }
}

fn get_sprint_tasks(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let sprint_id = match args.get("sprint_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return ToolResult::error("Missing required parameter: sprint_id".to_string()),
    };

    let include_done = args
        .get("include_done")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    match ctx.store.get_tasks_in_sprint(sprint_id) {
        Ok(mut tasks) => {
            if !include_done {
                tasks.retain(|t| !t.is_done());
            }

            let task_summaries: Vec<Value> = tasks
                .iter()
                .map(|t| {
                    json!({
                        "id": t.id,
                        "title": t.title,
                        "status": t.status.to_string(),
                        "priority": t.priority.to_string(),
                        "sprint_order": t.sprint_order,
                        "description": t.description
                    })
                })
                .collect();

            ToolResult::success(
                format!("Found {} tasks in sprint", task_summaries.len()),
                Some(json!({
                    "sprint_id": sprint_id,
                    "tasks": task_summaries,
                    "total": task_summaries.len()
                })),
            )
        }
        Err(e) => ToolResult::error(format!("Failed to get sprint tasks: {}", e)),
    }
}

fn get_backlog(ctx: &ToolContext, args: Value) -> ToolResult {
    let empty_map = serde_json::map::Map::new();
    let args = args.as_object().unwrap_or(&empty_map);

    let project = match args.get("project").and_then(|v| v.as_str()) {
        Some(p) => p,
        None => return ToolResult::error("Missing required parameter: project".to_string()),
    };

    match ctx.store.get_backlog_tasks(project) {
        Ok(tasks) => {
            let task_summaries: Vec<Value> = tasks
                .iter()
                .map(|t| {
                    json!({
                        "id": t.id,
                        "title": t.title,
                        "status": t.status.to_string(),
                        "priority": t.priority.to_string(),
                        "description": t.description
                    })
                })
                .collect();

            ToolResult::success(
                format!("Found {} tasks in backlog", task_summaries.len()),
                Some(json!({
                    "project": project,
                    "tasks": task_summaries,
                    "total": task_summaries.len()
                })),
            )
        }
        Err(e) => ToolResult::error(format!("Failed to get backlog: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::MemoryStore;

    fn create_test_context() -> ToolContext {
        let store = Arc::new(MemoryStore::new()) as Arc<dyn Store>;
        ToolContext::new(store)
    }

    #[test]
    fn test_list_tasks_empty() {
        let ctx = create_test_context();
        let result = list_tasks(&ctx, json!({}));
        assert!(result.success);
    }

    #[test]
    fn test_create_task() {
        let ctx = create_test_context();
        let result = create_task(
            &ctx,
            json!({
                "title": "Test task",
                "priority": "high",
                "tags": ["test", "demo"]
            }),
        );
        assert!(result.success);
        assert!(result.data.unwrap().get("id").is_some());
    }

    #[test]
    fn test_get_task_not_found() {
        let ctx = create_test_context();
        let result = get_task(&ctx, json!({"id": "nonexistent"}));
        assert!(!result.success);
    }

    #[test]
    fn test_complete_task() {
        let ctx = create_test_context();

        // Create a task
        let created = create_task(&ctx, json!({"title": "Task to complete"}));
        let id = created
            .data
            .unwrap()
            .get("id")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        // Complete it
        let result = complete_task(&ctx, json!({"id": id}));
        assert!(result.success);

        // Verify
        let task = get_task(&ctx, json!({"id": id}));
        assert!(task.success);
        let data = task.data.unwrap();
        let task_obj = data.get("task").unwrap();
        let status = task_obj.get("status").unwrap();
        assert_eq!(status, "done");
    }

    #[test]
    fn test_search_tasks() {
        let ctx = create_test_context();

        create_task(&ctx, json!({"title": "Buy grocery items"})); // contains "grocery"
        create_task(&ctx, json!({"title": "Finish report"}));
        create_task(&ctx, json!({"title": "Grocery shopping"})); // contains "grocery" (case-insensitive)

        let result = search_tasks(&ctx, json!({"query": "grocery"}));
        assert!(result.success);
        let count = result.data.unwrap().get("total").unwrap().as_u64().unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_get_stats() {
        let ctx = create_test_context();

        create_task(&ctx, json!({"title": "Task 1"}));
        create_task(&ctx, json!({"title": "Task 2"}));

        let result = get_stats(&ctx, json!({}));
        assert!(result.success);
        let data = result.data.unwrap();
        assert_eq!(data.get("pending").unwrap().as_u64().unwrap(), 2);
    }
}
