//! RAG (Retrieval-Augmented Generation) pipeline
//!
//! This module handles:
//! - Markdown parsing and chunking
//! - Embedding generation (Ollama)
//! - Semantic search via sqlite-vec

mod chunker;
mod embeddings;

pub use chunker::Chunker;
pub use embeddings::{EmbeddingError, EmbeddingService, EMBEDDING_DIM};

use crate::models::{ChunkConfig, Note};
use crate::store::{Store, StoreError};
use thiserror::Error;

/// Errors that can occur during RAG pipeline operations
#[derive(Error, Debug)]
pub enum RagError {
    #[error("Embedding error: {0}")]
    Embedding(#[from] EmbeddingError),

    #[error("Store error: {0}")]
    Store(#[from] StoreError),

    #[error("No content to process")]
    EmptyContent,
}

/// Result type for RAG operations
pub type RagResult<T> = Result<T, RagError>;

/// RAG pipeline for processing notes into searchable chunks
pub struct RagPipeline {
    chunker: Chunker,
    embedder: EmbeddingService,
}

impl RagPipeline {
    /// Create a new RAG pipeline with default configuration
    pub fn new() -> Self {
        Self {
            chunker: Chunker::default(),
            embedder: EmbeddingService::new(),
        }
    }

    /// Create a pipeline with custom configuration
    pub fn with_config(chunk_config: ChunkConfig, ollama_url: &str, model: &str) -> Self {
        Self {
            chunker: Chunker::new(chunk_config),
            embedder: EmbeddingService::with_config(ollama_url, model),
        }
    }

    /// Process a note: chunk content, generate embeddings, save to store
    ///
    /// Returns the number of chunks created
    pub async fn process_note<S: Store>(&self, store: &S, note: &Note) -> RagResult<usize> {
        // Skip if note has no content
        if note.content.trim().is_empty() {
            return Ok(0);
        }

        // 1. Chunk the note content
        let chunks = self.chunker.split(&note.id, &note.path, &note.content);

        if chunks.is_empty() {
            return Ok(0);
        }

        // 2. Generate embeddings for each chunk
        let texts: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
        let embeddings = self.embedder.embed_batch(&texts).await?;

        // 3. Attach embeddings to chunks
        let chunks_with_embeddings: Vec<_> = chunks
            .into_iter()
            .zip(embeddings)
            .map(|(chunk, embedding)| chunk.with_embedding(embedding))
            .collect();

        let chunk_count = chunks_with_embeddings.len();

        // 4. Save to store
        store.save_chunks(&note.id, &chunks_with_embeddings)?;

        Ok(chunk_count)
    }

    /// Delete all chunks for a note
    pub fn delete_note_chunks<S: Store>(&self, store: &S, note_id: &str) -> RagResult<()> {
        store.delete_chunks(note_id)?;
        Ok(())
    }

    /// Check if the embedding service is available
    pub async fn health_check(&self) -> Result<bool, EmbeddingError> {
        self.embedder.health_check().await
    }

    /// Get a reference to the chunker
    pub fn chunker(&self) -> &Chunker {
        &self.chunker
    }

    /// Get a reference to the embedding service
    pub fn embedder(&self) -> &EmbeddingService {
        &self.embedder
    }
}

impl Default for RagPipeline {
    fn default() -> Self {
        Self::new()
    }
}
