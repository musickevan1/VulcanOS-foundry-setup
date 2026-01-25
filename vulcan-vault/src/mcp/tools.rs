//! MCP tool implementations
//!
//! This module contains all the tool definitions and handlers for the MCP server.

use anyhow::Result;
use serde_json::{json, Value};

use crate::{Note, NoteType, Memory, MemoryType, SqliteStore};
use crate::store::Store;
use crate::rag::EmbeddingService;

use super::protocol::ToolDefinition;

/// Get all tool definitions for tools/list
pub fn get_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        // Note CRUD
        ToolDefinition {
            name: "create_note".to_string(),
            description: "Create a new note in the vault".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "title": { "type": "string", "description": "Note title" },
                    "note_type": { "type": "string", "enum": ["project", "task", "learning", "memory", "meta"] },
                    "content": { "type": "string", "description": "Markdown content" },
                    "project": { "type": "string", "description": "Project identifier (optional)" },
                    "task_id": { "type": "string", "description": "vulcan-todo task ID (optional)" },
                    "tags": { "type": "array", "items": { "type": "string" }, "description": "Tags for organization" }
                },
                "required": ["title", "note_type", "content"]
            }),
        },
        ToolDefinition {
            name: "get_note".to_string(),
            description: "Get a note by ID or path".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Note ID" },
                    "path": { "type": "string", "description": "Note path (alternative to id)" }
                }
            }),
        },
        ToolDefinition {
            name: "list_notes".to_string(),
            description: "List notes with optional filters".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "note_type": { "type": "string", "enum": ["project", "task", "learning", "memory", "meta"] },
                    "project": { "type": "string" },
                    "limit": { "type": "integer", "default": 20 }
                }
            }),
        },
        ToolDefinition {
            name: "search_notes".to_string(),
            description: "Search notes by keyword".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Search query" },
                    "limit": { "type": "integer", "default": 10 }
                },
                "required": ["query"]
            }),
        },
        ToolDefinition {
            name: "delete_note".to_string(),
            description: "Delete a note".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Note ID" }
                },
                "required": ["id"]
            }),
        },

        // Task integration
        ToolDefinition {
            name: "get_task_context".to_string(),
            description: "Get context notes linked to a vulcan-todo task".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "task_id": { "type": "string", "description": "vulcan-todo task ID" }
                },
                "required": ["task_id"]
            }),
        },
        ToolDefinition {
            name: "link_note_to_task".to_string(),
            description: "Link an existing vault note to a vulcan-todo task. Returns note_id for updating task.context_notes.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "note_id": { "type": "string", "description": "Vault note UUID" },
                    "task_id": { "type": "string", "description": "vulcan-todo task UUID" }
                },
                "required": ["note_id", "task_id"]
            }),
        },
        ToolDefinition {
            name: "unlink_note_from_task".to_string(),
            description: "Remove task link from a vault note. Clears note.task_id.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "note_id": { "type": "string", "description": "Vault note UUID" }
                },
                "required": ["note_id"]
            }),
        },
        ToolDefinition {
            name: "get_task_notes".to_string(),
            description: "Get comprehensive context for a task: linked notes, project context, and relevant memories. Use after start_task when auto_fetch_context is true.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "task_id": { "type": "string", "description": "vulcan-todo task UUID" },
                    "project": { "type": "string", "description": "Project name for additional context (optional)" },
                    "include_project": { "type": "boolean", "default": true, "description": "Include project-level notes" },
                    "include_memories": { "type": "boolean", "default": true, "description": "Include relevant memories" }
                },
                "required": ["task_id"]
            }),
        },
        ToolDefinition {
            name: "get_notes_by_ids".to_string(),
            description: "Get notes by their UUIDs. Use for validating context_notes from vulcan-todo. Returns found and missing IDs.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "note_ids": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "List of vault note UUIDs to retrieve"
                    }
                },
                "required": ["note_ids"]
            }),
        },
        ToolDefinition {
            name: "create_task_context".to_string(),
            description: "Create a context note linked to a task".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "task_id": { "type": "string" },
                    "title": { "type": "string" },
                    "content": { "type": "string" },
                    "context_type": { "type": "string", "enum": ["implementation", "research", "blockers", "notes"] }
                },
                "required": ["task_id", "title", "content"]
            }),
        },

        // Project context
        ToolDefinition {
            name: "get_project_context".to_string(),
            description: "Get all context notes for a project".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "project": { "type": "string", "description": "Project identifier" }
                },
                "required": ["project"]
            }),
        },
        ToolDefinition {
            name: "list_projects".to_string(),
            description: "List all projects in the vault".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        },

        // Agent memory
        ToolDefinition {
            name: "record_lesson".to_string(),
            description: "Record a lesson learned".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "title": { "type": "string" },
                    "content": { "type": "string" },
                    "context": { "type": "string", "description": "When this lesson applies" },
                    "tags": { "type": "array", "items": { "type": "string" } }
                },
                "required": ["title", "content", "context"]
            }),
        },
        ToolDefinition {
            name: "record_decision".to_string(),
            description: "Record a decision made".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "title": { "type": "string" },
                    "content": { "type": "string" },
                    "context": { "type": "string" },
                    "tags": { "type": "array", "items": { "type": "string" } }
                },
                "required": ["title", "content", "context"]
            }),
        },
        ToolDefinition {
            name: "record_preference".to_string(),
            description: "Record a user preference".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "title": { "type": "string" },
                    "content": { "type": "string" },
                    "context": { "type": "string" }
                },
                "required": ["title", "content", "context"]
            }),
        },
        ToolDefinition {
            name: "recall_memories".to_string(),
            description: "Search for relevant memories".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "context": { "type": "string", "description": "Context to search" },
                    "memory_type": { "type": "string", "enum": ["decision", "lesson", "preference", "session"] },
                    "min_confidence": { "type": "number", "default": 0.3 },
                    "limit": { "type": "integer", "default": 10 },
                    "semantic": {
                        "type": "boolean",
                        "default": false,
                        "description": "Use semantic (embedding-based) search instead of keyword matching. Requires Ollama with nomic-embed-text."
                    }
                },
                "required": ["context"]
            }),
        },
        ToolDefinition {
            name: "reinforce_memory".to_string(),
            description: "Reinforce a memory after successful application".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Memory ID" }
                },
                "required": ["id"]
            }),
        },

        // Stats
        ToolDefinition {
            name: "get_stats".to_string(),
            description: "Get vault statistics".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        },

        // Semantic search (RAG)
        ToolDefinition {
            name: "semantic_search".to_string(),
            description: "Search notes using semantic similarity (requires Ollama with nomic-embed-text)".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Natural language search query" },
                    "limit": { "type": "integer", "default": 10, "description": "Maximum results to return" },
                    "note_types": {
                        "type": "array",
                        "items": { "type": "string", "enum": ["project", "task", "learning", "memory", "meta"] },
                        "description": "Filter by note types"
                    },
                    "project": { "type": "string", "description": "Filter by project" },
                    "tags": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Filter by tags (matches any)"
                    },
                    "min_similarity": {
                        "type": "number",
                        "minimum": 0,
                        "maximum": 1,
                        "default": 0.0,
                        "description": "Minimum similarity score (0-1)"
                    }
                },
                "required": ["query"]
            }),
        },

        // Context Engineering Tools
        ToolDefinition {
            name: "get_session_context".to_string(),
            description: "Get comprehensive context for the current work session. Returns in-progress tasks, high-priority pending tasks, relevant memories, and project notes. Use at session start for context preflight.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "project": { "type": "string", "description": "Optional: limit context to a specific project" },
                    "depth": {
                        "type": "string",
                        "enum": ["shallow", "deep"],
                        "default": "shallow",
                        "description": "shallow: active tasks + recent memories; deep: includes related notes and full task context"
                    },
                    "include_memories": { "type": "boolean", "default": true, "description": "Include relevant memories" }
                }
            }),
        },
        ToolDefinition {
            name: "create_prp".to_string(),
            description: "Create a PRP (Product Requirements Prompt) - a structured implementation spec. PRPs combine Value (why), Scope (what), Success Criteria (how to measure), and Implementation Phases.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "title": { "type": "string", "description": "PRP title" },
                    "project": { "type": "string", "description": "Project this PRP belongs to" },
                    "value": { "type": "string", "description": "Why are we building this?" },
                    "scope": { "type": "string", "description": "What exactly are we building?" },
                    "success_criteria": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "How do we measure success?"
                    },
                    "phases": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "name": { "type": "string" },
                                "description": { "type": "string" },
                                "effort": { "type": "string", "enum": ["small", "medium", "large"] }
                            },
                            "required": ["name", "description"]
                        },
                        "description": "Implementation phases"
                    },
                    "content": { "type": "string", "description": "Additional markdown content" }
                },
                "required": ["title", "project", "value", "scope"]
            }),
        },
        ToolDefinition {
            name: "save_checkpoint".to_string(),
            description: "Save a context checkpoint - captures current session state for later restoration. Use before major changes or when you want to preserve a decision point.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "Checkpoint name/label (e.g., 'before-refactor')" },
                    "session_id": { "type": "string", "description": "Current session ID" },
                    "context_summary": { "type": "string", "description": "Summary of current context and decisions made" },
                    "active_tasks": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "List of active task IDs at this checkpoint"
                    },
                    "parent_checkpoint": { "type": "string", "description": "Optional: parent checkpoint ID for branching" }
                },
                "required": ["name", "session_id", "context_summary"]
            }),
        },
        ToolDefinition {
            name: "list_checkpoints".to_string(),
            description: "List available context checkpoints, optionally filtered by session".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "session_id": { "type": "string", "description": "Filter by session ID" },
                    "limit": { "type": "integer", "default": 20 }
                }
            }),
        },
        ToolDefinition {
            name: "get_checkpoint".to_string(),
            description: "Get a specific checkpoint by ID, including its full context".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Checkpoint note ID" }
                },
                "required": ["id"]
            }),
        },
    ]
}

