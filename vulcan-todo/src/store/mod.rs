//! Storage backend for vulcan-todo
//!
//! This module defines the `Store` trait and provides implementations
//! for JSON file storage and in-memory storage.

use crate::models::{Priority, Sprint, Task};
use anyhow::Result;

/// Trait for task storage backends
pub trait Store: Send + Sync {
    // ==================== Task Methods ====================

    /// Get all tasks (may return cached data)
    fn get_all(&self) -> Result<Vec<Task>>;

    /// Force reload from storage, bypassing any cache
    fn reload(&self) -> Result<Vec<Task>>;

    /// Get a task by ID
    fn get(&self, id: &str) -> Result<Option<Task>>;

    /// Add a new task
    fn add(&self, task: &Task) -> Result<Task>;

    /// Update an existing task
    fn update(&self, task: &Task) -> Result<Option<Task>>;

    /// Delete a task by ID
    fn delete(&self, id: &str) -> Result<bool>;

    /// Get tasks by status
    fn get_by_status(&self, status: &str) -> Result<Vec<Task>>;

    /// Get tasks by priority
    fn get_by_priority(&self, priority: &str) -> Result<Vec<Task>>;

    /// Get tasks by session scope
    fn get_by_scope(&self, scope: &str) -> Result<Vec<Task>>;

    /// Get global tasks (tasks with no session scope)
    fn get_global(&self) -> Result<Vec<Task>>;

    /// Search tasks
    fn search(&self, query: &str) -> Result<Vec<Task>>;

    /// Get task count (pending, done)
    fn count(&self) -> Result<(usize, usize)>;

    /// Get tasks by project
    fn get_by_project(&self, project: &str) -> Result<Vec<Task>>;

    /// Get all unique projects
    fn get_projects(&self) -> Result<Vec<String>>;

    /// Get project statistics
    fn get_project_stats(&self) -> Result<std::collections::HashMap<String, (usize, usize)>>;

    /// Auto-assign projects from tags for all tasks
    fn auto_assign_projects_from_tags(&self) -> Result<Vec<String>>;

    // ==================== Sprint Methods ====================

    /// Get all sprints
    fn get_all_sprints(&self) -> Result<Vec<Sprint>>;

    /// Get a sprint by ID
    fn get_sprint(&self, id: &str) -> Result<Option<Sprint>>;

    /// Add a new sprint
    fn add_sprint(&self, sprint: &Sprint) -> Result<Sprint>;

    /// Update an existing sprint
    fn update_sprint(&self, sprint: &Sprint) -> Result<Option<Sprint>>;

    /// Delete a sprint by ID (unassigns all tasks from the sprint)
    fn delete_sprint(&self, id: &str) -> Result<bool>;

    /// Get sprints by project
    fn get_sprints_by_project(&self, project: &str) -> Result<Vec<Sprint>>;

    /// Get sprints by status
    fn get_sprints_by_status(&self, status: &str) -> Result<Vec<Sprint>>;

    /// Get tasks in a sprint (ordered by sprint_order)
    fn get_tasks_in_sprint(&self, sprint_id: &str) -> Result<Vec<Task>>;

    /// Get backlog tasks (tasks in project but not in any sprint)
    fn get_backlog_tasks(&self, project: &str) -> Result<Vec<Task>>;

    /// Assign a task to a sprint (adds to end of sprint by default)
    fn assign_task_to_sprint(&self, task_id: &str, sprint_id: &str) -> Result<Option<Task>>;

    /// Remove a task from its sprint
    fn remove_task_from_sprint(&self, task_id: &str) -> Result<Option<Task>>;

    /// Reorder a task within its sprint
    fn reorder_task_in_sprint(&self, task_id: &str, new_position: i32) -> Result<Option<Task>>;

    /// Move a task from one sprint to another
    fn move_task_to_sprint(
        &self,
        task_id: &str,
        from_sprint_id: &str,
        to_sprint_id: &str,
    ) -> Result<Option<Task>>;
}

pub mod json_store;
pub use json_store::{JsonStore, MemoryStore};
