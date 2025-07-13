use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

mod commands;
mod db;
mod models;

use commands::{add_task, complete_task, delete_task, list_tasks, show_task, update_task};
use db::Database;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo CLI tool with SQLite storage")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// Task title
        title: String,
        /// Task description
        #[arg(long)]
        description: Option<String>,
        /// Due date (YYYY-MM-DD format)
        #[arg(short, long)]
        due: Option<String>,
        /// Priority level (low, medium, high)
        #[arg(short, long, value_enum, default_value = "medium")]
        priority: Priority,
    },
    /// List all tasks
    List {
        /// Show completed tasks
        #[arg(short, long)]
        completed: bool,
        /// Filter by priority
        #[arg(short, long, value_enum)]
        priority: Option<Priority>,
    },
    /// Mark a task as completed
    Complete {
        /// Task ID
        id: i32,
    },
    /// Delete a task
    Delete {
        /// Task ID
        id: i32,
    },
    /// Update a task
    Update {
        /// Task ID
        id: i32,
        /// New title
        #[arg(short, long)]
        title: Option<String>,
        /// New description
        #[arg(long)]
        description: Option<String>,
        /// New due date (YYYY-MM-DD format)
        #[arg(short, long)]
        due: Option<String>,
        /// New priority level
        #[arg(short, long, value_enum)]
        priority: Option<Priority>,
    },
    /// Show details of a specific task
    Show {
        /// Task ID
        id: i32,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    fn to_int(&self) -> i32 {
        match self {
            Priority::Low => 0,
            Priority::Medium => 1,
            Priority::High => 2,
        }
    }

    fn from_int(value: i32) -> Self {
        match value {
            0 => Priority::Low,
            1 => Priority::Medium,
            2 => Priority::High,
            _ => Priority::Medium,
        }
    }

    fn color(&self) -> colored::ColoredString {
        match self {
            Priority::Low => "LOW".blue(),
            Priority::Medium => "MEDIUM".yellow(),
            Priority::High => "HIGH".red(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize database
    let db_path = get_db_path()?;
    let db = Database::new(&db_path)?;
    db.init()?;

    match &cli.command {
        Commands::Add {
            title,
            description,
            due,
            priority,
        } => add_task(&db, title, description.as_deref(), due.as_deref(), priority)?,
        Commands::List {
            completed,
            priority,
        } => list_tasks(&db, *completed, priority.as_ref())?,
        Commands::Complete { id } => complete_task(&db, *id)?,
        Commands::Delete { id } => delete_task(&db, *id)?,
        Commands::Update {
            id,
            title,
            description,
            due,
            priority,
        } => update_task(
            &db,
            *id,
            title.as_deref(),
            description.as_deref(),
            due.as_deref(),
            priority.as_ref(),
        )?,
        Commands::Show { id } => show_task(&db, *id)?,
    }

    Ok(())
}

fn get_db_path() -> anyhow::Result<PathBuf> {
    let mut path =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    path.push(".todo.db");
    Ok(path)
}
