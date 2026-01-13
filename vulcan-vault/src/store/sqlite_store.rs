//! SQLite storage implementation with sqlite-vec for vector search

#![allow(clippy::missing_transmute_annotations)]

use rusqlite::{params, Connection, OptionalExtension};
use sqlite_vec::sqlite3_vec_init;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use zerocopy::AsBytes;

use crate::models::{Chunk, Memory, MemoryType, Note, NoteStatus, NoteType};

use super::{LinkInfo, SearchResult, Store, StoreError, StoreResult, VaultStats};

/// Expected embedding dimensions (nomic-embed-text)
const EMBEDDING_DIM: usize = 768;

/// SQLite-based storage with sqlite-vec for vector operations
pub struct SqliteStore {
    /// Database connection (protected by mutex for thread safety)
    conn: Mutex<Connection>,
}

impl SqliteStore {
    /// Create a new store, initializing the database at the given path
    pub fn new(db_path: impl AsRef<Path>) -> StoreResult<Self> {
        // Register sqlite-vec extension before opening connection
        unsafe {
            rusqlite::ffi::sqlite3_auto_extension(Some(std::mem::transmute(
                sqlite3_vec_init as *const (),
            )));
        }

        let conn = Connection::open(db_path)?;
        let store = Self {
            conn: Mutex::new(conn),
        };

        store.init_schema()?;
        Ok(store)
    }

    /// Create an in-memory store (for testing)
    pub fn in_memory() -> StoreResult<Self> {
        unsafe {
            rusqlite::ffi::sqlite3_auto_extension(Some(std::mem::transmute(
                sqlite3_vec_init as *const (),
            )));
        }

        let conn = Connection::open_in_memory()?;
        let store = Self {
            conn: Mutex::new(conn),
        };

        store.init_schema()?;
        Ok(store)
    }

    /// Acquire the database connection lock with proper error handling
    fn lock_conn(&self) -> StoreResult<std::sync::MutexGuard<'_, Connection>> {
        self.conn.lock().map_err(|_| StoreError::LockPoisoned)
    }

    /// Initialize database schema
    fn init_schema(&self) -> StoreResult<()> {
        let conn = self.lock_conn()?;

        // Verify sqlite-vec is loaded
        let vec_version: String = conn.query_row(
            "SELECT vec_version()",
            [],
            |row| row.get(0),
        ).map_err(|_| StoreError::VecExtensionNotLoaded)?;

        tracing::info!("sqlite-vec version: {}", vec_version);

        conn.execute_batch(
            r#"
            -- Notes metadata table
            CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                path TEXT UNIQUE NOT NULL,
                note_type TEXT NOT NULL,
                title TEXT NOT NULL,
                created TEXT NOT NULL,
                modified TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'active',
                tags TEXT DEFAULT '[]',
                aliases TEXT DEFAULT '[]',
                project TEXT,
                task_id TEXT,
                context_type TEXT,
                auto_fetch INTEGER DEFAULT 0,
                category TEXT,
                source TEXT,
                course TEXT,
                confidence REAL,
                review_date TEXT,
                memory_type TEXT,
                context TEXT,
                agent TEXT,
                session_id TEXT,
                times_applied INTEGER DEFAULT 0,
                last_applied TEXT,
                content_hash TEXT
            );

            -- Indexes for common queries
            CREATE INDEX IF NOT EXISTS idx_notes_type ON notes(note_type);
            CREATE INDEX IF NOT EXISTS idx_notes_project ON notes(project);
            CREATE INDEX IF NOT EXISTS idx_notes_task ON notes(task_id);
            CREATE INDEX IF NOT EXISTS idx_notes_path ON notes(path);

            -- Chunk metadata table
            CREATE TABLE IF NOT EXISTS chunk_meta (
                id TEXT PRIMARY KEY,
                note_id TEXT NOT NULL,
                content TEXT NOT NULL,
                heading TEXT,
                chunk_index INTEGER NOT NULL,
                char_start INTEGER NOT NULL,
                char_end INTEGER NOT NULL,
                FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_chunk_note ON chunk_meta(note_id);

            -- Links table for wikilink graph
            CREATE TABLE IF NOT EXISTS links (
                source_id TEXT NOT NULL,
                target_id TEXT NOT NULL,
                target_path TEXT NOT NULL,
                link_text TEXT NOT NULL,
                heading TEXT,
                PRIMARY KEY (source_id, target_id, link_text),
                FOREIGN KEY (source_id) REFERENCES notes(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_links_target ON links(target_id);
            CREATE INDEX IF NOT EXISTS idx_links_source ON links(source_id);

            -- Memories table for agent memory
            CREATE TABLE IF NOT EXISTS memories (
                id TEXT PRIMARY KEY,
                memory_type TEXT NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                context TEXT NOT NULL,
                tags TEXT DEFAULT '[]',
                agent TEXT NOT NULL,
                session_id TEXT,
                project TEXT,
                confidence REAL NOT NULL,
                times_applied INTEGER DEFAULT 0,
                created TEXT NOT NULL,
                last_applied TEXT,
                note_id TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_memories_type ON memories(memory_type);
            CREATE INDEX IF NOT EXISTS idx_memories_context ON memories(context);
            CREATE INDEX IF NOT EXISTS idx_memories_confidence ON memories(confidence);
            "#,
        )?;

        // Create virtual table for vector search (note chunks)
        // Note: We create this separately as it uses special syntax
        conn.execute(
            &format!(
                "CREATE VIRTUAL TABLE IF NOT EXISTS chunks USING vec0(
                    id TEXT PRIMARY KEY,
                    note_id TEXT,
                    embedding FLOAT[{}]
                )",
                EMBEDDING_DIM
            ),
            [],
        )?;

        // Create virtual table for memory embeddings (separate from chunks)
        conn.execute(
            &format!(
                "CREATE VIRTUAL TABLE IF NOT EXISTS memory_embeddings USING vec0(
                    id TEXT PRIMARY KEY,
                    embedding FLOAT[{}]
                )",
                EMBEDDING_DIM
            ),
            [],
        )?;

        Ok(())
    }

    /// Helper to serialize tags/aliases to JSON
    fn to_json_array(items: &[String]) -> String {
        serde_json::to_string(items).unwrap_or_else(|_| "[]".to_string())
    }

    /// Helper to deserialize JSON array to Vec<String>
    fn from_json_array(json: &str) -> Vec<String> {
        serde_json::from_str(json).unwrap_or_default()
    }
}

