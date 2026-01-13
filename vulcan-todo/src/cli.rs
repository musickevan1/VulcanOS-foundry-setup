use crate::models::{Priority, Status};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// VulcanOS Task Manager - A keyboard-driven TUI task manager
#[derive(Parser, Debug)]
#[command(name = "vulcan-todo")]
#[command(author = "VulcanOS")]
#[command(version = "0.1.0")]
#[command(about = "VulcanOS Task Manager - TUI and MCP server for task management", long_about = None)]
pub struct Cli {
    /// Run in MCP server mode (for OpenCode integration)
    #[arg(long)]
    pub mcp: bool,

    /// Run in TUI mode (default if no other flags)
    #[arg(long)]
    pub tui: bool,

    /// Path to task store (default: ~/.config/vulcan-todo/tasks.json)
    #[arg(long, short = 'p')]
    pub path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List tasks
    #[command(name = "list")]
    List {
        /// Filter by status: pending, done, all
        #[arg(long)]
        status: Option<StatusArg>,

        /// Filter by priority: low, medium, high, urgent
        #[arg(long)]
        priority: Option<PriorityArg>,

        /// Filter by project
        #[arg(long, short = 'P')]
        project: Option<String>,

        /// Search query
        #[arg(long, short = 's')]
        search: Option<String>,

        /// Limit results
        #[arg(long, short = 'n', default_value = "50")]
        limit: usize,
    },

    /// Show a single task
    #[command(name = "show")]
    Show {
        /// Task ID
        id: String,
    },

    /// Add a new task
    #[command(name = "add")]
    Add {
        /// Task title
        title: String,

        /// Task description
        #[arg(long, short = 'd')]
        description: Option<String>,

        /// Priority: low, medium, high, urgent
        #[arg(long, short = 'p')]
        priority: Option<PriorityArg>,

        /// Tags (can be specified multiple times)
        #[arg(long, short = 't')]
        tags: Vec<String>,

        /// Project name for organization
        #[arg(long, short = 'P')]
        project: Option<String>,

        /// Due date (YYYY-MM-DD)
        #[arg(long, short = 'D')]
        due: Option<String>,

        /// Assign to sprint (sprint ID)
        #[arg(long, short = 's')]
        sprint: Option<String>,
    },

    /// Edit a task
    #[command(name = "edit")]
    Edit {
        /// Task ID
        id: String,

        /// New title
        #[arg(long, short = 't')]
        title: Option<String>,

        /// New description
        #[arg(long, short = 'd')]
        description: Option<String>,

        /// New priority
        #[arg(long, short = 'p')]
        priority: Option<PriorityArg>,

        /// Set tags (replaces existing)
        #[arg(long, short = 'T')]
        tags: Vec<String>,

        /// Set project (use empty string to remove)
        #[arg(long, short = 'P')]
        project: Option<String>,
    },

    /// Complete a task
    #[command(name = "done")]
    Done {
        /// Task ID
        id: String,
    },

    /// Reopen a completed task
    #[command(name = "undone")]
    Undone {
        /// Task ID
        id: String,
    },

    /// Delete a task
    #[command(name = "delete")]
    Delete {
        /// Task ID
        id: String,
    },

    /// Assign a task to a sprint
    #[command(name = "assign")]
    Assign {
        /// Task ID
        id: String,

        /// Sprint ID (omit to remove from sprint)
        #[arg(long, short = 's')]
        sprint: Option<String>,
    },

    /// Reorder a task within its sprint
    #[command(name = "reorder")]
    Reorder {
        /// Task ID
        id: String,

        /// New position (1-based)
        #[arg(long, short = 'p')]
        position: i32,
    },

    /// Search tasks
    #[command(name = "search")]
    Search {
        /// Search query
        query: String,

        /// Limit results
        #[arg(long, short = 'n', default_value = "20")]
        limit: usize,
    },

    /// Show statistics
    #[command(name = "stats")]
    Stats,

    /// List all projects with task counts
    #[command(name = "projects")]
    Projects,

    /// Show tasks in a specific project
    #[command(name = "project")]
    Project {
        /// Project name
        name: String,
    },

