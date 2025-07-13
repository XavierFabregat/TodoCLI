use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

pub mod commands;
pub mod db;
pub mod models;

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
pub enum Priority {
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_db() -> (Database, NamedTempFile) {
        let temp_file = NamedTempFile::new().unwrap();
        let db = Database::new(temp_file.path()).unwrap();
        db.init().unwrap();
        (db, temp_file)
    }

    #[test]
    fn test_priority_to_int() {
        assert_eq!(Priority::Low.to_int(), 0);
        assert_eq!(Priority::Medium.to_int(), 1);
        assert_eq!(Priority::High.to_int(), 2);
    }

    #[test]
    fn test_priority_from_int() {
        assert!(matches!(Priority::from_int(0), Priority::Low));
        assert!(matches!(Priority::from_int(1), Priority::Medium));
        assert!(matches!(Priority::from_int(2), Priority::High));
        assert!(matches!(Priority::from_int(99), Priority::Medium)); // Default case
    }

    #[test]
    fn test_priority_color() {
        let low_color = Priority::Low.color();
        let medium_color = Priority::Medium.color();
        let high_color = Priority::High.color();

        assert!(low_color.to_string().contains("LOW"));
        assert!(medium_color.to_string().contains("MEDIUM"));
        assert!(high_color.to_string().contains("HIGH"));
    }

    #[test]
    fn test_get_db_path() {
        let result = get_db_path();
        assert!(result.is_ok());

        let path = result.unwrap();
        assert!(path.to_string_lossy().contains(".todo.db"));
    }

    #[test]
    fn test_database_initialization() {
        let (db, _temp_file) = create_test_db();

        // Test that database is properly initialized
        let task = models::Task::new("Test task".to_string(), None, None, 1);

        let id = db.add_task(&task).unwrap();
        assert_eq!(id, 1);
    }

    #[test]
    fn test_cli_commands_enum() {
        // Test that all command variants exist
        let _add = Commands::Add {
            title: "Test".to_string(),
            description: None,
            due: None,
            priority: Priority::Medium,
        };

        let _list = Commands::List {
            completed: false,
            priority: None,
        };

        let _complete = Commands::Complete { id: 1 };
        let _delete = Commands::Delete { id: 1 };
        let _show = Commands::Show { id: 1 };

        let _update = Commands::Update {
            id: 1,
            title: None,
            description: None,
            due: None,
            priority: None,
        };
    }

    #[test]
    fn test_priority_enum_variants() {
        // Test that all priority variants exist
        let _low = Priority::Low;
        let _medium = Priority::Medium;
        let _high = Priority::High;
    }

    #[test]
    fn test_cli_struct() {
        // Test that CLI struct can be created
        let _cli = Cli {
            command: Commands::List {
                completed: false,
                priority: None,
            },
        };
    }

    #[test]
    fn test_priority_ordering() {
        // Test that priorities are ordered correctly
        let priorities = vec![Priority::Low, Priority::Medium, Priority::High];
        let int_values: Vec<i32> = priorities.iter().map(|p| p.to_int()).collect();

        assert_eq!(int_values, vec![0, 1, 2]);
    }

    #[test]
    fn test_priority_roundtrip() {
        // Test that priority conversion is reversible
        let original = Priority::High;
        let int_value = original.to_int();
        let converted = Priority::from_int(int_value);

        assert!(matches!(converted, Priority::High));
    }

    #[test]
    fn test_invalid_priority_handling() {
        // Test that invalid priority values default to Medium
        let invalid_priority = Priority::from_int(999);
        assert!(matches!(invalid_priority, Priority::Medium));
    }
}
