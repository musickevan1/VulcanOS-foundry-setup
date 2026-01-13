# vulcan-vault Architecture

Deep technical documentation of the vulcan-vault system.

## System Overview

vulcan-vault is a Rust application providing:
- **Storage**: SQLite with sqlite-vec extension for vector operations
- **RAG Pipeline**: Markdown chunking + Ollama embeddings
- **Memory System**: Formation, retrieval, and decay mechanics
- **MCP Interface**: JSON-RPC 2.0 over stdio for AI agent integration

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              vulcan-vault                                │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                         MCP Layer                                │   │
│  │   ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐    │   │
│  │   │   server    │  │  protocol   │  │       tools         │    │   │
│  │   │ (JSON-RPC)  │  │   (types)   │  │  (28 operations)    │    │   │
│  │   └─────────────┘  └─────────────┘  └─────────────────────┘    │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                    │                                     │
│           ┌────────────────────────┴────────────────────────┐           │
│           ▼                                                  ▼           │
│  ┌─────────────────────────┐              ┌─────────────────────────┐  │
│  │      RAG Pipeline       │              │     Memory System       │  │
│  │  ┌───────────────────┐  │              │  ┌───────────────────┐  │  │
│  │  │     Chunker       │  │              │  │    Formation      │  │  │
│  │  │  (pulldown-cmark) │  │              │  │ (record memories) │  │  │
│  │  └─────────┬─────────┘  │              │  └───────────────────┘  │  │
│  │            ▼            │              │  ┌───────────────────┐  │  │
│  │  ┌───────────────────┐  │              │  │    Retrieval      │  │  │
│  │  │    Embeddings     │  │              │  │ (semantic search) │  │  │
│  │  │     (Ollama)      │  │              │  └───────────────────┘  │  │
│  │  └───────────────────┘  │              │  ┌───────────────────┐  │  │
│  └─────────────────────────┘              │  │      Decay        │  │  │
│                                           │  │ (confidence mgmt) │  │  │
│                                           │  └───────────────────┘  │  │
│                                           └─────────────────────────┘  │
│                                    │                                     │
│                                    ▼                                     │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                        Store Layer                               │   │
│  │   ┌─────────────────────────────────────────────────────────┐   │   │
│  │   │                     SqliteStore                          │   │   │
│  │   │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐ │   │   │
│  │   │  │  notes   │  │  chunks  │  │  links   │  │memories │ │   │   │
│  │   │  └──────────┘  └──────────┘  └──────────┘  └─────────┘ │   │   │
│  │   │                    ┌──────────────────┐                 │   │   │
│  │   │                    │    sqlite-vec    │                 │   │   │
│  │   │                    │ (vector search)  │                 │   │   │
│  │   │                    └──────────────────┘                 │   │   │
│  │   └─────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                        Models Layer                              │   │
│  │   ┌──────────┐      ┌──────────┐      ┌──────────────────┐     │   │
│  │   │   Note   │      │  Chunk   │      │      Memory      │     │   │
│  │   │ NoteType │      │ChunkConfig│     │   MemoryType     │     │   │
│  │   │NoteStatus│      │          │      │                  │     │   │
│  │   └──────────┘      └──────────┘      └──────────────────┘     │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

## Module Details

### Models (`src/models/`)

#### Note (`note.rs`)

Core document type with YAML frontmatter support.

```rust
pub struct Note {
    // Core identity
    pub id: String,              // UUID v4
    pub path: String,            // Relative to vault root
    pub note_type: NoteType,     // Zone categorization
    pub title: String,

    // Timestamps
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,

    // Organization
    pub tags: Vec<String>,
    pub aliases: Vec<String>,    // Alternative titles
    pub status: NoteStatus,      // Active, Draft, Archived

    // Type-specific fields
    pub project: Option<String>,           // For project notes
    pub task_id: Option<String>,           // vulcan-todo link
    pub auto_fetch: bool,                  // Auto-load on task start
    pub context_type: Option<String>,      // implementation, research, etc.

    // Learning fields
    pub category: Option<String>,
    pub source: Option<String>,            // URL
    pub course: Option<String>,
    pub confidence: Option<f32>,           // Spaced repetition
    pub review_date: Option<DateTime<Utc>>,

    // Memory fields
    pub memory_type: Option<MemoryType>,
    pub context: Option<String>,
    pub agent: Option<String>,
    pub session_id: Option<String>,
    pub times_applied: u32,
    pub last_applied: Option<DateTime<Utc>>,

    // Content
    pub content: String,         // Markdown body (without frontmatter)
}

pub enum NoteType {
    Project,   // Projects/
    Task,      // Tasks/
    Learning,  // Learning/
    Memory,    // Agent-Memories/
    Meta,      // Meta/
}

pub enum NoteStatus {
    Active,
    Draft,
    Archived,
}
```