/// Call a tool by name with given arguments
pub async fn call_tool(store: &SqliteStore, name: &str, args: Value) -> Result<Value> {
    match name {
        "create_note" => create_note(store, args),
        "get_note" => get_note(store, args),
        "list_notes" => list_notes(store, args),
        "search_notes" => search_notes(store, args),
        "delete_note" => delete_note(store, args),
        "get_task_context" => get_task_context(store, args),
        "create_task_context" => create_task_context(store, args),
        "link_note_to_task" => link_note_to_task(store, args),
        "unlink_note_from_task" => unlink_note_from_task(store, args),
        "get_task_notes" => get_task_notes(store, args),
        "get_notes_by_ids" => get_notes_by_ids(store, args),
        "get_project_context" => get_project_context(store, args),
        "list_projects" => list_projects(store),
        "record_lesson" => record_memory(store, args, MemoryType::Lesson).await,
        "record_decision" => record_memory(store, args, MemoryType::Decision).await,
        "record_preference" => record_memory(store, args, MemoryType::Preference).await,
        "recall_memories" => recall_memories(store, args).await,
        "reinforce_memory" => reinforce_memory(store, args),
        "get_stats" => get_stats(store),
        "semantic_search" => semantic_search(store, args).await,
        // Context engineering tools
        "get_session_context" => get_session_context(store, args),
        "create_prp" => create_prp(store, args),
        "save_checkpoint" => save_checkpoint(store, args),
        "list_checkpoints" => list_checkpoints(store, args),
        "get_checkpoint" => get_checkpoint(store, args),
        _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
    }
}

