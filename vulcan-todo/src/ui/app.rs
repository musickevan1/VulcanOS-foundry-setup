use crate::models::{Priority, Sprint, SprintStatus, Status, Task};
use crate::store::Store;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs},
    Frame,
};
use std::collections::HashSet;
use std::sync::Arc;

/// Project tab information
#[derive(Debug, Clone)]
pub struct ProjectTab {
    pub name: String,
    pub display_name: String,
    pub pending_count: usize,
    pub done_count: usize,
}

/// Application state for the TUI
#[derive(Clone)]
pub struct App {
    /// Task store
    pub store: Arc<dyn Store>,

    /// All tasks
    pub tasks: Vec<Task>,

    /// Filtered tasks (shown in list)
    pub filtered_tasks: Vec<Task>,

    /// Currently selected task index
    pub selected: usize,

    /// Current view mode
    pub view: ViewMode,

    /// Filter settings
    pub filter: TaskFilter,

    /// Search query
    pub search_query: String,

    /// Show help overlay
    pub show_help: bool,

    /// Input mode state
    pub input_mode: Option<InputMode>,

    /// Message to display (notification)
    pub message: Option<String>,

    /// Sort order
    pub sort_by: SortBy,

    /// Input buffer for text entry (used in NewTask, EditTitle, Search modes)
    pub input_buffer: String,

    /// Cursor position in input buffer
    pub input_cursor: usize,

    /// Viewport start position for input scrolling (first visible character)
    pub input_viewport_start: usize,

    /// Current project filter
    pub project_filter: Option<String>,

    /// Show project selector overlay
    pub show_project_selector: bool,

    /// Show filter builder modal
    pub show_filter_builder: bool,

    /// Show sort selector modal
    pub show_sort_selector: bool,

    /// Filter builder: selected status (None = all)
    pub filter_builder_status: Option<Status>,

    /// Filter builder: selected priority (None = all)
    pub filter_builder_priority: Option<Priority>,

    /// Filter builder: selected project (None = all)
    pub filter_builder_project: Option<String>,

    /// Filter builder: search query
    pub filter_builder_search: String,

    /// Filter builder: selected field for input
    pub filter_builder_field: FilterField,

    /// Sort selector: selected option
    pub sort_selector_option: SortBy,

    /// Project tabs (computed from tasks)
    pub tabs: Vec<ProjectTab>,

    /// Active tab index (0 = All)
    pub active_tab: usize,

    /// Status filter (None = all, Some = specific status)
    pub status_filter: Option<Status>,

    /// Multi-select mode
    pub multi_select_mode: bool,

    /// Selected task IDs (for multi-select)
    pub selected_tasks: HashSet<String>,

    /// Detail view: currently viewed task ID
    pub detail_task_id: Option<String>,

    /// Detail view: selected field for highlighting
    pub detail_field: DetailField,

    /// Detail view: edit mode (false = read-only, true = editing)
    pub detail_edit_mode: bool,

    // ==================== Sprint State ====================
    /// All sprints (loaded from store)
    pub sprints: Vec<Sprint>,

    /// Sprint view mode (shows tasks grouped by sprint)
    pub sprint_view_mode: bool,

    /// Currently selected sprint (None = backlog/all)
    pub selected_sprint: Option<String>,

    /// Sprint selector: show sprint picker
    pub show_sprint_selector: bool,

    /// Sprint selector: index in the sprint list
    pub sprint_selector_index: usize,

    /// Sprint being edited (for edit/delete operations)
    pub editing_sprint_id: Option<String>,

    /// Show move-to-sprint dialog
    pub show_move_to_sprint: bool,

    /// Move-to-sprint selector index
    pub move_to_sprint_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    List,
    Detail,
}

/// Detail view field being highlighted/edited
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetailField {
    Title,
    Description,
    Priority,
    Project,
    Tags,
    DueDate,
}

/// Filter builder field for navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterField {
    Status,
    Priority,
    Project,
    Search,
}

#[derive(Debug, Clone, Default)]
pub struct TaskFilter {
    pub status: Option<Status>,
    pub priority: Option<Priority>,
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortBy {
    Priority,
    Date,
    Alphabetical,
}

#[derive(Debug, Clone)]
pub enum InputMode {
    NewTask,
    EditTitle,
    EditDescription,
    EditTags,
    Search,
    ConfirmDelete,
    ConfirmBulkDelete,
    // Sprint modes
    NewSprint,
    EditSprintName,
    EditSprintGoal,
    ConfirmDeleteSprint,
}

impl App {
    /// Create a new app instance
    pub fn new(store: Arc<dyn Store>) -> Self {
        let mut app = Self {
            store,
            tasks: Vec::new(),
            filtered_tasks: Vec::new(),
            selected: 0,
            view: ViewMode::List,
            filter: TaskFilter::default(),
            search_query: String::new(),
            show_help: false,
            input_mode: None,
            message: None,
            sort_by: SortBy::Priority,
            input_buffer: String::new(),
            input_cursor: 0,
            input_viewport_start: 0,
            project_filter: None,
            show_project_selector: false,
            show_filter_builder: false,
            show_sort_selector: false,
            filter_builder_status: None,
            filter_builder_priority: None,
            filter_builder_project: None,
            filter_builder_search: String::new(),
            filter_builder_field: FilterField::Status,
            sort_selector_option: SortBy::Priority,
            tabs: Vec::new(),
            active_tab: 0,
            status_filter: None,
            multi_select_mode: false,
            selected_tasks: HashSet::new(),
            detail_task_id: None,
            detail_field: DetailField::Title,
            detail_edit_mode: false,
            // Sprint state
            sprints: Vec::new(),
            sprint_view_mode: false,
            selected_sprint: None,
            show_sprint_selector: false,
            sprint_selector_index: 0,
            editing_sprint_id: None,
            show_move_to_sprint: false,
            move_to_sprint_index: 0,
        };

        app.refresh_tasks();
        app.refresh_sprints();
        app
    }

    /// Refresh sprints from store
    pub fn refresh_sprints(&mut self) {
        if let Ok(sprints) = self.store.get_all_sprints() {
            self.sprints = sprints;
        }
    }

    /// Refresh tasks from store (uses cache, fast)
    pub fn refresh_tasks(&mut self) {
        if let Ok(tasks) = self.store.get_all() {
            self.tasks = tasks;
            self.rebuild_tabs();
            self.apply_filter();
        }
    }

    /// Force reload tasks from disk, bypassing cache
    pub fn reload_tasks(&mut self) {
        if let Ok(tasks) = self.store.reload() {
            self.tasks = tasks;
            self.rebuild_tabs();
            self.apply_filter();
        }
    }

    /// Rebuild tabs from current tasks
    fn rebuild_tabs(&mut self) {
        let mut tabs = Vec::new();

        // Count totals for "All" tab
        let all_pending = self.tasks.iter().filter(|t| t.is_pending()).count();
        let all_done = self.tasks.iter().filter(|t| t.is_done()).count();

        tabs.push(ProjectTab {
            name: String::new(), // Empty = all tasks
            display_name: "All".to_string(),
            pending_count: all_pending,
            done_count: all_done,
        });

        // Get project stats
        if let Ok(stats) = self.store.get_project_stats() {
            let mut projects: Vec<_> = stats.into_iter().collect();
            // Sort by pending count descending
            projects.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));

            for (name, (pending, done)) in projects {
                tabs.push(ProjectTab {
                    display_name: name.clone(),
                    name,
                    pending_count: pending,
                    done_count: done,
                });
            }
        }

        self.tabs = tabs;

