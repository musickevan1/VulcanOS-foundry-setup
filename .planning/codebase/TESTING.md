# Testing Patterns

**Analysis Date:** 2026-01-23

## Test Framework

### Runner

**Rust:**
- Framework: `cargo test` (built-in Rust test runner)
- No external test runner configuration found
- Tokio async support: `#[tokio::test]` macro for async test functions

**Run Commands:**
```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p vulcan-todo
cargo test -p vulcan-vault

# Run tests with output visible
cargo test -- --nocapture

# Run integration tests only
cargo test --test '*'
```

### Assertion Library

**Rust:**
- Built-in: `assert!()`, `assert_eq!()`, `assert_ne!()` macros
- No external assertion crate detected (cargo-test is native)
- Comparison: `assert_eq!(actual, expected)` pattern

## Test File Organization

### Location

**Pattern: Co-located with implementation**

Rust tests live in the same file as implementation code, not in separate directories:

- `vulcan-todo/src/models/task.rs` contains `#[cfg(test)] mod tests { ... }` at end of file
- `vulcan-vault/src/models/note.rs` contains no visible tests (integration tests in separate directory)
- `vulcan-vault/tests/rag_integration.rs` - standalone integration test file

**Observed structure:**
```
vulcan-todo/src/
├── models/
│   ├── task.rs          # Unit tests at end of file
│   └── sprint.rs        # Unit tests at end of file
└── ... (no tests/ directory)

vulcan-vault/
├── src/
│   ├── models/
│   └── store/
└── tests/
    └── rag_integration.rs  # Integration tests here
```

### Naming

**Unit tests:** In source files, test module named `tests`
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() { ... }
}
```

**Integration tests:** File in `tests/` directory
- File: `rag_integration.rs`
- Convention: `*_integration.rs` suffix
- Describes what's being tested: "RAG Pipeline Integration Tests"

## Test Structure

### Suite Organization

**Pattern from `vulcan-vault/tests/rag_integration.rs`:**

```rust
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
    // Arrange
    let chunker = Chunker::default();
    let content = r#"## Overview..."#;

    // Act
    let chunks = chunker.split("note-1", "test.md", content);

    // Assert
    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0].heading.as_deref(), Some("Overview"));
}

// Conditional integration tests requiring external service
#[cfg(feature = "integration")]
mod integration {
    use super::*;

    #[tokio::test]
    async fn test_full_pipeline() {
        // Tests requiring Ollama service
    }
}
```

**Patterns:**
- File-level doc comment with feature description
- Use of `use` statements to import what's being tested
- Arrange-Act-Assert (AAA) pattern within test functions
- Doc comments on test functions describe what's being tested
- Feature-gated integration tests: `#[cfg(feature = "integration")]` wraps heavy tests

### Setup/Teardown

**Setup pattern:**
```rust
#[test]
fn test_vector_search() {
    // Setup
    let store = SqliteStore::in_memory().unwrap();  // Create fresh store
    let note1 = Note::project_note("Rust Guide", "rust-project");
    store.save_note(&note1).unwrap();

    // Test code
    let results = store.vector_search(&embedding, None, None, None, 10).unwrap();

    // Implicit teardown: store dropped, in-memory DB cleared
}
```

**Teardown pattern:**
- No explicit teardown in observed tests
- Reliance on RAII (Rust Automatic Resource Management)
- In-memory databases (`SqliteStore::in_memory()`) are dropped at end of test scope
- Temporary files use `tempfile` crate (in Cargo.toml dev-dependencies)

### Assertion Patterns