// Tool implementations

fn create_note(store: &SqliteStore, args: Value) -> Result<Value> {
    let title = args.get("title").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing title"))?;
    let note_type_str = args.get("note_type").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing note_type"))?;
    let content = args.get("content").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing content"))?;

    let note_type = parse_note_type(note_type_str)?;
    let project = args.get("project").and_then(|v| v.as_str());
    let task_id = args.get("task_id").and_then(|v| v.as_str());

    let mut note = match note_type {
        NoteType::Project => {
            let proj = project.ok_or_else(|| anyhow::anyhow!("Project notes require project field"))?;
            Note::project_note(title, proj)
        }
        NoteType::Task => {
            let tid = task_id.ok_or_else(|| anyhow::anyhow!("Task notes require task_id field"))?;
            Note::task_note(title, tid)
        }
        NoteType::Learning => Note::learning_note(title, "topics"),
        NoteType::Memory => Note::memory_note(title, "lessons"),
        NoteType::Meta => Note::new(title, NoteType::Meta, format!("Meta/{}.md", slug(title))),
        NoteType::Prp => {
            let proj = project.ok_or_else(|| anyhow::anyhow!("PRP notes require project field"))?;
            let value = args.get("prp_value").and_then(|v| v.as_str()).unwrap_or("TBD");
            let scope = args.get("prp_scope").and_then(|v| v.as_str()).unwrap_or("TBD");
            Note::prp_note(title, proj, value, scope)
        }
        NoteType::Checkpoint => {
            let session_id = args.get("session_id").and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Checkpoint notes require session_id field"))?;
            let context_summary = args.get("checkpoint_context").and_then(|v| v.as_str()).unwrap_or("");
            Note::checkpoint_note(title, session_id, context_summary)
        }
    };

    note.content = content.to_string();

    if let Some(tags) = args.get("tags").and_then(|v| v.as_array()) {
        note.tags = tags.iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();
    }

    store.save_note(&note)?;

    Ok(json!({
        "content": [{
            "type": "text",
            "text": format!("Created note: {} ({})", note.title, note.id)
        }]
    }))
}

fn get_note(store: &SqliteStore, args: Value) -> Result<Value> {
    let note = if let Some(id) = args.get("id").and_then(|v| v.as_str()) {
        store.get_note(id)?
    } else if let Some(path) = args.get("path").and_then(|v| v.as_str()) {
        store.get_note_by_path(path)?
    } else {
        return Err(anyhow::anyhow!("Must provide id or path"));
    };

    match note {
        Some(n) => Ok(json!({
            "content": [{
                "type": "text",
                "text": serde_json::to_string_pretty(&n)?
            }]
        })),
        None => Ok(json!({
            "content": [{
                "type": "text",
                "text": "Note not found"
            }],
            "isError": true
        })),
    }
}