        // Ensure active_tab is valid
        if self.active_tab >= self.tabs.len() {
            self.active_tab = 0;
        }
    }

    /// Apply current filter and sort
    pub fn apply_filter(&mut self) {
        let mut tasks: Vec<Task> = self.tasks.clone();

        // Apply project filter from active tab
        if self.active_tab > 0 {
            if let Some(tab) = self.tabs.get(self.active_tab) {
                let project = tab.name.clone();
                tasks.retain(|t| t.belongs_to_project(&project));
            }
        } else if let Some(ref project) = self.project_filter {
            // Legacy project filter (from P selector)
            tasks.retain(|t| t.belongs_to_project(project));
        }

        // Apply sprint filter (if in sprint view mode)
        if self.sprint_view_mode {
            if let Some(ref sprint_id) = self.selected_sprint {
                // Show tasks in specific sprint
                tasks.retain(|t| {
                    t.sprint_id
                        .as_ref()
                        .map(|s| s == sprint_id)
                        .unwrap_or(false)
                });
            } else {
                // Show backlog (tasks not in any sprint)
                tasks.retain(|t| t.sprint_id.is_none());
            }
        }

        // Apply status filter
        if let Some(ref status) = self.status_filter {
            tasks.retain(|t| &t.status == status);
        }

        // Apply search filter
        if !self.search_query.is_empty() {
            let query = self.search_query.to_lowercase();
            tasks.retain(|t| {
                t.title.to_lowercase().contains(&query)
                    || t.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&query))
                        .unwrap_or(false)
                    || t.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
            });
        }

        // Sort tasks
        if self.sprint_view_mode && self.selected_sprint.is_some() {
            // In sprint view with a sprint selected, sort by sprint_order
            tasks.sort_by(|a, b| {
                a.sprint_order
                    .unwrap_or(i32::MAX)
                    .cmp(&b.sprint_order.unwrap_or(i32::MAX))
            });
        } else {
            // Normal sorting
            match self.sort_by {
                SortBy::Priority => {
                    tasks.sort_by(|a, b| {
                        b.priority
                            .level()
                            .cmp(&a.priority.level())
                            .then_with(|| b.created_at.cmp(&a.created_at))
                    });
                }
                SortBy::Date => {
                    tasks.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                }
                SortBy::Alphabetical => {
                    tasks.sort_by(|a, b| a.title.cmp(&b.title));
                }
            }
        }

        self.filtered_tasks = tasks;

        // Ensure selected index is valid
        if self.selected >= self.filtered_tasks.len() {
            self.selected = self.filtered_tasks.len().saturating_sub(1);
        }
    }

    /// Get current task (if any)
    pub fn current_task(&self) -> Option<&Task> {
        self.filtered_tasks.get(self.selected)
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        if self.selected + 1 < self.filtered_tasks.len() {
            self.selected += 1;
        }
    }

    /// Toggle task completion
    pub fn toggle_task(&mut self) {
        if let Some(task) = self.filtered_tasks.get_mut(self.selected) {
            let mut task = task.clone();
            task.toggle();
            if let Ok(_) = self.store.update(&task) {
                self.refresh_tasks();
                let msg = if task.is_done() {
                    "completed"
                } else {
                    "reopened"
                };
                self.set_message(msg.to_string());
            }
        }
    }

    /// Delete current task
    pub fn delete_task(&mut self) {
        if let Some(task) = self.filtered_tasks.get(self.selected) {
            if let Ok(true) = self.store.delete(&task.id) {
                self.refresh_tasks();
                self.set_message("Task deleted".to_string());
            }
        }
    }

    /// Cycle task priority
    pub fn cycle_priority(&mut self) {
        if let Some(task) = self.filtered_tasks.get(self.selected) {
            let mut task = task.clone();
            task.priority = task.priority.next();
            if let Ok(_) = self.store.update(&task) {
                self.refresh_tasks();
                self.set_message(format!("Priority: {}", task.priority));
            }
        }
    }

    /// Add a new task
    pub fn add_task(&mut self, title: String) {
        let task = Task::new(title);
        if let Ok(_) = self.store.add(&task) {
            self.refresh_tasks();
            self.selected = self.filtered_tasks.len().saturating_sub(1);
            self.set_message("Task created".to_string());
        }
    }

    /// Update task title
    pub fn update_task_title(&mut self, title: String) {
        if let Some(task) = self.filtered_tasks.get(self.selected) {
            let mut task = task.clone();
            task.title = title;
            if let Ok(_) = self.store.update(&task) {
                self.refresh_tasks();
                self.set_message("Task updated".to_string());
            }
        }
    }

    /// Set search query
    pub fn set_search(&mut self, query: String) {
        self.search_query = query;
        self.apply_filter();
    }

    /// Clear search
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.apply_filter();
    }

    /// Cycle sort order
    pub fn cycle_sort(&mut self) {
        self.sort_by = match self.sort_by {
            SortBy::Priority => SortBy::Date,
            SortBy::Date => SortBy::Alphabetical,
            SortBy::Alphabetical => SortBy::Priority,
        };
        self.apply_filter();
    }

    /// Get statistics
    pub fn stats(&self) -> (usize, usize) {
        let pending = self.tasks.iter().filter(|t| t.is_pending()).count();
        let done = self.tasks.iter().filter(|t| t.is_done()).count();
        (pending, done)
    }

    /// Set notification message
    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }

    /// Clear message
    pub fn clear_message(&mut self) {
        self.message = None;
    }

    /// Toggle help
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Enter input mode
    pub fn enter_input_mode(&mut self, mode: InputMode) {
        // Pre-populate input buffer based on mode
        match &mode {
            InputMode::EditTitle => {
                if let Some(task) = self.current_task() {
                    self.input_buffer = task.title.clone();
                    self.input_cursor = self.input_buffer.len();
                }
            }
            InputMode::Search => {
                self.input_buffer = self.search_query.clone();
                self.input_cursor = self.input_buffer.len();
            }
            _ => {
                self.input_buffer.clear();
                self.input_cursor = 0;
                self.input_viewport_start = 0;
            }
        }
        self.input_mode = Some(mode);
    }

    /// Exit input mode
    pub fn exit_input_mode(&mut self) {
        self.input_mode = None;
        self.input_buffer.clear();
        self.input_cursor = 0;
        self.input_viewport_start = 0;
    }

    /// Check if in input mode
    pub fn is_input_mode(&self) -> bool {
        self.input_mode.is_some()
    }

    /// Get current input mode
    pub fn input_mode(&self) -> Option<&InputMode> {
        self.input_mode.as_ref()
    }

    /// Insert character at cursor position
    pub fn input_char(&mut self, c: char) {
        self.input_buffer.insert(self.input_cursor, c);
        self.input_cursor += 1;
        self.adjust_input_viewport();
    }

    /// Delete character before cursor (backspace)
    pub fn input_backspace(&mut self) {
        if self.input_cursor > 0 {
            self.input_cursor -= 1;
            self.input_buffer.remove(self.input_cursor);
            self.adjust_input_viewport();
        }
    }

    /// Delete character at cursor (delete)
    pub fn input_delete(&mut self) {
        if self.input_cursor < self.input_buffer.len() {
            self.input_buffer.remove(self.input_cursor);
        }
    }

    /// Move cursor left
    pub fn input_cursor_left(&mut self) {
        if self.input_cursor > 0 {
            self.input_cursor -= 1;
            self.adjust_input_viewport();
        }
    }

    /// Move cursor right
    pub fn input_cursor_right(&mut self) {
        if self.input_cursor < self.input_buffer.len() {
            self.input_cursor += 1;
            self.adjust_input_viewport();
        }
    }

    /// Move cursor to start
    pub fn input_cursor_home(&mut self) {
        self.input_cursor = 0;
        self.input_viewport_start = 0;
    }

    /// Move cursor to end
    pub fn input_cursor_end(&mut self) {
        self.input_cursor = self.input_buffer.len();
        // Viewport will be adjusted to show cursor
        self.adjust_input_viewport();
    }

    /// Adjust viewport to keep cursor visible (edge-triggered scrolling)
    /// Uses conservative width estimate (popup max 60, minus borders/margins/indicators)
    fn adjust_input_viewport(&mut self) {
        const VISIBLE_WIDTH: usize = 50; // Safe estimate for visible chars

        // Scroll left if cursor moved before viewport
        if self.input_cursor < self.input_viewport_start {
            self.input_viewport_start = self.input_cursor;
        }
        // Scroll right if cursor moved past viewport
        if self.input_cursor >= self.input_viewport_start + VISIBLE_WIDTH {
            self.input_viewport_start = self.input_cursor.saturating_sub(VISIBLE_WIDTH - 1);
        }
    }

    /// Get current input buffer content
    pub fn get_input(&self) -> &str {
        &self.input_buffer
    }

    /// Submit input based on current mode
    pub fn submit_input(&mut self) {
        let input = self.input_buffer.clone();
        let mode = self.input_mode.clone();

        match mode {
            Some(InputMode::NewTask) => {
                if !input.trim().is_empty() {
                    self.add_task(input.trim().to_string());
                }
            }
            Some(InputMode::EditTitle) => {
                if !input.trim().is_empty() {
                    self.update_task_title(input.trim().to_string());
                }
            }
            Some(InputMode::Search) => {
                self.set_search(input);
            }
            Some(InputMode::ConfirmDelete) => {
                if input.to_lowercase() == "y" || input.to_lowercase() == "yes" {
                    self.delete_task();
                }
            }
            Some(InputMode::ConfirmBulkDelete) => {
                if input.to_lowercase() == "y" || input.to_lowercase() == "yes" {
                    self.do_delete_selected_tasks();
                } else {
                    self.set_message("Bulk delete cancelled".to_string());
                }
            }
            // Sprint modes
            Some(InputMode::NewSprint) => {
                self.create_sprint();
                return; // create_sprint handles exit_input_mode
            }
            Some(InputMode::EditSprintName) => {
                self.save_sprint_edit();
                return; // save_sprint_edit handles exit_input_mode
            }
            Some(InputMode::EditSprintGoal) => {
                self.save_sprint_goal();
                return; // save_sprint_goal handles exit_input_mode
            }
            Some(InputMode::ConfirmDeleteSprint) => {
                if input.to_lowercase() == "y" || input.to_lowercase() == "yes" {
                    self.delete_sprint();
                } else {
                    self.set_message("Sprint delete cancelled".to_string());
                    self.editing_sprint_id = None;
                    self.exit_input_mode();
                }
                return;
            }
            _ => {}
        }

        self.exit_input_mode();
    }

    /// Cancel input and exit input mode
    pub fn cancel_input(&mut self) {
        self.exit_input_mode();
    }

    /// Get available projects for selector
    pub fn get_projects(&self) -> Vec<String> {
        self.store.get_projects().unwrap_or_default()
    }

    /// Set project filter
    pub fn set_project_filter(&mut self, project: Option<String>) {
        self.project_filter = project;
        self.apply_filter();
    }

    /// Toggle project selector
    pub fn toggle_project_selector(&mut self) {
        self.show_project_selector = !self.show_project_selector;
    }

    // === Tab Navigation ===

    /// Go to next tab
    pub fn next_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = (self.active_tab + 1) % self.tabs.len();
            self.sync_project_filter_from_tab();
            self.apply_filter();
            self.set_tab_message();
        }
    }

    /// Go to previous tab
    pub fn prev_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = if self.active_tab == 0 {
                self.tabs.len() - 1
            } else {
                self.active_tab - 1
            };
            self.sync_project_filter_from_tab();
            self.apply_filter();
            self.set_tab_message();
        }
    }

    /// Go to tab by number (0-9)
    pub fn go_to_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab = index;
            self.sync_project_filter_from_tab();
            self.apply_filter();
            self.set_tab_message();
        }
    }

    /// Sync project_filter from active tab
    fn sync_project_filter_from_tab(&mut self) {
        if let Some(tab) = self.tabs.get(self.active_tab) {
            if tab.name.is_empty() {
                self.project_filter = None;
            } else {
                self.project_filter = Some(tab.name.clone());
            }
        }
    }

    /// Set message showing current tab
    fn set_tab_message(&mut self) {
        if let Some(tab) = self.tabs.get(self.active_tab) {
            let msg = if tab.name.is_empty() {
                "All tasks".to_string()
            } else {
                format!("Project: {}", tab.display_name)
            };
            self.set_message(msg);
        }
    }

    // === Status Filter ===

    /// Cycle through status filters (All -> Pending -> InProgress -> Done)
    pub fn cycle_status_filter(&mut self) {
        self.status_filter = match self.status_filter {
            None => Some(Status::Pending),
            Some(Status::Pending) => Some(Status::InProgress),
            Some(Status::InProgress) => Some(Status::Done),
            Some(Status::Done) => None,
            _ => None,
        };
        self.apply_filter();
        let msg = match self.status_filter {
            None => "Showing all statuses".to_string(),
            Some(Status::Pending) => "Showing pending only".to_string(),
            Some(Status::InProgress) => "Showing in-progress only".to_string(),
            Some(Status::Done) => "Showing done only".to_string(),
            _ => "Showing all statuses".to_string(),
        };
        self.set_message(msg);
    }

    // === Multi-select ===

    /// Toggle multi-select mode
    pub fn toggle_multi_select_mode(&mut self) {
        self.multi_select_mode = !self.multi_select_mode;
        if !self.multi_select_mode {
            self.selected_tasks.clear();
        }
        let msg = if self.multi_select_mode {
            "Multi-select ON (Space to select, v to exit)".to_string()
        } else {
            "Multi-select OFF".to_string()
        };
        self.set_message(msg);
    }

    /// Toggle selection of current task (in multi-select mode)
    pub fn toggle_current_selection(&mut self) {
        if let Some(task) = self.current_task() {
            let id = task.id.clone();
            if self.selected_tasks.contains(&id) {
                self.selected_tasks.remove(&id);
            } else {
                self.selected_tasks.insert(id);
            }
        }
    }

    /// Select all visible tasks
    pub fn select_all_visible(&mut self) {
        if !self.multi_select_mode {
            self.multi_select_mode = true;
        }
        for task in &self.filtered_tasks {
            self.selected_tasks.insert(task.id.clone());
        }
        self.set_message(format!("Selected {} tasks", self.selected_tasks.len()));
    }

    /// Check if a task is selected (for multi-select)
    pub fn is_task_selected(&self, task_id: &str) -> bool {
        self.selected_tasks.contains(task_id)
    }

    /// Complete all selected tasks
    pub fn complete_selected_tasks(&mut self) {
        let mut completed = 0;
        for id in self.selected_tasks.clone() {
            if let Ok(Some(mut task)) = self.store.get(&id) {
                if task.is_pending() {
                    task.toggle();
                    if self.store.update(&task).is_ok() {
                        completed += 1;
                    }
                }
            }
        }
        self.selected_tasks.clear();
        self.multi_select_mode = false;
        self.refresh_tasks();
        self.set_message(format!("Completed {} tasks", completed));
    }

    /// Prompt for confirmation before deleting selected tasks
    pub fn delete_selected_tasks(&mut self) {
        if self.selected_tasks.is_empty() {
            return;
        }
        self.enter_input_mode(InputMode::ConfirmBulkDelete);
    }

    /// Actually delete all selected tasks (after confirmation)
    fn do_delete_selected_tasks(&mut self) {
        let mut deleted = 0;
        for id in self.selected_tasks.clone() {
            if self.store.delete(&id).is_ok() {
                deleted += 1;
            }
        }
        self.selected_tasks.clear();
        self.multi_select_mode = false;
        self.refresh_tasks();
        self.set_message(format!("Deleted {} tasks", deleted));
    }

    /// Get count of selected tasks (for display)
    pub fn selected_count(&self) -> usize {
        self.selected_tasks.len()
    }

    // === Detail View ===

    /// Open detail view for current task
    pub fn open_detail_view(&mut self) {
        if let Some(task) = self.current_task() {
            self.detail_task_id = Some(task.id.clone());
            self.detail_field = DetailField::Title;
            self.detail_edit_mode = false;
            self.view = ViewMode::Detail;
        }
    }

    /// Close detail view
    pub fn close_detail_view(&mut self) {
        self.detail_task_id = None;
        self.detail_edit_mode = false;
        self.view = ViewMode::List;
    }

    /// Check if in detail view
    pub fn is_detail_view(&self) -> bool {
        self.view == ViewMode::Detail && self.detail_task_id.is_some()
    }

    /// Get the task being viewed in detail (if any)
    pub fn detail_task(&self) -> Option<&Task> {
        if let Some(ref id) = self.detail_task_id {
            self.tasks.iter().find(|t| &t.id == id)
        } else {
            None
        }
    }

    /// Move to next field in detail view
    pub fn detail_next_field(&mut self) {
        self.detail_field = match self.detail_field {
            DetailField::Title => DetailField::Description,
            DetailField::Description => DetailField::Priority,
            DetailField::Priority => DetailField::Project,
            DetailField::Project => DetailField::Tags,
            DetailField::Tags => DetailField::DueDate,
            DetailField::DueDate => DetailField::Title, // Wrap around
        };
    }

    /// Move to previous field in detail view
    pub fn detail_prev_field(&mut self) {
        self.detail_field = match self.detail_field {
            DetailField::Title => DetailField::DueDate, // Wrap around
            DetailField::Description => DetailField::Title,
            DetailField::Priority => DetailField::Description,
            DetailField::Project => DetailField::Priority,
            DetailField::Tags => DetailField::Project,
            DetailField::DueDate => DetailField::Tags,
        };
    }

    /// Toggle task completion from detail view
    pub fn detail_toggle_task(&mut self) {
        if let Some(ref id) = self.detail_task_id.clone() {
            if let Ok(Some(mut task)) = self.store.get(id) {
                task.toggle();
                if self.store.update(&task).is_ok() {
                    self.refresh_tasks();
                    let msg = if task.is_done() {
                        "Task completed"
                    } else {
                        "Task reopened"
                    };
                    self.set_message(msg.to_string());
                }
            }
        }
    }

    /// Cycle priority from detail view
    pub fn detail_cycle_priority(&mut self) {
        if let Some(ref id) = self.detail_task_id.clone() {
            if let Ok(Some(mut task)) = self.store.get(id) {
                task.priority = task.priority.next();
                if self.store.update(&task).is_ok() {
                    self.refresh_tasks();
                    self.set_message(format!("Priority: {}", task.priority));
                }
            }
        }
    }

    /// Enter edit mode for current field (Sprint A3b)
    pub fn detail_enter_edit(&mut self) {
        if self.detail_task_id.is_some() {
            self.detail_edit_mode = true;
            // Pre-populate input buffer based on field
            if let Some(task) = self.detail_task() {
                match self.detail_field {
                    DetailField::Title => {
                        self.input_buffer = task.title.clone();
                    }
                    DetailField::Description => {
                        self.input_buffer = task.description.clone().unwrap_or_default();
                    }
                    DetailField::Project => {
                        self.input_buffer = task.project.clone().unwrap_or_default();
                    }
                    DetailField::Tags => {
                        self.input_buffer = task.tags.join(", ");
                    }
                    DetailField::DueDate => {
                        self.input_buffer = task
                            .due_date
                            .map(|d| d.format("%Y-%m-%d").to_string())
                            .unwrap_or_default();
                    }
                    DetailField::Priority => {
                        // Priority cycles, doesn't use text input
                        self.detail_cycle_priority();
                        self.detail_edit_mode = false;
                        return;
                    }
                }
                self.input_cursor = self.input_buffer.len();
            }
        }
    }

    /// Exit edit mode (cancel changes)
    pub fn detail_exit_edit(&mut self) {
        self.detail_edit_mode = false;
        self.input_buffer.clear();
        self.input_cursor = 0;
        self.input_viewport_start = 0;
    }

    /// Save edit and exit edit mode
    pub fn detail_save_edit(&mut self) {
        if let Some(ref id) = self.detail_task_id.clone() {
            if let Ok(Some(mut task)) = self.store.get(id) {
                let input = self.input_buffer.trim().to_string();

                match self.detail_field {
                    DetailField::Title => {
                        if !input.is_empty() {
                            task.title = input;
                        }
                    }
                    DetailField::Description => {
                        task.description = if input.is_empty() { None } else { Some(input) };
                    }
                    DetailField::Project => {
                        task.project = if input.is_empty() { None } else { Some(input) };
                    }
                    DetailField::Tags => {
                        task.tags = if input.is_empty() {
                            Vec::new()
                        } else {
                            input.split(',').map(|s| s.trim().to_string()).collect()
                        };
                    }
                    DetailField::DueDate => {
                        if input.is_empty() {
                            task.due_date = None;
                        } else if let Ok(date) =
                            chrono::NaiveDate::parse_from_str(&input, "%Y-%m-%d")
                        {
                            task.due_date = Some(date.and_hms_opt(12, 0, 0).unwrap().and_utc());
                        }
                    }
                    DetailField::Priority => {
                        // Priority doesn't use text edit
                    }
                }

                if self.store.update(&task).is_ok() {
                    self.refresh_tasks();
                    self.set_message("Saved".to_string());
                }
            }
        }
        self.detail_exit_edit();
    }

    // === Filter Builder ===

    /// Toggle filter builder modal
    pub fn toggle_filter_builder(&mut self) {
        self.show_filter_builder = !self.show_filter_builder;
        if self.show_filter_builder {
            // Initialize with current filters
            self.filter_builder_status = self.status_filter;
            self.filter_builder_priority = None; // Priority filter not currently used
            self.filter_builder_project = self.project_filter.clone();
            self.filter_builder_search = self.search_query.clone();
            self.filter_builder_field = FilterField::Status;
        }
    }

    /// Apply filter builder selections
    pub fn apply_filter_builder(&mut self) {
        self.status_filter = self.filter_builder_status;
        self.project_filter = self.filter_builder_project.clone();
        self.set_search(self.filter_builder_search.clone());
        self.apply_filter();
        self.show_filter_builder = false;
    }

    /// Navigate filter builder fields
    pub fn filter_builder_next_field(&mut self) {
        self.filter_builder_field = match self.filter_builder_field {
            FilterField::Status => FilterField::Priority,
            FilterField::Priority => FilterField::Project,
            FilterField::Project => FilterField::Search,
            FilterField::Search => FilterField::Status,
        };
    }

    pub fn filter_builder_prev_field(&mut self) {
        self.filter_builder_field = match self.filter_builder_field {
            FilterField::Status => FilterField::Search,
            FilterField::Search => FilterField::Project,
            FilterField::Project => FilterField::Priority,
            FilterField::Priority => FilterField::Status,
        };
    }

    /// Cycle filter builder status
    pub fn filter_builder_cycle_status(&mut self) {
        self.filter_builder_status = match self.filter_builder_status {
            None => Some(Status::Pending),
            Some(Status::Pending) => Some(Status::InProgress),
            Some(Status::InProgress) => Some(Status::Done),
            Some(Status::Done) => Some(Status::Archived),
            Some(Status::Archived) => None,
        };
    }

    /// Cycle filter builder priority
    pub fn filter_builder_cycle_priority(&mut self) {
        self.filter_builder_priority = match self.filter_builder_priority {
            None => Some(Priority::None),
            Some(Priority::None) => Some(Priority::Low),
            Some(Priority::Low) => Some(Priority::Medium),
            Some(Priority::Medium) => Some(Priority::High),
            Some(Priority::High) => Some(Priority::Urgent),
            Some(Priority::Urgent) => None,
        };
    }

    /// Cycle filter builder project
    pub fn filter_builder_cycle_project(&mut self) {
        let projects = self.get_projects();
        let current = self.filter_builder_project.clone();

        if projects.is_empty() {
            self.filter_builder_project = None;
            return;
        }

        if current.is_none() {
            self.filter_builder_project = Some(projects[0].clone());
            return;
        }

        if let Some(curr) = current {
            if let Some(pos) = projects.iter().position(|p| p == &curr) {
                let next_pos = (pos + 1) % projects.len();
                self.filter_builder_project = Some(projects[next_pos].clone());
            }
        }
    }

    /// Update filter builder search
    pub fn filter_builder_search_char(&mut self, c: char) {
        self.filter_builder_search.push(c);
    }

    pub fn filter_builder_search_backspace(&mut self) {
        self.filter_builder_search.pop();
    }

    pub fn filter_builder_clear_search(&mut self) {
        self.filter_builder_search.clear();
    }

    // === Sort Selector ===

    /// Toggle sort selector modal
    pub fn toggle_sort_selector(&mut self) {
        self.show_sort_selector = !self.show_sort_selector;
        if self.show_sort_selector {
            self.sort_selector_option = self.sort_by;
        }
    }

    /// Select sort option
    pub fn select_sort_option(&mut self) {
        self.sort_by = self.sort_selector_option;
        self.apply_filter();
        self.show_sort_selector = false;
        let msg = match self.sort_by {
            SortBy::Priority => "Sorted by priority".to_string(),
            SortBy::Date => "Sorted by date".to_string(),
            SortBy::Alphabetical => "Sorted alphabetically".to_string(),
        };
        self.set_message(msg);
    }

    /// Cycle sort options
    pub fn cycle_sort_option(&mut self) {
        self.sort_selector_option = match self.sort_selector_option {
            SortBy::Priority => SortBy::Date,
            SortBy::Date => SortBy::Alphabetical,
            SortBy::Alphabetical => SortBy::Priority,
        };
    }

    // ==================== Sprint Methods ====================

    /// Toggle sprint view mode
    pub fn toggle_sprint_view(&mut self) {
        self.sprint_view_mode = !self.sprint_view_mode;
        if self.sprint_view_mode {
            self.set_message("Sprint view enabled".to_string());
        } else {
            self.selected_sprint = None;
            self.set_message("Sprint view disabled".to_string());
        }
        self.apply_filter();
    }

    /// Toggle sprint selector modal
    pub fn toggle_sprint_selector(&mut self) {
        self.show_sprint_selector = !self.show_sprint_selector;
        if self.show_sprint_selector {
            self.sprint_selector_index = 0;
            self.refresh_sprints();
        }
    }

    /// Get sprints for current project
    pub fn get_project_sprints(&self) -> Vec<&Sprint> {
        if let Some(ref project) = self.project_filter {
            self.sprints
                .iter()
                .filter(|s| s.project == *project && s.status != SprintStatus::Archived)
                .collect()
        } else {
            self.sprints
                .iter()
                .filter(|s| s.status != SprintStatus::Archived)
                .collect()
        }
    }

    /// Navigate sprint selector up
    pub fn sprint_selector_up(&mut self) {
        if self.sprint_selector_index > 0 {
            self.sprint_selector_index -= 1;
        }
    }

    /// Navigate sprint selector down
    pub fn sprint_selector_down(&mut self) {
        let sprints = self.get_project_sprints();
        // +1 for "Backlog" option
        let max_index = sprints.len();
        if self.sprint_selector_index < max_index {
            self.sprint_selector_index += 1;
        }
    }

    /// Select sprint from selector
    pub fn select_sprint(&mut self) {
        if self.sprint_selector_index == 0 {
            // Backlog selected
            self.selected_sprint = None;
            self.set_message("Showing backlog".to_string());
        } else {
            // Sprint selected - get sprint info before modifying self
            let sprint_info: Option<(String, String)> = {
                let sprints = self.get_project_sprints();
                sprints
                    .get(self.sprint_selector_index - 1)
                    .map(|s| (s.id.clone(), s.name.clone()))
            };

            if let Some((id, name)) = sprint_info {
                self.selected_sprint = Some(id);
                self.set_message(format!("Sprint: {}", name));
            }
        }
        self.show_sprint_selector = false;
        self.apply_filter();
    }

    /// Get current sprint name for display
    pub fn current_sprint_name(&self) -> String {
        if let Some(ref sprint_id) = self.selected_sprint {
            self.sprints
                .iter()
                .find(|s| s.id == *sprint_id)
                .map(|s| s.name.clone())
                .unwrap_or_else(|| "Unknown".to_string())
        } else if self.sprint_view_mode {
            "Backlog".to_string()
        } else {
            "All".to_string()
        }
    }

    /// Get current task's sprint order (if in a sprint)
    pub fn current_task_sprint_order(&self) -> Option<i32> {
        if let Some(task) = self.current_task() {
            task.sprint_order
        } else {
            None
        }
    }

    /// Assign current task to a sprint
    pub fn assign_current_task_to_sprint(&mut self, sprint_id: &str) {
        if let Some(task) = self.current_task() {
            let task_id = task.id.clone();
            if self
                .store
                .assign_task_to_sprint(&task_id, sprint_id)
                .is_ok()
            {
                self.refresh_tasks();
                self.refresh_sprints();
                self.set_message("Task assigned to sprint".to_string());
            }
        }
    }

    /// Remove current task from its sprint
    pub fn remove_current_task_from_sprint(&mut self) {
        if let Some(task) = self.current_task() {
            let task_id = task.id.clone();
            if self.store.remove_task_from_sprint(&task_id).is_ok() {
                self.refresh_tasks();
                self.set_message("Task removed from sprint".to_string());
            }
        }
    }

    /// Reorder current task within its sprint
    pub fn reorder_current_task(&mut self, direction: i32) {
        if let Some(task) = self.current_task() {
            if let Some(current_order) = task.sprint_order {
                let task_id = task.id.clone();
                let new_position = (current_order + direction).max(1);
                if self
                    .store
                    .reorder_task_in_sprint(&task_id, new_position)
                    .is_ok()
                {
                    self.refresh_tasks();
                    self.apply_filter();
                }
            }
        }
    }

    // ==================== Sprint CRUD Methods ====================

    /// Enter sprint creation mode
    pub fn enter_new_sprint_mode(&mut self) {
        self.input_buffer.clear();
        self.input_cursor = 0;
        self.input_viewport_start = 0;
        self.input_mode = Some(InputMode::NewSprint);
    }

    /// Create a new sprint with the current input
    pub fn create_sprint(&mut self) {
        let name = self.input_buffer.trim().to_string();
        if name.is_empty() {
            self.set_message("Sprint name cannot be empty".to_string());
            return;
        }

        // Determine project for the sprint
        let project = self
            .project_filter
            .clone()
            .unwrap_or_else(|| "default".to_string());

        let sprint = Sprint::new(name.clone(), project);
        if let Err(e) = self.store.add_sprint(&sprint) {
            self.set_message(format!("Failed to create sprint: {}", e));
        } else {
            self.refresh_sprints();
            self.set_message(format!("Sprint '{}' created", name));
        }
        self.exit_input_mode();
    }

    /// Enter sprint edit mode for the selected sprint
    pub fn enter_edit_sprint_mode(&mut self) {
        if self.sprint_selector_index == 0 {
            // Can't edit backlog
            self.set_message("Cannot edit backlog".to_string());
            return;
        }

        // Get sprint info before modifying self
        let sprint_info: Option<(String, String)> = {
            let sprints = self.get_project_sprints();
            sprints
                .get(self.sprint_selector_index - 1)
                .map(|s| (s.id.clone(), s.name.clone()))
        };

        if let Some((id, name)) = sprint_info {
            self.editing_sprint_id = Some(id);
            self.input_buffer = name;
            self.input_cursor = self.input_buffer.len();
            self.input_mode = Some(InputMode::EditSprintName);
        }
    }

    /// Save sprint name edit
    pub fn save_sprint_edit(&mut self) {
        let new_name = self.input_buffer.trim().to_string();
        if new_name.is_empty() {
            self.set_message("Sprint name cannot be empty".to_string());
            self.exit_input_mode();
            return;
        }

        if let Some(ref sprint_id) = self.editing_sprint_id.clone() {
            if let Ok(Some(mut sprint)) = self.store.get_sprint(sprint_id) {
                sprint.name = new_name.clone();
                if let Err(e) = self.store.update_sprint(&sprint) {
                    self.set_message(format!("Failed to update sprint: {}", e));
                } else {
                    self.refresh_sprints();
                    self.set_message(format!("Sprint renamed to '{}'", new_name));
                }
            }
        }
        self.editing_sprint_id = None;
        self.exit_input_mode();
    }

    /// Enter sprint goal edit mode
    pub fn enter_edit_sprint_goal_mode(&mut self) {
        if self.sprint_selector_index == 0 {
            self.set_message("Cannot set goal for backlog".to_string());
            return;
        }

        // Get sprint info before modifying self
        let sprint_info: Option<(String, Option<String>)> = {
            let sprints = self.get_project_sprints();
            sprints
                .get(self.sprint_selector_index - 1)
                .map(|s| (s.id.clone(), s.goal.clone()))
        };

        if let Some((id, goal)) = sprint_info {
            self.editing_sprint_id = Some(id);
            self.input_buffer = goal.unwrap_or_default();
            self.input_cursor = self.input_buffer.len();
            self.input_mode = Some(InputMode::EditSprintGoal);
        }
    }

    /// Save sprint goal edit
    pub fn save_sprint_goal(&mut self) {
        let new_goal = self.input_buffer.trim().to_string();

        if let Some(ref sprint_id) = self.editing_sprint_id.clone() {
            if let Ok(Some(mut sprint)) = self.store.get_sprint(sprint_id) {
                sprint.goal = if new_goal.is_empty() {
                    None
                } else {
                    Some(new_goal.clone())
                };
                if let Err(e) = self.store.update_sprint(&sprint) {
                    self.set_message(format!("Failed to update sprint: {}", e));
                } else {
                    self.refresh_sprints();
                    if new_goal.is_empty() {
                        self.set_message("Sprint goal cleared".to_string());
                    } else {
                        self.set_message("Sprint goal updated".to_string());
                    }
                }
            }
        }
        self.editing_sprint_id = None;
        self.exit_input_mode();
    }

    /// Enter sprint delete confirmation mode
    pub fn enter_delete_sprint_mode(&mut self) {
        let sprints = self.get_project_sprints();
        if self.sprint_selector_index == 0 {
            self.set_message("Cannot delete backlog".to_string());
            return;
        }

        if let Some(sprint) = sprints.get(self.sprint_selector_index - 1) {
            self.editing_sprint_id = Some(sprint.id.clone());
            self.input_buffer.clear();
            self.input_cursor = 0;
            self.input_viewport_start = 0;
            self.input_mode = Some(InputMode::ConfirmDeleteSprint);
        }
    }

    /// Delete the sprint being edited
    pub fn delete_sprint(&mut self) {
        if let Some(ref sprint_id) = self.editing_sprint_id.clone() {
            if let Err(e) = self.store.delete_sprint(sprint_id) {
                self.set_message(format!("Failed to delete sprint: {}", e));
            } else {
                self.refresh_sprints();
                self.refresh_tasks(); // Tasks are unassigned from deleted sprint
                self.set_message("Sprint deleted".to_string());
                // Reset selector index if needed
                if self.sprint_selector_index > 0 {
                    self.sprint_selector_index = self.sprint_selector_index.saturating_sub(1);
                }
            }
        }
        self.editing_sprint_id = None;
        self.exit_input_mode();
    }

    /// Start the selected sprint (change status to Active)
    pub fn start_sprint(&mut self) {
        let sprints = self.get_project_sprints();
        if self.sprint_selector_index == 0 {
            self.set_message("Cannot start backlog".to_string());
            return;
        }

        if let Some(sprint) = sprints.get(self.sprint_selector_index - 1) {
            let sprint_id = sprint.id.clone();
            if let Ok(Some(mut s)) = self.store.get_sprint(&sprint_id) {
                if s.status == SprintStatus::Active {
                    self.set_message("Sprint is already active".to_string());
                    return;
                }
                s.status = SprintStatus::Active;
                s.start_date = Some(chrono::Utc::now());
                if let Err(e) = self.store.update_sprint(&s) {
                    self.set_message(format!("Failed to start sprint: {}", e));
                } else {
                    self.refresh_sprints();
                    self.set_message(format!("Sprint '{}' started", s.name));
                }
            }
        }
    }

    /// Complete the selected sprint (change status to Completed)
    pub fn complete_sprint(&mut self) {
        let sprints = self.get_project_sprints();
        if self.sprint_selector_index == 0 {
            self.set_message("Cannot complete backlog".to_string());
            return;
        }

        if let Some(sprint) = sprints.get(self.sprint_selector_index - 1) {
            let sprint_id = sprint.id.clone();
            if let Ok(Some(mut s)) = self.store.get_sprint(&sprint_id) {
                if s.status == SprintStatus::Completed {
                    self.set_message("Sprint is already completed".to_string());
                    return;
                }
                s.status = SprintStatus::Completed;
                s.end_date = Some(chrono::Utc::now());
                if let Err(e) = self.store.update_sprint(&s) {
                    self.set_message(format!("Failed to complete sprint: {}", e));
                } else {
                    self.refresh_sprints();
                    self.set_message(format!("Sprint '{}' completed", s.name));
                }
            }
        }
    }

    // ==================== Move Task to Sprint ====================

    /// Toggle move-to-sprint dialog
    pub fn toggle_move_to_sprint(&mut self) {
        if self.current_task().is_none() {
            self.set_message("No task selected".to_string());
            return;
        }
        self.show_move_to_sprint = !self.show_move_to_sprint;
        if self.show_move_to_sprint {
            self.move_to_sprint_index = 0;
            self.refresh_sprints();
        }
    }

    /// Navigate move-to-sprint selector up
    pub fn move_to_sprint_up(&mut self) {
        if self.move_to_sprint_index > 0 {
            self.move_to_sprint_index -= 1;
        }
    }

    /// Navigate move-to-sprint selector down
    pub fn move_to_sprint_down(&mut self) {
        let sprints = self.get_project_sprints();
        // +1 for "Remove from sprint" option
        let max_index = sprints.len() + 1;
        if self.move_to_sprint_index < max_index {
            self.move_to_sprint_index += 1;
        }
    }

    /// Execute move task to sprint
    pub fn execute_move_to_sprint(&mut self) {
        let task = match self.current_task() {
            Some(t) => t.clone(),
            None => return,
        };

        let sprints = self.get_project_sprints();

        if self.move_to_sprint_index == 0 {
            // Remove from sprint
            if task.sprint_id.is_some() {
                if let Err(e) = self.store.remove_task_from_sprint(&task.id) {
                    self.set_message(format!("Failed to remove from sprint: {}", e));
                } else {
                    self.refresh_tasks();
                    self.set_message("Task moved to backlog".to_string());
                }
            } else {
                self.set_message("Task is already in backlog".to_string());
            }
        } else if let Some(sprint) = sprints.get(self.move_to_sprint_index - 1) {
            // Assign to sprint
            let sprint_id = sprint.id.clone();
            let sprint_name = sprint.name.clone();
            if let Err(e) = self.store.assign_task_to_sprint(&task.id, &sprint_id) {
                self.set_message(format!("Failed to assign to sprint: {}", e));
            } else {
                self.refresh_tasks();
                self.set_message(format!("Task moved to '{}'", sprint_name));
            }
        }

        self.show_move_to_sprint = false;
        self.apply_filter();
    }

    /// Get sprint name by ID (for display)
    pub fn get_sprint_name(&self, sprint_id: &str) -> Option<String> {
        self.sprints
            .iter()
            .find(|s| s.id == sprint_id)
            .map(|s| s.name.clone())
    }
}