**Basic assertions:**
```rust
assert_eq!(chunks.len(), 2);                    // Length checks
assert!(chunks[0].content.contains("```"));     // String contains
assert!(!results.is_empty());                   // Negation
```

**Option/Result assertions:**
```rust
assert_eq!(chunks[0].heading.as_deref(), Some("Overview"));  // Option comparison
let retrieved = store.get_chunks(&note.id).unwrap();          // Unwrap for tests
```

**Equality with custom types:**
```rust
assert_eq!(results[0].note_title, "Rust Guide");
assert_eq!(chunk.chunk_index, 0);
```

## Mocking

### Framework

**No explicit mocking framework detected.**

Rust tests avoid mocking via:
1. **Trait-based design**: `Store` trait allows in-memory implementations
2. **Factory methods**: `SqliteStore::in_memory()` creates test doubles
3. **Default values**: `Chunker::default()` and `Chunker::new(config)`

### Patterns

**Mock storage (in-memory):**
```rust
#[test]
fn test_store_chunks() {
    // Use in-memory store instead of actual SQLite
    let store = SqliteStore::in_memory().unwrap();

    // Create and store test data
    let note = Note::project_note("Test Note", "test-project");
    store.save_note(&note).unwrap();

    // Verify behavior
    let retrieved = store.get_chunks(&note.id).unwrap();
    assert_eq!(retrieved.len(), 2);
}
```

**Mock embeddings:**
```rust
#[test]
fn test_vector_search() {
    // Create mock embeddings with predictable values
    let rust_embedding: Vec<f32> = (0..768)
        .map(|i| if i < 384 { 0.8 } else { 0.2 })
        .collect();

    let chunk = Chunk::new(...)
        .with_embedding(rust_embedding.clone());

    store.save_chunks(&note1.id, &chunks1).unwrap();

    // Search uses mock embeddings
    let results = store.vector_search(&rust_embedding, ...).unwrap();
}
```

### What to Mock

**Mock when:**
- Testing against external services (Ollama embeddings API)
- Testing storage without database overhead
- Creating test fixtures with known values

**Don't mock (test real implementations):**
- Chunking logic (no I/O, deterministic)
- Model constructors (simple, pure functions)
- Vector search algorithms

**Observed approach:**
- Feature flag heavy tests: `#[cfg(feature = "integration")]`
- Skip tests if dependency unavailable:
```rust
#[tokio::test]
async fn test_full_pipeline() {
    if !pipeline.health_check().await.unwrap_or(false) {
        println!("Skipping: Ollama not available");
        return;  // Skip test gracefully
    }
    // Heavy test code
}
```

## Fixtures and Factories

### Test Data

**Factory pattern for models:**

```rust
// Note has factory methods
let project_note = Note::project_note("Project Doc", "my-project");
let learning_note = Note::learning_note("Learning Doc", "rust");

// Task has similar pattern
let task = Task::new("Test task".to_string());

// Chunk builders use fluent API
let chunk = Chunk::new(&note.id, &note.path, "content", 0, 0, 30)
    .with_heading("Rust")
    .with_embedding(rust_embedding.clone());
```

**Predictable test data:**
```rust
// Test chunking preserves code blocks
let content = r#"## Code Example

Here's a Rust example:

```rust
fn main() {
    println!("Hello, world!");
}
```

That's the code."#;

// Test empty content handling
let chunks = chunker.split("note-1", "test.md", "");
assert!(chunks.is_empty());
```

### Location

**Co-located:** No separate fixtures directory
- Factories/builders are methods on the model types themselves
- Test data created inline in test functions
- Constants used where appropriate (e.g., `768` for embedding dimension)

## Coverage

### Requirements

**No explicit coverage enforcement detected:**
- No codecov config
- No minimum coverage threshold in CI
- No coverage reports generated (none observed)

### View Coverage

**How to generate (not currently automated):**
```bash
# Generate coverage report (if tarpaulin installed)
cargo tarpaulin --out Html

# Alternative: using LLVM coverage (Rust nightly)
cargo +nightly tarpaulin
```

**Note:** Observed coverage in shown code:
- Unit tests in `task.rs`: Cover task creation, state transitions, serialization
- Integration tests in `rag_integration.rs`: Cover full pipeline end-to-end

## Test Types

### Unit Tests

**Scope:**
- Individual function behavior
- Model state transitions
- Data structure invariants

**Approach:**
```rust
#[test]
fn test_task_toggle() {
    let mut task = Task::new("Test".to_string());
    assert!(task.is_pending());

    task.toggle();
    assert!(task.is_in_progress());

    task.toggle();
    assert!(task.is_done());
}
```

**Observed tests in `vulcan-todo/src/models/task.rs`:**
1. `test_task_creation` - Verify initial state
2. `test_task_toggle` - State machine transitions
3. `test_priority_cycling` - Enum behavior
4. `test_task_store` - Collection operations
5. `test_task_store_search` - Search functionality

