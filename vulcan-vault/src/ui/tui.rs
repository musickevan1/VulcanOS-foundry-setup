//! Terminal setup and main event loop

use std::io::stdout;
use std::sync::Arc;
use std::sync::mpsc;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, EventKind};
use ratatui::prelude::*;

use super::app::App;
use crate::store::Store;

/// Run the TUI application
pub fn run_tui<S: Store + 'static>(store: Arc<S>) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new(store);
    app.load_data()?;

    // Setup file watcher
    let _watcher = setup_file_watcher(&mut app);

    // Main loop
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

/// Setup file watcher for vault directory
fn setup_file_watcher(app: &mut App) -> Option<RecommendedWatcher> {
    let vault_dir = crate::vault_dir();
    if !vault_dir.exists() {
        return None;
    }

    let (tx, rx) = mpsc::channel();
    app.set_file_watcher(rx);

    // Create watcher with debounce
    let watcher_result = RecommendedWatcher::new(
        move |res: Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                // Only react to modify/create/remove events on .md files
                match event.kind {
                    EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                        let is_markdown = event.paths.iter().any(|p| {
                            p.extension().map(|e| e == "md").unwrap_or(false)
                        });
                        if is_markdown {
                            let _ = tx.send(());
                        }
                    }
                    _ => {}
                }
            }
        },
        Config::default().with_poll_interval(Duration::from_secs(2)),
    );

    match watcher_result {
        Ok(mut watcher) => {
            if watcher.watch(&vault_dir, RecursiveMode::Recursive).is_ok() {
                Some(watcher)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn run_app<B: Backend + std::io::Write>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| app.render(f))?;

        // Check for file changes and message timeout
        app.check_file_changes()?;
        app.check_message_timeout();

        // Poll for events with 100ms timeout
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Handle quit
                if key.code == KeyCode::Char('q') && app.input_mode.is_none() {
                    return Ok(());
                }

                // Handle escape
                if key.code == KeyCode::Esc {
                    if app.input_mode.is_some() {
                        app.input_mode = None;
                        app.input_buffer.clear();
                    } else if app.show_help {
                        app.show_help = false;
                    } else if app.view_mode == crate::ui::app::ViewMode::Detail {
                        app.exit_detail();
                    } else if !app.search_query.is_empty() {
                        app.clear_search();
                    } else {
                        return Ok(());
                    }
                    continue;
                }

                // Handle input mode
                if let Some(ref mode) = app.input_mode.clone() {
                    handle_input_mode(app, key.code, key.modifiers, mode.clone())?;
                    continue;
                }

                // Handle detail view mode separately
                if app.view_mode == crate::ui::app::ViewMode::Detail {
                    match key.code {
                        KeyCode::Char('?') => app.show_help = !app.show_help,
                        KeyCode::Char('j') | KeyCode::Down => app.scroll_detail_down(),
                        KeyCode::Char('k') | KeyCode::Up => app.scroll_detail_up(),
                        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            for _ in 0..10 { app.scroll_detail_up(); }
                        }
                        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            for _ in 0..10 { app.scroll_detail_down(); }
                        }
                        KeyCode::Char('g') | KeyCode::Home => app.detail_scroll = 0,
                        KeyCode::Char('G') | KeyCode::End => app.detail_scroll = usize::MAX,
                        _ => {}
                    }
                    continue;
                }

                // Handle global keys (list/preview mode)
                match key.code {
                    KeyCode::Char('?') => app.show_help = !app.show_help,
                    KeyCode::Char('/') => {
                        app.input_mode = Some(InputMode::Search);
                        app.input_buffer.clear();
                    }
                    KeyCode::Char('s') if key.modifiers.is_empty() => {
                        app.input_mode = Some(InputMode::SemanticSearch);
                        app.input_buffer.clear();
                    }
                    KeyCode::Char('j') | KeyCode::Down => app.next(),
                    KeyCode::Char('k') | KeyCode::Up => app.previous(),
                    KeyCode::Char('g') | KeyCode::Home => app.first(),
                    KeyCode::Char('G') | KeyCode::End => app.last(),
                    KeyCode::Tab => app.toggle_pane(),
                    KeyCode::Enter => app.view_detail(),
                    KeyCode::Char('1') => app.set_type_filter(Some(crate::NoteType::Project)),
                    KeyCode::Char('2') => app.set_type_filter(Some(crate::NoteType::Task)),
                    KeyCode::Char('3') => app.set_type_filter(Some(crate::NoteType::Learning)),
                    KeyCode::Char('4') => app.set_type_filter(Some(crate::NoteType::Memory)),
                    KeyCode::Char('5') => app.set_type_filter(Some(crate::NoteType::Meta)),
                    KeyCode::Char('0') => app.set_type_filter(None),
                    KeyCode::Char('m') => app.toggle_memory_view(),
                    KeyCode::Char('r') => app.reinforce_memory()?,
                    KeyCode::Char('c') => app.clear_filters(),
                    // Note CRUD
                    KeyCode::Char('n') => app.start_create_note(),
                    KeyCode::Char('e') => {
                        if app.edit_note()? {
                            // Need to exit TUI to run editor
                            if let Some(path) = app.get_edit_path() {
                                // Store the path for edit_and_resume
                                return edit_and_resume(terminal, app, &path);
                            }
                        }
                    }
                    KeyCode::Char('d') if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.start_delete_note();
                    }
                    KeyCode::Char('l') => app.start_link_task(),
                    KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.scroll_preview_up();
                    }
                    KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.scroll_preview_down();
                    }
                    _ => {}
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum InputMode {
    Search,
    SemanticSearch,
    /// Creating a new note - entering title
    CreateTitle,
    /// Creating a new note - entering project (optional)
    CreateProject,
    /// Confirming note deletion
    DeleteConfirm,
    /// Linking note to a task
    LinkTask,
}

