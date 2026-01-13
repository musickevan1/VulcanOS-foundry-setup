//! Store error types

use thiserror::Error;

/// Errors that can occur in storage operations
#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Database connection lock poisoned")]
    LockPoisoned,

    #[error("Note not found: {0}")]
    NoteNotFound(String),

    #[error("Chunk not found: {0}")]
    ChunkNotFound(String),

    #[error("Memory not found: {0}")]
    MemoryNotFound(String),

    #[error("Path already exists: {0}")]
    PathExists(String),

    #[error("Invalid embedding dimension: expected {expected}, got {got}")]
    InvalidEmbeddingDimension { expected: usize, got: usize },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("sqlite-vec not available")]
    VecExtensionNotLoaded,
}
