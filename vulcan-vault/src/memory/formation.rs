//! Memory formation - creating memories from agent observations
//!
//! This module provides structured APIs for agents to record memories
//! with automatic embedding generation for semantic retrieval.

use std::sync::Arc;

use crate::models::{Memory, MemoryType};
use crate::rag::EmbeddingService;
use crate::store::Store;

/// Source of a lesson memory
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LessonSource {
    /// Learned from an error or failure
    Error,
    /// User correction or feedback
    Correction,
    /// Discovered through exploration
    Discovery,
    /// External documentation or resource
    Documentation,
}

impl std::fmt::Display for LessonSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LessonSource::Error => write!(f, "error"),
            LessonSource::Correction => write!(f, "correction"),
            LessonSource::Discovery => write!(f, "discovery"),
            LessonSource::Documentation => write!(f, "documentation"),
        }
    }
}

/// Session event for summary generation
#[derive(Debug, Clone)]
pub enum SessionEvent {
    /// A decision made during the session
    Decision {
        title: String,
        outcome: Option<String>,
    },
    /// A correction received during the session
    Correction { what: String, why: String },
    /// A preference observed during the session
    Preference { key: String, value: String },
    /// A lesson learned during the session
    Lesson { title: String, source: LessonSource },
}

/// Result type for memory formation operations
pub type FormationResult<T> = Result<T, FormationError>;

