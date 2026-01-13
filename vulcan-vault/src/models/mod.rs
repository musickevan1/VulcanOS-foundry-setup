//! Data models for vulcan-vault
//!
//! This module contains the core data structures:
//! - Note: Obsidian markdown notes with YAML frontmatter
//! - Chunk: Text chunks for vector embedding
//! - Memory: Agent memories (decisions, lessons, preferences)

mod note;
mod chunk;
mod memory;

pub use note::{Note, NoteType, NoteStatus};
pub use chunk::{Chunk, ChunkConfig};
pub use memory::{Memory, MemoryType};
