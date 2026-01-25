use crate::models::sprint::Sprint;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Task status enum
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum Status {
    #[default]
    Pending,
    InProgress,
    Done,
    Archived,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Pending => write!(f, "pending"),
            Status::InProgress => write!(f, "in_progress"),
            Status::Done => write!(f, "done"),
            Status::Archived => write!(f, "archived"),
        }
    }
}

impl From<String> for Status {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "in_progress" | "inprogress" | "in-progress" | "wip" | "working" => Status::InProgress,
            "done" | "complete" | "completed" => Status::Done,
            "archived" | "archive" => Status::Archived,
            _ => Status::Pending,
        }
    }
}

impl Status {
    /// Get emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            Status::Pending => "○",
            Status::InProgress => "◐",
            Status::Done => "✓",
            Status::Archived => "▣",
        }
    }

    /// Check if task is active (pending or in progress)
    pub fn is_active(&self) -> bool {
        matches!(self, Status::Pending | Status::InProgress)
    }
}

/// Task priority enum
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum Priority {
    #[default]
    None,
    Low,
    Medium,
    High,
    Urgent,
}

impl Priority {
    /// Returns the display level (higher = more urgent)
    pub fn level(&self) -> u8 {
        match self {
            Priority::None => 0,
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
            Priority::Urgent => 4,
        }
    }

    /// Cycle to next priority
    pub fn next(&self) -> Self {
        match self {
            Priority::None => Priority::Low,
            Priority::Low => Priority::Medium,
            Priority::Medium => Priority::High,
            Priority::High => Priority::Urgent,
            Priority::Urgent => Priority::None,
        }
    }

    /// Get emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            Priority::None => "⚪",
            Priority::Low => "🟢",
            Priority::Medium => "🟡",
            Priority::High => "🔴",
            Priority::Urgent => "🚨",
        }
    }

    /// Get short label
    pub fn label(&self) -> &'static str {
        match self {
            Priority::None => "NONE",
            Priority::Low => "LOW",
            Priority::Medium => "MED",
            Priority::High => "HIGH",
            Priority::Urgent => "URGENT",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

impl From<String> for Priority {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "urgent" | "critical" => Priority::Urgent,
            "high" => Priority::High,
            "medium" | "med" => Priority::Medium,
            "low" => Priority::Low,
            _ => Priority::None,
        }
    }
}

/// Main Task struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier (UUID v4)
    pub id: String,
    /// Task title (required)
    pub title: String,
    /// Task description (optional)
    #[serde(default)]
    pub description: Option<String>,
    /// Task status
    #[serde(default)]
    pub status: Status,
    /// Task priority
    #[serde(default)]
    pub priority: Priority,
    /// Tags for organization
    #[serde(default)]
    pub tags: Vec<String>,
    /// Creation timestamp
    #[serde(default)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Completion timestamp (None if pending)
    #[serde(default)]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Due date (optional)
    #[serde(default)]
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
    /// Project for organization (e.g., "vulcan-os", "personal", "work")
    /// Can be auto-assigned from tags with format "project:name"
    #[serde(default)]
    pub project: Option<String>,
    /// Session scope: tasks created in a specific OpenCode session (e.g., "session:abc123")
    /// None or "global" means task is visible in all sessions
    #[serde(default)]
    pub scope: Option<String>,
    /// Sprint ID this task belongs to (optional)
    #[serde(default)]
    pub sprint_id: Option<String>,
    /// Position/order within the sprint (1-indexed, None if not in sprint)
    #[serde(default)]
    pub sprint_order: Option<i32>,
    /// UUIDs of linked vulcan-vault notes
    #[serde(default)]
    pub context_notes: Vec<String>,
    /// Automatically fetch vault context when task starts
    #[serde(default)]
    pub auto_fetch_context: bool,
    /// Enable ralph loop mode for this task (iterative self-correction)
    #[serde(default)]
    pub ralph_mode: bool,
    /// Success criteria for ralph loop (e.g., "tests pass", "lint clean")
    #[serde(default)]
    pub success_criteria: Vec<String>,
    /// Quality gates to run before completion (e.g., "test", "typecheck", "lint")
    #[serde(default)]
    pub quality_gates: Vec<String>,
}

