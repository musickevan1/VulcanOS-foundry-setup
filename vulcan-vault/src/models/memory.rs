//! Memory model - represents agent memories (decisions, lessons, preferences)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Type of agent memory
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum MemoryType {
    /// A decision made by an agent with context and outcome
    Decision,
    /// A lesson learned from experience (errors, corrections)
    #[default]
    Lesson,
    /// An observed user preference
    Preference,
    /// A session summary
    Session,
}


impl std::fmt::Display for MemoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryType::Decision => write!(f, "decision"),
            MemoryType::Lesson => write!(f, "lesson"),
            MemoryType::Preference => write!(f, "preference"),
            MemoryType::Session => write!(f, "session"),
        }
    }
}

impl std::str::FromStr for MemoryType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "decision" => Ok(MemoryType::Decision),
            "lesson" => Ok(MemoryType::Lesson),
            "preference" => Ok(MemoryType::Preference),
            "session" => Ok(MemoryType::Session),
            _ => Err(anyhow::anyhow!("Unknown memory type: {}", s)),
        }
    }
}

/// An agent memory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// Unique identifier
    pub id: String,

    /// Type of memory
    pub memory_type: MemoryType,

    /// Short title/summary
    pub title: String,

    /// Full content/description
    pub content: String,

    /// Context in which this memory applies
    /// (e.g., "code-review", "error-handling", "rust")
    pub context: String,

    /// Tags for filtering
    #[serde(default)]
    pub tags: Vec<String>,

    /// Which agent recorded this memory
    pub agent: String,

    /// Session ID when recorded (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    /// Related project (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,

    /// Confidence level (0.0-1.0) - decays over time if not reinforced
    pub confidence: f32,

    /// How many times this memory was successfully applied
    pub times_applied: u32,

    /// When this memory was created
    pub created: DateTime<Utc>,

    /// Last time this memory was applied/reinforced
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_applied: Option<DateTime<Utc>>,

    /// Related note ID in vault (for full context)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_id: Option<String>,
}

impl Memory {
    /// Create a new memory
    pub fn new(
        memory_type: MemoryType,
        title: impl Into<String>,
        content: impl Into<String>,
        context: impl Into<String>,
        agent: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            memory_type,
            title: title.into(),
            content: content.into(),
            context: context.into(),
            tags: Vec::new(),
            agent: agent.into(),
            session_id: None,
            project: None,
            confidence: 0.8, // Start with high confidence
            times_applied: 0,
            created: Utc::now(),
            last_applied: None,
            note_id: None,
        }
    }

    /// Create a decision memory
    pub fn decision(
        title: impl Into<String>,
        content: impl Into<String>,
        context: impl Into<String>,
        agent: impl Into<String>,
    ) -> Self {
        Self::new(MemoryType::Decision, title, content, context, agent)
    }

    /// Create a lesson memory
    pub fn lesson(
        title: impl Into<String>,
        content: impl Into<String>,
        context: impl Into<String>,
        agent: impl Into<String>,
    ) -> Self {
        Self::new(MemoryType::Lesson, title, content, context, agent)
    }

    /// Create a preference memory
    pub fn preference(
        title: impl Into<String>,
        content: impl Into<String>,
        context: impl Into<String>,
        agent: impl Into<String>,
    ) -> Self {
        let mut mem = Self::new(MemoryType::Preference, title, content, context, agent);
        mem.confidence = 0.9; // Preferences start with higher confidence
        mem
    }

    /// Create a session summary memory
    pub fn session(
        title: impl Into<String>,
        content: impl Into<String>,
        session_id: impl Into<String>,
        agent: impl Into<String>,
    ) -> Self {
        let session_id = session_id.into();
        let mut mem = Self::new(
            MemoryType::Session,
            title,
            content,
            "session-summary".to_string(),
            agent,
        );
        mem.session_id = Some(session_id);
        mem.confidence = 1.0; // Sessions are factual records
        mem
    }

    /// Add tags to this memory
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Set the project context
    pub fn with_project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(project.into());
        self
    }

    /// Link to a vault note
    pub fn with_note(mut self, note_id: impl Into<String>) -> Self {
        self.note_id = Some(note_id.into());
        self
    }

    /// Reinforce this memory (increase confidence, update last_applied)
    pub fn reinforce(&mut self) {
        self.times_applied += 1;
        self.last_applied = Some(Utc::now());
        // Increase confidence, max 1.0
        self.confidence = (self.confidence + 0.1).min(1.0);
    }

    /// Apply confidence decay based on time since last use
    ///
    /// decay_rate: confidence decrease per day of inactivity
    pub fn decay(&mut self, decay_rate: f32) {
        if let Some(last) = self.last_applied {
            let days_inactive = (Utc::now() - last).num_days() as f32;
            if days_inactive > 0.0 {
                self.confidence = (self.confidence - decay_rate * days_inactive).max(0.0);
            }
        } else {
            // Never applied - decay from creation date
            let days_since_creation = (Utc::now() - self.created).num_days() as f32;
            if days_since_creation > 7.0 {
                // Grace period of 7 days
                let decay_days = days_since_creation - 7.0;
                self.confidence = (self.confidence - decay_rate * decay_days).max(0.0);
            }
        }
    }

    /// Check if this memory is still relevant (above minimum confidence)
    pub fn is_relevant(&self, min_confidence: f32) -> bool {
        self.confidence >= min_confidence
    }

    /// Check if this memory matches given context
    pub fn matches_context(&self, query_context: &str) -> bool {
        // Simple substring match - could be enhanced with semantic matching
        self.context.to_lowercase().contains(&query_context.to_lowercase())
            || query_context.to_lowercase().contains(&self.context.to_lowercase())
            || self.tags.iter().any(|t| {
                t.to_lowercase().contains(&query_context.to_lowercase())
                    || query_context.to_lowercase().contains(&t.to_lowercase())
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_creation() {
        let mem = Memory::lesson(
            "Use Result<T,E>",
            "Always use Result instead of unwrap in production",
            "error-handling",
            "vulcan-build",
        );

        assert_eq!(mem.memory_type, MemoryType::Lesson);
        assert_eq!(mem.confidence, 0.8);
        assert_eq!(mem.times_applied, 0);
    }

    #[test]
    fn test_memory_reinforce() {
        let mut mem = Memory::lesson("Test", "Content", "ctx", "agent");
        let original_confidence = mem.confidence;

        mem.reinforce();

        assert_eq!(mem.times_applied, 1);
        assert!(mem.confidence > original_confidence);
        assert!(mem.last_applied.is_some());
    }

    #[test]
    fn test_memory_context_match() {
        let mem = Memory::preference("Style", "Content", "code-review", "agent")
            .with_tags(vec!["rust".to_string(), "formatting".to_string()]);

        assert!(mem.matches_context("code-review"));
        assert!(mem.matches_context("rust"));
        assert!(!mem.matches_context("python"));
    }
}
