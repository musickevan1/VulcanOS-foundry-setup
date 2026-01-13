use crate::models::{Priority, Status};
use crate::store::{JsonStore, Store};
use crate::ui::app::{App, FilterField, InputMode, SortBy};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::stdout;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

/// Run the TUI application
pub fn run_tui(store: Arc<dyn Store>) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new(store);

    // Setup file watcher for real-time sync with MCP server
    let watch_path = JsonStore::default_path()?;
    let (file_tx, file_rx) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                // Only send on modify/write events
                if event.kind.is_modify() || event.kind.is_create() {
                    let _ = file_tx.send(());
                }
            }
        },
        Config::default().with_poll_interval(Duration::from_millis(100)),
    )?;

    // Watch the tasks.json file (or its parent directory if file doesn't exist yet)
    if watch_path.exists() {
        watcher.watch(&watch_path, RecursiveMode::NonRecursive)?;
    } else if let Some(parent) = watch_path.parent() {
        watcher.watch(parent, RecursiveMode::NonRecursive)?;
    }

    let result = run_app(&mut terminal, &mut app, file_rx);

    // Cleanup terminal
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    file_rx: mpsc::Receiver<()>,
) -> Result<()> {
    // Track last file event to debounce rapid changes
    let mut last_file_sync = std::time::Instant::now();
    let debounce_duration = Duration::from_millis(100);

    loop {
        terminal.draw(|f| {
            crate::ui::app::render(app, f);
        })?;

        // Check for file changes (non-blocking)
        // Debounce to avoid excessive reloads on rapid saves
        if let Ok(()) = file_rx.try_recv() {
            let now = std::time::Instant::now();
            if now.duration_since(last_file_sync) > debounce_duration {
                app.reload_tasks();
                app.set_message("📡 Synced from MCP".to_string());
                last_file_sync = now;
            }
            // Drain any additional events that came in during debounce period
            while file_rx.try_recv().is_ok() {}
        }

        // Check for keyboard events (with timeout)
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // Clear any previous message after a key press
                app.clear_message();

                // Route to appropriate handler based on current state
                if app.show_filter_builder {
                    if handle_filter_builder(app, &key) {
                        continue;
                    }
                } else if app.show_sort_selector {
                    if handle_sort_selector(app, &key) {
                        continue;
                    }
                } else if app.show_project_selector {
                    if handle_project_selector(app, &key) {
                        continue;
                    }
                } else if app.show_sprint_selector {
                    if handle_sprint_selector(app, &key) {
                        continue;
                    }
                } else if app.show_move_to_sprint {
                    if handle_move_to_sprint(app, &key) {
                        continue;
                    }
                }

                if app.show_help {
                    if key.code == KeyCode::Esc
                        || key.code == KeyCode::Char('?')
                        || key.code == KeyCode::Char('q')
                    {
                        app.toggle_help();
                    }
                    continue;
                }

                if app.is_input_mode() {
                    handle_input_mode(app, &key);
                } else if app.is_detail_view() {
                    if handle_detail_mode(app, &key) {
                        break; // Exit requested
                    }
                } else if handle_normal_mode(app, &key) {
                    break; // Exit requested
                }
            }
        }
    }

    Ok(())
}

