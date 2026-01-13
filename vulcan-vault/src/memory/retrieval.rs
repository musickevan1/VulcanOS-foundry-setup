//! Memory retrieval - semantic search for agent memories
//!
//! This module provides embedding-based similarity search for memories,
//! enabling agents to find relevant past experiences based on meaning
//! rather than just keyword matching.

use std::sync::Arc;

use crate::models::{Memory, MemoryType};
use crate::rag::EmbeddingService;
use crate::store::Store;

/// Result type for retrieval operations
pub type RetrievalResult<T> = Result<T, RetrievalError>;

/// Errors that can occur during memory retrieval
#[derive(Debug, thiserror::Error)]
pub enum RetrievalError {
    #[error("Store error: {0}")]
    Store(#[from] crate::store::StoreError),

    #[error("Embedding error: {0}")]
    Embedding(#[from] crate::rag::EmbeddingError),

    #[error("Memory not found: {0}")]
    NotFound(String),
}

/// A memory with its similarity score
#[derive(Debug, Clone)]
pub struct ScoredMemory {
    /// The retrieved memory
    pub memory: Memory,
    /// Semantic similarity (0.0-1.0, higher is more similar)
    pub similarity: f32,
    /// Combined score accounting for similarity and confidence
    pub score: f32,
}

/// Configuration for memory retrieval
#[derive(Debug, Clone)]
pub struct RetrievalConfig {
    /// Weight for semantic similarity in combined score (default: 0.7)
    pub similarity_weight: f32,
    /// Weight for confidence in combined score (default: 0.3)
    pub confidence_weight: f32,
    /// Default minimum confidence threshold
    pub default_min_confidence: f32,
    /// Default result limit
    pub default_limit: usize,
}

impl Default for RetrievalConfig {
    fn default() -> Self {
        Self {
            similarity_weight: 0.7,
            confidence_weight: 0.3,
            default_min_confidence: 0.3,
            default_limit: 10,
        }
    }
}

/// Service for semantic memory retrieval
///
/// Provides embedding-based search that finds memories by meaning,
/// with optional filtering by context, type, and confidence.
pub struct MemoryRetrieval<S: Store> {
    store: Arc<S>,
    embedder: EmbeddingService,
    config: RetrievalConfig,
}

impl<S: Store> MemoryRetrieval<S> {
    /// Create a new memory retrieval service
    pub fn new(store: Arc<S>, embedder: EmbeddingService) -> Self {
        Self {
            store,
            embedder,
            config: RetrievalConfig::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(store: Arc<S>, embedder: EmbeddingService, config: RetrievalConfig) -> Self {
        Self {
            store,
            embedder,
            config,
        }
    }

    /// Create with default Ollama embedder
    pub fn with_store(store: Arc<S>) -> Self {
        Self::new(store, EmbeddingService::new())
    }

    /// Search memories by semantic similarity only
    ///
    /// Embeds the query and finds memories with similar embeddings.
    /// Results are ranked by combined score (similarity + confidence).
    pub async fn search_semantic(
        &self,
        query: &str,
        min_confidence: Option<f32>,
        limit: Option<usize>,
    ) -> RetrievalResult<Vec<ScoredMemory>> {
        let min_conf = min_confidence.unwrap_or(self.config.default_min_confidence);
        let lim = limit.unwrap_or(self.config.default_limit);

        // Embed the query
        let embedding = self.embedder.embed(query).await?;

        // Search by embedding similarity
        let results = self.store.search_memories_semantic(&embedding, min_conf, lim)?;

        // Convert to ScoredMemory with calculated scores
        let scored = self.score_results(results);

        Ok(scored)
    }

    /// Hybrid search combining semantic similarity with filters
    ///
    /// First performs semantic search, then filters by context and type.
    /// Useful for scoped queries like "error handling in rust".
    pub async fn search_hybrid(
        &self,
        query: &str,
        context: Option<&str>,
        memory_type: Option<MemoryType>,
        min_confidence: Option<f32>,
        limit: Option<usize>,
    ) -> RetrievalResult<Vec<ScoredMemory>> {
        let min_conf = min_confidence.unwrap_or(self.config.default_min_confidence);
        let lim = limit.unwrap_or(self.config.default_limit);

        // Embed the query
        let embedding = self.embedder.embed(query).await?;

        // Get more results than needed to allow for filtering
        let fetch_limit = lim * 3;
        let results = self
            .store
            .search_memories_semantic(&embedding, min_conf, fetch_limit)?;

        // Apply filters
        let filtered: Vec<_> = results
            .into_iter()
            .filter(|(memory, _)| {
                // Context filter
                if let Some(ctx) = context {
                    if !memory.matches_context(ctx) {
                        return false;
                    }
                }
                // Type filter
                if let Some(ref mt) = memory_type {
                    if memory.memory_type != *mt {
                        return false;
                    }
                }
                true
            })
            .take(lim)
            .collect();

        let scored = self.score_results(filtered);

        Ok(scored)
    }

    /// Find memories related to a specific memory
    ///
    /// Useful for discovering connections between memories
    /// and building context around a known memory.
    pub async fn find_related(
        &self,
        memory_id: &str,
        limit: Option<usize>,
    ) -> RetrievalResult<Vec<ScoredMemory>> {
        let lim = limit.unwrap_or(self.config.default_limit);

        // Get the source memory
        let source = self
            .store
            .get_memory(memory_id)?
            .ok_or_else(|| RetrievalError::NotFound(memory_id.to_string()))?;

        // Create query from the source memory
        let query = format!("{} {}", source.title, source.content);

        // Search for similar memories (fetch extra to exclude self)
        let embedding = self.embedder.embed(&query).await?;
        let results = self
            .store
            .search_memories_semantic(&embedding, 0.0, lim + 1)?;

        // Filter out the source memory and score
        let filtered: Vec<_> = results
            .into_iter()
            .filter(|(m, _)| m.id != memory_id)
            .take(lim)
            .collect();

        let scored = self.score_results(filtered);

        Ok(scored)
    }

    /// Convert raw results to scored memories
    ///
    /// Applies the scoring formula: score = (similarity * weight) + (confidence * weight)
    fn score_results(&self, results: Vec<(Memory, f32)>) -> Vec<ScoredMemory> {
        results
            .into_iter()
            .map(|(memory, distance)| {
                // Convert cosine distance to similarity
                // Cosine distance from sqlite-vec is 0 (identical) to 2 (opposite)
                let similarity = 1.0 - (distance / 2.0);

                // Calculate combined score
                let score = (similarity * self.config.similarity_weight)
                    + (memory.confidence * self.config.confidence_weight);

                ScoredMemory {
                    memory,
                    similarity,
                    score,
                }
            })
            .collect()
    }

    /// Get retrieval config
    pub fn config(&self) -> &RetrievalConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to_similarity() {
        // Distance 0 (identical) -> similarity 1.0
        assert_eq!(1.0 - (0.0_f32 / 2.0), 1.0);

        // Distance 2 (opposite) -> similarity 0.0
        assert_eq!(1.0 - (2.0_f32 / 2.0), 0.0);

        // Distance 1 (orthogonal) -> similarity 0.5
        assert_eq!(1.0 - (1.0_f32 / 2.0), 0.5);
    }

    #[test]
    fn test_scoring_formula() {
        let config = RetrievalConfig::default();

        // Similarity 0.8, confidence 0.9
        let similarity = 0.8;
        let confidence = 0.9;
        let expected_score =
            (similarity * config.similarity_weight) + (confidence * config.confidence_weight);

        assert!((expected_score - 0.83).abs() < 0.01); // 0.8 * 0.7 + 0.9 * 0.3 = 0.83
    }

    #[test]
    fn test_retrieval_config_default() {
        let config = RetrievalConfig::default();

        assert_eq!(config.similarity_weight, 0.7);
        assert_eq!(config.confidence_weight, 0.3);
        assert_eq!(config.default_min_confidence, 0.3);
        assert_eq!(config.default_limit, 10);
    }
}
