//! Application state and rendering for the TUI

use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use tokio::runtime::Runtime;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::models::{Memory, MemoryType, Note, NoteType};
use crate::rag::EmbeddingService;
use crate::store::{SearchResult, Store};

use super::tui::InputMode;

/// Active pane in split view
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pane {
    NoteList,
    Preview,
}

/// View mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewMode {
    Notes,
    Memories,
    /// Full-screen detail view
    Detail,
}

/// Application state
pub struct App {
    /// Store reference
    store: Arc<dyn Store>,

    /// Embedding service for semantic search
    embedding_service: EmbeddingService,

    /// Tokio runtime for async operations
    runtime: Runtime,

    // Note data
    pub notes: Vec<Note>,
    pub filtered_notes: Vec<Note>,
    pub selected_note: usize,

    // Memory data
    pub memories: Vec<Memory>,
    pub filtered_memories: Vec<Memory>,
    pub selected_memory: usize,

    // Semantic search results
    pub semantic_results: Vec<SearchResult>,
    pub semantic_memories: Vec<(Memory, f32)>,
    pub is_semantic_search: bool,

    // View state
    pub view_mode: ViewMode,
    pub active_pane: Pane,
    pub type_filter: Option<NoteType>,

    // Search/filter state
    pub search_query: String,
    pub project_filter: Option<String>,

    // Input state
    pub input_mode: Option<InputMode>,
    pub input_buffer: String,

    // Note creation state
    pub create_title: String,
    pub create_type: NoteType,

    // UI state
    pub show_help: bool,
    pub preview_scroll: usize,
    pub message: Option<String>,
    message_time: Option<Instant>,

    // Detail view state
    pub detail_scroll: usize,
    /// Previous view mode (to return to after detail view)
    previous_view: Option<ViewMode>,

    // List state for ratatui
    list_state: ListState,

    // File watcher
    pub file_change_rx: Option<std::sync::mpsc::Receiver<()>>,
}

impl App {
    pub fn new<S: Store + 'static>(store: Arc<S>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let runtime = Runtime::new().expect("Failed to create tokio runtime");
        let embedding_service = EmbeddingService::new();