impl Store for SqliteStore {
    fn save_note(&self, note: &Note) -> StoreResult<()> {
        let conn = self.lock_conn()?;

        conn.execute(
            r#"
            INSERT OR REPLACE INTO notes (
                id, path, note_type, title, created, modified, status,
                tags, aliases, project, task_id, context_type, auto_fetch,
                category, source, course, confidence, review_date,
                memory_type, context, agent, session_id, times_applied,
                last_applied, content_hash
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13,
                ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25
            )
            "#,
            params![
                note.id,
                note.path,
                note.note_type.to_string(),
                note.title,
                note.created.to_rfc3339(),
                note.modified.to_rfc3339(),
                format!("{:?}", note.status).to_lowercase(),
                Self::to_json_array(&note.tags),
                Self::to_json_array(&note.aliases),
                note.project,
                note.task_id,
                note.context_type,
                note.auto_fetch as i32,
                note.category,
                note.source,
                note.course,
                note.confidence,
                note.review_date.map(|d| d.to_rfc3339()),
                note.memory_type,
                note.context,
                note.agent,
                note.session_id,
                note.times_applied,
                note.last_applied.map(|d| d.to_rfc3339()),
                note.content_hash,
            ],
        )?;

        Ok(())
    }

    fn get_note(&self, id: &str) -> StoreResult<Option<Note>> {
        let conn = self.lock_conn()?;

        let result = conn
            .query_row(
                "SELECT * FROM notes WHERE id = ?1",
                params![id],
                Self::note_from_row,
            )
            .optional()?;

        Ok(result)
    }

    fn get_note_by_path(&self, path: &str) -> StoreResult<Option<Note>> {
        let conn = self.lock_conn()?;

        let result = conn
            .query_row(
                "SELECT * FROM notes WHERE path = ?1",
                params![path],
                Self::note_from_row,
            )
            .optional()?;

        Ok(result)
    }