fn handle_input_mode(app: &mut App, key: KeyCode, _key_modifiers: KeyModifiers, mode: InputMode) -> Result<()> {
    match key {
        KeyCode::Enter => {
            let input = app.input_buffer.clone();
            app.input_mode = None;

            match mode {
                InputMode::Search => {
                    if !input.is_empty() {
                        app.is_semantic_search = false;
                        app.search_query = input;
                        app.apply_filters();
                    }
                }
                InputMode::SemanticSearch => {
                    if !input.is_empty() {
                        app.semantic_search(&input)?;
                    }
                }
                InputMode::CreateTitle => {
                    app.create_title = input.clone();
                    if !input.is_empty() {
                        // Go to project input (optional)
                        app.input_buffer.clear();
                        app.input_mode = Some(InputMode::CreateProject);
                    }
                }
                InputMode::CreateProject => {
                    // Use input as project if provided, otherwise use current filter
                    let title = app.create_title.clone();
                    if !input.is_empty() {
                        app.project_filter = Some(input);
                    }
                    app.finish_create_note(title)?;
                }
                InputMode::DeleteConfirm => {
                    if input.to_lowercase() == "y" || input.to_lowercase() == "yes" {
                        app.delete_note()?;
                    } else {
                        app.set_message("Delete cancelled");
                    }
                }
                InputMode::LinkTask => {
                    app.link_note_to_task(input)?;
                }
            }
        }
        KeyCode::Tab if matches!(mode, InputMode::CreateTitle) => {
            app.cycle_create_type();
        }
        KeyCode::Char(c) => {
            app.input_buffer.push(c);
        }
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        KeyCode::Esc => {
            app.input_mode = None;
            app.input_buffer.clear();
        }
        _ => {}
    }
    Ok(())
}

/// Exit TUI, run editor, then resume
fn edit_and_resume<B: Backend + std::io::Write>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    path: &std::path::Path,
) -> Result<()> {
    use std::process::Command;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Get editor
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());

    // Run editor
    let status = Command::new(&editor)
        .arg(path)
        .status();

    // Re-setup terminal
    enable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;

    match status {
        Ok(s) if s.success() => {
            app.set_message("Note saved");
            app.load_data()?;
        }
        Ok(_) => {
            app.set_message("Editor exited with error");
        }
        Err(e) => {
            app.set_message(format!("Failed to run {}: {}", editor, e));
        }
    }

    // Continue the main loop
    run_app(terminal, app)
}