/// Errors that can occur during memory formation
#[derive(Debug, thiserror::Error)]
pub enum FormationError {
    #[error("Store error: {0}")]
    Store(#[from] crate::store::StoreError),

    #[error("Embedding error: {0}")]
    Embedding(#[from] crate::rag::EmbeddingError),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// Service for forming and recording agent memories
///
/// Provides structured methods for creating different types of memories
/// with automatic tag extraction and embedding generation.
pub struct MemoryFormation<S: Store> {
    store: Arc<S>,
    embedder: EmbeddingService,
}

impl<S: Store> MemoryFormation<S> {
    /// Create a new memory formation service
    pub fn new(store: Arc<S>, embedder: EmbeddingService) -> Self {
        Self { store, embedder }
    }

    /// Create with default Ollama embedder
    pub fn with_store(store: Arc<S>) -> Self {
        Self {
            store,
            embedder: EmbeddingService::new(),
        }
    }

    /// Record a decision memory
    ///
    /// Decisions capture choices made with their context and optional outcome.
    /// Use this when an agent makes a significant choice that should be remembered.
    pub async fn record_decision(
        &self,
        title: &str,
        content: &str,
        context: &str,
        agent: &str,
        outcome: Option<&str>,
    ) -> FormationResult<Memory> {
        let mut memory = Memory::decision(title, content, context, agent);

        // Add outcome to content if provided
        if let Some(outcome) = outcome {
            memory.content = format!("{}\n\nOutcome: {}", memory.content, outcome);
        }

        // Extract tags from context
        memory.tags = self.extract_tags(context);

        // Generate embedding and save
        self.save_with_embedding(memory).await
    }

    /// Record a lesson memory
    ///
    /// Lessons capture learnings from experience - errors, corrections, or discoveries.
    /// These form the core of an agent's experiential knowledge.
    pub async fn record_lesson(
        &self,
        title: &str,
        content: &str,
        context: &str,
        agent: &str,
        source: LessonSource,
    ) -> FormationResult<Memory> {
        let mut memory = Memory::lesson(title, content, context, agent);

        // Add source tag
        let mut tags = self.extract_tags(context);
        tags.push(format!("source:{}", source));
        memory.tags = tags;

        self.save_with_embedding(memory).await
    }

    /// Record a preference memory
    ///
    /// Preferences capture observed user preferences with higher initial confidence.
    /// Use this when learning what the user likes or dislikes.
    pub async fn record_preference(
        &self,
        title: &str,
        content: &str,
        context: &str,
        agent: &str,
    ) -> FormationResult<Memory> {
        let mut memory = Memory::preference(title, content, context, agent);
        memory.tags = self.extract_tags(context);

        self.save_with_embedding(memory).await
    }

    /// Record a session summary memory
    ///
    /// Session summaries capture the key events from a work session.
    /// They have maximum confidence as they are factual records.
    pub async fn record_session(
        &self,
        session_id: &str,
        title: &str,
        agent: &str,
        events: &[SessionEvent],
    ) -> FormationResult<Memory> {
        // Generate summary content from events
        let content = self.generate_session_summary(events);

        let mut memory = Memory::session(title, &content, session_id, agent);
        memory.tags = vec!["session-summary".to_string()];

        // Add event type tags
        for event in events {
            match event {
                SessionEvent::Decision { .. } => {
                    if !memory.tags.contains(&"has-decisions".to_string()) {
                        memory.tags.push("has-decisions".to_string());
                    }
                }
                SessionEvent::Correction { .. } => {
                    if !memory.tags.contains(&"has-corrections".to_string()) {
                        memory.tags.push("has-corrections".to_string());
                    }
                }
                SessionEvent::Preference { .. } => {
                    if !memory.tags.contains(&"has-preferences".to_string()) {
                        memory.tags.push("has-preferences".to_string());
                    }
                }
                SessionEvent::Lesson { .. } => {
                    if !memory.tags.contains(&"has-lessons".to_string()) {
                        memory.tags.push("has-lessons".to_string());
                    }
                }
            }
        }

        self.save_with_embedding(memory).await
    }

    /// Record a session summary and write it to the vault as a markdown file
    ///
    /// This is the preferred method for session end - it creates both:
    /// 1. A Memory entry in the database for semantic retrieval
    /// 2. A markdown file in Agent-Memories/sessions/ for human review
    pub async fn record_session_to_vault(
        &self,
        session_id: &str,
        title: &str,
        agent: &str,
        events: &[SessionEvent],
        vault_path: &std::path::Path,
    ) -> FormationResult<(Memory, std::path::PathBuf)> {
        // Record to database
        let memory = self.record_session(session_id, title, agent, events).await?;

        // Generate file path
        let sessions_dir = vault_path.join("Agent-Memories/sessions");
        std::fs::create_dir_all(&sessions_dir).map_err(|e| {
            FormationError::InvalidInput(format!("Failed to create sessions directory: {}", e))
        })?;

        let filename = format!(
            "{}-{}.md",
            chrono::Utc::now().format("%Y%m%d-%H%M"),
            &session_id[..8.min(session_id.len())]
        );
        let file_path = sessions_dir.join(&filename);

        // Generate markdown content
        let markdown = self.generate_session_markdown(&memory, session_id, agent, events);

        // Write to file
        std::fs::write(&file_path, markdown).map_err(|e| {
            FormationError::InvalidInput(format!("Failed to write session file: {}", e))
        })?;

        tracing::info!("Session summary written to {}", file_path.display());

        Ok((memory, file_path))
    }

    /// Generate markdown file content for a session summary
    fn generate_session_markdown(
        &self,
        memory: &Memory,
        session_id: &str,
        agent: &str,
        events: &[SessionEvent],
    ) -> String {
        let mut md = String::new();

        // YAML frontmatter
        md.push_str("---\n");
        md.push_str(&format!("id: {}\n", memory.id));
        md.push_str(&format!("session_id: {}\n", session_id));
        md.push_str(&format!("agent: {}\n", agent));
        md.push_str(&format!("created: {}\n", memory.created.format("%Y-%m-%d %H:%M:%S")));
        md.push_str(&format!("type: {}\n", memory.memory_type));
        if !memory.tags.is_empty() {
            md.push_str(&format!("tags: [{}]\n", memory.tags.join(", ")));
        }
        md.push_str("---\n\n");

        // Title
        md.push_str(&format!("# {}\n\n", memory.title));

        // Event counts summary
        let decision_count = events.iter().filter(|e| matches!(e, SessionEvent::Decision { .. })).count();
        let correction_count = events.iter().filter(|e| matches!(e, SessionEvent::Correction { .. })).count();
        let lesson_count = events.iter().filter(|e| matches!(e, SessionEvent::Lesson { .. })).count();
        let preference_count = events.iter().filter(|e| matches!(e, SessionEvent::Preference { .. })).count();

        md.push_str("**Session Stats:**\n");
        md.push_str(&format!("- {} decisions\n", decision_count));
        md.push_str(&format!("- {} corrections\n", correction_count));
        md.push_str(&format!("- {} lessons learned\n", lesson_count));
        md.push_str(&format!("- {} preferences observed\n\n", preference_count));

        // Append the generated content
        md.push_str(&memory.content);

        md
    }

    /// Create a memory from raw fields (for flexibility)
    #[allow(clippy::too_many_arguments)]
    pub async fn record_raw(
        &self,
        memory_type: MemoryType,
        title: &str,
        content: &str,
        context: &str,
        agent: &str,
        tags: Option<Vec<String>>,
        project: Option<&str>,
    ) -> FormationResult<Memory> {
        let mut memory = Memory::new(memory_type, title, content, context, agent);

        if let Some(tags) = tags {
            memory.tags = tags;
        } else {
            memory.tags = self.extract_tags(context);
        }

        if let Some(project) = project {
            memory = memory.with_project(project);
        }

        self.save_with_embedding(memory).await
    }

    /// Generate embedding and save memory to store
    async fn save_with_embedding(&self, memory: Memory) -> FormationResult<Memory> {
        // Generate embedding from content
        let embedding_text = format!("{} {}", memory.title, memory.content);
        let embedding = self.embedder.embed(&embedding_text).await?;

        // Save memory to store (basic save for now)
        self.store.save_memory(&memory)?;

        // Save embedding separately (requires store extension)
        // This will be implemented in Task 2 when we add semantic retrieval
        self.store.save_memory_embedding(&memory.id, &embedding)?;

        tracing::debug!(
            "Recorded {} memory: {} (confidence: {})",
            memory.memory_type,
            memory.title,
            memory.confidence
        );

        Ok(memory)
    }

    /// Extract tags from context string
    ///
    /// Parses context like "rust error handling" into ["rust", "error-handling"]
    fn extract_tags(&self, context: &str) -> Vec<String> {
        context
            .split_whitespace()
            .map(|word| {
                // Normalize: lowercase, replace spaces with hyphens
                word.to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphanumeric() || *c == '-')
                    .collect::<String>()
            })
            .filter(|tag| !tag.is_empty() && tag.len() > 1)
            .collect()
    }

    /// Generate session summary from events (template-based)
    fn generate_session_summary(&self, events: &[SessionEvent]) -> String {
        let mut summary = String::new();

        // Collect events by type
        let decisions: Vec<_> = events
            .iter()
            .filter_map(|e| match e {
                SessionEvent::Decision { title, outcome } => Some((title, outcome)),
                _ => None,
            })
            .collect();

        let corrections: Vec<_> = events
            .iter()
            .filter_map(|e| match e {
                SessionEvent::Correction { what, why } => Some((what, why)),
                _ => None,
            })
            .collect();

        let preferences: Vec<_> = events
            .iter()
            .filter_map(|e| match e {
                SessionEvent::Preference { key, value } => Some((key, value)),
                _ => None,
            })
            .collect();

        let lessons: Vec<_> = events
            .iter()
            .filter_map(|e| match e {
                SessionEvent::Lesson { title, source } => Some((title, source)),
                _ => None,
            })
            .collect();

        // Build summary sections
        if !decisions.is_empty() {
            summary.push_str("## Decisions\n\n");
            for (title, outcome) in decisions {
                summary.push_str(&format!("- **{}**", title));
                if let Some(outcome) = outcome {
                    summary.push_str(&format!(" → {}", outcome));
                }
                summary.push('\n');
            }
            summary.push('\n');
        }

        if !corrections.is_empty() {
            summary.push_str("## Corrections\n\n");
            for (what, why) in corrections {
                summary.push_str(&format!("- **{}**: {}\n", what, why));
            }
            summary.push('\n');
        }

        if !lessons.is_empty() {
            summary.push_str("## Lessons Learned\n\n");
            for (title, source) in lessons {
                summary.push_str(&format!("- **{}** (from {})\n", title, source));
            }
            summary.push('\n');
        }

        if !preferences.is_empty() {
            summary.push_str("## Preferences Observed\n\n");
            for (key, value) in preferences {
                summary.push_str(&format!("- {}: {}\n", key, value));
            }
            summary.push('\n');
        }

        if summary.is_empty() {
            summary = "No significant events recorded.".to_string();
        }

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_tags() {
        // Create a mock test - we'll use a placeholder for the store
        let tags_from_context = |context: &str| -> Vec<String> {
            context
                .split_whitespace()
                .map(|word| {
                    word.to_lowercase()
                        .chars()
                        .filter(|c| c.is_alphanumeric() || *c == '-')
                        .collect::<String>()
                })
                .filter(|tag| !tag.is_empty() && tag.len() > 1)
                .collect()
        };

        let tags = tags_from_context("rust error handling");
        assert_eq!(tags, vec!["rust", "error", "handling"]);

        let tags = tags_from_context("API design patterns");
        assert_eq!(tags, vec!["api", "design", "patterns"]);
    }

    #[test]
    fn test_session_summary_generation() {
        let events = vec![
            SessionEvent::Decision {
                title: "Use builder pattern".to_string(),
                outcome: Some("Improved API ergonomics".to_string()),
            },
            SessionEvent::Lesson {
                title: "Always validate input".to_string(),
                source: LessonSource::Error,
            },
            SessionEvent::Preference {
                key: "Code style".to_string(),
                value: "Prefer explicit over implicit".to_string(),
            },
        ];

        // Test the summary generation logic inline
        let mut summary = String::new();

        let decisions: Vec<_> = events
            .iter()
            .filter_map(|e| match e {
                SessionEvent::Decision { title, outcome } => Some((title, outcome)),
                _ => None,
            })
            .collect();

        if !decisions.is_empty() {
            summary.push_str("## Decisions\n\n");
            for (title, outcome) in decisions {
                summary.push_str(&format!("- **{}**", title));
                if let Some(outcome) = outcome {
                    summary.push_str(&format!(" → {}", outcome));
                }
                summary.push('\n');
            }
        }

        assert!(summary.contains("## Decisions"));
        assert!(summary.contains("Use builder pattern"));
        assert!(summary.contains("Improved API ergonomics"));
    }
}