**Design Decisions:**
- Single struct with optional fields vs. enum variants: Chosen for simpler serialization
- `skip_serializing_if` allows schema evolution without breaking existing notes
- Path is relative for vault portability

#### Chunk (`chunk.rs`)

Text segment for embedding and retrieval.

```rust
pub struct Chunk {
    pub id: String,              // UUID v4
    pub note_id: String,         // Parent note
    pub note_path: String,       // For display
    pub content: String,         // Text to embed
    pub heading: Option<String>, // Section context
    pub chunk_index: u32,        // Order within note
    pub char_start: u32,         // Position tracking
    pub char_end: u32,
    pub embedding: Option<Vec<f32>>, // 768-dim vector
}

pub struct ChunkConfig {
    pub max_size: usize,             // Default: 1000 chars
    pub overlap: usize,              // Default: 100 chars
    pub split_on_headings: bool,     // Default: true
    pub preserve_code_blocks: bool,  // Default: true
}
```

**Design Decisions:**
- Position tracking enables source mapping for citations
- Heading context improves retrieval relevance
- Code blocks preserved intact (critical for technical content)

#### Memory (`memory.rs`)

Agent memory with confidence mechanics.

```rust
pub struct Memory {
    pub id: String,
    pub memory_type: MemoryType,
    pub title: String,
    pub content: String,
    pub context: String,              // Searchable categorization
    pub tags: Vec<String>,
    pub agent: String,                // Recording agent ID
    pub project: Option<String>,
    pub session_id: Option<String>,

    // Confidence mechanics
    pub confidence: f32,              // 0.0-1.0
    pub times_applied: u32,           // Reinforcement counter
    pub last_applied: Option<DateTime<Utc>>,

    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

pub enum MemoryType {
    Decision,    // Architecture choices
    Lesson,      // Learnings from experience
    Preference,  // User preferences
    Session,     // Session summaries
}

impl Memory {
    /// Reinforce memory when successfully applied
    pub fn reinforce(&mut self) {
        self.times_applied += 1;
        self.last_applied = Some(Utc::now());
        self.confidence = (self.confidence + 0.1).min(1.0);
    }

    /// Apply time-based decay
    pub fn decay(&mut self, decay_rate: f32) {
        if let Some(last) = self.last_applied {
            let days_inactive = (Utc::now() - last).num_days() as f32;
            if days_inactive > 0.0 {
                self.confidence = (self.confidence - decay_rate * days_inactive).max(0.0);
            }
        }
    }
}
```

**Initial Confidence by Type:**
| Type | Confidence | Rationale |
|------|-----------|-----------|
| Decision | 0.8 | May need refinement |
| Lesson | 0.8 | Context-dependent |
| Preference | 0.9 | Usually stable |
| Session | 1.0 | Factual record |

---

### Store Layer (`src/store/`)

#### Store Trait (`mod.rs`)

Abstraction allowing multiple backends.

