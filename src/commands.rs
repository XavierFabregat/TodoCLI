use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};

use crate::db::Database;
use crate::models::Task;

pub fn add_task(
    db: &Database,
    title: &str,
    description: Option<&str>,
    due_date: Option<&str>,
    priority: &crate::Priority,
) -> Result<()> {
    let due_date_parsed = if let Some(due_str) = due_date {
        Some(parse_due_date(due_str)?)
    } else {
        None
    };

    let task = Task::new(
        title.to_string(),
        description.map(|s| s.to_string()),
        due_date_parsed,
        priority.to_int(),
    );

    let id = db.add_task(&task)?;
    println!("✅ Task added successfully with ID: {}", id);
    Ok(())
}

pub fn list_tasks(
    db: &Database,
    include_completed: bool,
    priority_filter: Option<&crate::Priority>,
) -> Result<()> {
    let priority_int = priority_filter.map(|p| p.to_int());
    let tasks = db.get_all_tasks(include_completed, priority_int)?;

    if tasks.is_empty() {
        println!("📝 No tasks found.");
        return Ok(());
    }

    println!("📋 Your tasks:");
    println!("{}", "─".repeat(80));

    let task_count = tasks.len();
    for task in tasks {
        println!("{}", task.display_summary());
    }

    println!("{}", "─".repeat(80));
    println!("Total: {} tasks", task_count);
    Ok(())
}

pub fn complete_task(db: &Database, id: i32) -> Result<()> {
    if !db.task_exists(id)? {
        return Err(anyhow::anyhow!("Task with ID {} not found", id));
    }

    db.complete_task(id)?;
    println!("✅ Task {} marked as completed!", id);
    Ok(())
}

pub fn delete_task(db: &Database, id: i32) -> Result<()> {
    if !db.task_exists(id)? {
        return Err(anyhow::anyhow!("Task with ID {} not found", id));
    }

    db.delete_task(id)?;
    println!("🗑️  Task {} deleted successfully!", id);
    Ok(())
}

pub fn update_task(
    db: &Database,
    id: i32,
    title: Option<&str>,
    description: Option<&str>,
    due_date: Option<&str>,
    priority: Option<&crate::Priority>,
) -> Result<()> {
    if !db.task_exists(id)? {
        return Err(anyhow::anyhow!("Task with ID {} not found", id));
    }

    let mut task = db.get_task_by_id(id)?.unwrap();

    if let Some(new_title) = title {
        task.title = new_title.to_string();
    }

    if let Some(new_description) = description {
        task.description = Some(new_description.to_string());
    }

    if let Some(due_str) = due_date {
        task.due_date = Some(parse_due_date(due_str)?);
    }

    if let Some(new_priority) = priority {
        task.priority = new_priority.to_int();
    }

    task.updated_at = Utc::now();

    db.update_task(id, &task)?;
    println!("✅ Task {} updated successfully!", id);
    Ok(())
}

pub fn show_task(db: &Database, id: i32) -> Result<()> {
    let task = db.get_task_by_id(id)?;

    match task {
        Some(task) => {
            println!("📋 Task Details:");
            println!("{}", "─".repeat(80));
            println!("{}", task.display_detailed());
            println!("{}", "─".repeat(80));
        }
        None => {
            return Err(anyhow::anyhow!("Task with ID {} not found", id));
        }
    }

    Ok(())
}

fn parse_due_date(date_str: &str) -> Result<DateTime<Utc>> {
    // Try parsing as YYYY-MM-DD format
    let parsed = if let Ok(naive_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
        DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc)
    } else if let Ok(datetime) = DateTime::parse_from_rfc3339(date_str) {
        datetime.with_timezone(&Utc)
    } else {
        return Err(anyhow::anyhow!(
            "Invalid date format. Please use YYYY-MM-DD or RFC3339 format"
        ));
    };

    if parsed < Utc::now() {
        return Err(anyhow::anyhow!("Due date must be in the future"));
    }

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use tempfile::NamedTempFile;

    fn create_test_db() -> (Database, NamedTempFile) {
        let temp_file = NamedTempFile::new().unwrap();
        let db = Database::new(temp_file.path()).unwrap();
        db.init().unwrap();
        (db, temp_file)
    }

    #[test]
    fn test_add_task() {
        let (db, _temp_file) = create_test_db();

        let priority = crate::Priority::High;
        add_task(
            &db,
            "Test task",
            Some("Test description"),
            Some("2024-12-31"),
            &priority,
        )
        .unwrap();

        let tasks = db.get_all_tasks(true, None).unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Test task");
        assert_eq!(tasks[0].priority, 2); // High priority
    }

    #[test]
    fn test_parse_due_date() {
        // Test YYYY-MM-DD format
        let date = parse_due_date("2024-12-31").unwrap();
        assert_eq!(date.format("%Y-%m-%d").to_string(), "2024-12-31");

        // Test RFC3339 format
        let rfc_date = parse_due_date("2024-12-31T00:00:00Z").unwrap();
        assert_eq!(rfc_date.format("%Y-%m-%d").to_string(), "2024-12-31");

        // Test invalid format
        assert!(parse_due_date("invalid-date").is_err());
    }

    #[test]
    fn test_complete_task() {
        let (db, _temp_file) = create_test_db();

        // Add a task first
        let priority = crate::Priority::Medium;
        add_task(&db, "Test task", None, None, &priority).unwrap();

        // Complete the task
        complete_task(&db, 1).unwrap();

        let task = db.get_task_by_id(1).unwrap().unwrap();
        assert!(task.completed);
    }

    #[test]
    fn test_complete_nonexistent_task() {
        let (db, _temp_file) = create_test_db();

        let result = complete_task(&db, 999);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_delete_task() {
        let (db, _temp_file) = create_test_db();

        // Add a task first
        let priority = crate::Priority::Medium;
        add_task(&db, "Test task", None, None, &priority).unwrap();

        // Delete the task
        delete_task(&db, 1).unwrap();

        // Verify task is deleted
        assert!(db.get_task_by_id(1).unwrap().is_none());
    }

    #[test]
    fn test_update_task() {
        let (db, _temp_file) = create_test_db();

        // Add a task first
        let priority = crate::Priority::Medium;
        add_task(&db, "Original title", None, None, &priority).unwrap();

        // Update the task
        let new_priority = crate::Priority::High;
        update_task(
            &db,
            1,
            Some("New title"),
            Some("New description"),
            Some("2024-12-31"),
            Some(&new_priority),
        )
        .unwrap();

        let task = db.get_task_by_id(1).unwrap().unwrap();
        assert_eq!(task.title, "New title");
        assert_eq!(task.description, Some("New description".to_string()));
        assert_eq!(task.priority, 2); // High priority
    }
}