impl Task {
    /// Create a new pending task with current timestamp
    pub fn new(title: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description: None,
            status: Status::Pending,
            priority: Priority::None,
            tags: Vec::new(),
            created_at: chrono::Utc::now(),
            completed_at: None,
            due_date: None,
            project: None,
            scope: None,
            sprint_id: None,
            sprint_order: None,
            context_notes: Vec::new(),
            auto_fetch_context: false,
            ralph_mode: false,
            success_criteria: Vec::new(),
            quality_gates: Vec::new(),
        }
    }

    /// Create a new pending task with session scope
    pub fn new_with_scope(title: String, scope: Option<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description: None,
            status: Status::Pending,
            priority: Priority::None,
            tags: Vec::new(),
            created_at: chrono::Utc::now(),
            completed_at: None,
            due_date: None,
            project: None,
            scope,
            sprint_id: None,
            sprint_order: None,
            context_notes: Vec::new(),
            auto_fetch_context: false,
            ralph_mode: false,
            success_criteria: Vec::new(),
            quality_gates: Vec::new(),
        }
    }

    /// Check if task is pending
    pub fn is_pending(&self) -> bool {
        self.status == Status::Pending
    }

    /// Check if task is in progress
    pub fn is_in_progress(&self) -> bool {
        self.status == Status::InProgress
    }

    /// Check if task is done
    pub fn is_done(&self) -> bool {
        self.status == Status::Done
    }

    /// Check if task is active (pending or in progress)
    pub fn is_active(&self) -> bool {
        self.status.is_active()
    }

    /// Mark task as in progress
    pub fn start(&mut self) {
        self.status = Status::InProgress;
    }

    /// Mark task as complete
    pub fn complete(&mut self) {
        self.status = Status::Done;
        self.completed_at = Some(chrono::Utc::now());
    }

    /// Reopen a completed task
    pub fn uncomplete(&mut self) {
        self.status = Status::Pending;
        self.completed_at = None;
    }

    /// Toggle completion status (cycles: Pending -> InProgress -> Done -> Pending)
    pub fn toggle(&mut self) {
        match self.status {
            Status::Pending => self.start(),
            Status::InProgress => self.complete(),
            Status::Done => self.uncomplete(),
            Status::Archived => self.uncomplete(),
        }
    }

    /// Cycle status forward (Pending -> InProgress -> Done)
    pub fn cycle_status(&mut self) {
        match self.status {
            Status::Pending => self.start(),
            Status::InProgress => self.complete(),
            Status::Done => self.uncomplete(),
            Status::Archived => self.uncomplete(),
        }
    }

    /// Get a short preview of the title (truncated)
    pub fn title_preview(&self, max_len: usize) -> String {
        if self.title.len() <= max_len {
            self.title.clone()
        } else {
            format!("{}...", &self.title[..max_len - 3])
        }
    }

    /// Format creation date for display
    pub fn created_formatted(&self) -> String {
        self.created_at.format("%Y-%m-%d %H:%M").to_string()
    }

    /// Format due date for display
    pub fn due_formatted(&self) -> Option<String> {
        self.due_date.map(|d| d.format("%Y-%m-%d").to_string())
    }

    /// Check if task is overdue
    pub fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_date {
            self.is_pending() && due < chrono::Utc::now()
        } else {
            false
        }
    }

    /// Set project by extracting from tags with format "project:name"
    /// Only sets if project is not already set
    pub fn set_project_from_tags(&mut self) {
        if self.project.is_none() {
            for tag in &self.tags {
                if let Some(project) = tag.strip_prefix("project:") {
                    self.project = Some(project.to_string());
                    break;
                }
            }
        }
    }

    /// Check if task belongs to a specific project
    pub fn belongs_to_project(&self, project: &str) -> bool {
        self.project.as_ref().map(|p| p == project).unwrap_or(false)
    }

    /// Check if task belongs to a specific sprint
    pub fn belongs_to_sprint(&self, sprint_id: &str) -> bool {
        self.sprint_id
            .as_ref()
            .map(|s| s == sprint_id)
            .unwrap_or(false)
    }

    /// Check if task is assigned to any sprint
    pub fn has_sprint(&self) -> bool {
        self.sprint_id.is_some()
    }

    /// Assign task to a sprint with a specific order
    pub fn assign_to_sprint(&mut self, sprint_id: &str, order: i32) {
        self.sprint_id = Some(sprint_id.to_string());
        self.sprint_order = Some(order);
    }

    /// Remove task from its current sprint
    pub fn unassign_from_sprint(&mut self) {
        self.sprint_id = None;
        self.sprint_order = None;
    }

    // ==================== Context Methods ====================

    /// Check if task should auto-fetch context from vulcan-vault
    pub fn should_fetch_context(&self) -> bool {
        self.auto_fetch_context
    }

    /// Check if task has linked context notes
    pub fn has_context_notes(&self) -> bool {
        !self.context_notes.is_empty()
    }

    /// Add a context note UUID
    pub fn add_context_note(&mut self, note_id: &str) {
        if !self.context_notes.contains(&note_id.to_string()) {
            self.context_notes.push(note_id.to_string());
        }
    }

    /// Remove a context note UUID
    pub fn remove_context_note(&mut self, note_id: &str) {
        self.context_notes.retain(|id| id != note_id);
    }
}

