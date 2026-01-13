use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use tokio;

mod cli;
mod mcp;
mod models;
mod store;
mod ui;

pub use models::{Sprint, Task};

/// Store type alias
type StoreType = store::JsonStore;

fn get_store(path: Option<PathBuf>) -> Result<Arc<dyn store::Store>> {
    let store = if let Some(p) = path {
        StoreType::with_path(p)?
    } else {
        StoreType::new()?
    };
    Ok(Arc::new(store))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse arguments
    let cli = cli::Cli::parse();

    // Determine path from args or environment
    let path = cli.path.or_else(|| {
        std::env::var("VULCAN_TODO_PATH")
            .ok()
            .map(|p| PathBuf::from(p))
    });

    // Get store
    let store = get_store(path)?;

    // Handle mode
    if cli.mcp {
        // Run MCP server mode
        mcp::run_mcp_server(store).await
    } else if cli.command.is_some() {
        // Handle CLI commands
        handle_command(cli.command.unwrap(), &store)
    } else {
        // Default: run TUI (if feature enabled)
        #[cfg(feature = "tui")]
        {
            ui::run_tui(store)?;
            Ok(())
        }
        #[cfg(not(feature = "tui"))]
        {
            eprintln!("TUI feature not enabled. Use --mcp for MCP server mode.");
            eprintln!("Or rebuild with --features tui");
            Ok(())
        }
    }
}