/// Render the main TUI
pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Tab bar
            Constraint::Length(1), // Title bar
            Constraint::Min(1),    // Content
            Constraint::Length(1), // Status bar
        ])
        .split(frame.size());

    render_tab_bar(app, frame, chunks[0]);
    render_title_bar(app, frame, chunks[1]);
    render_content(app, frame, chunks[2]);
    render_status_bar(app, frame, chunks[3]);

    // Render overlays (order matters - later overlays appear on top)
    if app.is_detail_view() {
        render_detail_view(app, frame);
    }

    if app.show_filter_builder {
        render_filter_builder(app, frame);
    } else if app.show_sort_selector {
        render_sort_selector(app, frame);
    } else if app.show_project_selector {
        render_project_selector(app, frame);
    } else if app.show_sprint_selector {
        render_sprint_selector(app, frame);
    } else if app.show_move_to_sprint {
        render_move_to_sprint(app, frame);
    }

    if app.show_help {
        render_help_overlay(frame);
    }

    if app.is_input_mode() {
        render_input_dialog(app, frame);
    }
}

/// Render tab bar with project tabs
fn render_tab_bar(app: &App, frame: &mut Frame, area: Rect) {
    let tab_titles: Vec<Line> = app
        .tabs
        .iter()
        .enumerate()
        .map(|(i, tab)| {
            let num = if i < 10 {
                format!("{}.", i)
            } else {
                String::new()
            };
            let count = tab.pending_count;
            let name = &tab.display_name;
            Line::from(format!("{}{} ({})", num, name, count))
        })
        .collect();

    let tabs = Tabs::new(tab_titles)
        .select(app.active_tab)
        .style(Style::default().fg(Color::DarkGray))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .divider(Span::raw(" | "));

    frame.render_widget(tabs, area);
}