/// Handle filter builder key events. Returns true if event was consumed.
fn handle_filter_builder(app: &mut App, key: &KeyEvent) -> bool {
    match key.code {
        // Navigation
        KeyCode::Down | KeyCode::Char('j') => {
            if app.filter_builder_field == FilterField::Status {
                app.filter_builder_cycle_status();
            } else if app.filter_builder_field == FilterField::Priority {
                app.filter_builder_cycle_priority();
            } else if app.filter_builder_field == FilterField::Project {
                app.filter_builder_cycle_project();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.filter_builder_field == FilterField::Status {
                // Reverse cycle
                app.filter_builder_status = match app.filter_builder_status {
                    None => Some(Status::Archived),
                    Some(Status::Archived) => Some(Status::Done),
                    Some(Status::Done) => Some(Status::InProgress),
                    Some(Status::InProgress) => Some(Status::Pending),
                    Some(Status::Pending) => None,
                };
            } else if app.filter_builder_field == FilterField::Priority {
                // Reverse cycle
                app.filter_builder_priority = match app.filter_builder_priority {
                    None => Some(Priority::Urgent),
                    Some(Priority::Urgent) => Some(Priority::High),
                    Some(Priority::High) => Some(Priority::Medium),
                    Some(Priority::Medium) => Some(Priority::Low),
                    Some(Priority::Low) => Some(Priority::None),
                    Some(Priority::None) => None,
                };
            } else if app.filter_builder_field == FilterField::Project {
                app.filter_builder_cycle_project();
            }
        }
        KeyCode::Right | KeyCode::Char('l') => app.filter_builder_next_field(),
        KeyCode::Left | KeyCode::Char('h') => app.filter_builder_prev_field(),

        // Apply filter
        KeyCode::Enter => {
            app.apply_filter_builder();
            return true;
        }

        // Cancel
        KeyCode::Esc | KeyCode::Char('q') => {
            app.toggle_filter_builder();
            return true;
        }

        // Search input
        KeyCode::Char(c) if app.filter_builder_field == FilterField::Search => {
            app.filter_builder_search_char(c);
        }

        KeyCode::Backspace if app.filter_builder_field == FilterField::Search => {
            app.filter_builder_search_backspace();
        }

        KeyCode::Char('u') if app.filter_builder_field == FilterField::Search => {
            app.filter_builder_clear_search();
        }

        _ => {}
    }
    false
}

/// Handle sort selector key events. Returns true if event was consumed.
fn handle_sort_selector(app: &mut App, key: &KeyEvent) -> bool {
    match key.code {
        // Navigation
        KeyCode::Down | KeyCode::Char('j') => app.cycle_sort_option(),
        KeyCode::Up | KeyCode::Char('k') => {
            app.sort_selector_option = match app.sort_selector_option {
                SortBy::Priority => SortBy::Alphabetical,
                SortBy::Alphabetical => SortBy::Date,
                SortBy::Date => SortBy::Priority,
            };
        }

        // Apply
        KeyCode::Enter => {
            app.select_sort_option();
            return true;
        }

        // Cancel
        KeyCode::Esc | KeyCode::Char('q') => {
            app.toggle_sort_selector();
            return true;
        }

        _ => {}
    }
    false
}

/// Handle normal mode key events. Returns true if exit requested.
fn handle_normal_mode(app: &mut App, key: &KeyEvent) -> bool {
    // Handle multi-select mode differently
    if app.multi_select_mode {
        return handle_multi_select_mode(app, key);
    }

    // Sprint task reordering (Ctrl+Up/Down when in sprint view) - check before navigation
    if app.sprint_view_mode && app.selected_sprint.is_some() {
        match (key.code, key.modifiers.contains(KeyModifiers::CONTROL)) {
            (KeyCode::Up, true) | (KeyCode::Char('K'), _) => {
                app.reorder_current_task(-1);
                return false;
            }
            (KeyCode::Down, true) | (KeyCode::Char('J'), _) => {
                app.reorder_current_task(1);
                return false;
            }
            _ => {}
        }
    }

    match key.code {
        // Navigation
        KeyCode::Down | KeyCode::Char('j') => app.move_down(),
        KeyCode::Up | KeyCode::Char('k') => app.move_up(),
        KeyCode::Home | KeyCode::Char('g') => {
            app.selected = 0;
        }
        KeyCode::End | KeyCode::Char('G') => {
            if !app.filtered_tasks.is_empty() {
                app.selected = app.filtered_tasks.len() - 1;
            }
        }

        // Tab navigation
        KeyCode::Tab => {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                app.prev_tab();
            } else {
                app.next_tab();
            }
        }
        KeyCode::BackTab => app.prev_tab(),

        // Number keys for tab jumping (0-9)
        KeyCode::Char(c) if c.is_ascii_digit() && !key.modifiers.contains(KeyModifiers::SHIFT) => {
            let index = c.to_digit(10).unwrap() as usize;
            app.go_to_tab(index);
        }

        // Task actions
        KeyCode::Char('n') => app.enter_input_mode(InputMode::NewTask),
        KeyCode::Char('e') => {
            if app.current_task().is_some() {
                app.enter_input_mode(InputMode::EditTitle);
            }
        }
        KeyCode::Char('x') | KeyCode::Char(' ') => app.toggle_task(),
        KeyCode::Enter => {
            // Open detail view for current task
            app.open_detail_view();
        }
        KeyCode::Char('d') => {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                // Shift+D: delete immediately
                app.delete_task();
            } else if app.current_task().is_some() {
                // d: confirm delete
                app.enter_input_mode(InputMode::ConfirmDelete);
            }
        }
        KeyCode::Char('p') => {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                // Shift+P: open project selector
                app.toggle_project_selector();
            } else {
                // p: cycle priority
                app.cycle_priority();
            }
        }

        // Search and filter
        KeyCode::Char('/') => app.enter_input_mode(InputMode::Search),
        KeyCode::Char('s') => app.cycle_status_filter(),
        KeyCode::Char('o') => app.toggle_sort_selector(),
        KeyCode::Char('O') => app.toggle_filter_builder(),
        KeyCode::Char('P') => app.toggle_project_selector(),

        // Sprint view (S key)
        KeyCode::Char('S') => {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                // Shift+S: open sprint selector
                app.toggle_sprint_selector();
            } else {
                // S: toggle sprint view mode
                app.toggle_sprint_view();
            }
        }

        // Move task to sprint (m key)
        KeyCode::Char('m') => {
            app.toggle_move_to_sprint();
        }

        // Multi-select
        KeyCode::Char('v') => {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                // Shift+V: select all visible
                app.select_all_visible();
            } else {
                // v: toggle multi-select mode
                app.toggle_multi_select_mode();
            }
        }
        KeyCode::Char('V') => app.select_all_visible(),

        // Help
        KeyCode::Char('?') => app.toggle_help(),

        // Refresh (force reload from disk)
        KeyCode::Char('r') => {
            app.reload_tasks();
            app.set_message("Synced from disk".to_string());
        }

        // Clear filters
        KeyCode::Char('c') => {
            app.clear_search();
            app.status_filter = None;
            app.go_to_tab(0); // Return to "All" tab
            app.set_message("Filters cleared".to_string());
        }

        // Quit
        KeyCode::Char('q') => return true,
        KeyCode::Esc => {
            // Esc clears filters step by step, then quits
            if !app.search_query.is_empty() {
                app.clear_search();
            } else if app.status_filter.is_some() {
                app.status_filter = None;
                app.apply_filter();
            } else if app.active_tab > 0 {
                app.go_to_tab(0);
            } else {
                return true;
            }
        }

        _ => {}
    }
    false
}