fn handle_command(command: cli::Commands, store: &Arc<dyn store::Store>) -> Result<()> {
    match command {
        cli::Commands::List {
            status,
            priority,
            project,
            search,
            limit,
        } => {
            let all_tasks = store.get_all()?;
            let mut tasks: Vec<Task> = all_tasks
                .into_iter()
                .filter(|t| {
                    let status_match = match status {
                        Some(cli::StatusArg::Pending) => t.is_pending(),
                        Some(cli::StatusArg::Done) => t.is_done(),
                        Some(cli::StatusArg::All) => true,
                        None => true,
                    };
                    let priority_match = match priority {
                        Some(p) => t.priority == p.into(),
                        None => true,
                    };
                    let project_match = match project {
                        Some(ref proj) => t.belongs_to_project(proj),
                        None => true,
                    };
                    let search_match = match &search {
                        Some(q) => {
                            let q = q.to_lowercase();
                            t.title.to_lowercase().contains(&q)
                                || t.description
                                    .as_ref()
                                    .map(|d| d.to_lowercase().contains(&q))
                                    .unwrap_or(false)
                        }
                        None => true,
                    };
                    status_match && priority_match && project_match && search_match
                })
                .collect();

            // Sort by priority
            tasks.sort_by(|a, b| b.priority.level().cmp(&a.priority.level()));

            // Apply limit
            let tasks: Vec<_> = tasks.into_iter().take(limit).collect();

            // Print
            println!("Tasks ({}):\n", tasks.len());
            for (i, task) in tasks.iter().enumerate() {
                let status = if task.is_done() { "[✓]" } else { "[ ]" };
                let priority = task.priority.emoji();
                println!("{}. {} {} {}", i + 1, status, priority, task.title);
            }
            Ok(())
        }

        cli::Commands::Show { id } => {
            let task = store.get(&id)?;
            match task {
                Some(t) => {
                    println!("Task: {}", t.title);
                    println!("ID: {}", t.id);
                    println!("Status: {}", t.status);
                    println!("Priority: {}", t.priority);
                    println!("Tags: {:?}", t.tags);
                    println!("Created: {}", t.created_formatted());
                    if let Some(desc) = &t.description {
                        println!("\nDescription:\n{}", desc);
                    }
                    Ok(())
                }
                None => {
                    eprintln!("Task not found: {}", id);
                    Ok(())
                }
            }
        }

        cli::Commands::Add {
            title,
            description,
            priority,
            tags,
            project,
            due: _,
            sprint,
        } => {
            let mut task = models::Task::new(title);
            task.description = description;
            if let Some(p) = priority {
                task.priority = p.into();
            }
            task.tags = tags;
            task.project = project;

            let created = store.add(&task)?;

            // Assign to sprint if specified
            if let Some(sprint_id) = sprint {
                store.assign_task_to_sprint(&created.id, &sprint_id)?;
                println!("Task created and assigned to sprint: {}", created.id);
            } else {
                println!("Task created: {}", created.id);
            }

            println!("Title: {}", created.title);
            if let Some(ref proj) = created.project {
                println!("Project: {}", proj);
            }
            Ok(())
        }

        cli::Commands::Edit {
            id,
            title,
            description,
            priority,
            tags,
            project,
        } => {
            let existing = store.get(&id)?;
            match existing {
                Some(mut t) => {
                    if let Some(title) = title {
                        t.title = title;
                    }
                    if let Some(desc) = description {
                        t.description = Some(desc);
                    }
                    if let Some(priority) = priority {
                        t.priority = priority.into();
                    }
                    if !tags.is_empty() {
                        t.tags = tags;
                    }
                    if let Some(proj) = project {
                        if proj.is_empty() {
                            t.project = None;
                        } else {
                            t.project = Some(proj);
                        }
                    }

                    let updated = store.update(&t)?;
                    match updated {
                        Some(u) => {
                            println!("Task updated: {}", u.id);
                        }
                        None => {
                            eprintln!("Task not found: {}", id);
                        }
                    }
                    Ok(())
                }
                None => {
                    eprintln!("Task not found: {}", id);
                    Ok(())
                }
            }
        }

        cli::Commands::Done { id } => {
            let mut task = store.get(&id)?;
            match task {
                Some(mut t) => {
                    t.complete();
                    store.update(&t)?;
                    println!("Task completed: {}", t.title);
                    Ok(())
                }
                None => {
                    eprintln!("Task not found: {}", id);
                    Ok(())
                }
            }
        }

        cli::Commands::Undone { id } => {
            let mut task = store.get(&id)?;
            match task {
                Some(mut t) => {
                    t.uncomplete();
                    store.update(&t)?;
                    println!("Task reopened: {}", t.title);
                    Ok(())
                }
                None => {
                    eprintln!("Task not found: {}", id);
                    Ok(())
                }
            }
        }

        cli::Commands::Delete { id } => {
            let deleted = store.delete(&id)?;
            if deleted {
                println!("Task deleted: {}", id);
            } else {
                eprintln!("Task not found: {}", id);
            }
            Ok(())
        }

        cli::Commands::Search { query, limit } => {
            let results = store.search(&query)?;
            let results: Vec<_> = results.into_iter().take(limit).collect();

            println!("Results for '{}':\n", query);
            for (i, task) in results.iter().enumerate() {
                let status = if task.is_done() { "[✓]" } else { "[ ]" };
                let priority = task.priority.emoji();
                println!("{}. {} {} {}", i + 1, status, priority, task.title);
            }
            Ok(())
        }

        cli::Commands::Stats => {
            let (pending, done) = store.count()?;
            let total = pending + done;
            let percent = if total > 0 {
                (done as f64 / total as f64 * 100.0).round() as u16
            } else {
                0
            };

            println!("Task Statistics:");
            println!("  Pending: {}", pending);
            println!("  Done: {}", done);
            println!("  Total: {}", total);
            println!("  Complete: {}%", percent);
            Ok(())
        }

        cli::Commands::Projects => {
            let projects = store.get_projects()?;
            let stats = store.get_project_stats()?;

            if projects.is_empty() {
                println!("No projects found. Use 'project:tagname' in tags to organize tasks.");
            } else {
                println!("Projects ({}):\n", projects.len());
                println!(
                    "{:<20} {:>10} {:>10} {:>10}",
                    "PROJECT", "PENDING", "DONE", "TOTAL"
                );
                println!("{:-<52}", "-");

                for project in projects {
                    let (pending, done) = stats.get(&project).copied().unwrap_or((0, 0));
                    let total = pending + done;
                    println!("{:<20} {:>10} {:>10} {:>10}", project, pending, done, total);
                }
            }
            Ok(())
        }

        cli::Commands::Project { name } => {
            let tasks = store.get_by_project(&name)?;

            println!("Tasks in '{}' ({}):\n", name, tasks.len());
            for (i, task) in tasks.iter().enumerate() {
                let status = if task.is_done() { "[✓]" } else { "[ ]" };
                let priority = task.priority.emoji();
                println!("{}. {} {} {}", i + 1, status, priority, task.title);
            }
            Ok(())
        }

        cli::Commands::Tui => {
            #[cfg(feature = "tui")]
            {
                ui::run_tui(store.clone())?;
                Ok(())
            }
            #[cfg(not(feature = "tui"))]
            {
                eprintln!("TUI feature not enabled. Rebuild with --features tui");
                Ok(())
            }
        }

        cli::Commands::Assign { id, sprint } => {
            match sprint {
                Some(sprint_id) => {
                    store.assign_task_to_sprint(&id, &sprint_id)?;
                    println!("Task {} assigned to sprint {}", id, sprint_id);
                }
                None => {
                    store.remove_task_from_sprint(&id)?;
                    println!("Task {} removed from sprint (moved to backlog)", id);
                }
            }
            Ok(())
        }

        cli::Commands::Reorder { id, position } => {
            store.reorder_task_in_sprint(&id, position)?;
            println!("Task {} reordered to position {}", id, position);
            Ok(())
        }

        cli::Commands::Sprint { command } => handle_sprint_command(command, store),
    }
}

