use serde::{Deserialize, Serialize};
use std::fmt;

/// Sprint status enum
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum SprintStatus {
    /// Sprint is being planned, collecting tasks
    #[default]
    Planning,
    /// Sprint is currently active/in progress
    Active,
    /// Sprint has been completed
    Completed,
    /// Sprint is archived (hidden from normal views)
    Archived,
}

impl fmt::Display for SprintStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SprintStatus::Planning => write!(f, "planning"),
            SprintStatus::Active => write!(f, "active"),
            SprintStatus::Completed => write!(f, "completed"),
            SprintStatus::Archived => write!(f, "archived"),
        }
    }
}

impl From<String> for SprintStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "active" | "in_progress" | "in-progress" | "started" => SprintStatus::Active,
            "completed" | "complete" | "done" | "finished" => SprintStatus::Completed,
            "archived" | "archive" => SprintStatus::Archived,
            _ => SprintStatus::Planning,
        }
    }
}

impl SprintStatus {
    /// Get emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            SprintStatus::Planning => "📋",
            SprintStatus::Active => "🏃",
            SprintStatus::Completed => "✅",
            SprintStatus::Archived => "📦",
        }
    }

    /// Get short label
    pub fn label(&self) -> &'static str {
        match self {
            SprintStatus::Planning => "PLANNING",
            SprintStatus::Active => "ACTIVE",
            SprintStatus::Completed => "COMPLETED",
            SprintStatus::Archived => "ARCHIVED",
        }
    }

    /// Check if sprint is active (can have tasks worked on)
    pub fn is_active(&self) -> bool {
        matches!(self, SprintStatus::Planning | SprintStatus::Active)
    }

    /// Check if sprint is visible in normal views (not archived)
    pub fn is_visible(&self) -> bool {
        !matches!(self, SprintStatus::Archived)
    }

    /// Cycle to next status
    pub fn next(&self) -> Self {
        match self {
            SprintStatus::Planning => SprintStatus::Active,
            SprintStatus::Active => SprintStatus::Completed,
            SprintStatus::Completed => SprintStatus::Archived,
            SprintStatus::Archived => SprintStatus::Planning,
        }
    }
}

/// Sprint entity for grouping and ordering tasks within a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprint {
    /// Unique identifier (UUID v4)
    pub id: String,
    /// Sprint name (e.g., "Sprint 1", "MVP", "Q1 Goals")
    pub name: String,
    /// Project this sprint belongs to (required)
    pub project: String,
    /// Sprint status
    #[serde(default)]
    pub status: SprintStatus,
    /// Optional start date
    #[serde(default)]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    /// Optional end date
    #[serde(default)]
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    /// Sprint goal or description
    #[serde(default)]
    pub goal: Option<String>,
    /// Creation timestamp
    #[serde(default)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Sprint {
    /// Create a new sprint with the given name and project
    pub fn new(name: String, project: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            project,
            status: SprintStatus::Planning,
            start_date: None,
            end_date: None,
            goal: None,
            created_at: chrono::Utc::now(),
        }
    }

    /// Check if sprint belongs to a specific project
    pub fn belongs_to_project(&self, project: &str) -> bool {
        self.project == project
    }

    /// Mark sprint as active
    pub fn start(&mut self) {
        self.status = SprintStatus::Active;
        if self.start_date.is_none() {
            self.start_date = Some(chrono::Utc::now());
        }
    }

    /// Mark sprint as completed
    pub fn complete(&mut self) {
        self.status = SprintStatus::Completed;
        if self.end_date.is_none() {
            self.end_date = Some(chrono::Utc::now());
        }
    }

    /// Archive the sprint
    pub fn archive(&mut self) {
        self.status = SprintStatus::Archived;
    }

    /// Reopen the sprint (set back to planning)
    pub fn reopen(&mut self) {
        self.status = SprintStatus::Planning;
    }

    /// Format start date for display
    pub fn start_date_formatted(&self) -> Option<String> {
        self.start_date.map(|d| d.format("%Y-%m-%d").to_string())
    }

    /// Format end date for display
    pub fn end_date_formatted(&self) -> Option<String> {
        self.end_date.map(|d| d.format("%Y-%m-%d").to_string())
    }

    /// Check if sprint is overdue (has end date in the past and not completed)
    pub fn is_overdue(&self) -> bool {
        if let Some(end) = self.end_date {
            self.status.is_active() && end < chrono::Utc::now()
        } else {
            false
        }
    }

    /// Get display name with status indicator
    pub fn display_name(&self) -> String {
        format!("{} {}", self.status.emoji(), self.name)
    }
}

impl Default for Sprint {
    fn default() -> Self {
        Self::new("New Sprint".to_string(), "default".to_string())
    }
}

impl PartialEq for Sprint {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprint_creation() {
        let sprint = Sprint::new("Sprint 1".to_string(), "vulcan-todo".to_string());
        assert_eq!(sprint.name, "Sprint 1");
        assert_eq!(sprint.project, "vulcan-todo");
        assert_eq!(sprint.status, SprintStatus::Planning);
        assert!(!sprint.id.is_empty());
    }

    #[test]
    fn test_sprint_lifecycle() {
        let mut sprint = Sprint::new("Test Sprint".to_string(), "test".to_string());

        // Planning -> Active
        assert_eq!(sprint.status, SprintStatus::Planning);
        sprint.start();
        assert_eq!(sprint.status, SprintStatus::Active);
        assert!(sprint.start_date.is_some());

        // Active -> Completed
        sprint.complete();
        assert_eq!(sprint.status, SprintStatus::Completed);
        assert!(sprint.end_date.is_some());

        // Completed -> Archived
        sprint.archive();
        assert_eq!(sprint.status, SprintStatus::Archived);
    }

    #[test]
    fn test_sprint_status_cycling() {
        assert_eq!(SprintStatus::Planning.next(), SprintStatus::Active);
        assert_eq!(SprintStatus::Active.next(), SprintStatus::Completed);
        assert_eq!(SprintStatus::Completed.next(), SprintStatus::Archived);
        assert_eq!(SprintStatus::Archived.next(), SprintStatus::Planning);
    }

    #[test]
    fn test_sprint_status_from_string() {
        assert_eq!(
            SprintStatus::from("active".to_string()),
            SprintStatus::Active
        );
        assert_eq!(
            SprintStatus::from("ACTIVE".to_string()),
            SprintStatus::Active
        );
        assert_eq!(
            SprintStatus::from("completed".to_string()),
            SprintStatus::Completed
        );
        assert_eq!(
            SprintStatus::from("done".to_string()),
            SprintStatus::Completed
        );
        assert_eq!(
            SprintStatus::from("archived".to_string()),
            SprintStatus::Archived
        );
        assert_eq!(
            SprintStatus::from("unknown".to_string()),
            SprintStatus::Planning
        );
    }

    #[test]
    fn test_sprint_belongs_to_project() {
        let sprint = Sprint::new("Sprint 1".to_string(), "vulcan-todo".to_string());
        assert!(sprint.belongs_to_project("vulcan-todo"));
        assert!(!sprint.belongs_to_project("other-project"));
    }
}