impl Default for Task {
    fn default() -> Self {
        Self::new("Untitled Task".to_string())
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// Container for multiple tasks with metadata
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaskStore {
    /// Schema version for future migrations
    #[serde(default = "TaskStore::current_version")]
    pub version: u32,
    /// List of all tasks
    #[serde(default)]
    pub tasks: Vec<Task>,
    /// List of all sprints
    #[serde(default)]
    pub sprints: Vec<Sprint>,
}

impl TaskStore {
    /// Current schema version (bump when adding new fields)
    pub const CURRENT_VERSION: u32 = 4;

    /// Current schema version (for serde default)
    fn current_version() -> u32 {
        Self::CURRENT_VERSION
    }

    /// Create empty store
    pub fn new() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            tasks: Vec::new(),
            sprints: Vec::new(),
        }
    }

    /// Migrate from older schema versions if needed
    pub fn migrate(&mut self) {
        if self.version < 2 {
            // Migration from v1 to v2:
            // - Added sprints Vec (defaults to empty)
            // - Added sprint_id and sprint_order to Task (defaults to None)
            // No data transformation needed, serde defaults handle it
            self.version = 2;
        }
        if self.version < 3 {
            // Migration from v2 to v3:
            // - Added context_notes: Vec<String> to Task (defaults to empty)
            // - Added auto_fetch_context: bool to Task (defaults to false)
            // No data transformation needed, serde defaults handle it
            self.version = 3;
        }
        if self.version < 4 {
            // Migration from v3 to v4:
            // - Added ralph_mode: bool to Task (defaults to false)
            // - Added success_criteria: Vec<String> to Task (defaults to empty)
            // - Added quality_gates: Vec<String> to Task (defaults to empty)
            // No data transformation needed, serde defaults handle it
            self.version = 4;
        }
        // Future migrations go here
    }

    /// Check if migration is needed
    pub fn needs_migration(&self) -> bool {
        self.version < Self::CURRENT_VERSION
    }

    /// Add a new task
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Get task by ID
    pub fn get(&self, id: &str) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    /// Get mutable task by ID
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    /// Remove task by ID
    pub fn remove(&mut self, id: &str) -> bool {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        self.tasks.len() < original_len
    }

    /// Get all tasks matching filter
    pub fn filter<F>(&self, f: F) -> Vec<&Task>
    where
        F: Fn(&Task) -> bool,
    {
        self.tasks.iter().filter(|t| f(t)).collect()
    }

    /// Get all pending tasks sorted by priority
    pub fn pending_by_priority(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.filter(|t| t.is_pending());
        tasks.sort_by(|a, b| b.priority.level().cmp(&a.priority.level()));
        tasks
    }

    /// Get all done tasks
    pub fn done(&self) -> Vec<&Task> {
        self.filter(|t| t.is_done())
    }

    /// Get task count by status
    pub fn count_by_status(&self) -> (usize, usize) {
        let pending = self.filter(|t| t.is_pending()).len();
        let done = self.filter(|t| t.is_done()).len();
        (pending, done)
    }

    /// Search tasks by title, description, or tags
    pub fn search(&self, query: &str) -> Vec<&Task> {
        let query_lower = query.to_lowercase();
        self.filter(|t| {
            t.title.to_lowercase().contains(&query_lower)
                || t.description
                    .as_ref()
                    .map(|d| d.to_lowercase().contains(&query_lower))
                    .unwrap_or(false)
                || t.tags
                    .iter()
                    .any(|tag| tag.to_lowercase().contains(&query_lower))
        })
    }

    /// Get tasks by tag
    pub fn by_tag(&self, tag: &str) -> Vec<&Task> {
        self.filter(|t| t.tags.iter().any(|t| t == tag))
    }

    /// Get tasks by project
    pub fn by_project(&self, project: &str) -> Vec<&Task> {
        self.filter(|t| t.belongs_to_project(project))
    }

    /// Get all unique projects
    pub fn projects(&self) -> Vec<String> {
        let mut projects: Vec<String> = self
            .tasks
            .iter()
            .filter_map(|t| t.project.clone())
            .collect();
        projects.sort();
        projects.dedup();
        projects
    }

    /// Get project statistics
    pub fn project_stats(&self) -> HashMap<String, (usize, usize)> {
        let mut stats: HashMap<String, (usize, usize)> = HashMap::new();

        for task in &self.tasks {
            if let Some(ref project) = task.project {
                let (pending, done) = stats.entry(project.clone()).or_insert((0, 0));
                if task.is_pending() {
                    *pending += 1;
                } else if task.is_done() {
                    *done += 1;
                }
            }
        }

        stats
    }

    // ==================== Sprint Methods ====================

    /// Add a new sprint
    pub fn add_sprint(&mut self, sprint: Sprint) {
        self.sprints.push(sprint);
    }

