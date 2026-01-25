//! vulcan-vault CLI and MCP server
//!
//! Usage:
//!   vulcan-vault --mcp          # Run as MCP server
//!   vulcan-vault query "text"   # Semantic search
//!   vulcan-vault list           # List notes
//!   vulcan-vault stats          # Show statistics

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use vulcan_vault::{config_dir, db_path, vault_dir, SqliteStore, Store};

#[derive(Parser)]
#[command(name = "vulcan-vault")]
#[command(about = "VulcanOS Knowledge Vault - Obsidian-based RAG for AI agents")]
#[command(version)]
struct Cli {
    /// Run as MCP server (stdio mode)
    #[arg(long)]
    mcp: bool,

    /// Custom vault path
    #[arg(long)]
    path: Option<std::path::PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new vault
    Init,

    /// List notes with optional filters
    List {
        /// Filter by note type (project, task, learning, memory, meta)
        #[arg(short = 't', long)]
        note_type: Option<String>,

        /// Filter by project
        #[arg(short, long)]
        project: Option<String>,

        /// Maximum results
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },

    /// Semantic search query
    Query {
        /// Search query text
        query: String,

        /// Filter by project
        #[arg(short, long)]
        project: Option<String>,

        /// Maximum results
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Show vault statistics
    Stats,

    /// Rebuild embeddings for all notes
    Rebuild {
        /// Force rebuild even if unchanged
        #[arg(long)]
        force: bool,
    },

    /// Get context for a task
    TaskContext {
        /// Task ID from vulcan-todo
        task_id: String,
    },

    /// Record a memory
    Remember {
        /// Memory type (decision, lesson, preference)
        #[arg(short = 't', long, default_value = "lesson")]
        memory_type: String,

        /// Memory content
        content: String,

        /// Context for retrieval
        #[arg(short, long, default_value = "general")]
        context: String,
    },

    /// Recall memories
    Recall {
        /// Context to search
        context: String,

        /// Minimum confidence (0.0-1.0)
        #[arg(long, default_value = "0.3")]
        min_confidence: f32,

        /// Maximum results
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Launch interactive TUI
    #[cfg(feature = "tui")]
    Tui,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "vulcan_vault=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();

    let cli = Cli::parse();

    // MCP server mode
    if cli.mcp {
        return run_mcp_server().await;
    }

    // Ensure config directory exists
    let config = config_dir();
    std::fs::create_dir_all(&config)?;

    // Initialize store
    let store = SqliteStore::new(db_path())?;

    match cli.command {
        Some(Commands::Init) => {
            init_vault()?;
            println!("Vault initialized at {}", vault_dir().display());
        }

        Some(Commands::List {
            note_type,
            project,
            limit,
        }) => {
            let nt = note_type.as_ref().and_then(|s| parse_note_type(s));
            let notes = store.list_notes(nt, project.as_deref(), limit)?;

            if notes.is_empty() {
                println!("No notes found.");
            } else {
                println!("{:<40} {:<12} {:<20}", "Title", "Type", "Project");
                println!("{}", "-".repeat(72));
                for note in notes {
                    println!(
                        "{:<40} {:<12} {:<20}",
                        truncate(&note.title, 38),
                        note.note_type.to_string(),
                        note.project.as_deref().unwrap_or("-")
                    );
                }
            }
        }

        Some(Commands::Query {
            query,
            project,
            limit,
        }) => {
            // For now, do keyword search (semantic search requires embeddings)
            let notes = store.search_notes(&query, limit)?;

            if notes.is_empty() {
                println!("No matching notes found.");
            } else {
                println!("Found {} matching notes:\n", notes.len());
                for note in notes {
                    println!("  {} ({})", note.title, note.path);
                }
            }

            if project.is_some() {
                println!("\n(Note: semantic search with project filter requires embeddings)");
            }
        }

        Some(Commands::Stats) => {
            let stats = store.get_stats()?;
            println!("Vault Statistics");
            println!("{}", "=".repeat(40));
            println!("Total Notes:    {}", stats.total_notes);
            println!("Total Chunks:   {}", stats.total_chunks);
            println!("Total Links:    {}", stats.total_links);
            println!("Total Memories: {}", stats.total_memories);
            println!("\nNotes by Type:");
            for (note_type, count) in &stats.notes_by_type {
                println!("  {:<12}: {}", note_type, count);
            }
            if !stats.projects.is_empty() {
                println!("\nProjects: {}", stats.projects.join(", "));
            }
        }

        Some(Commands::Rebuild { force }) => {
            use vulcan_vault::RagPipeline;

            println!("Rebuilding embeddings{}...", if force { " (force)" } else { "" });

            // Initialize RAG pipeline
            let rag = RagPipeline::new();

            // Check if Ollama is available
            match rag.health_check().await {
                Ok(true) => {}
                Ok(false) => {
                    eprintln!("Error: Ollama is not available. Start it with: ollama serve");
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("Error checking Ollama: {}", e);
                    std::process::exit(1);
                }
            }

            // Get all notes
            let notes = store.list_notes(None, None, 10000)?;
            println!("Found {} notes to process.", notes.len());

            let mut processed = 0;
            let mut chunks_total = 0;
            let mut errors = 0;

            for note in &notes {
                // If not forcing, skip notes that already have chunks (basic check)
                if !force {
                    let existing = store.get_chunks(&note.id)?;
                    if !existing.is_empty() {
                        continue;
                    }
                }

                match rag.process_note(&store, note).await {
                    Ok(chunk_count) => {
                        if chunk_count > 0 {
                            println!("  {} -> {} chunks", truncate(&note.title, 40), chunk_count);
                            chunks_total += chunk_count;
                            processed += 1;
                        }
                    }
                    Err(e) => {
                        eprintln!("  Error processing {}: {}", note.title, e);
                        errors += 1;
                    }
                }
            }

            println!("\nRebuild complete:");
            println!("  Notes processed: {}", processed);
            println!("  Chunks created:  {}", chunks_total);
            if errors > 0 {
                println!("  Errors:          {}", errors);
            }
        }

        Some(Commands::TaskContext { task_id }) => {
            let notes = store.get_notes_by_task(&task_id)?;
            if notes.is_empty() {
                println!("No context notes found for task {}", task_id);
            } else {
                println!("Context for task {}:\n", &task_id[..8]);
                for note in notes {
                    println!("  {} ({})", note.title, note.path);
                }
            }
        }

        Some(Commands::Remember {
            memory_type,
            content,
            context,
        }) => {
            use vulcan_vault::{Memory, MemoryType};

            let mem_type = match memory_type.as_str() {
                "decision" => MemoryType::Decision,
                "preference" => MemoryType::Preference,
                _ => MemoryType::Lesson,
            };

            let memory = Memory::new(mem_type, &content[..40.min(content.len())], &content, &context, "cli");

            store.save_memory(&memory)?;
            println!("Memory recorded: {}", memory.id);
        }

        Some(Commands::Recall {
            context,
            min_confidence,
            limit,
        }) => {
            let memories = store.search_memories(&context, None, min_confidence, limit)?;

            if memories.is_empty() {
                println!("No matching memories found.");
            } else {
                println!("Found {} memories:\n", memories.len());
                for mem in memories {
                    println!(
                        "  [{}] {} (confidence: {:.2})",
                        mem.memory_type, mem.title, mem.confidence
                    );
                    println!("       {}", truncate(&mem.content, 60));
                }
            }
        }

        #[cfg(feature = "tui")]
        Some(Commands::Tui) => {
            use std::sync::Arc;
            let store = Arc::new(store);
            vulcan_vault::ui::run_tui(store)?;
        }

        None => {
            println!("vulcan-vault - VulcanOS Knowledge Vault\n");
            println!("Run with --help for usage information.");
            println!("Run with --mcp to start the MCP server.");
        }
    }

    Ok(())
}

/// Initialize the vault directory structure
fn init_vault() -> Result<()> {
    use std::fs;

    let vault = vault_dir();

    // Create zone directories
    let zones = [
        "Projects",
        "Tasks/by-id",
        "Tasks/templates",
        "Learning/courses",
        "Learning/topics",
        "Learning/reading-notes",
        "Agent-Memories/decisions",
        "Agent-Memories/lessons",
        "Agent-Memories/preferences",
        "Agent-Memories/sessions",
        "Meta",
        "Templates",
    ];

    for zone in zones {
        fs::create_dir_all(vault.join(zone))?;
    }

    // Create .obsidian directory
    fs::create_dir_all(vault.join(".obsidian"))?;

    // Create welcome note if it doesn't exist
    let welcome_path = vault.join("Welcome.md");
    if !welcome_path.exists() {
        fs::write(
            &welcome_path,
            r#"---
id: welcome
type: meta
title: Welcome to vulcan-vault
created: 2024-01-01T00:00:00Z
modified: 2024-01-01T00:00:00Z
tags: [getting-started]
---

# Welcome to vulcan-vault

This is your personal knowledge vault for AI agents.

## Zones

- **Projects/** - Project documentation, architecture, conventions
- **Tasks/** - Context notes linked to vulcan-todo tasks
- **Learning/** - Courses, topics, reading notes
- **Agent-Memories/** - Decisions, lessons, preferences from AI interactions

## Getting Started

1. Create project context in `Projects/<project-name>/`
2. Link notes to tasks using the `task_id` frontmatter field
3. Use the MCP server for AI agent access: `vulcan-vault --mcp`

For more information, see the [[Templates]] folder.
"#,
        )?;
    }

    Ok(())
}

/// Run the MCP server
async fn run_mcp_server() -> Result<()> {
    vulcan_vault::mcp::run_server().await
}

/// Parse note type from string
fn parse_note_type(s: &str) -> Option<vulcan_vault::NoteType> {
    match s.to_lowercase().as_str() {
        "project" => Some(vulcan_vault::NoteType::Project),
        "task" => Some(vulcan_vault::NoteType::Task),
        "learning" => Some(vulcan_vault::NoteType::Learning),
        "memory" => Some(vulcan_vault::NoteType::Memory),
        "meta" => Some(vulcan_vault::NoteType::Meta),
        "prp" => Some(vulcan_vault::NoteType::Prp),
        "checkpoint" => Some(vulcan_vault::NoteType::Checkpoint),
        _ => None,
    }
}

/// Truncate string to max length
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max - 3])
    }
}
