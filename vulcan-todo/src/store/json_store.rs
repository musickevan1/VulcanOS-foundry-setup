use crate::models::{Sprint, SprintStatus, Task, TaskStore};
use anyhow::{Context, Result};
use fs4::FileExt;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Storage backend for persisting tasks to JSON
#[derive(Debug, Clone)]
pub struct JsonStore {
    /// Path to the tasks.json file
    path: PathBuf,
    /// In-memory cache with file locking
    cache: Arc<Mutex<Option<TaskStore>>>,
}

impl JsonStore {
    /// Create a new JSON store at the default location
    pub fn new() -> Result<Self> {
        let path = Self::default_path()?;
        Self::with_path(path)
    }

    /// Create a new JSON store at a specific path
    pub fn with_path(path: PathBuf) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {:?}", parent))?;
        }

        // Initialize with empty store if file doesn't exist
        if !path.exists() {
            let store = TaskStore::new();
            let store_ser = serde_json::to_string_pretty(&store)
                .context("Failed to serialize initial store")?;
            std::fs::write(&path, store_ser + "\n")
                .with_context(|| format!("Failed to create store file: {:?}", path))?;
        }

        Ok(Self {
            path,
            cache: Arc::new(Mutex::new(None)),
        })
    }

    /// Get the default store path
    pub fn default_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Could not determine config directory")?
            .join("vulcan-todo");

        Ok(config_dir.join("tasks.json"))
    }

    /// Get file lock (exclusive for write, shared for read)
    fn lock_file(&self, exclusive: bool) -> Result<File> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.path)
            .with_context(|| format!("Failed to open store file: {:?}", self.path))?;

        if exclusive {
            file.lock_exclusive()
                .with_context(|| "Failed to acquire exclusive lock".to_string())?;
        } else {
            file.lock_shared()
                .with_context(|| "Failed to acquire shared lock".to_string())?;
        }

        Ok(file)
    }

    /// Load store from disk (with caching)
    fn load(&self) -> Result<TaskStore> {
        let mut cache = self.cache.lock().unwrap();

        // Return cached version if available
        if let Some(ref store) = *cache {
            return Ok(store.clone());
        }

        // Load from disk
        let mut file = self.lock_file(false)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .with_context(|| "Failed to read store file".to_string())?;

        let mut store: TaskStore = serde_json::from_str(&contents)
            .with_context(|| "Failed to parse store file".to_string())?;

        // Run migrations if needed
        if store.needs_migration() {
            store.migrate();
            // Save migrated store (release locks first to avoid deadlock)
            drop(cache); // Release cache lock
            drop(file); // Release file lock BEFORE saving
            self.save(&store)?;
            let mut cache = self.cache.lock().unwrap();
            *cache = Some(store.clone());
            return Ok(store);
        }

        // Update cache
        let store_clone = store.clone();
        *cache = Some(store);

        Ok(store_clone)
    }

    /// Save store to disk (with caching)
    fn save(&self, store: &TaskStore) -> Result<()> {
        let mut file = self.lock_file(true)?;
        let contents = serde_json::to_string_pretty(store)
            .with_context(|| "Failed to serialize store".to_string())?;
        let contents = contents + "\n";

        // Seek to beginning first
        file.seek(SeekFrom::Start(0))
            .with_context(|| "Failed to seek to start of file".to_string())?;

        // Truncate file to prevent corruption if new content is shorter
        file.set_len(0)
            .with_context(|| "Failed to truncate store file".to_string())?;

        // Write new content
        file.write_all(contents.as_bytes())
            .with_context(|| "Failed to write store file".to_string())?;

        // Ensure data is flushed to disk
        file.sync_all()
            .with_context(|| "Failed to sync store file".to_string())?;

        // Update cache
        let mut cache = self.cache.lock().unwrap();
        *cache = Some(store.clone());

        Ok(())
    }

    /// Force reload from disk (invalidates cache)
    pub fn reload(&self) -> Result<TaskStore> {
        let mut cache = self.cache.lock().unwrap();
        *cache = None;
        self.load()
    }
}

impl crate::store::Store for JsonStore {
    /// Get all tasks (may return cached data)
    fn get_all(&self) -> Result<Vec<Task>> {
        let store = self.load()?;
        Ok(store.tasks.clone())
    }