    /// Get sprint by ID
    pub fn get_sprint(&self, id: &str) -> Option<&Sprint> {
        self.sprints.iter().find(|s| s.id == id)
    }

    /// Get mutable sprint by ID
    pub fn get_sprint_mut(&mut self, id: &str) -> Option<&mut Sprint> {
        self.sprints.iter_mut().find(|s| s.id == id)
    }

    /// Remove sprint by ID
    pub fn remove_sprint(&mut self, id: &str) -> bool {
        let original_len = self.sprints.len();
        self.sprints.retain(|s| s.id != id);
        self.sprints.len() < original_len
    }

    /// Get all sprints for a project
    pub fn sprints_by_project(&self, project: &str) -> Vec<&Sprint> {
        self.sprints
            .iter()
            .filter(|s| s.belongs_to_project(project))
            .collect()
    }

    /// Get all unique sprint projects
    pub fn sprint_projects(&self) -> Vec<String> {
        let mut projects: Vec<String> = self.sprints.iter().map(|s| s.project.clone()).collect();
        projects.sort();
        projects.dedup();
        projects
    }

    /// Get tasks in a specific sprint, ordered by sprint_order
    pub fn tasks_in_sprint(&self, sprint_id: &str) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|t| t.belongs_to_sprint(sprint_id))
            .collect();
        tasks.sort_by(|a, b| {
            a.sprint_order
                .unwrap_or(i32::MAX)
                .cmp(&b.sprint_order.unwrap_or(i32::MAX))
        });
        tasks
    }

    /// Get the next available sprint_order for a sprint
    pub fn next_sprint_order(&self, sprint_id: &str) -> i32 {
        self.tasks
            .iter()
            .filter(|t| t.belongs_to_sprint(sprint_id))
            .filter_map(|t| t.sprint_order)
            .max()
            .unwrap_or(0)
            + 1
    }

    /// Get tasks not assigned to any sprint (backlog) for a project
    pub fn backlog_tasks(&self, project: &str) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.belongs_to_project(project) && !t.has_sprint())
            .collect()
    }

    /// Renumber tasks in a sprint to be sequential (1, 2, 3, ...)
    pub fn renumber_sprint_tasks(&mut self, sprint_id: &str) {
        // Collect task IDs in current order
        let mut sprint_tasks: Vec<(String, i32)> = self
            .tasks
            .iter()
            .filter(|t| t.belongs_to_sprint(sprint_id))
            .map(|t| (t.id.clone(), t.sprint_order.unwrap_or(i32::MAX)))
            .collect();

        // Sort by current order
        sprint_tasks.sort_by_key(|(_, order)| *order);

        // Assign new sequential orders
        for (new_order, (task_id, _)) in sprint_tasks.into_iter().enumerate() {
            if let Some(task) = self.get_mut(&task_id) {
                task.sprint_order = Some((new_order + 1) as i32);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new("Test task".to_string());
        assert_eq!(task.title, "Test task");
        assert_eq!(task.status, Status::Pending);
        assert_eq!(task.priority, Priority::None);
        assert!(!task.id.is_empty());
    }

    #[test]
    fn test_task_toggle() {
        let mut task = Task::new("Test".to_string());
        assert!(task.is_pending());
        task.toggle(); // Pending -> InProgress
        assert!(task.is_in_progress());
        task.toggle(); // InProgress -> Done
        assert!(task.is_done());
        task.toggle(); // Done -> Pending
        assert!(task.is_pending());
    }

    #[test]
    fn test_priority_cycling() {
        assert_eq!(Priority::None.next(), Priority::Low);
        assert_eq!(Priority::Low.next(), Priority::Medium);
        assert_eq!(Priority::Medium.next(), Priority::High);
        assert_eq!(Priority::High.next(), Priority::Urgent);
        assert_eq!(Priority::Urgent.next(), Priority::None);
    }

    #[test]
    fn test_task_store() {
        let mut store = TaskStore::new();
        assert_eq!(store.tasks.len(), 0);

        store.add(Task::new("Task 1".to_string()));
        store.add(Task::new("Task 2".to_string()));
        assert_eq!(store.tasks.len(), 2);

        let task = store.get(&store.tasks[0].id).unwrap();
        assert_eq!(task.title, "Task 1");

        // Get ID before removing (borrow checker fix)
        let first_id = store.tasks[0].id.clone();
        let removed = store.remove(&first_id);
        assert!(removed);
        assert_eq!(store.tasks.len(), 1);
    }

    #[test]
    fn test_task_store_search() {
        let mut store = TaskStore::new();
        store.add(Task::new("Buy grocery items".to_string())); // contains "grocery"
        store.add(Task::new("Finish report".to_string()));
        store.add(Task::new("Grocery shopping".to_string())); // contains "grocery" (case-insensitive)

        let results = store.search("grocery");
        assert_eq!(results.len(), 2);
    }
}
