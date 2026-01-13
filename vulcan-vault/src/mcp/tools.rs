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
                    "limit": { "type": "integer", "default": 10 }
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
        "record_lesson" => record_memory(store, args, MemoryType::Lesson),
        "record_decision" => record_memory(store, args, MemoryType::Decision),
        "record_preference" => record_memory(store, args, MemoryType::Preference),
        "recall_memories" => recall_memories(store, args),
        "reinforce_memory" => reinforce_memory(store, args),
        "get_stats" => get_stats(store),
        "semantic_search" => semantic_search(store, args).await,
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

fn record_memory(store: &SqliteStore, args: Value, memory_type: MemoryType) -> Result<Value> {
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

    store.save_memory(&memory)?;

    Ok(json!({
        "content": [{
            "type": "text",
            "text": format!("Recorded {}: {} ({})", memory.memory_type, memory.title, memory.id)
        }]
    }))
}

fn recall_memories(store: &SqliteStore, args: Value) -> Result<Value> {
    let context = args.get("context").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing context"))?;
    let memory_type = args.get("memory_type").and_then(|v| v.as_str());
    let min_confidence = args.get("min_confidence").and_then(|v| v.as_f64()).unwrap_or(0.3) as f32;
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;

    let memories = store.search_memories(context, memory_type, min_confidence, limit)?;

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

fn parse_note_type(s: &str) -> Result<NoteType> {
    match s.to_lowercase().as_str() {
        "project" => Ok(NoteType::Project),
        "task" => Ok(NoteType::Task),
        "learning" => Ok(NoteType::Learning),
        "memory" => Ok(NoteType::Memory),
        "meta" => Ok(NoteType::Meta),
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

    // Tags filter (TODO: requires SearchResult to include tags, not yet implemented)
    let _tags: Option<Vec<String>> = args.get("tags")
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
    let results = store.vector_search(&embedding, note_types_ref, project, fetch_limit)?;

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