```rust
pub trait Store: Send + Sync {
    // Note operations
    fn save_note(&self, note: &Note) -> StoreResult<()>;
    fn get_note(&self, id: &str) -> StoreResult<Option<Note>>;
    fn get_note_by_path(&self, path: &str) -> StoreResult<Option<Note>>;
    fn list_notes(&self, note_type: Option<NoteType>,
                  project: Option<&str>, limit: usize) -> StoreResult<Vec<Note>>;
    fn search_notes(&self, query: &str, limit: usize) -> StoreResult<Vec<Note>>;
    fn delete_note(&self, id: &str) -> StoreResult<()>;

    // Chunk operations
    fn save_chunks(&self, note_id: &str, chunks: &[Chunk]) -> StoreResult<()>;
    fn get_chunks(&self, note_id: &str) -> StoreResult<Vec<Chunk>>;
    fn vector_search(&self, embedding: &[f32], note_types: Option<Vec<NoteType>>,
                     project: Option<&str>, limit: usize) -> StoreResult<Vec<SearchResult>>;

    // Link operations (wikilinks)
    fn save_links(&self, source_id: &str, targets: &[LinkInfo]) -> StoreResult<()>;
    fn get_outlinks(&self, note_id: &str) -> StoreResult<Vec<LinkInfo>>;
    fn get_backlinks(&self, note_id: &str) -> StoreResult<Vec<LinkInfo>>;

    // Memory operations
    fn save_memory(&self, memory: &Memory) -> StoreResult<()>;
    fn get_memory(&self, id: &str) -> StoreResult<Option<Memory>>;
    fn search_memories(&self, context: &str, memory_type: Option<MemoryType>,
                       min_confidence: f32, limit: usize) -> StoreResult<Vec<Memory>>;
    fn search_memories_semantic(&self, embedding: &[f32],
                                min_confidence: f32, limit: usize) -> StoreResult<Vec<ScoredMemory>>;
    fn update_memory_confidence(&self, id: &str, confidence: f32) -> StoreResult<()>;

    // Statistics
    fn get_stats(&self) -> StoreResult<VaultStats>;
}
```

#### SqliteStore (`sqlite_store.rs`)

SQLite implementation with sqlite-vec for vectors.

**Database Schema:**

```sql
-- Notes table
CREATE TABLE notes (
    id TEXT PRIMARY KEY,
    path TEXT UNIQUE NOT NULL,
    note_type TEXT NOT NULL,
    title TEXT NOT NULL,
    created TEXT NOT NULL,
    modified TEXT NOT NULL,
    status TEXT DEFAULT 'active',
    tags TEXT,              -- JSON array
    aliases TEXT,           -- JSON array
    project TEXT,
    task_id TEXT,
    auto_fetch INTEGER DEFAULT 0,
    context_type TEXT,
    -- Learning fields
    category TEXT,
    source TEXT,
    course TEXT,
    confidence REAL,
    review_date TEXT,
    -- Memory fields
    memory_type TEXT,
    context TEXT,
    agent TEXT,
    session_id TEXT,
    times_applied INTEGER DEFAULT 0,
    last_applied TEXT,
    -- Content
    content TEXT NOT NULL
);

-- Chunk metadata
CREATE TABLE chunk_meta (
    id TEXT PRIMARY KEY,
    note_id TEXT NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    heading TEXT,
    chunk_index INTEGER NOT NULL,
    char_start INTEGER NOT NULL,
    char_end INTEGER NOT NULL
);

-- Vector storage (sqlite-vec virtual table)
CREATE VIRTUAL TABLE chunks USING vec0(
    id TEXT PRIMARY KEY,
    note_id TEXT,
    embedding FLOAT[768]
);

-- Wikilinks
CREATE TABLE links (
    source_id TEXT NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
    target_id TEXT,         -- NULL if target doesn't exist
    target_path TEXT NOT NULL,
    link_text TEXT,
    heading TEXT,
    PRIMARY KEY (source_id, target_path)
);

-- Memories
CREATE TABLE memories (
    id TEXT PRIMARY KEY,
    memory_type TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    context TEXT NOT NULL,
    tags TEXT,
    agent TEXT NOT NULL,
    project TEXT,
    session_id TEXT,
    confidence REAL NOT NULL,
    times_applied INTEGER DEFAULT 0,
    last_applied TEXT,
    created TEXT NOT NULL,
    modified TEXT NOT NULL
);

-- Memory embeddings (sqlite-vec)
CREATE VIRTUAL TABLE memory_embeddings USING vec0(
    id TEXT PRIMARY KEY,
    embedding FLOAT[768]
);

-- Indexes
CREATE INDEX idx_notes_type ON notes(note_type);
CREATE INDEX idx_notes_project ON notes(project);
CREATE INDEX idx_notes_task ON notes(task_id);
CREATE INDEX idx_chunks_note ON chunk_meta(note_id);
CREATE INDEX idx_memories_type ON memories(memory_type);
CREATE INDEX idx_memories_context ON memories(context);
CREATE INDEX idx_memories_confidence ON memories(confidence);
```

