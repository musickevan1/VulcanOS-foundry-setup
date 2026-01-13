//! Chunk model - represents a text chunk for vector embedding

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A text chunk extracted from a note for embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    /// Unique chunk identifier (UUID v4)
    pub id: String,

    /// Parent note ID
    pub note_id: String,

    /// Relative path of parent note
    pub note_path: String,

    /// The chunk text content
    pub content: String,

    /// Section heading this chunk belongs to (if any)
    pub heading: Option<String>,

    /// Index of this chunk within the note (0-based)
    pub chunk_index: u32,

    /// Character position where this chunk starts in the original content
    pub char_start: u32,

    /// Character position where this chunk ends
    pub char_end: u32,

    /// Vector embedding (768 dimensions for nomic-embed-text)
    #[serde(skip)]
    pub embedding: Option<Vec<f32>>,
}

impl Chunk {
    /// Create a new chunk
    pub fn new(
        note_id: impl Into<String>,
        note_path: impl Into<String>,
        content: impl Into<String>,
        chunk_index: u32,
        char_start: u32,
        char_end: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            note_id: note_id.into(),
            note_path: note_path.into(),
            content: content.into(),
            heading: None,
            chunk_index,
            char_start,
            char_end,
            embedding: None,
        }
    }

    /// Set the section heading for this chunk
    pub fn with_heading(mut self, heading: impl Into<String>) -> Self {
        self.heading = Some(heading.into());
        self
    }

    /// Set the embedding vector
    pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
        self.embedding = Some(embedding);
        self
    }

    /// Check if this chunk has been embedded
    pub fn is_embedded(&self) -> bool {
        self.embedding.is_some()
    }

    /// Get embedding dimension count (should be 768 for nomic-embed-text)
    pub fn embedding_dims(&self) -> Option<usize> {
        self.embedding.as_ref().map(|v| v.len())
    }
}

/// Configuration for chunking behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkConfig {
    /// Maximum characters per chunk
    pub max_size: usize,

    /// Overlap between chunks (for context continuity)
    pub overlap: usize,

    /// Whether to split on headings
    pub split_on_headings: bool,

    /// Whether to keep code blocks intact
    pub preserve_code_blocks: bool,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            max_size: 1000,
            overlap: 100,
            split_on_headings: true,
            preserve_code_blocks: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_creation() {
        let chunk = Chunk::new(
            "note-123",
            "Projects/test.md",
            "This is some content",
            0,
            0,
            20,
        );

        assert!(!chunk.id.is_empty());
        assert_eq!(chunk.note_id, "note-123");
        assert_eq!(chunk.chunk_index, 0);
        assert!(!chunk.is_embedded());
    }

    #[test]
    fn test_chunk_with_embedding() {
        let embedding = vec![0.1, 0.2, 0.3];
        let chunk = Chunk::new("note-123", "test.md", "content", 0, 0, 7)
            .with_embedding(embedding.clone());

        assert!(chunk.is_embedded());
        assert_eq!(chunk.embedding_dims(), Some(3));
    }
}
