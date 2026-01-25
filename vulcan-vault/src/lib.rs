//! vulcan-vault - Obsidian-based knowledge vault with RAG for AI agents
//!
//! This crate provides:
//! - Note storage with YAML frontmatter metadata
//! - Vector embeddings for semantic search
//! - Agent memory system (decisions, lessons, preferences)
//! - vulcan-todo integration for task context
//! - MCP server interface for AI agent access
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                        vulcan-vault                              │
//! ├─────────────────────────────────────────────────────────────────┤
//! │  MCP Server ◄──── Claude Code ◄──── vulcan-todo                │
//! │       │                                                          │
//! │       ▼                                                          │
//! │  ┌─────────────────────────────────────────────────────────┐    │
//! │  │                    Core Components                       │    │
//! │  │  ┌──────────┐  ┌───────────┐  ┌──────────────────────┐ │    │
//! │  │  │ Markdown │  │  Chunker  │  │   Ollama Embeddings  │ │    │
//! │  │  │  Parser  │──│ (section) │──│   (nomic-embed-text) │ │    │
//! │  │  └──────────┘  └───────────┘  └──────────────────────┘ │    │
//! │  │                       │                                 │    │
//! │  │                       ▼                                 │    │
//! │  │              ┌─────────────────┐                       │    │
//! │  │              │   SQLite-vec    │                       │    │
//! │  │              │    vault.db     │                       │    │
//! │  │              └─────────────────┘                       │    │
//! │  └─────────────────────────────────────────────────────────┘    │
//! │                                                                  │
//! │  Zones: Projects/ | Tasks/ | Learning/ | Agent-Memories/        │
//! └─────────────────────────────────────────────────────────────────┘
//! ```

pub mod models;
pub mod store;
pub mod mcp;
pub mod rag;
pub mod memory;

#[cfg(feature = "tui")]
pub mod ui;

pub use models::{Note, NoteType, NoteStatus, Chunk, ChunkConfig, Memory, MemoryType, PrpPhase, PhaseStatus};
pub use store::{Store, SqliteStore, StoreError, SearchResult, LinkInfo, VaultStats};
pub use rag::{RagPipeline, RagError, RagResult, Chunker, EmbeddingService, EmbeddingError};
pub use memory::{
    MemoryFormation, FormationError, FormationResult, LessonSource, SessionEvent,
    MemoryRetrieval, RetrievalConfig, RetrievalError, RetrievalResult, ScoredMemory,
    MemoryDecay, DecayConfig, DecayError, DecayReport, DecayResult, CleanupReport,
};

/// Re-export for convenience
pub use anyhow::Result;

/// Default embedding dimension (nomic-embed-text)
pub const EMBEDDING_DIM: usize = 768;

/// Default config directory
pub fn config_dir() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("vulcan-vault")
}

/// Default vault directory
pub fn vault_dir() -> std::path::PathBuf {
    config_dir().join("vault")
}

/// Default database path
pub fn db_path() -> std::path::PathBuf {
    config_dir().join("vault.db")
}