**Vector Search Implementation:**

```rust
fn vector_search(&self, embedding: &[f32], ...) -> StoreResult<Vec<SearchResult>> {
    let conn = self.lock_conn()?;

    // Convert embedding to bytes for sqlite-vec
    let embedding_bytes = embedding.as_bytes();

    let sql = r#"
        SELECT
            c.id, c.note_id, m.content, m.heading,
            n.path, n.title, n.note_type, n.project,
            vec_distance_cosine(c.embedding, ?1) as distance
        FROM chunks c
        JOIN chunk_meta m ON c.id = m.id
        JOIN notes n ON c.note_id = n.id
        WHERE distance < 1.5  -- Filter distant matches
        ORDER BY distance
        LIMIT ?2
    "#;

    // ... execute and map results
}
```

**Thread Safety:**

```rust
pub struct SqliteStore {
    conn: Mutex<Connection>,
}

impl SqliteStore {
    fn lock_conn(&self) -> StoreResult<MutexGuard<'_, Connection>> {
        self.conn.lock().map_err(|_| StoreError::LockPoisoned)
    }
}
```

---

### RAG Pipeline (`src/rag/`)

#### Chunker (`chunker.rs`)

Markdown-aware text segmentation.

```rust
impl Chunker {
    pub fn split(&self, note_id: &str, note_path: &str, content: &str) -> Vec<Chunk> {
        // Phase 1: Parse into sections by headings
        let sections = self.parse_sections(content);

        // Phase 2: Chunk each section respecting size limits
        let mut chunks = Vec::new();
        let mut chunk_index = 0;

        for section in sections {
            let section_chunks = self.chunk_section(
                note_id, note_path,
                &section.heading, &section.content,
                section.char_start, &mut chunk_index,
            );
            chunks.extend(section_chunks);
        }

        chunks
    }
}
```

**Chunking Strategy:**

```
Input Markdown:
┌────────────────────────────────────────────┐
│ # Overview                                  │
│ Lorem ipsum dolor sit amet, consectetur... │
│                                            │
│ ## Implementation                          │
│ ```rust                                    │
│ fn main() { ... }                          │
│ ```                                        │
│ More text here explaining the code...      │
└────────────────────────────────────────────┘

Chunking Process:
┌─────────────────┐     ┌─────────────────┐
│ Section 1       │     │ Section 2       │
│ heading: Overview│    │ heading: Impl   │
│ content: Lorem..│     │ content: ```... │
└─────────────────┘     └─────────────────┘
        │                       │
        ▼                       ▼
┌─────────────────┐     ┌─────────────────┐
│ Chunk 0         │     │ Chunk 1         │
│ heading: Overview│    │ heading: Impl   │
│ content: Lorem..│     │ content: ```... │
│ char_start: 0   │     │ char_start: 120 │
└─────────────────┘     └─────────────────┘
```

**Code Block Handling:**
- Code blocks are never split mid-block
- If a code block exceeds max_size, it becomes its own chunk (oversized)
- Preserves language annotations

#### EmbeddingService (`embeddings.rs`)

Ollama integration for vector generation.

```rust
pub struct EmbeddingService {
    client: reqwest::Client,
    ollama_url: String,      // Default: http://localhost:11434
    model: String,           // Default: nomic-embed-text
}

impl EmbeddingService {
    pub async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        let url = format!("{}/api/embed", self.ollama_url);

        let response = self.client
            .post(&url)
            .json(&json!({
                "model": self.model,
                "input": text
            }))
            .timeout(Duration::from_secs(30))
            .send()
            .await?;

        // Parse and validate 768-dim vector
        let result: EmbedResponse = response.json().await?;

        if result.embeddings[0].len() != 768 {
            return Err(EmbeddingError::InvalidDimension { ... });
        }

        Ok(result.embeddings[0].clone())
    }

    pub async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        let mut embeddings = Vec::with_capacity(texts.len());
        for text in texts {
            embeddings.push(self.embed(text).await?);
        }
        Ok(embeddings)
    }
}
```

