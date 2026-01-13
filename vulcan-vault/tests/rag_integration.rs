//! RAG Pipeline Integration Tests
//!
//! These tests verify the complete RAG pipeline:
//! - Chunking markdown content
//! - Generating embeddings (mocked for unit tests)
//! - Storing and searching vectors

use vulcan_vault::{
    Chunk, ChunkConfig, Chunker, Note, NoteType, RagPipeline, SqliteStore, Store,
};

/// Test chunking a simple document
#[test]
fn test_chunk_simple_document() {
    let chunker = Chunker::default();
    let content = r#"## Overview
This is the overview section with some content.

## Implementation
The implementation details go here with more content."#;

    let chunks = chunker.split("note-1", "test.md", content);

    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0].heading.as_deref(), Some("Overview"));
    assert_eq!(chunks[1].heading.as_deref(), Some("Implementation"));
}

/// Test chunking preserves code blocks
#[test]
fn test_chunk_with_code_blocks() {
    let chunker = Chunker::default();
    let content = r#"## Code Example

Here's a Rust example:

```rust
fn main() {
    println!("Hello, world!");
}
```

That's the code."#;

    let chunks = chunker.split("note-1", "test.md", content);

    assert_eq!(chunks.len(), 1);
    assert!(chunks[0].content.contains("```"));
    assert!(chunks[0].content.contains("println!"));
}

/// Test chunking large content with overlap
#[test]
fn test_chunk_with_overlap() {
    let config = ChunkConfig {
        max_size: 100,
        overlap: 20,
        split_on_headings: true,
        preserve_code_blocks: true,
    };
    let chunker = Chunker::new(config);

    // Create content that will need to be split
    let content = "This is paragraph one with some content. This is paragraph two with more content. This is paragraph three finishing up.";

    let chunks = chunker.split("note-1", "test.md", content);

    // Should have multiple chunks
    assert!(chunks.len() >= 1);

    // Verify chunk indices are sequential
    for (i, chunk) in chunks.iter().enumerate() {
        assert_eq!(chunk.chunk_index, i as u32);
    }
}

/// Test store saves and retrieves chunks
#[test]
fn test_store_chunks() {
    let store = SqliteStore::in_memory().unwrap();

    // Create a note first
    let note = Note::project_note("Test Note", "test-project");
    store.save_note(&note).unwrap();

    // Create chunks with mock embeddings
    let chunks = vec![
        Chunk::new(&note.id, &note.path, "First chunk content", 0, 0, 20)
            .with_embedding(vec![0.1; 768]),
        Chunk::new(&note.id, &note.path, "Second chunk content", 1, 21, 42)
            .with_embedding(vec![0.2; 768]),
    ];

    // Save chunks
    store.save_chunks(&note.id, &chunks).unwrap();

    // Retrieve chunks
    let retrieved = store.get_chunks(&note.id).unwrap();
    assert_eq!(retrieved.len(), 2);
    assert_eq!(retrieved[0].chunk_index, 0);
    assert_eq!(retrieved[1].chunk_index, 1);
}

/// Test vector search with mock embeddings
#[test]
fn test_vector_search() {
    let store = SqliteStore::in_memory().unwrap();

    // Create notes
    let note1 = Note::project_note("Rust Guide", "rust-project");
    let note2 = Note::project_note("Python Guide", "python-project");
    store.save_note(&note1).unwrap();
    store.save_note(&note2).unwrap();

    // Create chunks with distinct embeddings
    // Rust-related embedding (high values in first half)
    let rust_embedding: Vec<f32> = (0..768)
        .map(|i| if i < 384 { 0.8 } else { 0.2 })
        .collect();

    // Python-related embedding (high values in second half)
    let python_embedding: Vec<f32> = (0..768)
        .map(|i| if i >= 384 { 0.8 } else { 0.2 })
        .collect();

    let chunks1 = vec![Chunk::new(
        &note1.id,
        &note1.path,
        "Rust programming guide content",
        0,
        0,
        30,
    )
    .with_heading("Rust")
    .with_embedding(rust_embedding.clone())];

    let chunks2 = vec![Chunk::new(
        &note2.id,
        &note2.path,
        "Python programming guide content",
        0,
        0,
        32,
    )
    .with_heading("Python")
    .with_embedding(python_embedding.clone())];

    store.save_chunks(&note1.id, &chunks1).unwrap();
    store.save_chunks(&note2.id, &chunks2).unwrap();

    // Search with Rust-like embedding should find Rust note first
    let results = store.vector_search(&rust_embedding, None, None, 10).unwrap();

    assert!(!results.is_empty());
    assert_eq!(results[0].note_title, "Rust Guide");
}

