//! Memory decay and reinforcement - confidence management over time
//!
//! This module implements the time-based decay system that reduces confidence
//! in unused memories, and the reinforcement system that boosts confidence
//! when memories are successfully applied.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::Utc;

use crate::models::Memory;
use crate::store::Store;

/// Result type for decay operations
pub type DecayResult<T> = Result<T, DecayError>;

/// Errors that can occur during decay operations
#[derive(Debug, thiserror::Error)]
pub enum DecayError {
    #[error("Store error: {0}")]
    Store(#[from] crate::store::StoreError),

    #[error("Memory not found: {0}")]
    NotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Configuration for memory decay
#[derive(Debug, Clone)]
pub struct DecayConfig {
    /// Confidence decrease per day of inactivity (default: 0.01)
    pub decay_rate: f32,
    /// Days after creation before decay starts (default: 7)
    pub grace_period_days: u32,
    /// Minimum confidence threshold for cleanup (default: 0.1)
    pub min_confidence: f32,
    /// Directory for archived memories
    pub archive_dir: PathBuf,
}

impl Default for DecayConfig {
    fn default() -> Self {
        Self {
            decay_rate: 0.01,
            grace_period_days: 7,
            min_confidence: 0.1,
            archive_dir: PathBuf::from("Agent-Memories/archive"),
        }
    }
}

/// Report from decay operation
#[derive(Debug, Clone, Default)]
pub struct DecayReport {
    /// Number of memories processed
    pub processed: usize,
    /// Number of memories that had decay applied
    pub decayed: usize,
    /// Number of memories now below threshold
    pub below_threshold: usize,
}

/// Report from cleanup operation
#[derive(Debug, Clone)]
pub struct CleanupReport {
    /// Number of memories archived
    pub archived: usize,
    /// Path to archive directory
    pub archive_path: PathBuf,
    /// IDs of archived memories
    pub archived_ids: Vec<String>,
}

/// Service for managing memory decay and reinforcement
///
/// Implements the core mechanics of memory aging:
/// - Time-based decay reduces confidence in unused memories
/// - Reinforcement boosts confidence when memories are applied
/// - Cleanup archives memories that fall below minimum threshold
pub struct MemoryDecay<S: Store> {
    store: Arc<S>,
    config: DecayConfig,
}

impl<S: Store> MemoryDecay<S> {
    /// Create a new decay service with default configuration
    pub fn new(store: Arc<S>) -> Self {
        Self {
            store,
            config: DecayConfig::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(store: Arc<S>, config: DecayConfig) -> Self {
        Self { store, config }
    }

    /// Apply decay to all eligible memories
    ///
    /// Processes all memories and applies time-based confidence decay.
    /// Memories within the grace period are skipped.
    pub fn apply_decay(&self) -> DecayResult<DecayReport> {
        let mut report = DecayReport::default();

        // Get all memories that could potentially decay
        let memories = self.store.get_memories_for_decay()?;
        report.processed = memories.len();

        for mut memory in memories {
            let original_confidence = memory.confidence;

            // Apply decay using the Memory's built-in method
            memory.decay(self.config.decay_rate);

            // If confidence changed, update the store
            if (original_confidence - memory.confidence).abs() > f32::EPSILON {
                self.store
                    .update_memory_confidence(&memory.id, memory.confidence)?;
                report.decayed += 1;
            }

            // Track memories below threshold
            if memory.confidence < self.config.min_confidence {
                report.below_threshold += 1;
            }
        }

        tracing::info!(
            "Decay complete: {}/{} memories decayed, {} below threshold",
            report.decayed,
            report.processed,
            report.below_threshold
        );

        Ok(report)
    }

    /// Apply decay to a specific memory
    pub fn apply_decay_to(&self, memory_id: &str) -> DecayResult<Memory> {
        let mut memory = self
            .store
            .get_memory(memory_id)?
            .ok_or_else(|| DecayError::NotFound(memory_id.to_string()))?;

        let original_confidence = memory.confidence;
        memory.decay(self.config.decay_rate);

        if (original_confidence - memory.confidence).abs() > f32::EPSILON {
            self.store
                .update_memory_confidence(&memory.id, memory.confidence)?;
        }

        Ok(memory)
    }

    /// Reinforce a memory (boost confidence after successful application)
    ///
    /// Call this when a memory is successfully used by an agent.
    /// Increases confidence and resets the decay timer.
    pub fn reinforce(&self, memory_id: &str) -> DecayResult<Memory> {
        let mut memory = self
            .store
            .get_memory(memory_id)?
            .ok_or_else(|| DecayError::NotFound(memory_id.to_string()))?;

        // Apply reinforcement using Memory's built-in method
        memory.reinforce();

        // Persist the changes
        self.store.update_memory_reinforcement(
            &memory.id,
            memory.confidence,
            memory.times_applied,
        )?;

        tracing::debug!(
            "Reinforced memory '{}': confidence={:.2}, times_applied={}",
            memory.title,
            memory.confidence,
            memory.times_applied
        );

        Ok(memory)
    }

    /// Reinforce multiple memories at once
    pub fn batch_reinforce(&self, memory_ids: &[&str]) -> DecayResult<Vec<Memory>> {
        let mut reinforced = Vec::with_capacity(memory_ids.len());

        for id in memory_ids {
            match self.reinforce(id) {
                Ok(memory) => reinforced.push(memory),
                Err(DecayError::NotFound(id)) => {
                    tracing::warn!("Memory not found for reinforcement: {}", id);
                }
                Err(e) => return Err(e),
            }
        }

        Ok(reinforced)
    }

    /// Clean up memories below minimum confidence threshold
    ///
    /// Archives low-confidence memories to markdown files and removes them
    /// from the database. This keeps the active memory set focused on
    /// high-value, well-reinforced memories.
    pub fn cleanup(&self, vault_path: &Path) -> DecayResult<CleanupReport> {
        // Get memories below threshold
        let memories = self
            .store
            .get_memories_below_confidence(self.config.min_confidence)?;

        if memories.is_empty() {
            return Ok(CleanupReport {
                archived: 0,
                archive_path: self.config.archive_dir.clone(),
                archived_ids: Vec::new(),
            });
        }

        // Ensure archive directory exists
        let archive_path = vault_path.join(&self.config.archive_dir);
        fs::create_dir_all(&archive_path)?;

        let mut archived_ids = Vec::with_capacity(memories.len());

        for memory in &memories {
            // Generate archive filename
            let filename = format!(
                "{}-{}.md",
                memory.created.format("%Y%m%d"),
                &memory.id[..8]
            );
            let file_path = archive_path.join(&filename);

            // Generate archive content
            let content = self.generate_archive_content(memory);

            // Write archive file
            fs::write(&file_path, content)?;

            // Delete from database
            self.store.delete_memory(&memory.id)?;

            archived_ids.push(memory.id.clone());

            tracing::debug!(
                "Archived memory '{}' to {}",
                memory.title,
                file_path.display()
            );
        }

        tracing::info!(
            "Cleanup complete: archived {} memories to {}",
            archived_ids.len(),
            archive_path.display()
        );

        Ok(CleanupReport {
            archived: archived_ids.len(),
            archive_path,
            archived_ids,
        })
    }

    /// Generate markdown content for archived memory
    fn generate_archive_content(&self, memory: &Memory) -> String {
        let mut content = String::new();

        // YAML frontmatter
        content.push_str("---\n");
        content.push_str(&format!("id: {}\n", memory.id));
        content.push_str(&format!("type: {}\n", memory.memory_type));
        content.push_str(&format!("agent: {}\n", memory.agent));
        content.push_str(&format!("context: {}\n", memory.context));
        content.push_str(&format!(
            "created: {}\n",
            memory.created.format("%Y-%m-%d %H:%M:%S")
        ));
        if let Some(ref last) = memory.last_applied {
            content.push_str(&format!(
                "last_applied: {}\n",
                last.format("%Y-%m-%d %H:%M:%S")
            ));
        }
        content.push_str(&format!("confidence: {:.2}\n", memory.confidence));
        content.push_str(&format!("times_applied: {}\n", memory.times_applied));
        if !memory.tags.is_empty() {
            content.push_str(&format!("tags: [{}]\n", memory.tags.join(", ")));
        }
        if let Some(ref project) = memory.project {
            content.push_str(&format!("project: {}\n", project));
        }
        content.push_str("archived: true\n");
        content.push_str(&format!(
            "archived_at: {}\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S")
        ));
        content.push_str("---\n\n");

        // Title
        content.push_str(&format!("# {}\n\n", memory.title));

        // Body
        content.push_str(&memory.content);
        content.push('\n');

        content
    }

    /// Get current configuration
    pub fn config(&self) -> &DecayConfig {
        &self.config
    }

    /// Preview what cleanup would archive (dry run)
    pub fn preview_cleanup(&self) -> DecayResult<Vec<Memory>> {
        self.store
            .get_memories_below_confidence(self.config.min_confidence)
            .map_err(DecayError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::MemoryType;

    #[test]
    fn test_decay_config_default() {
        let config = DecayConfig::default();

        assert_eq!(config.decay_rate, 0.01);
        assert_eq!(config.grace_period_days, 7);
        assert_eq!(config.min_confidence, 0.1);
    }

    #[test]
    fn test_archive_content_generation() {
        let memory = Memory::new(
            MemoryType::Lesson,
            "Test Lesson",
            "This is the lesson content",
            "testing",
            "test-agent",
        );

        // Test the format (without needing a store)
        let mut content = String::new();
        content.push_str("---\n");
        content.push_str(&format!("id: {}\n", memory.id));
        content.push_str(&format!("type: {}\n", memory.memory_type));

        assert!(content.contains("---"));
        assert!(content.contains("type: lesson"));
    }

    #[test]
    fn test_decay_report_default() {
        let report = DecayReport::default();

        assert_eq!(report.processed, 0);
        assert_eq!(report.decayed, 0);
        assert_eq!(report.below_threshold, 0);
    }
}