**Error Handling:**

```rust
pub enum EmbeddingError {
    Request(reqwest::Error),
    OllamaError(String),
    InvalidEmbeddingDimension { expected: usize, got: usize },
    ServiceUnavailable { url: String },
    ModelNotFound { model: String },
}
```

---

### Memory System (`src/memory/`)

#### Formation (`formation.rs`)

Recording agent observations.

```rust
pub struct MemoryFormation<S: Store> {
    store: Arc<S>,
    embedder: EmbeddingService,
}

impl<S: Store> MemoryFormation<S> {
    pub async fn record_decision(
        &self,
        title: &str,
        content: &str,
        context: &str,
        agent: &str,
        outcome: Option<&str>,
    ) -> FormationResult<Memory> {
        let mut memory = Memory::decision(title, content, context, agent);

        if let Some(outcome) = outcome {
            memory.content = format!("{}\n\nOutcome: {}", memory.content, outcome);
        }

        self.save_with_embedding(memory).await
    }

    pub async fn record_lesson(
        &self,
        title: &str,
        content: &str,
        context: &str,
        agent: &str,
        source: LessonSource,  // Error, Correction, Discovery, Documentation
    ) -> FormationResult<Memory> {
        let mut memory = Memory::lesson(title, content, context, agent);
        memory.tags.push(format!("source:{}", source));
        self.save_with_embedding(memory).await
    }

    async fn save_with_embedding(&self, memory: Memory) -> FormationResult<Memory> {
        // Generate embedding for semantic search
        let embedding_text = format!("{} {}", memory.title, memory.content);
        let embedding = self.embedder.embed(&embedding_text).await?;

        // Save memory and embedding
        self.store.save_memory(&memory)?;
        self.store.save_memory_embedding(&memory.id, &embedding)?;

        Ok(memory)
    }
}
```

#### Retrieval (`retrieval.rs`)

Semantic similarity search with confidence weighting.

```rust
pub struct MemoryRetrieval<S: Store> {
    store: Arc<S>,
    embedder: EmbeddingService,
    config: RetrievalConfig,
}

pub struct RetrievalConfig {
    pub similarity_weight: f32,       // Default: 0.7
    pub confidence_weight: f32,       // Default: 0.3
    pub default_min_confidence: f32,  // Default: 0.3
    pub default_limit: usize,         // Default: 10
}

pub struct ScoredMemory {
    pub memory: Memory,
    pub similarity: f32,  // 0.0-1.0 (semantic match)
    pub score: f32,       // Combined score
}

impl<S: Store> MemoryRetrieval<S> {
    pub async fn recall(&self, query: &str, min_confidence: f32, limit: usize)
        -> RetrievalResult<Vec<ScoredMemory>>
    {
        // Embed query
        let query_embedding = self.embedder.embed(query).await?;

        // Vector search returns (memory, distance)
        let results = self.store.search_memories_semantic(
            &query_embedding, min_confidence, limit * 2  // Over-fetch for scoring
        )?;

        // Score and rank
        let mut scored: Vec<ScoredMemory> = results
            .into_iter()
            .map(|(memory, distance)| {
                let similarity = self.distance_to_similarity(distance);
                let score = self.calculate_score(similarity, memory.confidence);
                ScoredMemory { memory, similarity, score }
            })
            .collect();

        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        scored.truncate(limit);

        Ok(scored)
    }

    fn distance_to_similarity(&self, distance: f32) -> f32 {
        // Cosine distance range: 0 (identical) to 2 (opposite)
        // Convert to similarity: 1 (identical) to 0 (opposite)
        1.0 - (distance / 2.0)
    }

    fn calculate_score(&self, similarity: f32, confidence: f32) -> f32 {
        (similarity * self.config.similarity_weight) +
        (confidence * self.config.confidence_weight)
    }
}
```