/// Handle detail view mode key events. Returns true if exit requested.
fn handle_detail_mode(app: &mut App, key: &KeyEvent) -> bool {
    // If in edit mode within detail view
    if app.detail_edit_mode {
        match key.code {
            KeyCode::Esc => {
                app.detail_exit_edit();
            }
            KeyCode::Enter => {
                app.detail_save_edit();
            }
            KeyCode::Char(c) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    match c {
                        'a' => app.input_cursor_home(),
                        'e' => app.input_cursor_end(),
                        'u' => {
                            app.input_buffer.clear();
                            app.input_cursor = 0;
                        }
                        _ => {}
                    }
                } else {
                    app.input_char(c);
                }
            }
            KeyCode::Backspace => app.input_backspace(),
            KeyCode::Delete => app.input_delete(),
            KeyCode::Left => app.input_cursor_left(),
            KeyCode::Right => app.input_cursor_right(),
            KeyCode::Home => app.input_cursor_home(),
            KeyCode::End => app.input_cursor_end(),
            _ => {}
        }
        return false;
    }

    // Read-only mode (vim-like)
    match key.code {
        // Navigation between fields
        KeyCode::Down | KeyCode::Char('j') => app.detail_next_field(),
        KeyCode::Up | KeyCode::Char('k') => app.detail_prev_field(),

        // Edit current field
        KeyCode::Char('i') | KeyCode::Enter => {
            app.detail_enter_edit();
        }

        // Toggle completion
        KeyCode::Char('x') | KeyCode::Char(' ') => {
            app.detail_toggle_task();
        }

        // Cycle priority
        KeyCode::Char('p') => {
            app.detail_cycle_priority();
        }

        // Close detail view
        KeyCode::Char('q') | KeyCode::Esc => {
            app.close_detail_view();
        }

        // Help
        KeyCode::Char('?') => app.toggle_help(),

        _ => {}
    }
    false
}

/// Handle multi-select mode key events. Returns true if exit requested.
fn handle_multi_select_mode(app: &mut App, key: &KeyEvent) -> bool {
    match key.code {
        // Navigation (same as normal mode)
        KeyCode::Down | KeyCode::Char('j') => app.move_down(),
        KeyCode::Up | KeyCode::Char('k') => app.move_up(),
        KeyCode::Home | KeyCode::Char('g') => {
            app.selected = 0;
        }
        KeyCode::End | KeyCode::Char('G') => {
            if !app.filtered_tasks.is_empty() {
                app.selected = app.filtered_tasks.len() - 1;
            }
        }

        // Toggle selection with Space
        KeyCode::Char(' ') => {
            app.toggle_current_selection();
            app.move_down(); // Move to next task after selecting
        }

        // Complete selected tasks
        KeyCode::Char('x') => {
            if !app.selected_tasks.is_empty() {
                app.complete_selected_tasks();
            }
        }

        // Delete selected tasks
        KeyCode::Char('d') => {
            if !app.selected_tasks.is_empty() {
                app.delete_selected_tasks();
            }
        }

        // Exit multi-select mode
        KeyCode::Char('v') | KeyCode::Esc => {
            app.toggle_multi_select_mode();
        }

        // Select all
        KeyCode::Char('V') => app.select_all_visible(),

        // Help
        KeyCode::Char('?') => app.toggle_help(),

        // Quit
        KeyCode::Char('q') => return true,

        _ => {}
    }
    false
}