### Integration Tests

**Scope:**
- Multiple components working together
- File I/O and storage
- RAG pipeline (chunking + storage + search)

**Approach:**
```rust
#[test]
fn test_store_chunks() {
    let store = SqliteStore::in_memory().unwrap();

    // Create note
    let note = Note::project_note("Test Note", "test-project");
    store.save_note(&note).unwrap();

    // Save chunks
    let chunks = vec![...];
    store.save_chunks(&note.id, &chunks).unwrap();

    // Verify retrieval
    let retrieved = store.get_chunks(&note.id).unwrap();
    assert_eq!(retrieved.len(), 2);
}
```

**Observed tests in `vulcan-vault/tests/rag_integration.rs`:**
1. `test_chunk_simple_document` - Chunker splits markdown
2. `test_chunk_with_code_blocks` - Preserves code fences
3. `test_chunk_with_overlap` - Overlap configuration works
4. `test_store_chunks` - SQLite storage integration
5. `test_vector_search` - Vector similarity with mocks
6. `test_vector_search_with_filters` - Search filtering
7. `test_empty_content` - Edge case handling
8. `test_pipeline_creation` - RAG pipeline initialization
9. `test_chunk_metadata` - Metadata preservation
10. `test_full_pipeline` (integration feature) - End-to-end with Ollama

### E2E Tests

**Framework:** Not used

**Note:** System-level testing would be manual or via shell scripts:
- `scripts/test-iso.sh` - Boots ISO in QEMU
- Manual testing of Hyprland configs
- No automated E2E test framework configured

## Common Patterns

### Async Testing

**Pattern with Tokio:**

```rust
#[tokio::test]
async fn test_full_pipeline() {
    let store = SqliteStore::in_memory().unwrap();
    let pipeline = RagPipeline::new();

    // Can use async/await in test
    let result = pipeline.process_note(&store, &note).await;

    assert!(result.is_ok());
}
```

**Markers:**
- `#[tokio::test]` instead of `#[test]`
- Enables `.await` syntax
- Runtime is automatically provided

### Error Testing

**Pattern: Test Result types**

```rust
#[test]
fn test_not_found() {
    let store = JsonStore::new().unwrap();
    let task = store.get("nonexistent").unwrap();

    assert_eq!(task, None);  // Should return Option::None, not panic
}
```

**Verifying errors:**
- Use `Result::unwrap_or_default()` or `?` operator
- Test error cases via return types
- No explicit error matching in observed code

### Parametrized Tests

**Not observed** in shown code.

Alternative pattern used:
```rust
#[test]
fn test_priority_cycling() {
    // Manual assertion for each variant
    assert_eq!(Priority::None.next(), Priority::Low);
    assert_eq!(Priority::Low.next(), Priority::Medium);
    assert_eq!(Priority::Medium.next(), Priority::High);
    assert_eq!(Priority::High.next(), Priority::Urgent);
    assert_eq!(Priority::Urgent.next(), Priority::None);
}
```

## Dev Dependencies

**Cargo.toml (vulcan-todo):**
```toml
[dev-dependencies]
tempfile = "3.24"
assert_fs = "1.1"
```

**Cargo.toml (vulcan-vault):**
```toml
[dev-dependencies]
tempfile = "3.10"
assert_fs = "1.1"
```

**Purpose:**
- `tempfile`: Create temporary files/directories for testing file I/O
- `assert_fs`: Assertions for filesystem operations (not observed in shown tests, but available)

## Feature-Gated Tests

**Pattern: Integration tests requiring external services**

```rust
// Regular unit tests (always run)
#[test]
fn test_chunk_simple_document() { ... }

// Integration tests (only when feature enabled)
#[cfg(feature = "integration")]
mod integration {
    #[tokio::test]
    async fn test_full_pipeline() {
        // Requires Ollama service running
        // Skip gracefully if unavailable
        if !pipeline.health_check().await.unwrap_or(false) {
            println!("Skipping: Ollama not available");
            return;
        }
    }
}
```

**Run integration tests:**
```bash
cargo test --features integration
```

---

*Testing analysis: 2026-01-23*