    /// Force reload from disk, bypassing cache
    fn reload(&self) -> Result<Vec<Task>> {
        // Invalidate cache
        {
            let mut cache = self.cache.lock().unwrap();
            *cache = None;
        }
        // Load fresh from disk
        let store = self.load()?;
        Ok(store.tasks.clone())
    }

    /// Get a task by ID
    fn get(&self, id: &str) -> Result<Option<Task>> {
        let store = self.load()?;
        Ok(store.get(id).cloned())
    }

    /// Add a new task
    fn add(&self, task: &Task) -> Result<Task> {
        let mut store = self.load()?;
        store.add(task.clone());
        self.save(&store)?;
        Ok(task.clone())
    }

    /// Update an existing task
    fn update(&self, task: &Task) -> Result<Option<Task>> {
        let mut store = self.load()?;

        if store.get_mut(&task.id).is_some() {
            // Replace the task
            store.tasks.retain(|t| t.id != task.id);
            store.add(task.clone());
            self.save(&store)?;
            Ok(Some(task.clone()))
        } else {
            Ok(None)
        }
    }

    /// Delete a task by ID
    fn delete(&self, id: &str) -> Result<bool> {
        let mut store = self.load()?;
        let removed = store.remove(id);
        if removed {
            self.save(&store)?;
        }
        Ok(removed)
    }

    /// Get tasks by status
    fn get_by_status(&self, status: &str) -> Result<Vec<Task>> {
        let store = self.load()?;
        let status: crate::models::Status = status.to_string().into();
        Ok(store
            .filter(|t| t.status == status)
            .into_iter()
            .cloned()
            .collect())
    }

    /// Get tasks by priority
    fn get_by_priority(&self, priority: &str) -> Result<Vec<Task>> {
        let store = self.load()?;
        let priority: crate::models::Priority = priority.to_string().into();
        Ok(store
            .filter(|t| t.priority == priority)
            .into_iter()
            .cloned()
            .collect())
    }

    /// Search tasks
    fn search(&self, query: &str) -> Result<Vec<Task>> {
        let store = self.load()?;
        Ok(store.search(query).into_iter().cloned().collect())
    }

    /// Get task count
    fn count(&self) -> Result<(usize, usize)> {
        let store = self.load()?;
        Ok(store.count_by_status())
    }

    /// Get tasks by session scope
    fn get_by_scope(&self, scope: &str) -> Result<Vec<Task>> {
        let store = self.load()?;
        Ok(store
            .tasks
            .into_iter()
            .filter(|t| t.scope.as_ref().map(|s| s == scope).unwrap_or(false))
            .collect())
    }

    /// Get global tasks (tasks with no session scope)
    fn get_global(&self) -> Result<Vec<Task>> {
        let store = self.load()?;
        Ok(store
            .tasks
            .into_iter()
            .filter(|t| t.scope.is_none())
            .collect())
    }

    /// Get tasks by project
    fn get_by_project(&self, project: &str) -> Result<Vec<Task>> {
        let store = self.load()?;
        Ok(store.by_project(project).into_iter().cloned().collect())
    }

    /// Get all unique projects
    fn get_projects(&self) -> Result<Vec<String>> {
        let store = self.load()?;
        Ok(store.projects())
    }

    /// Get project statistics
    fn get_project_stats(&self) -> Result<std::collections::HashMap<String, (usize, usize)>> {
        let store = self.load()?;
        Ok(store.project_stats())
    }

    /// Auto-assign projects from tags for all tasks
    fn auto_assign_projects_from_tags(&self) -> Result<Vec<String>> {
        let mut store = self.load()?;
        let mut updated: Vec<String> = Vec::new();

        for task in &mut store.tasks {
            let had_project = task.project.is_some();
            task.set_project_from_tags();

            if !had_project && task.project.is_some() {
                updated.push(task.id.clone());
            }
        }

        if !updated.is_empty() {
            self.save(&store)?;
        }

        Ok(updated)
    }

    // ==================== Sprint Methods ====================

    /// Get all sprints
    fn get_all_sprints(&self) -> Result<Vec<Sprint>> {
        let store = self.load()?;
        Ok(store.sprints.clone())
    }

    /// Get a sprint by ID
    fn get_sprint(&self, id: &str) -> Result<Option<Sprint>> {
        let store = self.load()?;
        Ok(store.get_sprint(id).cloned())
    }