/// Render title bar
fn render_title_bar(app: &App, frame: &mut Frame, area: Rect) {
    let (pending, done) = app.stats();
    let total = pending + done;
    let percent = if total > 0 {
        (done as f64 / total as f64 * 100.0).round() as u16
    } else {
        0
    };

    // Build progress bar (mini)
    let progress_width = 10;
    let filled = (percent as usize * progress_width / 100).min(progress_width);
    let progress_bar = format!(
        "[{}{}]",
        "█".repeat(filled),
        "░".repeat(progress_width - filled)
    );

    let search_info = if app.search_query.is_empty() {
        String::new()
    } else {
        format!(" │ 🔍 \"{}\"", app.search_query)
    };

    // Sprint info (if in sprint view)
    let sprint_info = if app.sprint_view_mode {
        format!(" │ 🏃 {}", app.current_sprint_name())
    } else {
        String::new()
    };

    // Left side: title and stats
    let left = format!(
        " 🌋 Vulcan Todo │ {} pending │ {} done {}{}{}",
        pending, done, progress_bar, sprint_info, search_info
    );

    // Right side: help hint
    let right = "? help ".to_string();

    let padding = if area.width as usize > left.len() + right.len() {
        " ".repeat(area.width as usize - left.len() - right.len())
    } else {
        String::new()
    };

    let title_line = Line::from(vec![
        Span::styled(left, Style::default().fg(Color::White)),
        Span::raw(padding),
        Span::styled(right, Style::default().fg(Color::DarkGray)),
    ]);

    let block = Block::default().style(Style::default().bg(Color::Rgb(40, 40, 50)));

    let paragraph = Paragraph::new(title_line).block(block);

    frame.render_widget(paragraph, area);
}