fn list_notes(store: &SqliteStore, args: Value) -> Result<Value> {
    let note_type = args.get("note_type")
        .and_then(|v| v.as_str())
        .map(parse_note_type)
        .transpose()?;
    let project = args.get("project").and_then(|v| v.as_str());
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(20) as usize;

    let notes = store.list_notes(note_type, project, limit)?;

    let result: Vec<Value> = notes.iter()
        .map(|n| json!({
            "id": n.id,
            "title": n.title,
            "type": n.note_type.to_string(),
            "path": n.path,
            "project": n.project,
        }))
        .collect();

    Ok(json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&result)?
        }]
    }))
}

fn search_notes(store: &SqliteStore, args: Value) -> Result<Value> {
    let query = args.get("query").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing query"))?;
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;

    let notes = store.search_notes(query, limit)?;

    let result: Vec<Value> = notes.iter()
        .map(|n| json!({
            "id": n.id,
            "title": n.title,
            "type": n.note_type.to_string(),
            "path": n.path,
        }))
        .collect();

    Ok(json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&result)?
        }]
    }))
}

fn delete_note(store: &SqliteStore, args: Value) -> Result<Value> {
    let id = args.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing id"))?;

    store.delete_note(id)?;

    Ok(json!({
        "content": [{
            "type": "text",
            "text": format!("Deleted note: {}", id)
        }]
    }))
}

fn get_task_context(store: &SqliteStore, args: Value) -> Result<Value> {
    let task_id = args.get("task_id").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing task_id"))?;

    let notes = store.get_notes_by_task(task_id)?;

    let result: Vec<Value> = notes.iter()
        .map(|n| json!({
            "id": n.id,
            "title": n.title,
            "path": n.path,
            "context_type": n.context_type,
            "content": n.content,
        }))
        .collect();

    Ok(json!({
        "content": [{
            "type": "text",
            "text": if result.is_empty() {
                format!("No context found for task {}", task_id)
            } else {
                serde_json::to_string_pretty(&result)?
            }
        }]
    }))
}

fn create_task_context(store: &SqliteStore, args: Value) -> Result<Value> {
    let task_id = args.get("task_id").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing task_id"))?;
    let title = args.get("title").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing title"))?;
    let content = args.get("content").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing content"))?;
    let context_type = args.get("context_type").and_then(|v| v.as_str());

    let mut note = Note::task_note(title, task_id);
    note.content = content.to_string();
    if let Some(ct) = context_type {
        note.context_type = Some(ct.to_string());
    }

    store.save_note(&note)?;

    Ok(json!({
        "content": [{
            "type": "text",
            "text": format!("Created task context: {} ({})", note.title, note.id)
        }]
    }))
}

fn link_note_to_task(store: &SqliteStore, args: Value) -> Result<Value> {
    let note_id = args.get("note_id").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing note_id"))?;
    let task_id = args.get("task_id").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing task_id"))?;

    let note = store.get_note(note_id)?;
    match note {
        Some(mut n) => {
            n.task_id = Some(task_id.to_string());
            store.save_note(&n)?;
            Ok(json!({
                "content": [{
                    "type": "text",
                    "text": format!("Linked note '{}' to task {}", n.title, task_id)
                }],
                "data": {
                    "note_id": n.id,
                    "task_id": task_id,
                    "note_title": n.title
                }
            }))
        }
        None => Ok(json!({
            "content": [{
                "type": "text",
                "text": format!("Note not found: {}", note_id)
            }],
            "isError": true
        })),
    }
}

fn unlink_note_from_task(store: &SqliteStore, args: Value) -> Result<Value> {
    let note_id = args.get("note_id").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing note_id"))?;

    let note = store.get_note(note_id)?;
    match note {
        Some(mut n) => {
            let old_task_id = n.task_id.clone();
            n.task_id = None;
            store.save_note(&n)?;
            Ok(json!({
                "content": [{
                    "type": "text",
                    "text": format!("Unlinked note '{}' from task", n.title)
                }],
                "data": {
                    "note_id": n.id,
                    "previous_task_id": old_task_id
                }
            }))
        }
        None => Ok(json!({
            "content": [{
                "type": "text",
                "text": format!("Note not found: {}", note_id)
            }],
            "isError": true
        })),
    }
}