    /// Add a new sprint
    fn add_sprint(&self, sprint: &Sprint) -> Result<Sprint> {
        let mut store = self.load()?;
        store.add_sprint(sprint.clone());
        self.save(&store)?;
        Ok(sprint.clone())
    }

    /// Update an existing sprint
    fn update_sprint(&self, sprint: &Sprint) -> Result<Option<Sprint>> {
        let mut store = self.load()?;

        if store.get_sprint(&sprint.id).is_some() {
            // Replace the sprint
            store.sprints.retain(|s| s.id != sprint.id);
            store.add_sprint(sprint.clone());
            self.save(&store)?;
            Ok(Some(sprint.clone()))
        } else {
            Ok(None)
        }
    }

    /// Delete a sprint by ID (unassigns all tasks from the sprint)
    fn delete_sprint(&self, id: &str) -> Result<bool> {
        let mut store = self.load()?;

        // First, unassign all tasks from this sprint
        for task in &mut store.tasks {
            if task.belongs_to_sprint(id) {
                task.unassign_from_sprint();
            }
        }

        // Then remove the sprint
        let removed = store.remove_sprint(id);
        if removed {
            self.save(&store)?;
        }
        Ok(removed)
    }

    /// Get sprints by project
    fn get_sprints_by_project(&self, project: &str) -> Result<Vec<Sprint>> {
        let store = self.load()?;
        Ok(store
            .sprints_by_project(project)
            .into_iter()
            .cloned()
            .collect())
    }

    /// Get sprints by status
    fn get_sprints_by_status(&self, status: &str) -> Result<Vec<Sprint>> {
        let store = self.load()?;
        let status: SprintStatus = status.to_string().into();
        Ok(store
            .sprints
            .iter()
            .filter(|s| s.status == status)
            .cloned()
            .collect())
    }

    /// Get tasks in a sprint (ordered by sprint_order)
    fn get_tasks_in_sprint(&self, sprint_id: &str) -> Result<Vec<Task>> {
        let store = self.load()?;
        Ok(store
            .tasks_in_sprint(sprint_id)
            .into_iter()
            .cloned()
            .collect())
    }

    /// Get backlog tasks (tasks in project but not in any sprint)
    fn get_backlog_tasks(&self, project: &str) -> Result<Vec<Task>> {
        let store = self.load()?;
        Ok(store.backlog_tasks(project).into_iter().cloned().collect())
    }

    /// Assign a task to a sprint (adds to end of sprint by default)
    fn assign_task_to_sprint(&self, task_id: &str, sprint_id: &str) -> Result<Option<Task>> {
        let mut store = self.load()?;

        // Verify sprint exists
        if store.get_sprint(sprint_id).is_none() {
            return Ok(None);
        }

        // Get next order position
        let next_order = store.next_sprint_order(sprint_id);

        // Assign task to sprint
        if let Some(task) = store.get_mut(task_id) {
            task.assign_to_sprint(sprint_id, next_order);
            let updated_task = task.clone();
            self.save(&store)?;
            Ok(Some(updated_task))
        } else {
            Ok(None)
        }
    }

    /// Remove a task from its sprint
    fn remove_task_from_sprint(&self, task_id: &str) -> Result<Option<Task>> {
        let mut store = self.load()?;

        if let Some(task) = store.get_mut(task_id) {
            let old_sprint_id = task.sprint_id.clone();
            task.unassign_from_sprint();
            let updated_task = task.clone();

            // Renumber remaining tasks in the sprint
            if let Some(sprint_id) = old_sprint_id {
                store.renumber_sprint_tasks(&sprint_id);
            }

            self.save(&store)?;
            Ok(Some(updated_task))
        } else {
            Ok(None)
        }
    }