    /// Interactive TUI mode
    #[command(name = "tui")]
    Tui,

    /// Sprint management
    #[command(name = "sprint")]
    Sprint {
        #[command(subcommand)]
        command: SprintCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum SprintCommands {
    /// List sprints
    #[command(name = "list")]
    List {
        /// Filter by project
        #[arg(long, short = 'P')]
        project: Option<String>,

        /// Filter by status: planning, active, completed
        #[arg(long, short = 's')]
        status: Option<SprintStatusArg>,
    },

    /// Create a new sprint
    #[command(name = "create")]
    Create {
        /// Sprint name
        name: String,

        /// Project name (required)
        #[arg(long, short = 'P')]
        project: String,

        /// Sprint goal or description
        #[arg(long, short = 'g')]
        goal: Option<String>,

        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start: Option<String>,

        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end: Option<String>,
    },

    /// Show sprint details
    #[command(name = "show")]
    Show {
        /// Sprint ID
        id: String,
    },

    /// Start a sprint (set status to active)
    #[command(name = "start")]
    Start {
        /// Sprint ID
        id: String,
    },

    /// Complete a sprint
    #[command(name = "complete")]
    Complete {
        /// Sprint ID
        id: String,
    },

    /// Delete a sprint (moves tasks to backlog)
    #[command(name = "delete")]
    Delete {
        /// Sprint ID
        id: String,
    },

    /// Show tasks in a sprint
    #[command(name = "tasks")]
    Tasks {
        /// Sprint ID
        id: String,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum StatusArg {
    /// Show pending tasks only
    Pending,
    /// Show completed tasks only
    Done,
    /// Show all tasks
    All,
}

impl From<StatusArg> for Status {
    fn from(val: StatusArg) -> Self {
        match val {
            StatusArg::Pending => Status::Pending,
            StatusArg::Done => Status::Done,
            StatusArg::All => Status::Pending, // Default filter
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PriorityArg {
    /// No priority
    None,
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Urgent priority
    Urgent,
}

impl From<PriorityArg> for Priority {
    fn from(val: PriorityArg) -> Self {
        match val {
            PriorityArg::None => Priority::None,
            PriorityArg::Low => Priority::Low,
            PriorityArg::Medium => Priority::Medium,
            PriorityArg::High => Priority::High,
            PriorityArg::Urgent => Priority::Urgent,
        }
    }
}

impl From<Priority> for Option<PriorityArg> {
    fn from(val: Priority) -> Self {
        match val {
            Priority::None => Some(PriorityArg::None),
            Priority::Low => Some(PriorityArg::Low),
            Priority::Medium => Some(PriorityArg::Medium),
            Priority::High => Some(PriorityArg::High),
            Priority::Urgent => Some(PriorityArg::Urgent),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SprintStatusArg {
    /// Planning status
    Planning,
    /// Active/in-progress status
    Active,
    /// Completed status
    Completed,
    /// All sprints
    All,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_add() {
        let cli = Cli::parse_from([
            "vulcan-todo",
            "add",
            "Buy groceries",
            "--description",
            "Get milk and bread",
            "--priority",
            "high",
            "-t",
            "shopping",
        ]);
        match cli.command {
            Some(Commands::Add {
                title,
                description,
                priority,
                tags,
                ..
            }) => {
                assert_eq!(title, "Buy groceries");
                assert_eq!(description, Some("Get milk and bread".to_string()));
                assert_eq!(priority, Some(PriorityArg::High));
                assert_eq!(tags, vec!["shopping"]);
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_cli_list() {
        let cli = Cli::parse_from(["vulcan-todo", "list", "--status", "pending"]);
        match cli.command {
            Some(Commands::List { status, .. }) => {
                assert_eq!(status, Some(StatusArg::Pending));
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_search() {
        let cli = Cli::parse_from(["vulcan-todo", "search", "groceries", "-n", "10"]);
        match cli.command {
            Some(Commands::Search { query, limit }) => {
                assert_eq!(query, "groceries");
                assert_eq!(limit, 10);
            }
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn test_cli_mcp_flag() {
        let cli = Cli::parse_from(["vulcan-todo", "--mcp"]);
        assert!(cli.mcp);
    }
}