fn handle_sprint_command(
    command: cli::SprintCommands,
    store: &Arc<dyn store::Store>,
) -> Result<()> {
    use cli::SprintCommands;
    use models::Sprint;

    match command {
        SprintCommands::List { project, status } => {
            let sprints = if let Some(proj) = project {
                store.get_sprints_by_project(&proj)?
            } else {
                store.get_all_sprints()?
            };

            // Filter by status if specified
            let sprints: Vec<Sprint> = sprints
                .into_iter()
                .filter(|s| match status {
                    Some(cli::SprintStatusArg::Planning) => {
                        s.status == models::SprintStatus::Planning
                    }
                    Some(cli::SprintStatusArg::Active) => s.status == models::SprintStatus::Active,
                    Some(cli::SprintStatusArg::Completed) => {
                        s.status == models::SprintStatus::Completed
                    }
                    Some(cli::SprintStatusArg::All) | None => true,
                })
                .collect();

            if sprints.is_empty() {
                println!("No sprints found.");
            } else {
                println!("Sprints ({}):\n", sprints.len());
                println!(
                    "{:<8} {:<20} {:<15} {:<10}",
                    "STATUS", "NAME", "PROJECT", "ID"
                );
                println!("{:-<53}", "-");

                for sprint in sprints {
                    let status_emoji = match sprint.status {
                        models::SprintStatus::Planning => "📋",
                        models::SprintStatus::Active => "🏃",
                        models::SprintStatus::Completed => "✅",
                        models::SprintStatus::Archived => "📦",
                    };
                    println!(
                        "{} {:<6} {:<20} {:<15} {}",
                        status_emoji,
                        sprint.status.to_string().to_uppercase(),
                        sprint.name,
                        sprint.project,
                        &sprint.id[..8]
                    );
                }
            }
            Ok(())
        }

        SprintCommands::Create {
            name,
            project,
            goal,
            start: _,
            end: _,
        } => {
            let mut sprint = Sprint::new(name.clone(), project);
            sprint.goal = goal;
            // TODO: Parse start/end dates if provided

            store.add_sprint(&sprint)?;
            println!("Sprint created: {}", sprint.id);
            println!("Name: {}", sprint.name);
            println!("Project: {}", sprint.project);
            if let Some(g) = &sprint.goal {
                println!("Goal: {}", g);
            }
            Ok(())
        }

        SprintCommands::Show { id } => {
            let sprint = store.get_sprint(&id)?;
            match sprint {
                Some(s) => {
                    println!("Sprint: {}", s.name);
                    println!("ID: {}", s.id);
                    println!("Project: {}", s.project);
                    println!("Status: {}", s.status);
                    if let Some(goal) = &s.goal {
                        println!("Goal: {}", goal);
                    }
                    if let Some(start) = s.start_date {
                        println!("Started: {}", start.format("%Y-%m-%d"));
                    }
                    if let Some(end) = s.end_date {
                        println!("Ended: {}", end.format("%Y-%m-%d"));
                    }
                    println!("Created: {}", s.created_at.format("%Y-%m-%d"));

                    // Show tasks in sprint
                    let tasks = store.get_tasks_in_sprint(&id)?;
                    println!("\nTasks in sprint: {}", tasks.len());
                    for task in tasks {
                        let status = if task.is_done() { "[✓]" } else { "[ ]" };
                        let priority = task.priority.emoji();
                        println!("  {} {} {}", status, priority, task.title);
                    }
                    Ok(())
                }
                None => {
                    eprintln!("Sprint not found: {}", id);
                    Ok(())
                }
            }
        }

        SprintCommands::Start { id } => {
            let sprint = store.get_sprint(&id)?;
            match sprint {
                Some(mut s) => {
                    s.start();
                    store.update_sprint(&s)?;
                    println!("Sprint started: {}", s.name);
                    Ok(())
                }
                None => {
                    eprintln!("Sprint not found: {}", id);
                    Ok(())
                }
            }
        }

        SprintCommands::Complete { id } => {
            let sprint = store.get_sprint(&id)?;
            match sprint {
                Some(mut s) => {
                    s.complete();
                    store.update_sprint(&s)?;
                    println!("Sprint completed: {}", s.name);
                    Ok(())
                }
                None => {
                    eprintln!("Sprint not found: {}", id);
                    Ok(())
                }
            }
        }

        SprintCommands::Delete { id } => {
            let deleted = store.delete_sprint(&id)?;
            if deleted {
                println!("Sprint deleted: {}", id);
                println!("Tasks have been moved to backlog.");
            } else {
                eprintln!("Sprint not found: {}", id);
            }
            Ok(())
        }

        SprintCommands::Tasks { id } => {
            let tasks = store.get_tasks_in_sprint(&id)?;

            // Sort by sprint_order
            let mut tasks = tasks;
            tasks.sort_by_key(|t| t.sprint_order.unwrap_or(i32::MAX));

            println!("Tasks in sprint ({}):\n", tasks.len());
            for (i, task) in tasks.iter().enumerate() {
                let status = if task.is_done() { "[✓]" } else { "[ ]" };
                let priority = task.priority.emoji();
                let order = task
                    .sprint_order
                    .map(|o| format!("#{}", o))
                    .unwrap_or_default();
                println!(
                    "{}. {} {} {} {}",
                    i + 1,
                    order,
                    status,
                    priority,
                    task.title
                );
            }
            Ok(())
        }
    }
}
