//! Storage layer for vulcan-vault
//!
//! Uses SQLite with sqlite-vec extension for vector storage.
//! The database stores:
//! - notes: Metadata cache for quick filtering
//! - chunks: Vector embeddings for semantic search
//! - chunk_meta: Chunk content and positions
//! - links: Wikilink graph cache

mod sqlite_store;
mod error;

pub use sqlite_store::SqliteStore;
pub use error::StoreError;

use crate::models::{Note, NoteType, Chunk, Memory};

/// Result type for store operations
pub type StoreResult<T> = Result<T, StoreError>;

/// Trait defining the storage interface
///
/// This abstraction allows for different storage backends
/// (e.g., in-memory for testing, SQLite for production)
pub trait Store: Send + Sync {
    // === Note Operations ===

    /// Insert or update a note
    fn save_note(&self, note: &Note) -> StoreResult<()>;

    /// Get note by ID
    fn get_note(&self, id: &str) -> StoreResult<Option<Note>>;

    /// Get note by path
    fn get_note_by_path(&self, path: &str) -> StoreResult<Option<Note>>;

    /// Delete a note
    fn delete_note(&self, id: &str) -> StoreResult<()>;

    /// List notes with optional filters
    fn list_notes(
        &self,
        note_type: Option<NoteType>,
        project: Option<&str>,
        limit: usize,
    ) -> StoreResult<Vec<Note>>;

    /// Search notes by keyword
    fn search_notes(&self, query: &str, limit: usize) -> StoreResult<Vec<Note>>;

    /// Get notes linked to a task
    fn get_notes_by_task(&self, task_id: &str) -> StoreResult<Vec<Note>>;

    /// Get notes in a project
    fn get_notes_by_project(&self, project: &str) -> StoreResult<Vec<Note>>;

    // === Chunk Operations ===

    /// Save chunks for a note (replaces existing)
    fn save_chunks(&self, note_id: &str, chunks: &[Chunk]) -> StoreResult<()>;

    /// Get chunks for a note
    fn get_chunks(&self, note_id: &str) -> StoreResult<Vec<Chunk>>;

    /// Delete chunks for a note
    fn delete_chunks(&self, note_id: &str) -> StoreResult<()>;

    /// Vector similarity search
    fn vector_search(
        &self,
        embedding: &[f32],
        note_types: Option<&[NoteType]>,
        project: Option<&str>,
        limit: usize,
    ) -> StoreResult<Vec<SearchResult>>;

    // === Link Operations ===

    /// Update links for a note
    fn save_links(&self, source_id: &str, targets: &[LinkInfo]) -> StoreResult<()>;

    /// Get outgoing links from a note
    fn get_outlinks(&self, note_id: &str) -> StoreResult<Vec<LinkInfo>>;

    /// Get incoming links to a note
    fn get_backlinks(&self, note_id: &str) -> StoreResult<Vec<LinkInfo>>;

    // === Memory Operations ===

    /// Save a memory entry
    fn save_memory(&self, memory: &Memory) -> StoreResult<()>;

    /// Get memory by ID
    fn get_memory(&self, id: &str) -> StoreResult<Option<Memory>>;

    /// Search memories by context
    fn search_memories(
        &self,
        context: &str,
        memory_type: Option<&str>,
        min_confidence: f32,
        limit: usize,
    ) -> StoreResult<Vec<Memory>>;

    /// Update memory confidence
    fn update_memory_confidence(&self, id: &str, confidence: f32) -> StoreResult<()>;

    /// Save memory embedding for semantic search
    fn save_memory_embedding(&self, memory_id: &str, embedding: &[f32]) -> StoreResult<()>;

    /// Search memories by embedding similarity
    fn search_memories_semantic(
        &self,
        embedding: &[f32],
        min_confidence: f32,
        limit: usize,
    ) -> StoreResult<Vec<(Memory, f32)>>; // (memory, distance)

    /// Get memories that need decay processing
    fn get_memories_for_decay(&self) -> StoreResult<Vec<Memory>>;

    /// Get memories below confidence threshold (for cleanup)
    fn get_memories_below_confidence(&self, threshold: f32) -> StoreResult<Vec<Memory>>;

    /// Delete a memory
    fn delete_memory(&self, id: &str) -> StoreResult<()>;

    /// Update memory after reinforcement (confidence, times_applied, last_applied)
    fn update_memory_reinforcement(
        &self,
        id: &str,
        confidence: f32,
        times_applied: u32,
    ) -> StoreResult<()>;

    // === Stats ===

    /// Get vault statistics
    fn get_stats(&self) -> StoreResult<VaultStats>;
}

/// Result of a vector similarity search
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Chunk that matched
    pub chunk_id: String,
    /// Parent note ID
    pub note_id: String,
    /// Note path
    pub note_path: String,
    /// Note title
    pub note_title: String,
    /// Note type
    pub note_type: NoteType,
    /// Project (if any)
    pub project: Option<String>,
    /// Chunk content
    pub content: String,
    /// Section heading
    pub heading: Option<String>,
    /// Cosine distance (0 = identical, 2 = opposite)
    pub distance: f32,
}

/// Information about a wikilink
#[derive(Debug, Clone)]
pub struct LinkInfo {
    /// Target note ID
    pub target_id: String,
    /// Target note path
    pub target_path: String,
    /// Link text as written
    pub link_text: String,
    /// Section heading (if [[note#heading]])
    pub heading: Option<String>,
}

/// Vault statistics
#[derive(Debug, Clone, Default)]
pub struct VaultStats {
    pub total_notes: u64,
    pub notes_by_type: std::collections::HashMap<String, u64>,
    pub total_chunks: u64,
    pub total_links: u64,
    pub total_memories: u64,
    pub projects: Vec<String>,
}