/// Render main content
fn render_content(app: &mut App, frame: &mut Frame, area: Rect) {
    if app.filtered_tasks.is_empty() {
        let message = if !app.search_query.is_empty() {
            format!(
                "No tasks found for '{}'. Press '/' to clear search.",
                app.search_query
            )
        } else {
            "No tasks yet. Press 'n' to create one.".to_string()
        };

        let paragraph = Paragraph::new(message)
            .style(Style::default().fg(Color::Gray))
            .block(
                Block::default()
                    .title("Tasks")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .alignment(ratatui::layout::Alignment::Center);

        frame.render_widget(paragraph, area);
        return;
    }

    let list_width = area.width.saturating_sub(4); // Account for borders and highlight symbol
    let items: Vec<ListItem> = app
        .filtered_tasks
        .iter()
        .enumerate()
        .map(|(_, task)| {
            let is_multi_selected = app.multi_select_mode && app.is_task_selected(&task.id);
            ListItem::new(render_task_row(task, is_multi_selected, list_width))
        })
        .collect();

    // Build title based on view mode
    let title = if app.sprint_view_mode {
        if app.selected_sprint.is_some() {
            format!(" 🏃 {} ", app.current_sprint_name())
        } else {
            " 📋 Backlog ".to_string()
        }
    } else {
        " Tasks ".to_string()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if app.sprint_view_mode {
                    Color::Magenta
                } else {
                    Color::DarkGray
                })),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    // Use stateful rendering to show selection
    let mut list_state = ListState::default();
    list_state.select(Some(app.selected));

    frame.render_stateful_widget(list, area, &mut list_state);
}

/// Render a single task row with enhanced display
fn render_task_row(task: &Task, is_multi_selected: bool, width: u16) -> Line<'static> {
    let checkbox = match task.status {
        Status::Done => "[✓]",
        Status::InProgress => "[◐]",
        _ => "[ ]",
    };
    let priority = task.priority.emoji();

    let project = match &task.project {
        Some(p) => format!("[{}] ", p),
        None => String::new(),
    };

    // Format due date if present
    let due_info = format_due_date(task);

    // Format age/completion info
    let age_info = format_task_age(task);

    // Calculate available width for title
    // Base: select(2) + checkbox(3) + space + priority(2) + space + project + due + age + padding
    let fixed_width = 2 + 3 + 1 + 2 + 1 + project.len() + due_info.len() + age_info.len() + 4;
    let available_width = (width as usize).saturating_sub(fixed_width);

    // Truncate title if needed
    let title = if task.title.len() > available_width && available_width > 3 {
        format!("{}...", &task.title[..available_width - 3])
    } else {
        task.title.clone()
    };

    let line_style = if task.is_done() {
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::CROSSED_OUT)
    } else {
        Style::default()
    };

    // Multi-select indicator
    let select_indicator = if is_multi_selected { "● " } else { "  " };

    // Due date color based on urgency
    let due_style = if task.is_done() {
        Style::default().fg(Color::DarkGray)
    } else if let Some(due) = &task.due_date {
        let now = chrono::Utc::now();
        if *due < now {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD) // Overdue
        } else if *due < now + chrono::Duration::days(1) {
            Style::default().fg(Color::Yellow) // Due today/tomorrow
        } else if *due < now + chrono::Duration::days(7) {
            Style::default().fg(Color::Cyan) // Due this week
        } else {
            Style::default().fg(Color::DarkGray)
        }
    } else {
        Style::default().fg(Color::DarkGray)
    };

    Line::from(vec![
        Span::styled(
            select_indicator,
            if is_multi_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            },
        ),
        Span::styled(checkbox, Style::default().fg(Color::Green)),
        Span::raw(" "),
        Span::raw(priority),
        Span::raw(" "),
        Span::styled(project, Style::default().fg(Color::Magenta)),
        Span::styled(title, line_style),
        Span::styled(due_info, due_style),
        Span::styled(age_info, Style::default().fg(Color::DarkGray)),
    ])
}