fn get_task_notes(store: &SqliteStore, args: Value) -> Result<Value> {
    let task_id = args.get("task_id").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing task_id"))?;
    let project = args.get("project").and_then(|v| v.as_str());
    let include_project = args.get("include_project").and_then(|v| v.as_bool()).unwrap_or(true);
    let include_memories = args.get("include_memories").and_then(|v| v.as_bool()).unwrap_or(true);

    // Get notes directly linked to task
    let task_notes = store.get_notes_by_task(task_id)?;
    let task_notes_json: Vec<Value> = task_notes.iter()
        .map(|n| json!({
            "id": n.id,
            "title": n.title,
            "path": n.path,
            "context_type": n.context_type,
            "content": n.content,
        }))
        .collect();

    // Get project context if requested and project is provided
    let project_notes_json: Vec<Value> = if include_project {
        if let Some(proj) = project {
            let project_notes = store.get_notes_by_project(proj)?;
            project_notes.iter()
                .filter(|n| n.task_id.as_ref() != Some(&task_id.to_string())) // Exclude task notes
                .map(|n| json!({
                    "id": n.id,
                    "title": n.title,
                    "path": n.path,
                    "type": n.note_type.to_string(),
                }))
                .collect()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    // Get relevant memories if requested
    let memories_json: Vec<Value> = if include_memories {
        // Use task_id as context for memory search
        let context = format!("task:{} {}", task_id, project.unwrap_or(""));
        let memories = store.search_memories(&context, None, 0.3, 5)?;
        memories.iter()
            .map(|m| json!({
                "id": m.id,
                "type": m.memory_type.to_string(),
                "title": m.title,
                "content": m.content,
                "confidence": m.confidence,
            }))
            .collect()
    } else {
        vec![]
    };

    // Build comprehensive response
    let response = json!({
        "task_notes": task_notes_json,
        "project_notes": project_notes_json,
        "memories": memories_json,
        "summary": {
            "task_note_count": task_notes_json.len(),
            "project_note_count": project_notes_json.len(),
            "memory_count": memories_json.len()
        }
    });

    Ok(json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&response)?
        }]
    }))
}

fn get_notes_by_ids(store: &SqliteStore, args: Value) -> Result<Value> {
    let note_ids = args.get("note_ids")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow::anyhow!("Missing note_ids array"))?;

    let ids: Vec<&str> = note_ids.iter()
        .filter_map(|v| v.as_str())
        .collect();

    let mut found: Vec<Value> = vec![];
    let mut missing: Vec<String> = vec![];

    for id in ids {
        match store.get_note(id)? {
            Some(note) => {
                found.push(json!({
                    "id": note.id,
                    "title": note.title,
                    "path": note.path,
                    "task_id": note.task_id,
                }));
            }
            None => {
                missing.push(id.to_string());
            }
        }
    }

    Ok(json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&json!({
                "found": found,
                "missing": missing,
                "summary": {
                    "found_count": found.len(),
                    "missing_count": missing.len()
                }
            }))?
        }]
    }))
}

fn get_project_context(store: &SqliteStore, args: Value) -> Result<Value> {
    let project = args.get("project").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing project"))?;

    let notes = store.get_notes_by_project(project)?;

    let result: Vec<Value> = notes.iter()
        .map(|n| json!({
            "id": n.id,
            "title": n.title,
            "path": n.path,
            "type": n.note_type.to_string(),
        }))
        .collect();

    Ok(json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&result)?
        }]
    }))
}

fn list_projects(store: &SqliteStore) -> Result<Value> {
    let stats = store.get_stats()?;

    Ok(json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&stats.projects)?
        }]
    }))
}

async fn record_memory(store: &SqliteStore, args: Value, memory_type: MemoryType) -> Result<Value> {
    let title = args.get("title").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing title"))?;
    let content = args.get("content").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing content"))?;
    let context = args.get("context").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing context"))?;

    let mut memory = Memory::new(memory_type, title, content, context, "mcp-client");

    if let Some(tags) = args.get("tags").and_then(|v| v.as_array()) {
        memory.tags = tags.iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();
    }

    // Save memory first (always succeeds if store is working)
    store.save_memory(&memory)?;

    // Attempt to generate and save embedding (non-blocking on failure)
    let embedding_status = match generate_and_save_embedding(store, &memory).await {
        Ok(()) => "with embedding",
        Err(e) => {
            tracing::warn!("Failed to generate embedding for memory {}: {}", memory.id, e);
            "without embedding (Ollama unavailable)"
        }
    };

    Ok(json!({
        "content": [{
            "type": "text",
            "text": format!("Recorded {}: {} ({}) [{}]",
                memory.memory_type, memory.title, memory.id, embedding_status)
        }]
    }))
}

async fn recall_memories(store: &SqliteStore, args: Value) -> Result<Value> {
    let context = args.get("context").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing context"))?;
    let memory_type = args.get("memory_type").and_then(|v| v.as_str());
    let min_confidence = args.get("min_confidence").and_then(|v| v.as_f64()).unwrap_or(0.3) as f32;
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
    let semantic = args.get("semantic").and_then(|v| v.as_bool()).unwrap_or(false);

    let memories = if semantic {
        // Use semantic search via embeddings
        recall_memories_semantic(store, context, memory_type, min_confidence, limit).await?
    } else {
        // Use existing keyword-based search
        store.search_memories(context, memory_type, min_confidence, limit)?
    };

    let result: Vec<Value> = memories.iter()
        .map(|m| json!({
            "id": m.id,
            "type": m.memory_type.to_string(),
            "title": m.title,
            "content": m.content,
            "context": m.context,
            "confidence": m.confidence,
            "times_applied": m.times_applied,
        }))
        .collect();

    Ok(json!({
        "content": [{
            "type": "text",
            "text": if result.is_empty() {
                format!("No memories found for context: {}", context)
            } else {
                serde_json::to_string_pretty(&result)?
            }
        }]
    }))
}