**Scoring Formula Visualization:**

```
                    Similarity
                   (0.0 - 1.0)
                       │
                       ▼
                 ┌───────────┐
                 │   × 0.7   │
                 └─────┬─────┘
                       │
                       ├──────────▶ score = 0.7s + 0.3c
                       │
                 ┌─────┴─────┐
                 │   × 0.3   │
                 └───────────┘
                       ▲
                       │
                  Confidence
                  (0.0 - 1.0)

Example:
  similarity = 0.85 (good semantic match)
  confidence = 0.6  (somewhat decayed)

  score = (0.85 × 0.7) + (0.6 × 0.3)
        = 0.595 + 0.18
        = 0.775
```

#### Decay (`decay.rs`)

Time-based confidence management.

```rust
pub struct MemoryDecay<S: Store> {
    store: Arc<S>,
    config: DecayConfig,
}

pub struct DecayConfig {
    pub decay_rate: f32,           // Default: 0.01 per day
    pub grace_period_days: u32,    // Default: 7 days
    pub min_confidence: f32,       // Default: 0.1
    pub archive_dir: PathBuf,
}

impl<S: Store> MemoryDecay<S> {
    pub fn apply_decay(&self) -> DecayResult<DecayReport> {
        let mut report = DecayReport::default();

        let memories = self.store.get_memories_for_decay()?;

        for mut memory in memories {
            // Skip session memories (never decay)
            if memory.memory_type == MemoryType::Session {
                continue;
            }

            let original = memory.confidence;
            memory.decay(self.config.decay_rate);

            if (original - memory.confidence).abs() > f32::EPSILON {
                self.store.update_memory_confidence(&memory.id, memory.confidence)?;
                report.decayed += 1;
            }

            if memory.confidence < self.config.min_confidence {
                report.below_threshold += 1;
            }
        }

        Ok(report)
    }

    pub fn cleanup_expired(&self) -> DecayResult<CleanupReport> {
        let expired = self.store.get_memories_below_confidence(self.config.min_confidence)?;

        for memory in expired {
            // Archive to markdown file
            let archive_path = self.archive_memory(&memory)?;
            self.store.delete_memory(&memory.id)?;
        }

        // ... return report
    }
}
```

**Decay Timeline:**

```
Day 0                Day 7              Day 37             Day 97
  │                    │                   │                  │
  ▼                    ▼                   ▼                  ▼
┌────┐    Grace    ┌────┐   Decay    ┌────┐   Decay    ┌────┐
│0.80│────Period───│0.80│────────────│0.50│────────────│0.10│──▶ Archive
└────┘   (no decay) └────┘  -0.01/day └────┘            └────┘

Reinforcement can reset the clock:
  memory.reinforce()  →  confidence += 0.1, last_applied = now
```

---

### MCP Server (`src/mcp/`)

#### Server (`server.rs`)

JSON-RPC 2.0 over stdio.

```rust
pub async fn run_server() -> Result<()> {
    let store = SqliteStore::new(db_path())?;

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;

        let response = match serde_json::from_str::<Request>(&line) {
            Ok(request) => handle_request(&store, request).await,
            Err(e) => Response::error(-32700, format!("Parse error: {}", e)),
        };

        writeln!(stdout, "{}", serde_json::to_string(&response)?)?;
        stdout.flush()?;
    }

    Ok(())
}

async fn handle_request(store: &SqliteStore, request: Request) -> Response {
    match request.method.as_str() {
        "tools/list" => list_tools(),
        "tools/call" => call_tool(store, request.params).await,
        _ => Response::error(-32601, "Method not found"),
    }
}
```

#### Protocol (`protocol.rs`)

MCP type definitions.

```rust
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,  // JSON Schema
}

pub struct ToolResult {
    pub content: Vec<ContentItem>,
    pub is_error: bool,
}

pub enum ContentItem {
    Text { text: String },
}
```

---

## Data Flow Diagrams

### Note Creation Flow