/// Handle input mode key events
fn handle_input_mode(app: &mut App, key: &KeyEvent) {
    match key.code {
        // Submit input
        KeyCode::Enter => {
            app.submit_input();
        }

        // Cancel input
        KeyCode::Esc => {
            app.cancel_input();
        }

        // Text editing
        KeyCode::Char(c) => {
            // Handle Ctrl+key combinations
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match c {
                    'a' => app.input_cursor_home(),
                    'e' => app.input_cursor_end(),
                    'u' => {
                        // Clear input
                        app.input_buffer.clear();
                        app.input_cursor = 0;
                    }
                    'w' => {
                        // Delete word before cursor
                        let before = &app.input_buffer[..app.input_cursor];
                        if let Some(last_space) = before.trim_end().rfind(' ') {
                            app.input_buffer = format!(
                                "{}{}",
                                &app.input_buffer[..last_space + 1],
                                &app.input_buffer[app.input_cursor..]
                            );
                            app.input_cursor = last_space + 1;
                        } else {
                            app.input_buffer = app.input_buffer[app.input_cursor..].to_string();
                            app.input_cursor = 0;
                        }
                    }
                    _ => {}
                }
            } else {
                app.input_char(c);
            }
        }

        KeyCode::Backspace => app.input_backspace(),
        KeyCode::Delete => app.input_delete(),
        KeyCode::Left => app.input_cursor_left(),
        KeyCode::Right => app.input_cursor_right(),
        KeyCode::Home => app.input_cursor_home(),
        KeyCode::End => app.input_cursor_end(),

        _ => {}
    }
}

/// Handle project selector key events. Returns true if event was consumed.
fn handle_project_selector(app: &mut App, key: &KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc | KeyCode::Char('P') => {
            app.toggle_project_selector();
            true
        }
        KeyCode::Char('0') => {
            app.set_project_filter(None);
            app.toggle_project_selector();
            app.set_message("Showing all tasks".to_string());
            true
        }
        KeyCode::Char(c) if c.is_ascii_digit() => {
            let index = c.to_digit(10).unwrap() as usize;
            if index > 0 {
                let projects = app.get_projects();
                if let Some(project) = projects.get(index - 1) {
                    app.set_message(format!("Project: {}", project));
                    app.set_project_filter(Some(project.clone()));
                    app.toggle_project_selector();
                }
            }
            true
        }
        _ => false,
    }
}

/// Handle sprint selector key events. Returns true if event was consumed.
fn handle_sprint_selector(app: &mut App, key: &KeyEvent) -> bool {
    match key.code {
        // Navigation
        KeyCode::Down | KeyCode::Char('j') => {
            app.sprint_selector_down();
            false
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.sprint_selector_up();
            false
        }

        // Select sprint to view
        KeyCode::Enter => {
            app.select_sprint();
            true
        }

        // Create new sprint
        KeyCode::Char('n') => {
            app.toggle_sprint_selector(); // Close selector first
            app.enter_new_sprint_mode();
            true
        }

        // Edit sprint name
        KeyCode::Char('e') => {
            app.toggle_sprint_selector(); // Close selector first
            app.enter_edit_sprint_mode();
            true
        }

        // Edit sprint goal
        KeyCode::Char('g') => {
            app.toggle_sprint_selector(); // Close selector first
            app.enter_edit_sprint_goal_mode();
            true
        }

        // Delete sprint
        KeyCode::Char('d') => {
            app.toggle_sprint_selector(); // Close selector first
            app.enter_delete_sprint_mode();
            true
        }

        // Start sprint
        KeyCode::Char('s') => {
            app.start_sprint();
            false
        }

        // Complete sprint
        KeyCode::Char('c') => {
            app.complete_sprint();
            false
        }

        // Cancel
        KeyCode::Esc | KeyCode::Char('S') => {
            app.toggle_sprint_selector();
            true
        }

        _ => false,
    }
}

/// Handle move-to-sprint selector key events. Returns true if event was consumed.
fn handle_move_to_sprint(app: &mut App, key: &KeyEvent) -> bool {
    match key.code {
        // Navigation
        KeyCode::Down | KeyCode::Char('j') => {
            app.move_to_sprint_down();
            false
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.move_to_sprint_up();
            false
        }

        // Select
        KeyCode::Enter => {
            app.execute_move_to_sprint();
            true
        }

        // Cancel
        KeyCode::Esc | KeyCode::Char('m') => {
            app.toggle_move_to_sprint();
            true
        }

        _ => false,
    }
}