/// Perform semantic search on memories using embeddings
async fn recall_memories_semantic(
    store: &SqliteStore,
    context: &str,
    memory_type: Option<&str>,
    min_confidence: f32,
    limit: usize,
) -> Result<Vec<Memory>> {
    let embedder = EmbeddingService::new();

    // Generate embedding for the search context
    let embedding = embedder.embed(context).await
        .map_err(|e| anyhow::anyhow!(
            "Semantic search requires Ollama with nomic-embed-text. Error: {}", e
        ))?;

    // Search by embedding similarity (request more to account for filtering)
    let results = store.search_memories_semantic(&embedding, min_confidence, limit * 2)?;

    // Filter by memory type if specified, and take limit
    let filtered: Vec<Memory> = results.into_iter()
        .map(|(memory, _distance)| memory)
        .filter(|m| {
            if let Some(mt) = memory_type {
                m.memory_type.to_string().to_lowercase() == mt.to_lowercase()
            } else {
                true
            }
        })
        .take(limit)
        .collect();

    Ok(filtered)
}

fn reinforce_memory(store: &SqliteStore, args: Value) -> Result<Value> {
    let id = args.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing id"))?;

    if let Some(mut memory) = store.get_memory(id)? {
        memory.reinforce();
        store.save_memory(&memory)?;
        Ok(json!({
            "content": [{
                "type": "text",
                "text": format!("Reinforced memory: {} (confidence: {:.2})", memory.title, memory.confidence)
            }]
        }))
    } else {
        Ok(json!({
            "content": [{
                "type": "text",
                "text": format!("Memory not found: {}", id)
            }],
            "isError": true
        }))
    }
}

fn get_stats(store: &SqliteStore) -> Result<Value> {
    let stats = store.get_stats()?;

    Ok(json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&json!({
                "total_notes": stats.total_notes,
                "notes_by_type": stats.notes_by_type,
                "total_chunks": stats.total_chunks,
                "total_links": stats.total_links,
                "total_memories": stats.total_memories,
                "projects": stats.projects,
            }))?
        }]
    }))
}

// Helper functions

/// Generate embedding for a memory and save it to the store
/// Returns Ok(()) on success, Err on failure (non-fatal - caller should handle gracefully)
async fn generate_and_save_embedding(store: &SqliteStore, memory: &Memory) -> Result<()> {
    let embedder = EmbeddingService::new();

    // Combine title, content, and context for richer embedding
    let embedding_text = format!("{} {} {}", memory.title, memory.content, memory.context);

    let embedding = embedder.embed(&embedding_text).await
        .map_err(|e| anyhow::anyhow!("Embedding generation failed: {}", e))?;

    store.save_memory_embedding(&memory.id, &embedding)
        .map_err(|e| anyhow::anyhow!("Failed to save embedding: {}", e))?;

    Ok(())
}

fn parse_note_type(s: &str) -> Result<NoteType> {
    match s.to_lowercase().as_str() {
        "project" => Ok(NoteType::Project),
        "task" => Ok(NoteType::Task),
        "learning" => Ok(NoteType::Learning),
        "memory" => Ok(NoteType::Memory),
        "meta" => Ok(NoteType::Meta),
        "prp" => Ok(NoteType::Prp),
        "checkpoint" => Ok(NoteType::Checkpoint),
        _ => Err(anyhow::anyhow!("Unknown note type: {}", s)),
    }
}