/// Format due date for display
fn format_due_date(task: &Task) -> String {
    match &task.due_date {
        Some(due) => {
            let now = chrono::Utc::now();
            let days_until = (*due - now).num_days();

            if days_until < 0 {
                format!(" ⚠️{}d ago", -days_until)
            } else if days_until == 0 {
                " 📅today".to_string()
            } else if days_until == 1 {
                " 📅tomorrow".to_string()
            } else if days_until < 7 {
                format!(" 📅{}d", days_until)
            } else {
                format!(" 📅{}", due.format("%m/%d"))
            }
        }
        None => String::new(),
    }
}

/// Format task age or completion time
fn format_task_age(task: &Task) -> String {
    let now = chrono::Utc::now();

    if task.is_done() {
        if let Some(completed) = &task.completed_at {
            let days_ago = (now - *completed).num_days();
            if days_ago == 0 {
                " ✓today".to_string()
            } else if days_ago == 1 {
                " ✓1d ago".to_string()
            } else if days_ago < 30 {
                format!(" ✓{}d ago", days_ago)
            } else {
                format!(" ✓{}", completed.format("%m/%d"))
            }
        } else {
            String::new()
        }
    } else {
        // Show age for pending tasks (how long it's been open)
        let days_old = (now - task.created_at).num_days();
        if days_old == 0 {
            " (new)".to_string()
        } else if days_old < 7 {
            format!(" ({}d)", days_old)
        } else if days_old < 30 {
            format!(" ({}w)", days_old / 7)
        } else {
            format!(" ({}mo)", days_old / 30)
        }
    }
}

/// Render status bar
fn render_status_bar(app: &App, frame: &mut Frame, area: Rect) {
    let message = app.message.clone().unwrap_or_default();

    // Build left side info
    let task_info = if app.filtered_tasks.len() == app.tasks.len() {
        format!("{} tasks", app.tasks.len())
    } else {
        format!("{}/{} tasks", app.filtered_tasks.len(), app.tasks.len())
    };

    let mode_info = if app.multi_select_mode {
        format!(" 📌 {} selected", app.selected_tasks.len())
    } else {
        String::new()
    };

    let status_info = match app.status_filter {
        Some(Status::Pending) => " │ ⏳ pending",
        Some(Status::InProgress) => " │ ◐ in-progress",
        Some(Status::Done) => " │ ✅ done",
        _ => "",
    };

    let sort_info = match app.sort_by {
        SortBy::Priority => " │ ↓pri",
        SortBy::Date => " │ ↓date",
        SortBy::Alphabetical => " │ ↓A-Z",
    };

    // Build right side (keybindings hint)
    let keys = if app.multi_select_mode {
        "Space:sel │ x:done │ d:del │ v:exit"
    } else {
        "?:help │ n:new │ x:done │ /:find │ q:quit"
    };

    // Calculate spacing
    let left_content = if !message.is_empty() {
        format!(" {}", message)
    } else {
        format!(" {}{}{}{}", task_info, mode_info, status_info, sort_info)
    };

    let right_content = format!("{} ", keys);
    let total_len = left_content.len() + right_content.len();
    let padding = if area.width as usize > total_len {
        " ".repeat(area.width as usize - total_len)
    } else {
        String::new()
    };

    let status_line = Line::from(vec![
        Span::styled(
            left_content,
            if !message.is_empty() {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Gray)
            },
        ),
        Span::raw(padding),
        Span::styled(right_content, Style::default().fg(Color::DarkGray)),
    ]);

    let block = Block::default().style(Style::default().bg(Color::Rgb(30, 30, 30)));

    let paragraph = Paragraph::new(status_line).block(block);

    frame.render_widget(paragraph, area);
}

/// Render help overlay
fn render_help_overlay(frame: &mut Frame) {
    let area = frame.size();
    let popup_width = std::cmp::min(55, area.width.saturating_sub(4));
    let popup_height = 46;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;
    let rect = Rect::new(popup_x, popup_y, popup_width, popup_height);

    frame.render_widget(Clear, rect);

    let help_text = r#" VulcanOS Todo - Keybindings

  Navigation:
    j/k, ↑/↓   Next/Prev task
    g/G        First/Last task
    Tab        Next project tab
    Shift+Tab  Previous project tab
    0-9        Jump to tab by number
    
  Task Actions:
    Enter      Open detail view
    n          New task
    e          Edit task title (quick)
    x/Space    Toggle complete
    d          Delete (with confirm)
    p          Cycle priority
    m          Move task to sprint
    P          Project selector

  Sprint Management:
    S          Toggle sprint view mode
    Shift+S    Open sprint selector
    J/K        Reorder task in sprint (Ctrl+↑/↓)
    In sprint selector:
      n/e/d    New/Edit/Delete sprint
      s/c      Start/Complete sprint
      g        Edit sprint goal

  Filtering:
    /          Search tasks
    s          Cycle status (all/pending/done)
    o          Cycle sort order
    c          Clear all filters

  Multi-select:
    v          Toggle multi-select mode
    V          Select all visible
    Space      Toggle selection (in mode)
    x/d        Complete/Delete selected

  Other:
    r          Sync from disk
    ?          Toggle help | q  Quit
"#;

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::White));

    let paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(block);

    frame.render_widget(paragraph, rect);
}

