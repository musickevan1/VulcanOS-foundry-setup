//! Note model - represents an Obsidian markdown note with YAML frontmatter

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Type of note in the vault
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum NoteType {
    /// Project documentation (architecture, conventions, decisions)
    Project,
    /// Task-linked context notes
    Task,
    /// Learning materials (courses, topics, reading notes)
    Learning,
    /// Agent memories (decisions, lessons, preferences, sessions)
    Memory,
    /// Cross-cutting metadata (tags, glossary)
    #[default]
    Meta,
    /// Product Requirements Prompt (structured implementation spec)
    Prp,
    /// Context checkpoint (conversation state snapshot)
    Checkpoint,
}


impl std::fmt::Display for NoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteType::Project => write!(f, "project"),
            NoteType::Task => write!(f, "task"),
            NoteType::Learning => write!(f, "learning"),
            NoteType::Memory => write!(f, "memory"),
            NoteType::Meta => write!(f, "meta"),
            NoteType::Prp => write!(f, "prp"),
            NoteType::Checkpoint => write!(f, "checkpoint"),
        }
    }
}

/// Status of a note
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum NoteStatus {
    #[default]
    Active,
    Draft,
    Archived,
}

/// Status of a PRP implementation phase
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PhaseStatus {
    #[default]
    Pending,
    InProgress,
    Completed,
    Blocked,
}

/// Implementation phase for PRP notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrpPhase {
    /// Phase name/title
    pub name: String,
    /// Phase description
    pub description: String,
    /// Current status
    #[serde(default)]
    pub status: PhaseStatus,
    /// Linked task ID (if spawned as vulcan-todo task)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// Estimated effort (e.g., "small", "medium", "large")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
}

impl PrpPhase {
    /// Create a new phase
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            status: PhaseStatus::Pending,
            task_id: None,
            effort: None,
        }
    }
}

/// A note in the vault with YAML frontmatter metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    /// Unique identifier (UUID v4)
    pub id: String,

    /// Relative path within vault (e.g., "Projects/vulcanos/architecture.md")
    pub path: String,

    /// Note type (determines which zone it belongs to)
    #[serde(rename = "type")]
    pub note_type: NoteType,

    /// Human-readable title
    pub title: String,

    /// When the note was created
    pub created: DateTime<Utc>,

    /// When the note was last modified
    pub modified: DateTime<Utc>,

    /// Tags for organization and filtering
    #[serde(default)]
    pub tags: Vec<String>,

    /// Alternate names for Obsidian search
    #[serde(default)]
    pub aliases: Vec<String>,

    /// Note status
    #[serde(default)]
    pub status: NoteStatus,

    // === Type-specific fields ===
    /// Project identifier (for project/task notes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,

    /// Linked vulcan-todo task ID (for task notes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,

    /// Context type for task notes (implementation, research, blockers, notes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_type: Option<String>,

    /// Auto-fetch this context when task starts
    #[serde(default)]
    pub auto_fetch: bool,

    /// Learning category (course, topic, reading, lecture)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Source URL for learning notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Course identifier for learning notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course: Option<String>,

    /// Self-assessed confidence (0.0-1.0) for learning/memory notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,

    /// Due date for spaced repetition review
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_date: Option<DateTime<Utc>>,

    /// Memory type for agent memories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_type: Option<String>,

    /// Context in which memory applies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    /// Which agent recorded this memory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,

    /// Session ID when memory was recorded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    /// Times this memory was successfully applied
    #[serde(default)]
    pub times_applied: u32,

    /// Last time this memory was applied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_applied: Option<DateTime<Utc>>,

    // === PRP-specific fields ===
    /// Why are we building this? (PRP notes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prp_value: Option<String>,

    /// What exactly are we building? (PRP notes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prp_scope: Option<String>,

    /// How do we measure success? (PRP notes)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub success_criteria: Vec<String>,

    /// Implementation phases with status (PRP notes)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub implementation_phases: Vec<PrpPhase>,

    /// Linked task IDs for this PRP (PRP notes)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_tasks: Vec<String>,

    // === Checkpoint-specific fields ===
    /// Checkpoint name/label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_name: Option<String>,

    /// Session context at checkpoint time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_context: Option<String>,

    /// Active task IDs at checkpoint
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checkpoint_tasks: Vec<String>,

    /// Parent checkpoint ID (for branching)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_checkpoint: Option<String>,

    // === Content ===
    /// The markdown content (body after frontmatter)
    #[serde(skip)]
    pub content: String,

    /// Hash of content for change detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
}

