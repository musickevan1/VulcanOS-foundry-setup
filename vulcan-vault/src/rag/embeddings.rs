//! Embedding generation service for RAG pipeline
//!
//! Generates 768-dimensional embeddings using Ollama's nomic-embed-text model.
//! Designed for local-first operation with no cloud dependencies.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

/// Expected embedding dimension for nomic-embed-text
pub const EMBEDDING_DIM: usize = 768;

/// Default Ollama API endpoint
pub const DEFAULT_OLLAMA_URL: &str = "http://localhost:11434";

/// Default embedding model
pub const DEFAULT_MODEL: &str = "nomic-embed-text";

/// Errors that can occur during embedding generation
#[derive(Error, Debug)]
pub enum EmbeddingError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Ollama returned an error: {0}")]
    OllamaError(String),

    #[error("Invalid embedding dimension: expected {expected}, got {got}")]
    InvalidDimension { expected: usize, got: usize },

    #[error("Ollama service unavailable at {url}")]
    ServiceUnavailable { url: String },

    #[error("Model '{model}' not found. Run: ollama pull {model}")]
    ModelNotFound { model: String },
}

/// Embedding service for generating vector embeddings from text
pub struct EmbeddingService {
    client: Client,
    ollama_url: String,
    model: String,
}

/// Request body for Ollama embedding API
#[derive(Serialize)]
struct EmbeddingRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}

/// Response from Ollama embedding API
#[derive(Deserialize)]
struct EmbeddingResponse {
    embedding: Vec<f32>,
}

/// Error response from Ollama
#[derive(Deserialize)]
struct OllamaErrorResponse {
    error: String,
}

impl EmbeddingService {
    /// Create a new embedding service with default configuration
    pub fn new() -> Self {
        Self::with_config(DEFAULT_OLLAMA_URL, DEFAULT_MODEL)
    }

    /// Create an embedding service with custom configuration
    pub fn with_config(ollama_url: impl Into<String>, model: impl Into<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            ollama_url: ollama_url.into(),
            model: model.into(),
        }
    }

    /// Generate embedding for a single text
    pub async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        let url = format!("{}/api/embeddings", self.ollama_url);

        let request = EmbeddingRequest {
            model: &self.model,
            prompt: text,
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() {
                    EmbeddingError::ServiceUnavailable {
                        url: self.ollama_url.clone(),
                    }
                } else {
                    EmbeddingError::Request(e)
                }
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();

            // Try to parse as Ollama error
            if let Ok(err) = serde_json::from_str::<OllamaErrorResponse>(&error_text) {
                if err.error.contains("not found") {
                    return Err(EmbeddingError::ModelNotFound {
                        model: self.model.clone(),
                    });
                }
                return Err(EmbeddingError::OllamaError(err.error));
            }

            return Err(EmbeddingError::OllamaError(format!(
                "HTTP {}: {}",
                status, error_text
            )));
        }

        let result: EmbeddingResponse = response.json().await?;

        // Validate embedding dimension
        if result.embedding.len() != EMBEDDING_DIM {
            return Err(EmbeddingError::InvalidDimension {
                expected: EMBEDDING_DIM,
                got: result.embedding.len(),
            });
        }

        Ok(result.embedding)
    }

    /// Generate embeddings for multiple texts
    ///
    /// Processes texts sequentially to avoid overwhelming Ollama.
    /// For large batches, consider using embed_batch_parallel.
    pub async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        let mut embeddings = Vec::with_capacity(texts.len());

        for text in texts {
            let embedding = self.embed(text).await?;
            embeddings.push(embedding);
        }

        Ok(embeddings)
    }

    /// Check if Ollama service is available and model is loaded
    pub async fn health_check(&self) -> Result<bool, EmbeddingError> {
        // Try to embed a simple test string
        match self.embed("test").await {
            Ok(_) => Ok(true),
            Err(EmbeddingError::ServiceUnavailable { .. }) => Ok(false),
            Err(EmbeddingError::ModelNotFound { .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Get the configured model name
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Get the configured Ollama URL
    pub fn ollama_url(&self) -> &str {
        &self.ollama_url
    }
}

impl Default for EmbeddingService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let service = EmbeddingService::new();
        assert_eq!(service.ollama_url(), DEFAULT_OLLAMA_URL);
        assert_eq!(service.model(), DEFAULT_MODEL);
    }

    #[test]
    fn test_custom_config() {
        let service = EmbeddingService::with_config("http://custom:1234", "custom-model");
        assert_eq!(service.ollama_url(), "http://custom:1234");
        assert_eq!(service.model(), "custom-model");
    }

    // Integration tests require Ollama running
    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_embed_integration() {
        let service = EmbeddingService::new();

        let embedding = service.embed("Hello, world!").await.unwrap();

        assert_eq!(embedding.len(), EMBEDDING_DIM);
        // Embeddings should be normalized (values roughly between -1 and 1)
        assert!(embedding.iter().all(|&v| v.abs() < 10.0));
    }

    #[tokio::test]
    #[ignore]
    async fn test_batch_embed_integration() {
        let service = EmbeddingService::new();

        let texts = vec![
            "First document".to_string(),
            "Second document".to_string(),
            "Third document".to_string(),
        ];

        let embeddings = service.embed_batch(&texts).await.unwrap();

        assert_eq!(embeddings.len(), 3);
        for emb in embeddings {
            assert_eq!(emb.len(), EMBEDDING_DIM);
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_health_check() {
        let service = EmbeddingService::new();
        let healthy = service.health_check().await.unwrap();
        assert!(healthy, "Ollama should be running with nomic-embed-text");
    }
}