fn slug(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Semantic search using vector embeddings
async fn semantic_search(store: &SqliteStore, args: Value) -> Result<Value> {
    let query = args.get("query").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing query"))?;
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
    let project = args.get("project").and_then(|v| v.as_str());
    let min_similarity = args.get("min_similarity").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;

    // Parse note type filters
    let note_types: Option<Vec<NoteType>> = args.get("note_types")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .filter_map(|s| parse_note_type(s).ok())
                .collect()
        });

    // Tags filter: match notes that have ANY of the requested tags
    let tags: Option<Vec<String>> = args.get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        });

    // Generate embedding for the query
    let embedder = EmbeddingService::new();
    let embedding = match embedder.embed(query).await {
        Ok(emb) => emb,
        Err(e) => {
            return Ok(json!({
                "content": [{
                    "type": "text",
                    "text": format!("Embedding error: {}. Is Ollama running with nomic-embed-text?", e)
                }],
                "isError": true
            }));
        }
    };

    // Request more results to account for filtering
    let fetch_limit = if min_similarity > 0.0 { limit * 3 } else { limit };

    // Perform vector search
    let note_types_ref: Option<&[NoteType]> = note_types.as_deref();
    let tags_ref: Option<&[String]> = tags.as_deref();
    let results = store.vector_search(&embedding, note_types_ref, project, tags_ref, fetch_limit)?;

    if results.is_empty() {
        return Ok(json!({
            "content": [{
                "type": "text",
                "text": format!("No results found for: {}", query)
            }]
        }));
    }

    // Format results with relevance scores, filtering by min_similarity
    let formatted: Vec<Value> = results.iter()
        .filter_map(|r| {
            // Convert cosine distance to similarity score (0-1)
            // Cosine distance: 0 = identical, 2 = opposite
            let similarity = 1.0 - (r.distance / 2.0);

            // Apply minimum similarity filter
            if similarity < min_similarity {
                return None;
            }

            Some(json!({
                "note_id": r.note_id,
                "note_title": r.note_title,
                "note_path": r.note_path,
                "note_type": r.note_type.to_string(),
                "project": r.project,
                "tags": r.tags,
                "heading": r.heading,
                "content": r.content,
                "similarity": format!("{:.2}", similarity),
            }))
        })
        .take(limit)
        .collect();

    if formatted.is_empty() {
        return Ok(json!({
            "content": [{
                "type": "text",
                "text": format!("No results above similarity threshold {} for: {}", min_similarity, query)
            }]
        }));
    }

    Ok(json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&formatted)?
        }]
    }))
}

// ============================================================================
// Context Engineering Tools
// ============================================================================

/// Get comprehensive context for the current work session
fn get_session_context(store: &SqliteStore, args: Value) -> Result<Value> {
    let project = args.get("project").and_then(|v| v.as_str());
    let depth = args.get("depth").and_then(|v| v.as_str()).unwrap_or("shallow");
    let include_memories = args.get("include_memories").and_then(|v| v.as_bool()).unwrap_or(true);

    let mut context = json!({
        "project": project,
        "depth": depth,
    });

    // Get project notes if project specified
    if let Some(proj) = project {
        let project_notes: Vec<Value> = store.list_notes(Some(NoteType::Project), Some(proj), 10)?
            .into_iter()
            .map(|n| json!({
                "id": n.id,
                "title": n.title,
                "path": n.path,
            }))
            .collect();
        context["project_notes"] = json!(project_notes);

        // Get PRPs for the project
        let prps: Vec<Value> = store.list_notes(Some(NoteType::Prp), Some(proj), 5)?
            .into_iter()
            .map(|n| json!({
                "id": n.id,
                "title": n.title,
                "value": n.prp_value,
                "scope": n.prp_scope,
                "phases_count": n.implementation_phases.len(),
            }))
            .collect();
        context["active_prps"] = json!(prps);
    }

    // Get recent memories if requested
    if include_memories {
        // Search for high-confidence memories with broad context
        let context_query = project.unwrap_or("development");
        let memories = store.search_memories(context_query, None, 0.3, 10)?;
        let recent_memories: Vec<Value> = memories.into_iter()
            .take(5)
            .map(|m| json!({
                "id": m.id,
                "memory_type": m.memory_type.to_string(),
                "title": m.title,
                "context": m.context,
                "confidence": m.confidence,
            }))
            .collect();
        context["recent_memories"] = json!(recent_memories);
    }

    // Get recent checkpoints
    let checkpoints: Vec<Value> = store.list_notes(Some(NoteType::Checkpoint), None, 5)?
        .into_iter()
        .map(|n| json!({
            "id": n.id,
            "name": n.checkpoint_name,
            "session_id": n.session_id,
            "created": n.created.to_rfc3339(),
        }))
        .collect();
    context["recent_checkpoints"] = json!(checkpoints);

    // Deep mode: include more context
    if depth == "deep" {
        // Get task-linked notes
        let task_notes: Vec<Value> = store.list_notes(Some(NoteType::Task), project, 10)?
            .into_iter()
            .filter(|n| n.auto_fetch)
            .map(|n| json!({
                "id": n.id,
                "title": n.title,
                "task_id": n.task_id,
                "context_type": n.context_type,
            }))
            .collect();
        context["auto_fetch_notes"] = json!(task_notes);
    }

    Ok(json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&context)?
        }]
    }))
}