impl Note {
    /// Create a new note with minimal required fields
    pub fn new(title: impl Into<String>, note_type: NoteType, path: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            path: path.into(),
            note_type,
            title: title.into(),
            created: now,
            modified: now,
            tags: Vec::new(),
            aliases: Vec::new(),
            status: NoteStatus::Active,
            project: None,
            task_id: None,
            context_type: None,
            auto_fetch: false,
            category: None,
            source: None,
            course: None,
            confidence: None,
            review_date: None,
            memory_type: None,
            context: None,
            agent: None,
            session_id: None,
            times_applied: 0,
            last_applied: None,
            // PRP fields
            prp_value: None,
            prp_scope: None,
            success_criteria: Vec::new(),
            implementation_phases: Vec::new(),
            linked_tasks: Vec::new(),
            // Checkpoint fields
            checkpoint_name: None,
            checkpoint_context: None,
            checkpoint_tasks: Vec::new(),
            parent_checkpoint: None,
            content: String::new(),
            content_hash: None,
        }
    }

    /// Create a project context note
    pub fn project_note(title: impl Into<String>, project: impl Into<String>) -> Self {
        let project = project.into();
        let title = title.into();
        let path = format!("Projects/{}/{}.md", project, slug(&title));
        let mut note = Self::new(&title, NoteType::Project, path);
        note.project = Some(project);
        note
    }

    /// Create a task context note linked to a vulcan-todo task
    pub fn task_note(title: impl Into<String>, task_id: impl Into<String>) -> Self {
        let task_id = task_id.into();
        let path = format!("Tasks/by-id/{}.md", &task_id[..8]);
        let mut note = Self::new(title, NoteType::Task, path);
        note.task_id = Some(task_id);
        note.auto_fetch = true;
        note
    }

    /// Create a learning note
    pub fn learning_note(title: impl Into<String>, category: impl Into<String>) -> Self {
        let title = title.into();
        let category = category.into();
        let path = format!("Learning/{}/{}.md", &category, slug(&title));
        let mut note = Self::new(&title, NoteType::Learning, path);
        note.category = Some(category);
        note.confidence = Some(0.5);
        note
    }

    /// Create an agent memory note
    pub fn memory_note(title: impl Into<String>, memory_type: impl Into<String>) -> Self {
        let title = title.into();
        let memory_type = memory_type.into();
        let path = format!("Agent-Memories/{}/{}.md", &memory_type, slug(&title));
        let mut note = Self::new(&title, NoteType::Memory, path);
        note.memory_type = Some(memory_type);
        note.confidence = Some(0.8);
        note
    }

    /// Create a PRP (Product Requirements Prompt) note
    ///
    /// PRPs are structured implementation specs combining:
    /// - Value: Why are we building this?
    /// - Scope: What exactly are we building?
    /// - Success criteria: How do we measure success?
    /// - Implementation phases: Ordered steps with status
    pub fn prp_note(
        title: impl Into<String>,
        project: impl Into<String>,
        value: impl Into<String>,
        scope: impl Into<String>,
    ) -> Self {
        let title = title.into();
        let project = project.into();
        let path = format!("PRPs/{}/{}.md", &project, slug(&title));
        let mut note = Self::new(&title, NoteType::Prp, path);
        note.project = Some(project);
        note.prp_value = Some(value.into());
        note.prp_scope = Some(scope.into());
        note
    }

    /// Create a context checkpoint note
    ///
    /// Checkpoints capture conversation state for later restoration:
    /// - Session context summary
    /// - Active task IDs
    /// - Parent checkpoint (for branching)
    pub fn checkpoint_note(
        name: impl Into<String>,
        session_id: impl Into<String>,
        context_summary: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let session_id = session_id.into();
        let path = format!("Checkpoints/{}/{}.md", &session_id[..8.min(session_id.len())], slug(&name));
        let mut note = Self::new(&name, NoteType::Checkpoint, path);
        note.session_id = Some(session_id);
        note.checkpoint_name = Some(name);
        note.checkpoint_context = Some(context_summary.into());
        note
    }

    /// Check if this note matches given filters
    pub fn matches(&self, note_type: Option<&NoteType>, project: Option<&str>, tag: Option<&str>) -> bool {
        if let Some(nt) = note_type {
            if &self.note_type != nt {
                return false;
            }
        }
        if let Some(p) = project {
            if self.project.as_deref() != Some(p) {
                return false;
            }
        }
        if let Some(t) = tag {
            if !self.tags.iter().any(|x| x == t) {
                return false;
            }
        }
        true
    }

    /// Generate YAML frontmatter string
    pub fn to_frontmatter(&self) -> String {
        // Create a serializable version without content
        let yaml = serde_yaml::to_string(self).unwrap_or_default();
        format!("---\n{}---\n", yaml)
    }

    /// Generate full markdown file content
    pub fn to_markdown(&self) -> String {
        format!("{}\n{}", self.to_frontmatter(), self.content)
    }
}