    /// Reorder a task within its sprint
    fn reorder_task_in_sprint(&self, task_id: &str, new_position: i32) -> Result<Option<Task>> {
        let mut store = self.load()?;

        // Get the task and its current sprint
        let sprint_id = match store.get(task_id) {
            Some(task) => match &task.sprint_id {
                Some(sid) => sid.clone(),
                None => return Ok(None), // Task not in a sprint
            },
            None => return Ok(None), // Task not found
        };

        // Get all tasks in the sprint, sorted by current order
        let mut sprint_task_ids: Vec<String> = store
            .tasks_in_sprint(&sprint_id)
            .iter()
            .map(|t| t.id.clone())
            .collect();

        // Find current position of the task
        let current_pos = match sprint_task_ids.iter().position(|id| id == task_id) {
            Some(pos) => pos,
            None => return Ok(None),
        };

        // Remove task from current position
        sprint_task_ids.remove(current_pos);

        // Calculate new position (1-indexed to 0-indexed, clamped to valid range)
        let new_pos = ((new_position - 1) as usize).min(sprint_task_ids.len());

        // Insert at new position
        sprint_task_ids.insert(new_pos, task_id.to_string());

        // Update all task orders
        for (i, tid) in sprint_task_ids.iter().enumerate() {
            if let Some(task) = store.get_mut(tid) {
                task.sprint_order = Some((i + 1) as i32);
            }
        }

        let updated_task = store.get(task_id).cloned();
        self.save(&store)?;
        Ok(updated_task)
    }

    /// Move a task from one sprint to another
    fn move_task_to_sprint(
        &self,
        task_id: &str,
        from_sprint_id: &str,
        to_sprint_id: &str,
    ) -> Result<Option<Task>> {
        let mut store = self.load()?;

        // Verify both sprints exist
        if store.get_sprint(from_sprint_id).is_none() || store.get_sprint(to_sprint_id).is_none() {
            return Ok(None);
        }

        // Verify task is in the from_sprint
        let task = match store.get(task_id) {
            Some(t) if t.belongs_to_sprint(from_sprint_id) => t.clone(),
            _ => return Ok(None),
        };

        // Get next order in destination sprint
        let next_order = store.next_sprint_order(to_sprint_id);

        // Update the task
        if let Some(task) = store.get_mut(task_id) {
            task.assign_to_sprint(to_sprint_id, next_order);
        }

        // Renumber tasks in the source sprint
        store.renumber_sprint_tasks(from_sprint_id);

        let updated_task = store.get(task_id).cloned();
        self.save(&store)?;
        Ok(updated_task)
    }
}

/// In-memory store for testing or temporary use
#[derive(Debug, Default)]
pub struct MemoryStore {
    tasks: Mutex<Vec<Task>>,
    sprints: Mutex<Vec<Sprint>>,
}

impl MemoryStore {
    /// Create a new in-memory store
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(Vec::new()),
            sprints: Mutex::new(Vec::new()),
        }
    }
}