/// Create a PRP (Product Requirements Prompt)
fn create_prp(store: &SqliteStore, args: Value) -> Result<Value> {
    use crate::PrpPhase;

    let title = args.get("title").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing title"))?;
    let project = args.get("project").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing project"))?;
    let value = args.get("value").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing value"))?;
    let scope = args.get("scope").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing scope"))?;

    let mut note = Note::prp_note(title, project, value, scope);

    // Parse success criteria
    if let Some(criteria) = args.get("success_criteria").and_then(|v| v.as_array()) {
        note.success_criteria = criteria.iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();
    }

    // Parse implementation phases
    if let Some(phases) = args.get("phases").and_then(|v| v.as_array()) {
        note.implementation_phases = phases.iter()
            .filter_map(|p| {
                let name = p.get("name")?.as_str()?;
                let desc = p.get("description")?.as_str()?;
                let mut phase = PrpPhase::new(name, desc);
                phase.effort = p.get("effort").and_then(|v| v.as_str()).map(String::from);
                Some(phase)
            })
            .collect();
    }

    // Set content
    if let Some(content) = args.get("content").and_then(|v| v.as_str()) {
        note.content = content.to_string();
    } else {
        // Generate default content from structured fields
        let mut content = String::new();
        content.push_str(&format!("# {}\n\n", title));
        content.push_str("## Value\n");
        content.push_str(&format!("{}\n\n", value));
        content.push_str("## Scope\n");
        content.push_str(&format!("{}\n\n", scope));

        if !note.success_criteria.is_empty() {
            content.push_str("## Success Criteria\n");
            for criterion in &note.success_criteria {
                content.push_str(&format!("- [ ] {}\n", criterion));
            }
            content.push_str("\n");
        }

        if !note.implementation_phases.is_empty() {
            content.push_str("## Implementation Phases\n\n");
            for (i, phase) in note.implementation_phases.iter().enumerate() {
                let effort = phase.effort.as_deref().unwrap_or("medium");
                content.push_str(&format!("### Phase {}: {} [{}]\n", i + 1, phase.name, effort));
                content.push_str(&format!("{}\n\n", phase.description));
            }
        }

        note.content = content;
    }

    let note_id = note.id.clone();
    store.save_note(&note)?;

    Ok(json!({
        "content": [{
            "type": "text",
            "text": format!("Created PRP: {} (id: {})", title, note_id)
        }]
    }))
}

/// Save a context checkpoint
fn save_checkpoint(store: &SqliteStore, args: Value) -> Result<Value> {
    let name = args.get("name").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing name"))?;
    let session_id = args.get("session_id").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing session_id"))?;
    let context_summary = args.get("context_summary").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing context_summary"))?;

    let mut note = Note::checkpoint_note(name, session_id, context_summary);

    // Parse active tasks
    if let Some(tasks) = args.get("active_tasks").and_then(|v| v.as_array()) {
        note.checkpoint_tasks = tasks.iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();
    }

    // Set parent checkpoint for branching
    if let Some(parent) = args.get("parent_checkpoint").and_then(|v| v.as_str()) {
        note.parent_checkpoint = Some(parent.to_string());
    }

    // Generate content
    let mut content = String::new();
    content.push_str(&format!("# Checkpoint: {}\n\n", name));
    content.push_str(&format!("**Session:** {}\n\n", session_id));
    content.push_str("## Context Summary\n\n");
    content.push_str(context_summary);
    content.push_str("\n\n");

    if !note.checkpoint_tasks.is_empty() {
        content.push_str("## Active Tasks\n\n");
        for task_id in &note.checkpoint_tasks {
            content.push_str(&format!("- {}\n", task_id));
        }
    }

    note.content = content;

    let note_id = note.id.clone();
    store.save_note(&note)?;

    Ok(json!({
        "content": [{
            "type": "text",
            "text": format!("Checkpoint saved: {} (id: {})", name, note_id)
        }]
    }))
}

/// List available checkpoints
fn list_checkpoints(store: &SqliteStore, args: Value) -> Result<Value> {
    let session_id = args.get("session_id").and_then(|v| v.as_str());
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(20) as usize;

    let checkpoints: Vec<Value> = store.list_notes(Some(NoteType::Checkpoint), None, limit)?
        .into_iter()
        .filter(|n| {
            if let Some(sid) = session_id {
                n.session_id.as_deref() == Some(sid)
            } else {
                true
            }
        })
        .map(|n| json!({
            "id": n.id,
            "name": n.checkpoint_name,
            "session_id": n.session_id,
            "created": n.created.to_rfc3339(),
            "parent": n.parent_checkpoint,
            "tasks_count": n.checkpoint_tasks.len(),
        }))
        .collect();

    Ok(json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&json!({
                "checkpoints": checkpoints,
                "count": checkpoints.len()
            }))?
        }]
    }))
}

/// Get a specific checkpoint by ID
fn get_checkpoint(store: &SqliteStore, args: Value) -> Result<Value> {
    let id = args.get("id").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing id"))?;

    let note = store.get_note(id)?
        .ok_or_else(|| anyhow::anyhow!("Checkpoint not found: {}", id))?;

    if note.note_type != NoteType::Checkpoint {
        return Err(anyhow::anyhow!("Note {} is not a checkpoint", id));
    }

    Ok(json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&json!({
                "id": note.id,
                "name": note.checkpoint_name,
                "session_id": note.session_id,
                "created": note.created.to_rfc3339(),
                "context": note.checkpoint_context,
                "active_tasks": note.checkpoint_tasks,
                "parent_checkpoint": note.parent_checkpoint,
                "content": note.content,
            }))?
        }]
    }))
}