```
User/Agent
    │
    │ create_note(title, content, type, ...)
    ▼
┌─────────────┐
│  MCP Tool   │
│ create_note │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────────────────────────────────┐
│                    Note Processing                           │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────────┐ │
│  │ Parse YAML  │───▶│   Chunk     │───▶│ Generate Embeds │ │
│  │ Frontmatter │    │   Content   │    │   (Ollama)      │ │
│  └─────────────┘    └─────────────┘    └─────────────────┘ │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                      Store Layer                             │
│  ┌───────────┐   ┌───────────┐   ┌────────────────────────┐│
│  │   notes   │   │chunk_meta │   │  chunks (sqlite-vec)   ││
│  │   table   │   │   table   │   │  embedding FLOAT[768]  ││
│  └───────────┘   └───────────┘   └────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

### Semantic Search Flow

```
Query: "error handling patterns"
    │
    ▼
┌─────────────────┐
│ Generate Query  │
│   Embedding     │
│    (Ollama)     │
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────────┐
│                   sqlite-vec Search                          │
│                                                              │
│   SELECT ... FROM chunks                                    │
│   WHERE vec_distance_cosine(embedding, query_vec) < 1.5     │
│   ORDER BY distance                                         │
│   LIMIT 10                                                  │
│                                                              │
└────────┬────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────┐
│  SearchResult   │
│  ┌───────────┐  │
│  │ chunk_id  │  │
│  │ note_id   │  │
│  │ content   │  │
│  │ heading   │  │
│  │ distance  │  │
│  └───────────┘  │
└─────────────────┘
```

### Memory Recall Flow

```
Context: "database design decisions"
    │
    ▼
┌──────────────────────────────────────────────────────────────┐
│                    Memory Retrieval                           │
│                                                               │
│  ┌─────────────┐    ┌──────────────┐    ┌────────────────┐  │
│  │   Embed     │───▶│ Vector Search │───▶│    Score &     │  │
│  │   Query     │    │  (memories)   │    │     Rank       │  │
│  └─────────────┘    └──────────────┘    └────────────────┘  │
│                                                ▼             │
│                                    ┌─────────────────────┐   │
│                                    │ score = 0.7×sim +   │   │
│                                    │         0.3×conf    │   │
│                                    └─────────────────────┘   │
└──────────────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────────┐
│  Results (sorted by score)                                   │
│                                                              │
│  1. "Chose SQLite for simplicity" (score: 0.82)            │
│  2. "PostgreSQL better for scale" (score: 0.71)            │
│  3. "Always index foreign keys"   (score: 0.65)            │
└─────────────────────────────────────────────────────────────┘
```

---

## Key Design Patterns

### Trait-Based Abstraction

```rust
// Enables testing with mock stores
pub trait Store: Send + Sync { ... }

impl Store for SqliteStore { ... }

// In tests:
impl Store for MockStore { ... }
```

### Async Where Needed

```rust
// Sync for database operations (SQLite isn't async-native)
fn save_note(&self, note: &Note) -> StoreResult<()>;

// Async for network operations
async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError>;

// Pipeline combines both
pub async fn process_note<S: Store>(&self, store: &S, note: &Note) -> RagResult<usize>;
```

### Error Handling

```rust
// Module-specific error types
pub enum StoreError { ... }
pub enum RagError { ... }
pub enum EmbeddingError { ... }

// Conversion via From trait
impl From<StoreError> for RagError {
    fn from(e: StoreError) -> Self {
        RagError::Store(e)
    }
}
```

---

## Performance Considerations

### Chunking

- **Max chunk size**: 1000 chars balances context vs. precision
- **Overlap**: 100 chars ensures cross-boundary concepts are found
- **Heading preservation**: Improves retrieval quality

### Embeddings

- **Batch processing**: Reduces HTTP overhead
- **768 dimensions**: nomic-embed-text standard
- **30s timeout**: Prevents hanging on slow responses

### Vector Search

- **sqlite-vec**: Uses SIMD for fast cosine distance
- **Distance threshold**: `< 1.5` filters irrelevant matches
- **Index-free**: sqlite-vec uses exhaustive search (fine for <100k vectors)

### Memory System

- **Confidence weighting**: Prioritizes reliable memories
- **Decay batching**: Run periodically, not per-query
- **Archive vs delete**: Preserves history for analysis
