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
    if let Ok(naive_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
        return Ok(DateTime::<Utc>::from_naive_utc_and_offset(
            naive_datetime,
            Utc,
        ));
    }

    // Try parsing as RFC3339 format
    if let Ok(datetime) = DateTime::parse_from_rfc3339(date_str) {
        return Ok(datetime.with_timezone(&Utc));
    }

    Err(anyhow::anyhow!(
        "Invalid date format. Please use YYYY-MM-DD or RFC3339 format"
    ))
}