/// Render input dialog overlay
fn render_input_dialog(app: &App, frame: &mut Frame) {
    let area = frame.size();
    let popup_width = std::cmp::min(60, area.width.saturating_sub(4));
    let popup_height = 5;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;
    let rect = Rect::new(popup_x, popup_y, popup_width, popup_height);

    frame.render_widget(Clear, rect);

    let (title, prompt) = match app.input_mode() {
        Some(InputMode::NewTask) => (" New Task ", "Enter task title:"),
        Some(InputMode::EditTitle) => (" Edit Task ", "Edit task title:"),
        Some(InputMode::EditDescription) => (" Description ", "Enter description:"),
        Some(InputMode::EditTags) => (" Tags ", "Enter tags (comma-separated):"),
        Some(InputMode::Search) => (" Search ", "Search tasks:"),
        Some(InputMode::ConfirmDelete) => (" Confirm Delete ", "Delete this task? (y/n):"),
        Some(InputMode::ConfirmBulkDelete) => {
            let count = app.selected_count();
            // We need to return owned strings for this case
            return render_bulk_delete_dialog(app, frame, count);
        }
        // Sprint modes
        Some(InputMode::NewSprint) => (" New Sprint ", "Enter sprint name:"),
        Some(InputMode::EditSprintName) => (" Edit Sprint ", "Edit sprint name:"),
        Some(InputMode::EditSprintGoal) => (" Sprint Goal ", "Enter sprint goal:"),
        Some(InputMode::ConfirmDeleteSprint) => {
            return render_sprint_delete_dialog(app, frame);
        }
        None => return,
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(block.clone(), rect);

    // Inner area for content
    let inner = rect.inner(&ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(inner);

    // Prompt text
    let prompt_widget = Paragraph::new(prompt).style(Style::default().fg(Color::Gray));
    frame.render_widget(prompt_widget, chunks[0]);

    // Input field with cursor and horizontal scrolling
    let input = app.get_input();
    let cursor_pos = app.input_cursor;
    let viewport_start = app.input_viewport_start;

    // Calculate available width for text (reserve space for scroll indicators)
    let area_width = chunks[1].width as usize;
    let has_left_overflow = viewport_start > 0;
    let has_right_overflow = input.len() > viewport_start + area_width.saturating_sub(2);

    // Reserve 1 char for left indicator, 1 for right indicator when needed
    let left_indicator_width = if has_left_overflow { 1 } else { 0 };
    let right_indicator_width = if has_right_overflow { 1 } else { 0 };
    let text_width = area_width.saturating_sub(left_indicator_width + right_indicator_width);

    // Extract visible portion of text
    let visible_end = std::cmp::min(viewport_start + text_width, input.len());
    let visible_text = if viewport_start < input.len() {
        &input[viewport_start..visible_end]
    } else {
        ""
    };

    // Calculate cursor position relative to viewport
    let relative_cursor = cursor_pos.saturating_sub(viewport_start);

    // Build spans for the input line
    let mut spans = Vec::new();

    // Left scroll indicator
    if has_left_overflow {
        spans.push(Span::styled("<", Style::default().fg(Color::Yellow)));
    }

    // Text before cursor (within visible area)
    let before_len = relative_cursor.min(visible_text.len());
    if before_len > 0 {
        spans.push(Span::styled(
            &visible_text[..before_len],
            Style::default().fg(Color::White),
        ));
    }

    // Cursor character
    let cursor_char = if relative_cursor < visible_text.len() {
        &visible_text[relative_cursor..relative_cursor + 1]
    } else {
        " " // Cursor at end of text
    };
    spans.push(Span::styled(
        cursor_char,
        Style::default().bg(Color::White).fg(Color::Black),
    ));

    // Text after cursor (within visible area)
    let after_start = relative_cursor + 1;
    if after_start < visible_text.len() {
        spans.push(Span::styled(
            &visible_text[after_start..],
            Style::default().fg(Color::White),
        ));
    }

    // Right scroll indicator
    if has_right_overflow {
        spans.push(Span::styled(">", Style::default().fg(Color::Yellow)));
    }

    let input_line = Line::from(spans);
    let input_widget = Paragraph::new(input_line).style(Style::default().bg(Color::DarkGray));
    frame.render_widget(input_widget, chunks[1]);
}

/// Render bulk delete confirmation dialog
fn render_bulk_delete_dialog(app: &App, frame: &mut Frame, count: usize) {
    let area = frame.size();
    let popup_width = std::cmp::min(60, area.width.saturating_sub(4));
    let popup_height = 5;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;
    let rect = Rect::new(popup_x, popup_y, popup_width, popup_height);

    frame.render_widget(Clear, rect);

    let title = " Confirm Bulk Delete ";
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(block.clone(), rect);

    // Inner area for content
    let inner = rect.inner(&ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(inner);

    let prompt = format!("Delete {} selected tasks? (y/n):", count);
    let prompt_widget = Paragraph::new(prompt).style(Style::default().fg(Color::Yellow));
    frame.render_widget(prompt_widget, chunks[0]);
}

/// Render detail view panel
fn render_detail_view(app: &App, frame: &mut Frame) {
    let task = match app.detail_task() {
        Some(t) => t,
        None => return,
    };

    let area = frame.size();
    let popup_width = std::cmp::min(70, area.width.saturating_sub(4));
    let popup_height = std::cmp::min(22, area.height.saturating_sub(4));
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;
    let rect = Rect::new(popup_x, popup_y, popup_width, popup_height);

    frame.render_widget(Clear, rect);

    // Title shows task status
    let status_icon = if task.is_done() { "✓" } else { "○" };
    let mode_hint = if app.detail_edit_mode {
        " [EDIT - Esc:cancel Enter:save]"
    } else {
        " [j/k:nav i:edit x:toggle p:pri q:close]"
    };
    let title = format!(" {} Task Detail{} ", status_icon, mode_hint);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if app.detail_edit_mode {
            Color::Yellow
        } else {
            Color::Cyan
        }))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(block.clone(), rect);

    let inner = rect.inner(&ratatui::layout::Margin {
        horizontal: 2,
        vertical: 1,
    });

    // Build content lines
    let mut lines: Vec<Line> = Vec::new();

    // Helper to render a field row
    let render_field = |label: &str,
                        value: &str,
                        field: DetailField,
                        current: DetailField,
                        editing: bool|
     -> Line<'static> {
        let is_selected = field == current;
        let label_style = Style::default().fg(Color::DarkGray);
        let value_style = if is_selected && editing {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let indicator = if is_selected { "▶ " } else { "  " };

        Line::from(vec![
            Span::styled(
                indicator,
                if is_selected {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                },
            ),
            Span::styled(format!("{:<12}", label), label_style),
            Span::styled(value.to_string(), value_style),
        ])
    };

    // Title
    let title_value = if app.detail_edit_mode && app.detail_field == DetailField::Title {
        format!("{}▏", &app.input_buffer)
    } else {
        task.title.clone()
    };
    lines.push(render_field(
        "Title:",
        &title_value,
        DetailField::Title,
        app.detail_field,
        app.detail_edit_mode,
    ));
    lines.push(Line::from(""));

    // Description (multi-line capable)
    let desc_value = if app.detail_edit_mode && app.detail_field == DetailField::Description {
        format!("{}▏", &app.input_buffer)
    } else {
        task.description
            .clone()
            .unwrap_or_else(|| "(none)".to_string())
    };
    lines.push(render_field(
        "Description:",
        &desc_value,
        DetailField::Description,
        app.detail_field,
        app.detail_edit_mode,
    ));
    lines.push(Line::from(""));

    // Priority
    let pri_value = format!("{} {}", task.priority.emoji(), task.priority.label());
    lines.push(render_field(
        "Priority:",
        &pri_value,
        DetailField::Priority,
        app.detail_field,
        app.detail_edit_mode,
    ));
    lines.push(Line::from(""));

    // Project
    let proj_value = if app.detail_edit_mode && app.detail_field == DetailField::Project {
        format!("{}▏", &app.input_buffer)
    } else {
        task.project.clone().unwrap_or_else(|| "(none)".to_string())
    };
    lines.push(render_field(
        "Project:",
        &proj_value,
        DetailField::Project,
        app.detail_field,
        app.detail_edit_mode,
    ));
    lines.push(Line::from(""));

    // Tags
    let tags_value = if app.detail_edit_mode && app.detail_field == DetailField::Tags {
        format!("{}▏", &app.input_buffer)
    } else if task.tags.is_empty() {
        "(none)".to_string()
    } else {
        task.tags
            .iter()
            .map(|t| format!("#{}", t))
            .collect::<Vec<_>>()
            .join(" ")
    };
    lines.push(render_field(
        "Tags:",
        &tags_value,
        DetailField::Tags,
        app.detail_field,
        app.detail_edit_mode,
    ));
    lines.push(Line::from(""));

    // Due Date
    let due_value = if app.detail_edit_mode && app.detail_field == DetailField::DueDate {
        format!("{}▏ (YYYY-MM-DD)", &app.input_buffer)
    } else {
        task.due_date
            .map(|d| {
                let formatted = d.format("%Y-%m-%d").to_string();
                let relative = format_due_date(task);
                format!("{}{}", formatted, relative)
            })
            .unwrap_or_else(|| "(none)".to_string())
    };
    lines.push(render_field(
        "Due Date:",
        &due_value,
        DetailField::DueDate,
        app.detail_field,
        app.detail_edit_mode,
    ));
    lines.push(Line::from(""));

    // Metadata section
    lines.push(Line::from(vec![Span::styled(
        "─".repeat(inner.width as usize - 2),
        Style::default().fg(Color::DarkGray),
    )]));

    // Status
    let (status_str, status_color) = match task.status {
        Status::Done => ("✓ Done", Color::Green),
        Status::InProgress => ("◐ In Progress", Color::Cyan),
        Status::Pending => ("○ Pending", Color::Yellow),
        Status::Archived => ("▣ Archived", Color::DarkGray),
    };
    lines.push(Line::from(vec![
        Span::styled("  Status:     ", Style::default().fg(Color::DarkGray)),
        Span::styled(status_str, Style::default().fg(status_color)),
    ]));

    // Created
    let created = task.created_at.format("%Y-%m-%d %H:%M").to_string();
    lines.push(Line::from(vec![
        Span::styled("  Created:    ", Style::default().fg(Color::DarkGray)),
        Span::styled(created, Style::default().fg(Color::DarkGray)),
    ]));

    // Completed (if done)
    if let Some(completed) = &task.completed_at {
        let completed_str = completed.format("%Y-%m-%d %H:%M").to_string();
        lines.push(Line::from(vec![
            Span::styled("  Completed:  ", Style::default().fg(Color::DarkGray)),
            Span::styled(completed_str, Style::default().fg(Color::DarkGray)),
        ]));
    }

    // ID (for debugging/reference)
    lines.push(Line::from(vec![
        Span::styled("  ID:         ", Style::default().fg(Color::DarkGray)),
        Span::styled(&task.id[..8], Style::default().fg(Color::DarkGray)),
    ]));

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Render filter builder modal
fn render_filter_builder(app: &App, frame: &mut Frame) {
    let area = frame.size();
    let popup_width = std::cmp::min(50, area.width.saturating_sub(4));
    let popup_height = 16;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;
    let rect = Rect::new(popup_x, popup_y, popup_width, popup_height);

    frame.render_widget(Clear, rect);

    let block = Block::default()
        .title(" 🔍 Filter Builder ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(block.clone(), rect);

    let inner = rect.inner(&ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });

    // Helper to render field row
    let render_field = |label: &str, value: &str, is_selected: bool| -> Line<'_> {
        let indicator = if is_selected { "▶ " } else { "  " };
        let style = if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        Line::from(vec![
            Span::styled(
                indicator,
                if is_selected {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                },
            ),
            Span::styled(format!("{}:", label), Style::default().fg(Color::DarkGray)),
            Span::styled(value.to_string(), style),
        ])
    };

    // Build field lines
    let mut lines: Vec<Line> = Vec::new();

    // Status
    let status_value = match app.filter_builder_status {
        None => "All statuses".to_string(),
        Some(Status::Pending) => "Pending".to_string(),
        Some(Status::InProgress) => "In Progress".to_string(),
        Some(Status::Done) => "Done".to_string(),
        Some(Status::Archived) => "Archived".to_string(),
    };
    lines.push(render_field(
        "Status",
        &status_value,
        app.filter_builder_field == FilterField::Status,
    ));
    lines.push(Line::from(""));

    // Priority
    let priority_value = match app.filter_builder_priority {
        None => "All priorities".to_string(),
        Some(Priority::None) => "None".to_string(),
        Some(Priority::Low) => "Low".to_string(),
        Some(Priority::Medium) => "Medium".to_string(),
        Some(Priority::High) => "High".to_string(),
        Some(Priority::Urgent) => "Urgent".to_string(),
    };
    lines.push(render_field(
        "Priority",
        &priority_value,
        app.filter_builder_field == FilterField::Priority,
    ));
    lines.push(Line::from(""));

    // Project
    let project_value = app
        .filter_builder_project
        .clone()
        .unwrap_or_else(|| "All projects".to_string());
    lines.push(render_field(
        "Project",
        &project_value,
        app.filter_builder_field == FilterField::Project,
    ));
    lines.push(Line::from(""));

    // Search
    let search_value = if app.filter_builder_search.is_empty() {
        "(no search)".to_string()
    } else {
        app.filter_builder_search.clone()
    };
    let search_display = if app.filter_builder_field == FilterField::Search {
        format!("{}▏", &search_value)
    } else {
        search_value
    };
    lines.push(render_field(
        "Search",
        &search_display,
        app.filter_builder_field == FilterField::Search,
    ));
    lines.push(Line::from(""));

    lines.push(Line::from(vec![Span::styled(
        "─".repeat(inner.width as usize - 2),
        Style::default().fg(Color::DarkGray),
    )]));

    // Instructions
    lines.push(Line::from(vec![
        Span::styled("j/k  ", Style::default().fg(Color::DarkGray)),
        Span::styled("Select field  ", Style::default().fg(Color::Gray)),
        Span::styled("Enter  ", Style::default().fg(Color::DarkGray)),
        Span::styled("Apply  ", Style::default().fg(Color::Gray)),
        Span::styled("Esc  ", Style::default().fg(Color::DarkGray)),
        Span::styled("Cancel", Style::default().fg(Color::Gray)),
    ]));

    // Field-specific instructions
    if app.filter_builder_field == FilterField::Status {
        lines.push(Line::from(vec![Span::styled(
            "Press 'j/k' to change status",
            Style::default().fg(Color::Gray),
        )]));
    } else if app.filter_builder_field == FilterField::Priority {
        lines.push(Line::from(vec![Span::styled(
            "Press 'j/k' to change priority",
            Style::default().fg(Color::Gray),
        )]));
    } else if app.filter_builder_field == FilterField::Project {
        lines.push(Line::from(vec![Span::styled(
            "Press 'j/k' to cycle projects",
            Style::default().fg(Color::Gray),
        )]));
    } else if app.filter_builder_field == FilterField::Search {
        lines.push(Line::from(vec![Span::styled(
            "Type to search, 'u' to clear",
            Style::default().fg(Color::Gray),
        )]));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Render sort selector modal
fn render_sort_selector(app: &App, frame: &mut Frame) {
    let area = frame.size();
    let popup_width = 30;
    let popup_height = 10;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;
    let rect = Rect::new(popup_x, popup_y, popup_width, popup_height);

    frame.render_widget(Clear, rect);

    let block = Block::default()
        .title(" ⚙️ Sort By ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(block.clone(), rect);

    let inner = rect.inner(&ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });

    // Helper to render option
    let render_option = |label: &str, is_selected: bool| -> Line<'static> {
        let indicator = if is_selected { "▶ " } else { "  " };
        Line::from(vec![
            Span::styled(
                indicator,
                if is_selected {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                },
            ),
            Span::styled(
                if is_selected {
                    format!("{} ✓", label)
                } else {
                    label.to_string()
                },
                if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                },
            ),
        ])
    };

    let mut lines: Vec<Line> = Vec::new();

    lines.push(render_option(
        "Priority (high first)",
        app.sort_selector_option == SortBy::Priority,
    ));
    lines.push(Line::from(""));
    lines.push(render_option(
        "Date (newest first)",
        app.sort_selector_option == SortBy::Date,
    ));
    lines.push(Line::from(""));
    lines.push(render_option(
        "Alphabetical (A-Z)",
        app.sort_selector_option == SortBy::Alphabetical,
    ));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        "─".repeat(inner.width as usize - 2),
        Style::default().fg(Color::DarkGray),
    )]));
    lines.push(Line::from(vec![
        Span::styled("j/k  ", Style::default().fg(Color::DarkGray)),
        Span::styled("Select  ", Style::default().fg(Color::Gray)),
        Span::styled("Enter  ", Style::default().fg(Color::DarkGray)),
        Span::styled("Apply  ", Style::default().fg(Color::Gray)),
        Span::styled("Esc  ", Style::default().fg(Color::DarkGray)),
        Span::styled("Cancel", Style::default().fg(Color::Gray)),
    ]));

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Render project selector overlay
fn render_project_selector(app: &App, frame: &mut Frame) {
    let projects = app.get_projects();
    let area = frame.size();

    let popup_height = std::cmp::min((projects.len() + 5) as u16, area.height.saturating_sub(4));
    let popup_width = std::cmp::min(40, area.width.saturating_sub(4));
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;
    let rect = Rect::new(popup_x, popup_y, popup_width, popup_height);

    frame.render_widget(Clear, rect);

    let block = Block::default()
        .title(" Select Project ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(block.clone(), rect);

    let inner = rect.inner(&ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });

    // Build project list
    let mut items: Vec<Line> = vec![Line::from(vec![
        ratatui::text::Span::styled("0", Style::default().fg(Color::Yellow)),
        ratatui::text::Span::raw(". "),
        ratatui::text::Span::styled(
            if app.project_filter.is_none() {
                "All Tasks (selected)"
            } else {
                "All Tasks"
            },
            if app.project_filter.is_none() {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            },
        ),
    ])];

    for (i, project) in projects.iter().enumerate().take(9) {
        let is_selected = app.project_filter.as_ref() == Some(project);
        let display = if is_selected {
            format!("{} (selected)", project)
        } else {
            project.clone()
        };

        items.push(Line::from(vec![
            ratatui::text::Span::styled(format!("{}", i + 1), Style::default().fg(Color::Yellow)),
            ratatui::text::Span::raw(". "),
            ratatui::text::Span::styled(
                display,
                if is_selected {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                },
            ),
        ]));
    }

    items.push(Line::from(""));
    items.push(Line::from(vec![ratatui::text::Span::styled(
        "Press 0-9 to select, Esc to cancel",
        Style::default().fg(Color::Gray),
    )]));

    let paragraph = Paragraph::new(items);
    frame.render_widget(paragraph, inner);
}