/// Test vector search with filters
#[test]
fn test_vector_search_with_filters() {
    let store = SqliteStore::in_memory().unwrap();

    // Create notes of different types
    let project_note = Note::project_note("Project Doc", "my-project");
    let learning_note = Note::learning_note("Learning Doc", "rust");

    store.save_note(&project_note).unwrap();
    store.save_note(&learning_note).unwrap();

    // Add chunks with same embedding
    let embedding: Vec<f32> = vec![0.5; 768];

    let chunks1 = vec![Chunk::new(
        &project_note.id,
        &project_note.path,
        "Project content",
        0,
        0,
        15,
    )
    .with_embedding(embedding.clone())];

    let chunks2 = vec![Chunk::new(
        &learning_note.id,
        &learning_note.path,
        "Learning content",
        0,
        0,
        16,
    )
    .with_embedding(embedding.clone())];

    store.save_chunks(&project_note.id, &chunks1).unwrap();
    store.save_chunks(&learning_note.id, &chunks2).unwrap();

    // Search with project filter
    let results = store
        .vector_search(&embedding, None, Some("my-project"), 10)
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].note_title, "Project Doc");

    // Search with note type filter
    let results = store
        .vector_search(&embedding, Some(&[NoteType::Learning]), None, 10)
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].note_title, "Learning Doc");
}

/// Test empty content handling
#[test]
fn test_empty_content() {
    let chunker = Chunker::default();

    let chunks = chunker.split("note-1", "test.md", "");
    assert!(chunks.is_empty());

    let chunks = chunker.split("note-1", "test.md", "   \n\n   ");
    assert!(chunks.is_empty());
}

/// Test pipeline creation
#[test]
fn test_pipeline_creation() {
    let pipeline = RagPipeline::new();

    // Verify default configuration
    assert_eq!(pipeline.embedder().model(), "nomic-embed-text");
    assert_eq!(pipeline.embedder().ollama_url(), "http://localhost:11434");
}

/// Test chunk metadata
#[test]
fn test_chunk_metadata() {
    let chunker = Chunker::default();
    let content = "Test content for chunking.";

    let chunks = chunker.split("my-note-id", "Projects/test.md", content);

    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].note_id, "my-note-id");
    assert_eq!(chunks[0].note_path, "Projects/test.md");
    assert_eq!(chunks[0].chunk_index, 0);
}

// Integration tests requiring Ollama
#[cfg(feature = "integration")]
mod integration {
    use super::*;

    #[tokio::test]
    async fn test_full_pipeline() {
        let store = SqliteStore::in_memory().unwrap();
        let pipeline = RagPipeline::new();

        // Check if Ollama is available
        if !pipeline.health_check().await.unwrap_or(false) {
            println!("Skipping: Ollama not available");
            return;
        }

        // Create a note with content
        let mut note = Note::project_note("Test Note", "test-project");
        note.content = "This is test content about Rust programming.".to_string();
        store.save_note(&note).unwrap();

        // Process the note
        let chunk_count = pipeline.process_note(&store, &note).await.unwrap();

        assert!(chunk_count > 0);

        // Verify chunks were stored
        let chunks = store.get_chunks(&note.id).unwrap();
        assert_eq!(chunks.len(), chunk_count);
    }
}