/// Convert title to URL-friendly slug
fn slug(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_creation() {
        let note = Note::project_note("VulcanOS Architecture", "vulcanos");
        assert_eq!(note.note_type, NoteType::Project);
        assert_eq!(note.project, Some("vulcanos".to_string()));
        assert!(note.path.starts_with("Projects/vulcanos/"));
    }

    #[test]
    fn test_task_note() {
        let note = Note::task_note("Implementation Plan", "a1b2c3d4-e5f6-7890-abcd-ef1234567890");
        assert_eq!(note.note_type, NoteType::Task);
        assert!(note.task_id.is_some());
        assert!(note.auto_fetch);
    }

    #[test]
    fn test_slug() {
        assert_eq!(slug("Hello World!"), "hello-world");
        assert_eq!(slug("VulcanOS Architecture"), "vulcanos-architecture");
    }

    #[test]
    fn test_prp_note() {
        let note = Note::prp_note(
            "Add PRP Support",
            "vulcan-vault",
            "Enable structured implementation specs for better AI alignment",
            "Add PrpPhase struct, NoteType::Prp variant, and prp_note constructor",
        );
        assert_eq!(note.note_type, NoteType::Prp);
        assert_eq!(note.project, Some("vulcan-vault".to_string()));
        assert!(note.prp_value.is_some());
        assert!(note.prp_scope.is_some());
        assert!(note.path.starts_with("PRPs/vulcan-vault/"));
    }

    #[test]
    fn test_prp_phase() {
        let mut phase = PrpPhase::new("Design", "Create the data model");
        assert_eq!(phase.status, PhaseStatus::Pending);
        phase.status = PhaseStatus::InProgress;
        assert_eq!(phase.status, PhaseStatus::InProgress);
    }

    #[test]
    fn test_checkpoint_note() {
        let note = Note::checkpoint_note(
            "before-refactor",
            "session-12345678-abcd",
            "Working on PRP implementation, added NoteType variants",
        );
        assert_eq!(note.note_type, NoteType::Checkpoint);
        assert_eq!(note.checkpoint_name, Some("before-refactor".to_string()));
        assert!(note.checkpoint_context.is_some());
        assert!(note.path.starts_with("Checkpoints/"));
        assert!(note.path.contains("before-refactor"));
    }
}