/// Render sprint delete confirmation dialog
fn render_sprint_delete_dialog(app: &App, frame: &mut Frame) {
    let area = frame.size();
    let popup_width = std::cmp::min(60, area.width.saturating_sub(4));
    let popup_height = 6;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;
    let rect = Rect::new(popup_x, popup_y, popup_width, popup_height);

    frame.render_widget(Clear, rect);

    let title = " Confirm Delete Sprint ";
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(block.clone(), rect);

    let inner = rect.inner(&ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);

    // Get sprint name
    let sprint_name = app
        .editing_sprint_id
        .as_ref()
        .and_then(|id| app.get_sprint_name(id))
        .unwrap_or_else(|| "Unknown".to_string());

    let warning = format!("Delete sprint '{}'?", sprint_name);
    let warning_widget = Paragraph::new(warning).style(Style::default().fg(Color::Yellow));
    frame.render_widget(warning_widget, chunks[0]);

    let note = "Tasks will be moved to backlog. (y/n):";
    let note_widget = Paragraph::new(note).style(Style::default().fg(Color::Gray));
    frame.render_widget(note_widget, chunks[1]);
}

/// Render move-to-sprint selector overlay
fn render_move_to_sprint(app: &App, frame: &mut Frame) {
    let sprints = app.get_project_sprints();
    let area = frame.size();

    let popup_height = std::cmp::min((sprints.len() + 6) as u16, area.height.saturating_sub(4));
    let popup_width = std::cmp::min(50, area.width.saturating_sub(4));
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;
    let rect = Rect::new(popup_x, popup_y, popup_width, popup_height);

    frame.render_widget(Clear, rect);

    let title = " Move Task to Sprint ";

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(block.clone(), rect);

    let inner = rect.inner(&ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });

    // Build sprint list with "Remove from sprint" as first option
    let mut items: Vec<ListItem> = Vec::new();

    // Remove from sprint option (index 0)
    let remove_style = if app.move_to_sprint_index == 0 {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let remove_indicator = if app.move_to_sprint_index == 0 {
        "▶ "
    } else {
        "  "
    };
    items.push(ListItem::new(Line::from(vec![
        Span::styled(remove_indicator, remove_style),
        Span::styled("📋 Backlog", remove_style),
        Span::styled(
            " (remove from sprint)",
            Style::default().fg(Color::DarkGray),
        ),
    ])));

    // Sprint options
    for (i, sprint) in sprints.iter().enumerate() {
        let is_selected = app.move_to_sprint_index == i + 1;
        let style = if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let indicator = if is_selected { "▶ " } else { "  " };

        let status_emoji = match sprint.status {
            SprintStatus::Planning => "📋",
            SprintStatus::Active => "🏃",
            SprintStatus::Completed => "✅",
            SprintStatus::Archived => "📦",
        };

        // Check if task is already in this sprint
        let current_sprint_marker = app
            .current_task()
            .and_then(|t| t.sprint_id.as_ref())
            .map(|sid| sid == &sprint.id)
            .unwrap_or(false);

        let suffix = if current_sprint_marker {
            " (current)"
        } else {
            ""
        };

        items.push(ListItem::new(Line::from(vec![
            Span::styled(indicator, style),
            Span::styled(format!("{} {}{}", status_emoji, sprint.name, suffix), style),
        ])));
    }

    // Instructions
    items.push(ListItem::new(Line::from("")));
    items.push(ListItem::new(Line::from(vec![
        Span::styled("j/k ", Style::default().fg(Color::DarkGray)),
        Span::styled("navigate  ", Style::default().fg(Color::Gray)),
        Span::styled("Enter ", Style::default().fg(Color::DarkGray)),
        Span::styled("select  ", Style::default().fg(Color::Gray)),
        Span::styled("Esc ", Style::default().fg(Color::DarkGray)),
        Span::styled("cancel", Style::default().fg(Color::Gray)),
    ])));

    let list = List::new(items);
    frame.render_widget(list, inner);
}

/// Render sprint selector overlay
fn render_sprint_selector(app: &App, frame: &mut Frame) {
    let sprints = app.get_project_sprints();
    let area = frame.size();

    // +8 for: border(2) + backlog(1) + empty line(1) + 2 instruction lines + some padding
    let popup_height = std::cmp::min((sprints.len() + 8) as u16, area.height.saturating_sub(4));
    let popup_width = std::cmp::min(50, area.width.saturating_sub(4));
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;
    let rect = Rect::new(popup_x, popup_y, popup_width, popup_height);

    frame.render_widget(Clear, rect);

    let title = if let Some(ref project) = app.project_filter {
        format!(" Select Sprint ({}) ", project)
    } else {
        " Select Sprint ".to_string()
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(block.clone(), rect);

    let inner = rect.inner(&ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });

    // Build sprint list with Backlog as first option
    let mut items: Vec<ListItem> = Vec::new();

    // Backlog option (index 0)
    let backlog_style = if app.sprint_selector_index == 0 {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let backlog_indicator = if app.sprint_selector_index == 0 {
        "▶ "
    } else {
        "  "
    };
    items.push(ListItem::new(Line::from(vec![
        Span::styled(backlog_indicator, backlog_style),
        Span::styled("📋 Backlog", backlog_style),
        Span::styled(
            " (tasks not in sprint)",
            Style::default().fg(Color::DarkGray),
        ),
    ])));

    // Sprint options
    for (i, sprint) in sprints.iter().enumerate() {
        let is_selected = app.sprint_selector_index == i + 1;
        let style = if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let indicator = if is_selected { "▶ " } else { "  " };

        let status_emoji = match sprint.status {
            SprintStatus::Planning => "📋",
            SprintStatus::Active => "🏃",
            SprintStatus::Completed => "✅",
            SprintStatus::Archived => "📦",
        };

        items.push(ListItem::new(Line::from(vec![
            Span::styled(indicator, style),
            Span::styled(format!("{} {}", status_emoji, sprint.name), style),
            Span::styled(
                format!(" [{}]", sprint.status.to_string()),
                Style::default().fg(Color::DarkGray),
            ),
        ])));
    }

    // Add instruction footer
    items.push(ListItem::new(Line::from("")));
    items.push(ListItem::new(Line::from(vec![
        Span::styled("j/k ", Style::default().fg(Color::DarkGray)),
        Span::styled("nav  ", Style::default().fg(Color::Gray)),
        Span::styled("Enter ", Style::default().fg(Color::DarkGray)),
        Span::styled("select  ", Style::default().fg(Color::Gray)),
        Span::styled("n ", Style::default().fg(Color::DarkGray)),
        Span::styled("new  ", Style::default().fg(Color::Gray)),
        Span::styled("e ", Style::default().fg(Color::DarkGray)),
        Span::styled("edit", Style::default().fg(Color::Gray)),
    ])));
    items.push(ListItem::new(Line::from(vec![
        Span::styled("s ", Style::default().fg(Color::DarkGray)),
        Span::styled("start  ", Style::default().fg(Color::Gray)),
        Span::styled("c ", Style::default().fg(Color::DarkGray)),
        Span::styled("complete  ", Style::default().fg(Color::Gray)),
        Span::styled("d ", Style::default().fg(Color::DarkGray)),
        Span::styled("delete  ", Style::default().fg(Color::Gray)),
        Span::styled("Esc ", Style::default().fg(Color::DarkGray)),
        Span::styled("close", Style::default().fg(Color::Gray)),
    ])));

    let list = List::new(items);
    frame.render_widget(list, inner);
}