    fn delete_note(&self, id: &str) -> StoreResult<()> {
        let conn = self.lock_conn()?;

        // Delete from vector table first
        conn.execute("DELETE FROM chunks WHERE note_id = ?1", params![id])?;

        // Then delete from notes (cascades to chunk_meta and links)
        conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;

        Ok(())
    }

    fn list_notes(
        &self,
        note_type: Option<NoteType>,
        project: Option<&str>,
        limit: usize,
    ) -> StoreResult<Vec<Note>> {
        let conn = self.lock_conn()?;

        let mut sql = String::from("SELECT * FROM notes WHERE 1=1");
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ref nt) = note_type {
            sql.push_str(" AND note_type = ?");
            params_vec.push(Box::new(nt.to_string()));
        }

        if let Some(p) = project {
            sql.push_str(" AND project = ?");
            params_vec.push(Box::new(p.to_string()));
        }

        sql.push_str(" ORDER BY modified DESC LIMIT ?");
        params_vec.push(Box::new(limit as i64));

        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn.prepare(&sql)?;
        let notes = stmt
            .query_map(params_refs.as_slice(), Self::note_from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(notes)
    }

    fn search_notes(&self, query: &str, limit: usize) -> StoreResult<Vec<Note>> {
        let conn = self.lock_conn()?;

        let pattern = format!("%{}%", query.to_lowercase());

        let mut stmt = conn.prepare(
            r#"
            SELECT * FROM notes
            WHERE LOWER(title) LIKE ?1 OR LOWER(path) LIKE ?1
            ORDER BY modified DESC
            LIMIT ?2
            "#,
        )?;

        let notes = stmt
            .query_map(params![pattern, limit as i64], |row| {
                Self::note_from_row(row)
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(notes)
    }

    fn get_notes_by_task(&self, task_id: &str) -> StoreResult<Vec<Note>> {
        let conn = self.lock_conn()?;

        let mut stmt = conn.prepare("SELECT * FROM notes WHERE task_id = ?1")?;
        let notes = stmt
            .query_map(params![task_id], Self::note_from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(notes)
    }

    fn get_notes_by_project(&self, project: &str) -> StoreResult<Vec<Note>> {
        let conn = self.lock_conn()?;

        let mut stmt = conn.prepare(
            "SELECT * FROM notes WHERE project = ?1 ORDER BY note_type, title",
        )?;
        let notes = stmt
            .query_map(params![project], Self::note_from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(notes)
    }

    fn save_chunks(&self, note_id: &str, chunks: &[Chunk]) -> StoreResult<()> {
        let conn = self.lock_conn()?;

        // Delete existing chunks for this note
        conn.execute("DELETE FROM chunks WHERE note_id = ?1", params![note_id])?;
        conn.execute(
            "DELETE FROM chunk_meta WHERE note_id = ?1",
            params![note_id],
        )?;

        // Insert new chunks
        for chunk in chunks {
            // Validate embedding dimension
            if let Some(ref emb) = chunk.embedding {
                if emb.len() != EMBEDDING_DIM {
                    return Err(StoreError::InvalidEmbeddingDimension {
                        expected: EMBEDDING_DIM,
                        got: emb.len(),
                    });
                }

                // Insert into vector table
                conn.execute(
                    "INSERT INTO chunks (id, note_id, embedding) VALUES (?1, ?2, ?3)",
                    params![chunk.id, note_id, emb.as_bytes()],
                )?;
            }

            // Insert metadata
            conn.execute(
                r#"
                INSERT INTO chunk_meta (
                    id, note_id, content, heading, chunk_index, char_start, char_end
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                "#,
                params![
                    chunk.id,
                    note_id,
                    chunk.content,
                    chunk.heading,
                    chunk.chunk_index,
                    chunk.char_start,
                    chunk.char_end,
                ],
            )?;
        }

        Ok(())
    }

    fn get_chunks(&self, note_id: &str) -> StoreResult<Vec<Chunk>> {
        let conn = self.lock_conn()?;

        let mut stmt = conn.prepare(
            r#"
            SELECT cm.id, cm.note_id, cm.content, cm.heading,
                   cm.chunk_index, cm.char_start, cm.char_end,
                   n.path
            FROM chunk_meta cm
            JOIN notes n ON cm.note_id = n.id
            WHERE cm.note_id = ?1
            ORDER BY cm.chunk_index
            "#,
        )?;

        let chunks = stmt
            .query_map(params![note_id], |row| {
                Ok(Chunk {
                    id: row.get(0)?,
                    note_id: row.get(1)?,
                    content: row.get(2)?,
                    heading: row.get(3)?,
                    chunk_index: row.get(4)?,
                    char_start: row.get(5)?,
                    char_end: row.get(6)?,
                    note_path: row.get(7)?,
                    embedding: None, // Don't load embedding by default
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(chunks)
    }

    fn delete_chunks(&self, note_id: &str) -> StoreResult<()> {
        let conn = self.lock_conn()?;

        conn.execute("DELETE FROM chunks WHERE note_id = ?1", params![note_id])?;
        conn.execute(
            "DELETE FROM chunk_meta WHERE note_id = ?1",
            params![note_id],
        )?;

        Ok(())
    }

    fn vector_search(
        &self,
        embedding: &[f32],
        note_types: Option<&[NoteType]>,
        project: Option<&str>,
        limit: usize,
    ) -> StoreResult<Vec<SearchResult>> {
        if embedding.len() != EMBEDDING_DIM {
            return Err(StoreError::InvalidEmbeddingDimension {
                expected: EMBEDDING_DIM,
                got: embedding.len(),
            });
        }

        let conn = self.lock_conn()?;

        // Build query with filters
        let mut sql = String::from(
            r#"
            SELECT
                c.id as chunk_id,
                c.note_id,
                n.path,
                n.title,
                n.note_type,
                n.project,
                cm.content,
                cm.heading,
                vec_distance_cosine(c.embedding, ?1) as distance
            FROM chunks c
            JOIN chunk_meta cm ON c.id = cm.id
            JOIN notes n ON c.note_id = n.id
            WHERE 1=1
            "#,
        );

        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        params_vec.push(Box::new(embedding.as_bytes().to_vec()));

        if let Some(types) = note_types {
            let type_list: Vec<String> = types.iter().map(|t| t.to_string()).collect();
            let placeholders = type_list.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            sql.push_str(&format!(" AND n.note_type IN ({})", placeholders));
            for t in type_list {
                params_vec.push(Box::new(t));
            }
        }

        if let Some(p) = project {
            sql.push_str(" AND n.project = ?");
            params_vec.push(Box::new(p.to_string()));
        }

        sql.push_str(" ORDER BY distance ASC LIMIT ?");
        params_vec.push(Box::new(limit as i64));

        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn.prepare(&sql)?;
        let results = stmt
            .query_map(params_refs.as_slice(), |row| {
                let note_type_str: String = row.get(4)?;
                let note_type = match note_type_str.as_str() {
                    "project" => NoteType::Project,
                    "task" => NoteType::Task,
                    "learning" => NoteType::Learning,
                    "memory" => NoteType::Memory,
                    _ => NoteType::Meta,
                };

                Ok(SearchResult {
                    chunk_id: row.get(0)?,
                    note_id: row.get(1)?,
                    note_path: row.get(2)?,
                    note_title: row.get(3)?,
                    note_type,
                    project: row.get(5)?,
                    content: row.get(6)?,
                    heading: row.get(7)?,
                    distance: row.get(8)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(results)
    }

    fn save_links(&self, source_id: &str, targets: &[LinkInfo]) -> StoreResult<()> {
        let conn = self.lock_conn()?;

        // Delete existing links from this source
        conn.execute(
            "DELETE FROM links WHERE source_id = ?1",
            params![source_id],
        )?;

        // Insert new links
        for link in targets {
            conn.execute(
                r#"
                INSERT INTO links (source_id, target_id, target_path, link_text, heading)
                VALUES (?1, ?2, ?3, ?4, ?5)
                "#,
                params![
                    source_id,
                    link.target_id,
                    link.target_path,
                    link.link_text,
                    link.heading,
                ],
            )?;
        }

        Ok(())
    }

    fn get_outlinks(&self, note_id: &str) -> StoreResult<Vec<LinkInfo>> {
        let conn = self.lock_conn()?;

        let mut stmt = conn.prepare(
            "SELECT target_id, target_path, link_text, heading FROM links WHERE source_id = ?1",
        )?;

        let links = stmt
            .query_map(params![note_id], |row| {
                Ok(LinkInfo {
                    target_id: row.get(0)?,
                    target_path: row.get(1)?,
                    link_text: row.get(2)?,
                    heading: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(links)
    }

    fn get_backlinks(&self, note_id: &str) -> StoreResult<Vec<LinkInfo>> {
        let conn = self.lock_conn()?;

        let mut stmt = conn.prepare(
            r#"
            SELECT l.source_id, n.path, l.link_text, l.heading
            FROM links l
            JOIN notes n ON l.source_id = n.id
            WHERE l.target_id = ?1
            "#,
        )?;

        let links = stmt
            .query_map(params![note_id], |row| {
                Ok(LinkInfo {
                    target_id: row.get(0)?,
                    target_path: row.get(1)?,
                    link_text: row.get(2)?,
                    heading: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(links)
    }

    fn save_memory(&self, memory: &Memory) -> StoreResult<()> {
        let conn = self.lock_conn()?;

        conn.execute(
            r#"
            INSERT OR REPLACE INTO memories (
                id, memory_type, title, content, context, tags, agent,
                session_id, project, confidence, times_applied, created,
                last_applied, note_id
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
            "#,
            params![
                memory.id,
                memory.memory_type.to_string(),
                memory.title,
                memory.content,
                memory.context,
                Self::to_json_array(&memory.tags),
                memory.agent,
                memory.session_id,
                memory.project,
                memory.confidence,
                memory.times_applied,
                memory.created.to_rfc3339(),
                memory.last_applied.map(|d| d.to_rfc3339()),
                memory.note_id,
            ],
        )?;

        Ok(())
    }

    fn get_memory(&self, id: &str) -> StoreResult<Option<Memory>> {
        let conn = self.lock_conn()?;

        let result = conn
            .query_row("SELECT * FROM memories WHERE id = ?1", params![id], |row| {
                Self::memory_from_row(row)
            })
            .optional()?;

        Ok(result)
    }

    fn search_memories(
        &self,
        context: &str,
        memory_type: Option<&str>,
        min_confidence: f32,
        limit: usize,
    ) -> StoreResult<Vec<Memory>> {
        let conn = self.lock_conn()?;

        let pattern = format!("%{}%", context.to_lowercase());

        let mut sql = String::from(
            r#"
            SELECT * FROM memories
            WHERE (LOWER(context) LIKE ?1 OR LOWER(tags) LIKE ?1)
            AND confidence >= ?2
            "#,
        );

        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        params_vec.push(Box::new(pattern));
        params_vec.push(Box::new(min_confidence));

        if let Some(mt) = memory_type {
            sql.push_str(" AND memory_type = ?");
            params_vec.push(Box::new(mt.to_string()));
        }

        sql.push_str(" ORDER BY confidence DESC, times_applied DESC LIMIT ?");
        params_vec.push(Box::new(limit as i64));

        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn.prepare(&sql)?;
        let memories = stmt
            .query_map(params_refs.as_slice(), Self::memory_from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(memories)
    }

    fn update_memory_confidence(&self, id: &str, confidence: f32) -> StoreResult<()> {
        let conn = self.lock_conn()?;

        conn.execute(
            "UPDATE memories SET confidence = ?1 WHERE id = ?2",
            params![confidence, id],
        )?;

        Ok(())
    }

    fn save_memory_embedding(&self, memory_id: &str, embedding: &[f32]) -> StoreResult<()> {
        if embedding.len() != EMBEDDING_DIM {
            return Err(StoreError::InvalidEmbeddingDimension {
                expected: EMBEDDING_DIM,
                got: embedding.len(),
            });
        }

        let conn = self.lock_conn()?;

        // Delete existing embedding if any (for upsert behavior)
        conn.execute(
            "DELETE FROM memory_embeddings WHERE id = ?1",
            params![memory_id],
        )?;

        // Insert new embedding
        conn.execute(
            "INSERT INTO memory_embeddings (id, embedding) VALUES (?1, ?2)",
            params![memory_id, embedding.as_bytes()],
        )?;

        Ok(())
    }

    fn search_memories_semantic(
        &self,
        embedding: &[f32],
        min_confidence: f32,
        limit: usize,
    ) -> StoreResult<Vec<(Memory, f32)>> {
        if embedding.len() != EMBEDDING_DIM {
            return Err(StoreError::InvalidEmbeddingDimension {
                expected: EMBEDDING_DIM,
                got: embedding.len(),
            });
        }

        let conn = self.lock_conn()?;

        let mut stmt = conn.prepare(
            r#"
            SELECT
                m.*,
                vec_distance_cosine(me.embedding, ?1) as distance
            FROM memory_embeddings me
            JOIN memories m ON me.id = m.id
            WHERE m.confidence >= ?2
            ORDER BY distance ASC
            LIMIT ?3
            "#,
        )?;

        let results = stmt
            .query_map(
                params![embedding.as_bytes(), min_confidence, limit as i64],
                |row| {
                    let memory = Self::memory_from_row(row)?;
                    let distance: f32 = row.get("distance")?;
                    Ok((memory, distance))
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(results)
    }

    fn get_memories_for_decay(&self) -> StoreResult<Vec<Memory>> {
        let conn = self.lock_conn()?;

        // Get all memories that could potentially decay
        // (have been around for more than 7 days or have been applied before)
        let mut stmt = conn.prepare(
            r#"
            SELECT * FROM memories
            WHERE confidence > 0
            ORDER BY confidence ASC
            "#,
        )?;

        let memories = stmt
            .query_map([], Self::memory_from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(memories)
    }

    fn get_memories_below_confidence(&self, threshold: f32) -> StoreResult<Vec<Memory>> {
        let conn = self.lock_conn()?;

        let mut stmt = conn.prepare(
            "SELECT * FROM memories WHERE confidence < ?1 ORDER BY confidence ASC",
        )?;

        let memories = stmt
            .query_map(params![threshold], Self::memory_from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(memories)
    }

    fn delete_memory(&self, id: &str) -> StoreResult<()> {
        let conn = self.lock_conn()?;

        // Delete embedding first
        conn.execute("DELETE FROM memory_embeddings WHERE id = ?1", params![id])?;

        // Delete memory
        conn.execute("DELETE FROM memories WHERE id = ?1", params![id])?;

        Ok(())
    }

    fn update_memory_reinforcement(
        &self,
        id: &str,
        confidence: f32,
        times_applied: u32,
    ) -> StoreResult<()> {
        let conn = self.lock_conn()?;

        conn.execute(
            r#"
            UPDATE memories
            SET confidence = ?1, times_applied = ?2, last_applied = ?3
            WHERE id = ?4
            "#,
            params![
                confidence,
                times_applied as i32,
                chrono::Utc::now().to_rfc3339(),
                id
            ],
        )?;

        Ok(())
    }

    fn get_stats(&self) -> StoreResult<VaultStats> {
        let conn = self.lock_conn()?;

        let total_notes: u64 =
            conn.query_row("SELECT COUNT(*) FROM notes", [], |row| row.get(0))?;

        let total_chunks: u64 =
            conn.query_row("SELECT COUNT(*) FROM chunk_meta", [], |row| row.get(0))?;

        let total_links: u64 =
            conn.query_row("SELECT COUNT(*) FROM links", [], |row| row.get(0))?;

        let total_memories: u64 =
            conn.query_row("SELECT COUNT(*) FROM memories", [], |row| row.get(0))?;

        // Notes by type
        let mut stmt = conn.prepare("SELECT note_type, COUNT(*) FROM notes GROUP BY note_type")?;
        let notes_by_type: HashMap<String, u64> = stmt
            .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, u64>(1)?)))?
            .filter_map(|r| r.ok())
            .collect();

        // Unique projects
        let mut stmt = conn.prepare("SELECT DISTINCT project FROM notes WHERE project IS NOT NULL")?;
        let projects: Vec<String> = stmt
            .query_map([], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(VaultStats {
            total_notes,
            notes_by_type,
            total_chunks,
            total_links,
            total_memories,
            projects,
        })
    }
}

impl SqliteStore {
    /// Helper to construct Note from database row
    fn note_from_row(row: &rusqlite::Row) -> rusqlite::Result<Note> {
        use chrono::DateTime;

        let note_type_str: String = row.get("note_type")?;
        let note_type = match note_type_str.as_str() {
            "project" => NoteType::Project,
            "task" => NoteType::Task,
            "learning" => NoteType::Learning,
            "memory" => NoteType::Memory,
            _ => NoteType::Meta,
        };

        let status_str: String = row.get("status")?;
        let status = match status_str.as_str() {
            "draft" => NoteStatus::Draft,
            "archived" => NoteStatus::Archived,
            _ => NoteStatus::Active,
        };

        let tags_json: String = row.get("tags")?;
        let aliases_json: String = row.get("aliases")?;

        let created_str: String = row.get("created")?;
        let modified_str: String = row.get("modified")?;

        let review_date_str: Option<String> = row.get("review_date")?;
        let last_applied_str: Option<String> = row.get("last_applied")?;

        Ok(Note {
            id: row.get("id")?,
            path: row.get("path")?,
            note_type,
            title: row.get("title")?,
            created: DateTime::parse_from_rfc3339(&created_str)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            modified: DateTime::parse_from_rfc3339(&modified_str)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            status,
            tags: Self::from_json_array(&tags_json),
            aliases: Self::from_json_array(&aliases_json),
            project: row.get("project")?,
            task_id: row.get("task_id")?,
            context_type: row.get("context_type")?,
            auto_fetch: row.get::<_, i32>("auto_fetch")? != 0,
            category: row.get("category")?,
            source: row.get("source")?,
            course: row.get("course")?,
            confidence: row.get("confidence")?,
            review_date: review_date_str.and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .map(|d| d.with_timezone(&chrono::Utc))
                    .ok()
            }),
            memory_type: row.get("memory_type")?,
            context: row.get("context")?,
            agent: row.get("agent")?,
            session_id: row.get("session_id")?,
            times_applied: row.get::<_, i32>("times_applied")? as u32,
            last_applied: last_applied_str.and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .map(|d| d.with_timezone(&chrono::Utc))
                    .ok()
            }),
            content: String::new(), // Content is loaded from file, not DB
            content_hash: row.get("content_hash")?,
        })
    }

    /// Helper to construct Memory from database row
    fn memory_from_row(row: &rusqlite::Row) -> rusqlite::Result<Memory> {
        use chrono::DateTime;

        let memory_type_str: String = row.get("memory_type")?;
        let memory_type = match memory_type_str.as_str() {
            "decision" => MemoryType::Decision,
            "lesson" => MemoryType::Lesson,
            "preference" => MemoryType::Preference,
            "session" => MemoryType::Session,
            _ => MemoryType::Lesson,
        };

        let tags_json: String = row.get("tags")?;
        let created_str: String = row.get("created")?;
        let last_applied_str: Option<String> = row.get("last_applied")?;

        Ok(Memory {
            id: row.get("id")?,
            memory_type,
            title: row.get("title")?,
            content: row.get("content")?,
            context: row.get("context")?,
            tags: SqliteStore::from_json_array(&tags_json),
            agent: row.get("agent")?,
            session_id: row.get("session_id")?,
            project: row.get("project")?,
            confidence: row.get("confidence")?,
            times_applied: row.get::<_, i32>("times_applied")? as u32,
            created: DateTime::parse_from_rfc3339(&created_str)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            last_applied: last_applied_str.and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .map(|d| d.with_timezone(&chrono::Utc))
                    .ok()
            }),
            note_id: row.get("note_id")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_creation() {
        let store = SqliteStore::in_memory();
        assert!(store.is_ok());
    }

    #[test]
    fn test_note_crud() {
        let store = SqliteStore::in_memory().unwrap();

        let note = Note::project_note("Test Note", "test-project");
        store.save_note(&note).unwrap();

        let retrieved = store.get_note(&note.id).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().title, "Test Note");

        store.delete_note(&note.id).unwrap();
        assert!(store.get_note(&note.id).unwrap().is_none());
    }

    #[test]
    fn test_memory_operations() {
        let store = SqliteStore::in_memory().unwrap();

        let memory = Memory::lesson(
            "Test Lesson",
            "Always test your code",
            "testing",
            "vulcan-build",
        );

        store.save_memory(&memory).unwrap();

        let retrieved = store.get_memory(&memory.id).unwrap();
        assert!(retrieved.is_some());

        let results = store
            .search_memories("testing", None, 0.5, 10)
            .unwrap();
        assert!(!results.is_empty());
    }
}