impl crate::store::Store for MemoryStore {
    fn get_all(&self) -> Result<Vec<Task>> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks.clone())
    }

    fn reload(&self) -> Result<Vec<Task>> {
        // MemoryStore has no external storage, just return current tasks
        self.get_all()
    }

    fn get(&self, id: &str) -> Result<Option<Task>> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks.iter().find(|t| t.id == id).cloned())
    }

    fn add(&self, task: &Task) -> Result<Task> {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(task.clone());
        Ok(task.clone())
    }

    fn update(&self, task: &Task) -> Result<Option<Task>> {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(pos) = tasks.iter().position(|t| t.id == task.id) {
            tasks[pos] = task.clone();
            Ok(Some(task.clone()))
        } else {
            Ok(None)
        }
    }

    fn delete(&self, id: &str) -> Result<bool> {
        let mut tasks = self.tasks.lock().unwrap();
        let len_before = tasks.len();
        tasks.retain(|t| t.id != id);
        Ok(tasks.len() < len_before)
    }

    fn get_by_status(&self, status: &str) -> Result<Vec<Task>> {
        let status: crate::models::Status = status.to_string().into();
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks
            .iter()
            .filter(|t| t.status == status)
            .cloned()
            .collect())
    }

    fn get_by_priority(&self, priority: &str) -> Result<Vec<Task>> {
        let priority: crate::models::Priority = priority.to_string().into();
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks
            .iter()
            .filter(|t| t.priority == priority)
            .cloned()
            .collect())
    }

    /// Get tasks by session scope
    fn get_by_scope(&self, scope: &str) -> Result<Vec<Task>> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks
            .iter()
            .filter(|t| t.scope.as_ref().map(|s| s == scope).unwrap_or(false))
            .cloned()
            .collect())
    }

    /// Get global tasks (tasks with no session scope)
    fn get_global(&self) -> Result<Vec<Task>> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks
            .iter()
            .filter(|t| t.scope.is_none())
            .cloned()
            .collect())
    }

    /// Get tasks by project
    fn get_by_project(&self, project: &str) -> Result<Vec<Task>> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks
            .iter()
            .filter(|t| t.belongs_to_project(project))
            .cloned()
            .collect())
    }

    /// Get all unique projects
    fn get_projects(&self) -> Result<Vec<String>> {
        let tasks = self.tasks.lock().unwrap();
        let mut projects: Vec<String> = tasks.iter().filter_map(|t| t.project.clone()).collect();
        projects.sort();
        projects.dedup();
        Ok(projects)
    }

    /// Get project statistics
    fn get_project_stats(&self) -> Result<std::collections::HashMap<String, (usize, usize)>> {
        let tasks = self.tasks.lock().unwrap();
        let mut stats: std::collections::HashMap<String, (usize, usize)> =
            std::collections::HashMap::new();

        for task in tasks.iter() {
            if let Some(ref project) = task.project {
                let (pending, done) = stats.entry(project.clone()).or_insert((0, 0));
                if task.is_pending() {
                    *pending += 1;
                } else if task.is_done() {
                    *done += 1;
                }
            }
        }

        Ok(stats)
    }

    /// Auto-assign projects from tags for all tasks
    fn auto_assign_projects_from_tags(&self) -> Result<Vec<String>> {
        let mut tasks = self.tasks.lock().unwrap();
        let mut updated: Vec<String> = Vec::new();

        for task in tasks.iter_mut() {
            if task.project.is_none() {
                for tag in &task.tags {
                    if let Some(project) = tag.strip_prefix("project:") {
                        task.project = Some(project.to_string());
                        updated.push(task.id.clone());
                        break;
                    }
                }
            }
        }

        Ok(updated)
    }

    fn search(&self, query: &str) -> Result<Vec<Task>> {
        let query_lower = query.to_lowercase();
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks
            .iter()
            .filter(|t| {
                t.title.to_lowercase().contains(&query_lower)
                    || t.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
            })
            .cloned()
            .collect())
    }

    fn count(&self) -> Result<(usize, usize)> {
        let tasks = self.tasks.lock().unwrap();
        let pending = tasks.iter().filter(|t| t.is_pending()).count();
        let done = tasks.iter().filter(|t| t.is_done()).count();
        Ok((pending, done))
    }

    // ==================== Sprint Methods ====================

    fn get_all_sprints(&self) -> Result<Vec<Sprint>> {
        let sprints = self.sprints.lock().unwrap();
        Ok(sprints.clone())
    }

    fn get_sprint(&self, id: &str) -> Result<Option<Sprint>> {
        let sprints = self.sprints.lock().unwrap();
        Ok(sprints.iter().find(|s| s.id == id).cloned())
    }

    fn add_sprint(&self, sprint: &Sprint) -> Result<Sprint> {
        let mut sprints = self.sprints.lock().unwrap();
        sprints.push(sprint.clone());
        Ok(sprint.clone())
    }

    fn update_sprint(&self, sprint: &Sprint) -> Result<Option<Sprint>> {
        let mut sprints = self.sprints.lock().unwrap();
        if let Some(pos) = sprints.iter().position(|s| s.id == sprint.id) {
            sprints[pos] = sprint.clone();
            Ok(Some(sprint.clone()))
        } else {
            Ok(None)
        }
    }

    fn delete_sprint(&self, id: &str) -> Result<bool> {
        // First, unassign all tasks from this sprint
        {
            let mut tasks = self.tasks.lock().unwrap();
            for task in tasks.iter_mut() {
                if task.belongs_to_sprint(id) {
                    task.unassign_from_sprint();
                }
            }
        }

        // Then remove the sprint
        let mut sprints = self.sprints.lock().unwrap();
        let len_before = sprints.len();
        sprints.retain(|s| s.id != id);
        Ok(sprints.len() < len_before)
    }

    fn get_sprints_by_project(&self, project: &str) -> Result<Vec<Sprint>> {
        let sprints = self.sprints.lock().unwrap();
        Ok(sprints
            .iter()
            .filter(|s| s.belongs_to_project(project))
            .cloned()
            .collect())
    }

    fn get_sprints_by_status(&self, status: &str) -> Result<Vec<Sprint>> {
        let status: SprintStatus = status.to_string().into();
        let sprints = self.sprints.lock().unwrap();
        Ok(sprints
            .iter()
            .filter(|s| s.status == status)
            .cloned()
            .collect())
    }

    fn get_tasks_in_sprint(&self, sprint_id: &str) -> Result<Vec<Task>> {
        let tasks = self.tasks.lock().unwrap();
        let mut sprint_tasks: Vec<Task> = tasks
            .iter()
            .filter(|t| t.belongs_to_sprint(sprint_id))
            .cloned()
            .collect();
        sprint_tasks.sort_by(|a, b| {
            a.sprint_order
                .unwrap_or(i32::MAX)
                .cmp(&b.sprint_order.unwrap_or(i32::MAX))
        });
        Ok(sprint_tasks)
    }

    fn get_backlog_tasks(&self, project: &str) -> Result<Vec<Task>> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks
            .iter()
            .filter(|t| t.belongs_to_project(project) && !t.has_sprint())
            .cloned()
            .collect())
    }

    fn assign_task_to_sprint(&self, task_id: &str, sprint_id: &str) -> Result<Option<Task>> {
        // Verify sprint exists
        {
            let sprints = self.sprints.lock().unwrap();
            if !sprints.iter().any(|s| s.id == sprint_id) {
                return Ok(None);
            }
        }

        let mut tasks = self.tasks.lock().unwrap();

        // Get next order position
        let next_order = tasks
            .iter()
            .filter(|t| t.belongs_to_sprint(sprint_id))
            .filter_map(|t| t.sprint_order)
            .max()
            .unwrap_or(0)
            + 1;

        // Assign task to sprint
        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            task.assign_to_sprint(sprint_id, next_order);
            Ok(Some(task.clone()))
        } else {
            Ok(None)
        }
    }

    fn remove_task_from_sprint(&self, task_id: &str) -> Result<Option<Task>> {
        let mut tasks = self.tasks.lock().unwrap();

        let old_sprint_id = tasks
            .iter()
            .find(|t| t.id == task_id)
            .and_then(|t| t.sprint_id.clone());

        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            task.unassign_from_sprint();
            let updated_task = task.clone();

            // Renumber remaining tasks in the sprint
            if let Some(sprint_id) = old_sprint_id {
                // Collect and sort tasks in sprint
                let mut sprint_task_ids: Vec<(String, i32)> = tasks
                    .iter()
                    .filter(|t| t.belongs_to_sprint(&sprint_id))
                    .map(|t| (t.id.clone(), t.sprint_order.unwrap_or(i32::MAX)))
                    .collect();
                sprint_task_ids.sort_by_key(|(_, order)| *order);

                // Renumber
                for (new_order, (tid, _)) in sprint_task_ids.into_iter().enumerate() {
                    if let Some(t) = tasks.iter_mut().find(|t| t.id == tid) {
                        t.sprint_order = Some((new_order + 1) as i32);
                    }
                }
            }

            Ok(Some(updated_task))
        } else {
            Ok(None)
        }
    }

    fn reorder_task_in_sprint(&self, task_id: &str, new_position: i32) -> Result<Option<Task>> {
        let mut tasks = self.tasks.lock().unwrap();

        // Get the task and its current sprint
        let sprint_id = match tasks.iter().find(|t| t.id == task_id) {
            Some(task) => match &task.sprint_id {
                Some(sid) => sid.clone(),
                None => return Ok(None),
            },
            None => return Ok(None),
        };

        // Get all tasks in the sprint, sorted by current order
        let mut sprint_task_ids: Vec<String> = tasks
            .iter()
            .filter(|t| t.belongs_to_sprint(&sprint_id))
            .map(|t| (t.id.clone(), t.sprint_order.unwrap_or(i32::MAX)))
            .collect::<Vec<_>>()
            .into_iter()
            .map(|(id, order)| {
                // Sort inline
                (id, order)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|(id, _)| id)
            .collect();

        // Actually sort properly
        let mut sprint_tasks_with_order: Vec<(String, i32)> = tasks
            .iter()
            .filter(|t| t.belongs_to_sprint(&sprint_id))
            .map(|t| (t.id.clone(), t.sprint_order.unwrap_or(i32::MAX)))
            .collect();
        sprint_tasks_with_order.sort_by_key(|(_, order)| *order);
        sprint_task_ids = sprint_tasks_with_order
            .into_iter()
            .map(|(id, _)| id)
            .collect();

        // Find current position of the task
        let current_pos = match sprint_task_ids.iter().position(|id| id == task_id) {
            Some(pos) => pos,
            None => return Ok(None),
        };

        // Remove task from current position
        sprint_task_ids.remove(current_pos);

        // Calculate new position (1-indexed to 0-indexed, clamped to valid range)
        let new_pos = ((new_position - 1) as usize).min(sprint_task_ids.len());

        // Insert at new position
        sprint_task_ids.insert(new_pos, task_id.to_string());

        // Update all task orders
        for (i, tid) in sprint_task_ids.iter().enumerate() {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == *tid) {
                task.sprint_order = Some((i + 1) as i32);
            }
        }

        let updated_task = tasks.iter().find(|t| t.id == task_id).cloned();
        Ok(updated_task)
    }

    fn move_task_to_sprint(
        &self,
        task_id: &str,
        from_sprint_id: &str,
        to_sprint_id: &str,
    ) -> Result<Option<Task>> {
        // Verify both sprints exist
        {
            let sprints = self.sprints.lock().unwrap();
            if !sprints.iter().any(|s| s.id == from_sprint_id)
                || !sprints.iter().any(|s| s.id == to_sprint_id)
            {
                return Ok(None);
            }
        }

        let mut tasks = self.tasks.lock().unwrap();

        // Verify task is in the from_sprint
        let task_in_from = tasks
            .iter()
            .find(|t| t.id == task_id)
            .map(|t| t.belongs_to_sprint(from_sprint_id))
            .unwrap_or(false);

        if !task_in_from {
            return Ok(None);
        }

        // Get next order in destination sprint
        let next_order = tasks
            .iter()
            .filter(|t| t.belongs_to_sprint(to_sprint_id))
            .filter_map(|t| t.sprint_order)
            .max()
            .unwrap_or(0)
            + 1;

        // Update the task
        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            task.assign_to_sprint(to_sprint_id, next_order);
        }

        // Renumber tasks in the source sprint
        let mut from_sprint_tasks: Vec<(String, i32)> = tasks
            .iter()
            .filter(|t| t.belongs_to_sprint(from_sprint_id))
            .map(|t| (t.id.clone(), t.sprint_order.unwrap_or(i32::MAX)))
            .collect();
        from_sprint_tasks.sort_by_key(|(_, order)| *order);

        for (new_order, (tid, _)) in from_sprint_tasks.into_iter().enumerate() {
            if let Some(t) = tasks.iter_mut().find(|t| t.id == tid) {
                t.sprint_order = Some((new_order + 1) as i32);
            }
        }

        let updated_task = tasks.iter().find(|t| t.id == task_id).cloned();
        Ok(updated_task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::Store;
    use tempfile::TempDir;

    #[test]
    fn test_json_store_basic_operations() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("tasks.json");
        let store = JsonStore::with_path(path).unwrap();

        // Add a task
        let task = Task::new("Test task".to_string());
        let added = store.add(&task).unwrap();
        assert_eq!(added.title, "Test task");

        // Get all tasks
        let all = store.get_all().unwrap();
        assert_eq!(all.len(), 1);

        // Get by ID
        let retrieved = store.get(&added.id).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().title, "Test task");

        // Update
        let mut updated = added.clone();
        updated.title = "Updated title".to_string();
        let result = store.update(&updated).unwrap();
        assert!(result.is_some());

        // Delete
        let deleted = store.delete(&added.id).unwrap();
        assert!(deleted);

        let all = store.get_all().unwrap();
        assert_eq!(all.len(), 0);
    }

    #[test]
    fn test_json_store_search() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("tasks.json");
        let store = JsonStore::with_path(path).unwrap();

        store
            .add(&Task::new("Buy grocery items".to_string()))
            .unwrap(); // contains "grocery"
        store.add(&Task::new("Finish report".to_string())).unwrap();
        store
            .add(&Task::new("Grocery shopping".to_string())) // contains "grocery" (case-insensitive)
            .unwrap();

        let results = store.search("grocery").unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_json_store_count() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("tasks.json");
        let store = JsonStore::with_path(path).unwrap();

        store.add(&Task::new("Task 1".to_string())).unwrap();
        store.add(&Task::new("Task 2".to_string())).unwrap();

        let (pending, done) = store.count().unwrap();
        assert_eq!(pending, 2);
        assert_eq!(done, 0);
    }
}