        Self {
            store,
            embedding_service,
            runtime,
            notes: Vec::new(),
            filtered_notes: Vec::new(),
            selected_note: 0,
            memories: Vec::new(),
            filtered_memories: Vec::new(),
            selected_memory: 0,
            semantic_results: Vec::new(),
            semantic_memories: Vec::new(),
            is_semantic_search: false,
            view_mode: ViewMode::Notes,
            active_pane: Pane::NoteList,
            type_filter: None,
            search_query: String::new(),
            project_filter: None,
            input_mode: None,
            input_buffer: String::new(),
            create_title: String::new(),
            create_type: NoteType::Project,
            show_help: false,
            preview_scroll: 0,
            message: None,
            message_time: None,
            detail_scroll: 0,
            previous_view: None,
            list_state,
            file_change_rx: None,
        }
    }

    /// Set the file change receiver (for file watching)
    pub fn set_file_watcher(&mut self, rx: std::sync::mpsc::Receiver<()>) {
        self.file_change_rx = Some(rx);
    }

    /// Check for file changes and reload if needed
    pub fn check_file_changes(&mut self) -> Result<bool> {
        if let Some(ref rx) = self.file_change_rx {
            match rx.try_recv() {
                Ok(()) => {
                    self.load_data()?;
                    self.set_message("Vault updated");
                    return Ok(true);
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.file_change_rx = None;
                }
            }
        }
        Ok(false)
    }

    /// Set a status message (will auto-clear after timeout)
    pub fn set_message(&mut self, msg: impl Into<String>) {
        self.message = Some(msg.into());
        self.message_time = Some(Instant::now());
    }

    /// Check message timeout and clear if expired (call in event loop)
    pub fn check_message_timeout(&mut self) {
        const MESSAGE_TIMEOUT_SECS: u64 = 3;

        if let Some(time) = self.message_time {
            if time.elapsed().as_secs() >= MESSAGE_TIMEOUT_SECS {
                self.message = None;
                self.message_time = None;
            }
        }
    }

    /// Load data from store
    pub fn load_data(&mut self) -> Result<()> {
        self.notes = self.store.list_notes(None, None, 1000)?;
        self.memories = self.store.search_memories("", None, 0.0, 1000)?;
        self.apply_filters();
        Ok(())
    }

    /// Apply current filters to notes/memories
    pub fn apply_filters(&mut self) {
        match self.view_mode {
            ViewMode::Notes => {
                self.filtered_notes = self
                    .notes
                    .iter()
                    .filter(|n| {
                        // Type filter
                        if let Some(ref t) = self.type_filter {
                            if &n.note_type != t {
                                return false;
                            }
                        }
                        // Project filter
                        if let Some(ref p) = self.project_filter {
                            if n.project.as_deref() != Some(p.as_str()) {
                                return false;
                            }
                        }
                        // Search query
                        if !self.search_query.is_empty() {
                            let q = self.search_query.to_lowercase();
                            if !n.title.to_lowercase().contains(&q)
                                && !n.content.to_lowercase().contains(&q)
                            {
                                return false;
                            }
                        }
                        true
                    })
                    .cloned()
                    .collect();

                // Reset selection if out of bounds
                if self.selected_note >= self.filtered_notes.len() {
                    self.selected_note = self.filtered_notes.len().saturating_sub(1);
                }
                self.list_state.select(Some(self.selected_note));
            }
            ViewMode::Memories => {
                self.filtered_memories = self
                    .memories
                    .iter()
                    .filter(|m| {
                        if !self.search_query.is_empty() {
                            let q = self.search_query.to_lowercase();
                            if !m.title.to_lowercase().contains(&q)
                                && !m.content.to_lowercase().contains(&q)
                            {
                                return false;
                            }
                        }
                        true
                    })
                    .cloned()
                    .collect();

                if self.selected_memory >= self.filtered_memories.len() {
                    self.selected_memory = self.filtered_memories.len().saturating_sub(1);
                }
                self.list_state.select(Some(self.selected_memory));
            }
            ViewMode::Detail => {} // Detail view doesn't need filter application
        }
    }

    /// Perform semantic search using Ollama embeddings
    pub fn semantic_search(&mut self, query: &str) -> Result<()> {
        self.set_message("Searching...");
        self.is_semantic_search = true;
        self.search_query = query.to_string();

        // Generate embedding for the query
        let embedding = match self.runtime.block_on(self.embedding_service.embed(query)) {
            Ok(emb) => emb,
            Err(e) => {
                self.set_message(format!("Ollama error: {}. Is Ollama running?", e));
                self.is_semantic_search = false;
                return Ok(());
            }
        };

        match self.view_mode {
            ViewMode::Notes | ViewMode::Detail => {
                // Search notes via vector similarity
                match self.store.vector_search(&embedding, None, None, 20) {
                    Ok(results) => {
                        let count = results.len();
                        self.semantic_results = results;

                        // Convert SearchResults to Notes for display
                        self.filtered_notes = self
                            .semantic_results
                            .iter()
                            .filter_map(|sr| self.notes.iter().find(|n| n.id == sr.note_id))
                            .cloned()
                            .collect();

                        self.selected_note = 0;
                        self.list_state.select(Some(0));
                        self.set_message(format!("Found {} semantic matches", count));
                    }
                    Err(e) => {
                        self.set_message(format!("Search error: {}", e));
                    }
                }
            }
            ViewMode::Memories => {
                // Search memories via vector similarity
                match self.store.search_memories_semantic(&embedding, 0.0, 20) {
                    Ok(results) => {
                        let count = results.len();
                        self.semantic_memories = results.clone();

                        // Convert to filtered_memories for display
                        self.filtered_memories = results.into_iter().map(|(m, _)| m).collect();

                        self.selected_memory = 0;
                        self.list_state.select(Some(0));
                        self.set_message(format!("Found {} semantic matches", count));
                    }
                    Err(e) => {
                        self.set_message(format!("Search error: {}", e));
                    }
                }
            }
        }

        Ok(())
    }

    /// Clear semantic search and return to normal filtering
    pub fn clear_semantic_search(&mut self) {
        self.is_semantic_search = false;
        self.semantic_results.clear();
        self.semantic_memories.clear();
        self.search_query.clear();
        self.apply_filters();
    }

    // Navigation
    pub fn next(&mut self) {
        let len = match self.view_mode {
            ViewMode::Notes | ViewMode::Detail => self.filtered_notes.len(),
            ViewMode::Memories => self.filtered_memories.len(),
        };
        if len == 0 {
            return;
        }

        match self.view_mode {
            ViewMode::Notes | ViewMode::Detail => {
                self.selected_note = (self.selected_note + 1).min(len - 1);
                self.list_state.select(Some(self.selected_note));
            }
            ViewMode::Memories => {
                self.selected_memory = (self.selected_memory + 1).min(len - 1);
                self.list_state.select(Some(self.selected_memory));
            }
        }
        self.preview_scroll = 0;
    }

    pub fn previous(&mut self) {
        match self.view_mode {
            ViewMode::Notes | ViewMode::Detail => {
                self.selected_note = self.selected_note.saturating_sub(1);
                self.list_state.select(Some(self.selected_note));
            }
            ViewMode::Memories => {
                self.selected_memory = self.selected_memory.saturating_sub(1);
                self.list_state.select(Some(self.selected_memory));
            }
        }
        self.preview_scroll = 0;
    }

    pub fn first(&mut self) {
        match self.view_mode {
            ViewMode::Notes | ViewMode::Detail => self.selected_note = 0,
            ViewMode::Memories => self.selected_memory = 0,
        }
        self.list_state.select(Some(0));
        self.preview_scroll = 0;
    }

    pub fn last(&mut self) {
        match self.view_mode {
            ViewMode::Notes | ViewMode::Detail => {
                self.selected_note = self.filtered_notes.len().saturating_sub(1);
                self.list_state.select(Some(self.selected_note));
            }
            ViewMode::Memories => {
                self.selected_memory = self.filtered_memories.len().saturating_sub(1);
                self.list_state.select(Some(self.selected_memory));
            }
        }
        self.preview_scroll = 0;
    }

    pub fn toggle_pane(&mut self) {
        self.active_pane = match self.active_pane {
            Pane::NoteList => Pane::Preview,
            Pane::Preview => Pane::NoteList,
        };
    }

    pub fn view_detail(&mut self) {
        // Check if we have something to view
        let has_content = match self.view_mode {
            ViewMode::Notes => !self.filtered_notes.is_empty(),
            ViewMode::Memories => !self.filtered_memories.is_empty(),
            ViewMode::Detail => return, // Already in detail view
        };

        if has_content {
            self.previous_view = Some(self.view_mode);
            self.view_mode = ViewMode::Detail;
            self.detail_scroll = 0;
        }
    }

    pub fn exit_detail(&mut self) {
        if let Some(prev) = self.previous_view.take() {
            self.view_mode = prev;
        }
    }

    pub fn scroll_detail_up(&mut self) {
        self.detail_scroll = self.detail_scroll.saturating_sub(1);
    }

    pub fn scroll_detail_down(&mut self) {
        self.detail_scroll += 1;
    }

    pub fn set_type_filter(&mut self, filter: Option<NoteType>) {
        self.type_filter = filter;
        self.view_mode = ViewMode::Notes;
        self.apply_filters();
    }

    pub fn toggle_memory_view(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::Notes | ViewMode::Detail => ViewMode::Memories,
            ViewMode::Memories => ViewMode::Notes,
        };
        self.previous_view = None; // Clear detail state when toggling
        self.apply_filters();
    }

    pub fn reinforce_memory(&mut self) -> Result<()> {
        if self.view_mode != ViewMode::Memories {
            return Ok(());
        }
        if let Some(memory) = self.filtered_memories.get(self.selected_memory).cloned() {
            // Reinforce: increase confidence (max 1.0) and increment times_applied
            let new_confidence = (memory.confidence + 0.1).min(1.0);
            let new_times = memory.times_applied + 1;
            self.store.update_memory_reinforcement(&memory.id, new_confidence, new_times)?;
            self.set_message(format!("Reinforced: {} ({:.0}%)", memory.title, new_confidence * 100.0));
            self.load_data()?;
        }
        Ok(())
    }

    // === Note CRUD Operations ===

    /// Start creating a new note
    pub fn start_create_note(&mut self) {
        // Use current filter type or default to Project
        self.create_type = self.type_filter.clone().unwrap_or(NoteType::Project);
        self.create_title.clear();
        self.input_buffer.clear();
        self.input_mode = Some(InputMode::CreateTitle);
    }

    /// Cycle through note types during creation (called with Tab)
    pub fn cycle_create_type(&mut self) {
        self.create_type = match self.create_type {
            NoteType::Project => NoteType::Task,
            NoteType::Task => NoteType::Learning,
            NoteType::Learning => NoteType::Memory,
            NoteType::Memory => NoteType::Meta,
            NoteType::Meta => NoteType::Project,
        };
    }

    /// Finalize note creation with title and type
    pub fn finish_create_note(&mut self, title: String) -> Result<()> {
        if title.is_empty() {
            self.set_message("Note title cannot be empty");
            return Ok(());
        }

        // Create note based on type
        let mut note = match self.create_type {
            NoteType::Project => {
                let project = self.project_filter.as_deref().unwrap_or("general");
                Note::project_note(&title, project)
            }
            NoteType::Task => Note::task_note(&title, uuid::Uuid::new_v4().to_string()),
            NoteType::Learning => Note::learning_note(&title, "topic"),
            NoteType::Memory => Note::memory_note(&title, "lesson"),
            NoteType::Meta => Note::new(&title, NoteType::Meta, format!("Meta/{}.md", slug(&title))),
        };

        // Add placeholder content
        note.content = format!("# {}\n\n<!-- Add your content here -->\n", title);

        // Save to store
        self.store.save_note(&note)?;
        self.set_message(format!("Created note: {}", title));

        // Reload and select the new note
        self.load_data()?;

        // Find and select the new note
        if let Some(idx) = self.filtered_notes.iter().position(|n| n.id == note.id) {
            self.selected_note = idx;
            self.list_state.select(Some(idx));
        }

        Ok(())
    }

    /// Edit the selected note in external editor
    pub fn edit_note(&mut self) -> Result<bool> {
        if self.view_mode != ViewMode::Notes && self.view_mode != ViewMode::Detail {
            self.set_message("Can only edit notes");
            return Ok(false);
        }

        let note = match self.filtered_notes.get(self.selected_note) {
            Some(n) => n.clone(),
            None => {
                self.set_message("No note selected");
                return Ok(false);
            }
        };

        // Get vault directory and full path
        let vault_dir = crate::vault_dir();
        let note_path = vault_dir.join(&note.path);

        // Ensure directory exists
        if let Some(parent) = note_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Write current content to file if it doesn't exist
        if !note_path.exists() {
            std::fs::write(&note_path, note.to_markdown())?;
        }

        // Get editor from environment
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());

        self.set_message(format!("Opening {} in {}...", note.title, editor));
        Ok(true)
    }

    /// Get the path to edit (called after terminal is restored)
    pub fn get_edit_path(&self) -> Option<std::path::PathBuf> {
        if let Some(note) = self.filtered_notes.get(self.selected_note) {
            let vault_dir = crate::vault_dir();
            Some(vault_dir.join(&note.path))
        } else {
            None
        }
    }

    /// Start delete confirmation
    pub fn start_delete_note(&mut self) {
        if self.view_mode != ViewMode::Notes {
            self.set_message("Can only delete notes");
            return;
        }

        if self.filtered_notes.is_empty() {
            self.set_message("No note selected");
            return;
        }

        self.input_mode = Some(InputMode::DeleteConfirm);
        self.input_buffer.clear();
    }

    /// Delete the currently selected note
    pub fn delete_note(&mut self) -> Result<()> {
        if let Some(note) = self.filtered_notes.get(self.selected_note).cloned() {
            // Delete from store
            self.store.delete_note(&note.id)?;

            // Delete file from vault
            let vault_dir = crate::vault_dir();
            let note_path = vault_dir.join(&note.path);
            if note_path.exists() {
                std::fs::remove_file(&note_path)?;
            }

            self.set_message(format!("Deleted: {}", note.title));
            self.load_data()?;
        }
        Ok(())
    }

    // === Task Linking ===

    /// Start linking a note to a task
    pub fn start_link_task(&mut self) {
        if self.view_mode != ViewMode::Notes {
            self.set_message("Can only link notes to tasks");
            return;
        }

        if self.filtered_notes.is_empty() {
            self.set_message("No note selected");
            return;
        }

        // Show current task link if any
        if let Some(note) = self.filtered_notes.get(self.selected_note) {
            if let Some(ref task_id) = note.task_id {
                self.input_buffer = task_id.clone();
            } else {
                self.input_buffer.clear();
            }
        }

        self.input_mode = Some(InputMode::LinkTask);
    }

    /// Link the selected note to a task
    pub fn link_note_to_task(&mut self, task_id: String) -> Result<()> {
        if let Some(mut note) = self.filtered_notes.get(self.selected_note).cloned() {
            if task_id.is_empty() {
                // Unlink task
                note.task_id = None;
                self.set_message(format!("Unlinked task from: {}", note.title));
            } else {
                // Link to task
                note.task_id = Some(task_id.clone());
                self.set_message(format!("Linked to task: {}", &task_id[..8.min(task_id.len())]));
            }

            // Update note type to Task if linking
            if note.task_id.is_some() && note.note_type != NoteType::Task {
                note.note_type = NoteType::Task;
            }

            self.store.save_note(&note)?;
            self.load_data()?;
        }
        Ok(())
    }

    pub fn clear_filters(&mut self) {
        self.type_filter = None;
        self.project_filter = None;
        self.search_query.clear();
        self.is_semantic_search = false;
        self.semantic_results.clear();
        self.semantic_memories.clear();
        self.apply_filters();
    }

    pub fn clear_search(&mut self) {
        if self.is_semantic_search {
            self.clear_semantic_search();
        } else {
            self.search_query.clear();
            self.apply_filters();
        }
    }

    pub fn scroll_preview_up(&mut self) {
        self.preview_scroll = self.preview_scroll.saturating_sub(5);
    }

    pub fn scroll_preview_down(&mut self) {
        self.preview_scroll += 5;
    }

    // Rendering
    pub fn render(&mut self, f: &mut Frame) {
        let size = f.area();

        // Main layout: header + content + footer
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(1), // Type tabs
                Constraint::Min(10),   // Content
                Constraint::Length(1), // Footer
            ])
            .split(size);

        self.render_header(f, main_chunks[0]);
        self.render_tabs(f, main_chunks[1]);
        self.render_content(f, main_chunks[2]);
        self.render_footer(f, main_chunks[3]);

        // Overlays
        if self.show_help {
            self.render_help(f);
        }

        if self.input_mode.is_some() {
            self.render_input(f);
        }
    }

    fn render_header(&self, f: &mut Frame, area: Rect) {
        let stats = self.store.get_stats().unwrap_or_default();

        let search_indicator = if self.is_semantic_search {
            format!("🔮 Semantic: \"{}\"", truncate(&self.search_query, 20))
        } else if !self.search_query.is_empty() {
            format!("🔍 \"{}\"", truncate(&self.search_query, 20))
        } else if let Some(ref p) = self.project_filter {
            format!("Project: {}", p)
        } else {
            "All Projects".to_string()
        };

        let title = format!(
            " vulcan-vault │ Notes: {} │ Memories: {} │ {} ",
            stats.total_notes,
            stats.total_memories,
            search_indicator
        );

        let header = Paragraph::new(title)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).border_style(
                Style::default().fg(if self.is_semantic_search {
                    Color::Magenta
                } else if self.view_mode == ViewMode::Notes || self.view_mode == ViewMode::Detail {
                    Color::Blue
                } else {
                    Color::Magenta
                }),
            ));

        f.render_widget(header, area);
    }

    fn render_tabs(&self, f: &mut Frame, area: Rect) {
        let tabs = vec![
            "0:All",
            "1:Project",
            "2:Task",
            "3:Learning",
            "4:Memory",
            "5:Meta",
            "m:Memories",
        ];

        let selected = match self.view_mode {
            ViewMode::Memories => 6,
            ViewMode::Detail => match self.previous_view {
                Some(ViewMode::Memories) => 6,
                _ => 0, // Default to "All" when viewing note detail
            },
            ViewMode::Notes => match self.type_filter {
                None => 0,
                Some(NoteType::Project) => 1,
                Some(NoteType::Task) => 2,
                Some(NoteType::Learning) => 3,
                Some(NoteType::Memory) => 4,
                Some(NoteType::Meta) => 5,
            },
        };

        let tabs = Tabs::new(tabs)
            .select(selected)
            .style(Style::default().fg(Color::DarkGray))
            .highlight_style(Style::default().fg(Color::Yellow).bold());

        f.render_widget(tabs, area);
    }

    fn render_content(&mut self, f: &mut Frame, area: Rect) {
        // Check if we're in detail view mode
        if self.view_mode == ViewMode::Detail {
            self.render_detail(f, area);
            return;
        }

        // Split into list and preview
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);

        self.render_list(f, chunks[0]);
        self.render_preview(f, chunks[1]);
    }

    fn render_detail(&self, f: &mut Frame, area: Rect) {
        let (title, content, meta) = match self.previous_view {
            Some(ViewMode::Notes) => {
                if let Some(note) = self.filtered_notes.get(self.selected_note) {
                    let meta = format!(
                        "Type: {} │ Project: {} │ Tags: {}",
                        note.note_type,
                        note.project.as_deref().unwrap_or("-"),
                        if note.tags.is_empty() { "-".to_string() } else { note.tags.join(", ") }
                    );
                    (note.title.clone(), note.content.clone(), meta)
                } else {
                    return;
                }
            }
            Some(ViewMode::Memories) => {
                if let Some(mem) = self.filtered_memories.get(self.selected_memory) {
                    let meta = format!(
                        "Type: {} │ Confidence: {:.0}% │ Applied: {} times │ Context: {}",
                        mem.memory_type,
                        mem.confidence * 100.0,
                        mem.times_applied,
                        truncate(&mem.context, 40)
                    );
                    (mem.title.clone(), mem.content.clone(), meta)
                } else {
                    return;
                }
            }
            _ => return,
        };

        // Layout: metadata bar + content
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(5)])
            .split(area);

        // Metadata bar
        let meta_line = Paragraph::new(meta)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(meta_line, chunks[0]);

        // Content area with scroll
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();
        let visible_height = chunks[1].height.saturating_sub(2) as usize;

        // Clamp scroll to valid range
        let max_scroll = total_lines.saturating_sub(visible_height);
        let scroll = self.detail_scroll.min(max_scroll);

        let visible_content: String = lines
            .iter()
            .skip(scroll)
            .take(visible_height)
            .copied()
            .collect::<Vec<_>>()
            .join("\n");

        // Scroll indicator
        let scroll_info = if total_lines > visible_height {
            format!(" {} │ {}/{} ", title, scroll + 1, total_lines.saturating_sub(visible_height) + 1)
        } else {
            format!(" {} ", title)
        };

        let content_block = Paragraph::new(visible_content)
            .block(
                Block::default()
                    .title(scroll_info)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(content_block, chunks[1]);
    }

    fn render_list(&mut self, f: &mut Frame, area: Rect) {
        let (items, title): (Vec<ListItem>, &str) = match self.view_mode {
            ViewMode::Notes => {
                let items: Vec<ListItem> = self
                    .filtered_notes
                    .iter()
                    .enumerate()
                    .map(|(i, n)| {
                        let type_color = match n.note_type {
                            NoteType::Project => Color::Green,
                            NoteType::Task => Color::Yellow,
                            NoteType::Learning => Color::Cyan,
                            NoteType::Memory => Color::Magenta,
                            NoteType::Meta => Color::DarkGray,
                        };

                        // Show similarity score in semantic mode, project otherwise
                        let suffix = if self.is_semantic_search {
                            // Find similarity score from semantic_results
                            let score = self.semantic_results.get(i)
                                .map(|sr| format!(" {:.0}%", (1.0 - sr.distance) * 100.0))
                                .unwrap_or_default();
                            score
                        } else {
                            let project = n.project.as_deref().unwrap_or("");
                            format!(" {}", truncate(project, 12))
                        };

                        ListItem::new(Line::from(vec![
                            Span::styled(
                                format!("[{}] ", n.note_type.to_string().chars().next().unwrap()),
                                Style::default().fg(type_color),
                            ),
                            Span::raw(truncate(&n.title, 25)),
                            Span::styled(suffix, Style::default().fg(Color::DarkGray)),
                        ]))
                    })
                    .collect();
                let title = if self.is_semantic_search { "Semantic Results" } else { "Notes" };
                (items, title)
            }
            ViewMode::Memories => {
                let items: Vec<ListItem> = self
                    .filtered_memories
                    .iter()
                    .enumerate()
                    .map(|(i, m)| {
                        let type_color = match m.memory_type {
                            MemoryType::Decision => Color::Blue,
                            MemoryType::Lesson => Color::Green,
                            MemoryType::Preference => Color::Yellow,
                            MemoryType::Session => Color::Cyan,
                        };

                        // Show similarity score in semantic mode, confidence otherwise
                        let suffix = if self.is_semantic_search {
                            self.semantic_memories.get(i)
                                .map(|(_, dist)| format!(" {:.0}%", (1.0 - dist) * 100.0))
                                .unwrap_or_default()
                        } else {
                            format!(" {:.0}%", m.confidence * 100.0)
                        };

                        ListItem::new(Line::from(vec![
                            Span::styled(
                                format!(
                                    "[{}] ",
                                    m.memory_type.to_string().chars().next().unwrap()
                                ),
                                Style::default().fg(type_color),
                            ),
                            Span::raw(truncate(&m.title, 25)),
                            Span::styled(suffix, Style::default().fg(Color::DarkGray)),
                        ]))
                    })
                    .collect();
                let title = if self.is_semantic_search { "Semantic Memories" } else { "Memories" };
                (items, title)
            }
            ViewMode::Detail => unreachable!("render_list not called in detail view"),
        };

        let count = items.len();
        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!(" {} ({}) ", title, count))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(if self.active_pane == Pane::NoteList {
                        Color::Cyan
                    } else {
                        Color::DarkGray
                    })),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_preview(&self, f: &mut Frame, area: Rect) {
        let (title, content) = match self.view_mode {
            ViewMode::Notes => {
                if let Some(note) = self.filtered_notes.get(self.selected_note) {
                    (
                        note.title.clone(),
                        format!(
                            "Type: {}\nProject: {}\nTags: {}\n\n{}",
                            note.note_type,
                            note.project.as_deref().unwrap_or("-"),
                            note.tags.join(", "),
                            &note.content
                        ),
                    )
                } else {
                    ("No Selection".to_string(), "Select a note to preview".to_string())
                }
            }
            ViewMode::Memories => {
                if let Some(mem) = self.filtered_memories.get(self.selected_memory) {
                    (
                        mem.title.clone(),
                        format!(
                            "Type: {}\nConfidence: {:.0}%\nContext: {}\nApplied: {} times\n\n{}",
                            mem.memory_type,
                            mem.confidence * 100.0,
                            mem.context,
                            mem.times_applied,
                            &mem.content
                        ),
                    )
                } else {
                    ("No Selection".to_string(), "Select a memory to preview".to_string())
                }
            }
            ViewMode::Detail => unreachable!("render_preview not called in detail view"),
        };

        // Apply scroll offset
        let lines: Vec<&str> = content.lines().collect();
        let visible_lines: String = lines
            .iter()
            .skip(self.preview_scroll)
            .take(area.height as usize - 2)
            .copied()
            .collect::<Vec<&str>>()
            .join("\n");

        let preview = Paragraph::new(visible_lines)
            .block(
                Block::default()
                    .title(format!(" {} ", truncate(&title, 40)))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(if self.active_pane == Pane::Preview {
                        Color::Cyan
                    } else {
                        Color::DarkGray
                    })),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(preview, area);
    }

    fn render_footer(&self, f: &mut Frame, area: Rect) {
        let default_msg = if self.view_mode == ViewMode::Detail {
            "j/k:Scroll  Esc:Back  ?:Help  q:Quit"
        } else {
            "j/k:Navigate  Enter:View  /:Search  Tab:Pane  m:Memories  r:Reinforce  ?:Help  q:Quit"
        };

        let msg = self.message.as_deref().unwrap_or(default_msg);

        let footer = Paragraph::new(msg)
            .style(Style::default().fg(Color::DarkGray))
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(footer, area);
    }

    fn render_help(&self, f: &mut Frame) {
        let area = centered_rect(60, 70, f.area());
        f.render_widget(Clear, area);

        let help_text = vec![
            "",
            "  Navigation",
            "  ──────────────────────────────────",
            "  j/k, ↑/↓    Move up/down",
            "  g/G         Go to first/last",
            "  Tab         Switch pane",
            "  Enter       View detail",
            "",
            "  Filtering",
            "  ──────────────────────────────────",
            "  1-5         Filter by type",
            "  0           Show all types",
            "  /           Keyword search",
            "  s           Semantic search (Ollama)",
            "  c           Clear all filters",
            "",
            "  Note Actions",
            "  ──────────────────────────────────",
            "  n           Create new note",
            "  e           Edit note in $EDITOR",
            "  d           Delete note",
            "  l           Link to vulcan-todo task",
            "",
            "  Memory Actions",
            "  ──────────────────────────────────",
            "  m           Toggle memory view",
            "  r           Reinforce memory",
            "",
            "  View Controls",
            "  ──────────────────────────────────",
            "  Ctrl+u/d    Scroll preview",
            "  ?           Toggle help",
            "  q/Esc       Quit",
            "",
        ];

        let help = Paragraph::new(help_text.join("\n"))
            .block(
                Block::default()
                    .title(" Help ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(help, area);
    }

    fn render_input(&self, f: &mut Frame) {
        let area = centered_rect(50, 20, f.area());
        f.render_widget(Clear, area);

        let (title, hint): (&str, String) = match self.input_mode {
            Some(InputMode::Search) => (" Search ", "Enter keyword to search".to_string()),
            Some(InputMode::SemanticSearch) => (" Semantic Search ", "Enter query for AI search".to_string()),
            Some(InputMode::CreateTitle) => (
                " Create Note ",
                format!("Type: {} (Tab to cycle) | Enter title:", self.create_type),
            ),
            Some(InputMode::CreateProject) => (
                " Project (optional) ",
                "Enter project name or press Enter to skip".to_string(),
            ),
            Some(InputMode::DeleteConfirm) => {
                let note_title = self.filtered_notes.get(self.selected_note)
                    .map(|n| n.title.as_str())
                    .unwrap_or("note");
                (" Confirm Delete ", format!("Delete '{}'? Type 'y' to confirm:", note_title))
            }
            Some(InputMode::LinkTask) => (
                " Link to Task ",
                "Enter task ID (use `vulcan-todo list` to find IDs)\nLeave empty to unlink:".to_string(),
            ),
            None => (" Input ", String::new()),
        };

        // Create content with hint and input
        let content = format!("{}\n\n> {}", hint, self.input_buffer);

        let input = Paragraph::new(content)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(input, area);
    }
}

/// Create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Truncate string to max length
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max - 1])
    }
}

/// Convert string to URL-friendly slug
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
