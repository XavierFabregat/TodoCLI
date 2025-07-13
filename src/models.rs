use chrono::{DateTime, Utc};
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: i32, // 0=low, 1=medium, 2=high
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(
        title: String,
        description: Option<String>,
        due_date: Option<DateTime<Utc>>,
        priority: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            title,
            description,
            due_date,
            priority,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn priority_text(&self) -> &'static str {
        match self.priority {
            0 => "LOW",
            1 => "MEDIUM",
            2 => "HIGH",
            _ => "MEDIUM",
        }
    }

    pub fn priority_color(&self) -> ColoredString {
        match self.priority {
            0 => "LOW".blue(),
            1 => "MEDIUM".yellow(),
            2 => "HIGH".red(),
            _ => "MEDIUM".yellow(),
        }
    }

    pub fn status_text(&self) -> ColoredString {
        if self.completed {
            "✓ COMPLETED".green()
        } else {
            "○ PENDING".white()
        }
    }

    pub fn due_date_text(&self) -> String {
        self.due_date
            .map(|date| date.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "No due date".to_string())
    }

    pub fn is_overdue(&self) -> bool {
        if self.completed {
            return false;
        }

        self.due_date.map(|due| Utc::now() > due).unwrap_or(false)
    }

    pub fn display_summary(&self) -> String {
        let id = self.id.unwrap_or(0);
        let priority = self.priority_color();
        let status = self.status_text();
        let due = if self.is_overdue() {
            self.due_date_text().red()
        } else {
            self.due_date_text().white()
        };

        format!("[{}] {} {} {} {}", id, self.title, priority, status, due)
    }

    pub fn display_detailed(&self) -> String {
        let id = self.id.unwrap_or(0);
        let priority = self.priority_color();
        let status = self.status_text();
        let due = if self.is_overdue() {
            self.due_date_text().red()
        } else {
            self.due_date_text().white()
        };

        let description = self
            .description
            .as_ref()
            .map(|desc| format!("\nDescription: {}", desc))
            .unwrap_or_else(|| "".to_string());

        format!(
            "Task #{}: {}\nPriority: {}\nStatus: {}\nDue: {}{}\nCreated: {}\nUpdated: {}",
            id,
            self.title,
            priority,
            status,
            due,
            description,
            self.created_at.format("%Y-%m-%d %H:%M"),
            self.updated_at.format("%Y-%m-%d %H:%M")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    fn create_test_task() -> Task {
        Task::new(
            "Test task".to_string(),
            Some("Test description".to_string()),
            Some(Utc::now() + Duration::days(1)),
            1,
        )
    }

    #[test]
    fn test_task_creation() {
        let task = create_test_task();
        
        assert_eq!(task.title, "Test task");
        assert_eq!(task.description, Some("Test description".to_string()));
        assert_eq!(task.priority, 1);
        assert_eq!(task.completed, false);
        assert!(task.id.is_none());
    }

    #[test]
    fn test_priority_text() {
        let mut task = create_test_task();
        
        task.priority = 0;
        assert_eq!(task.priority_text(), "LOW");
        
        task.priority = 1;
        assert_eq!(task.priority_text(), "MEDIUM");
        
        task.priority = 2;
        assert_eq!(task.priority_text(), "HIGH");
        
        task.priority = 99;
        assert_eq!(task.priority_text(), "MEDIUM"); // Default case
    }

    #[test]
    fn test_due_date_text() {
        let mut task = create_test_task();
        
        // With due date
        let due_date = Utc::now() + Duration::days(1);
        task.due_date = Some(due_date);
        let due_text = task.due_date_text();
        assert!(due_text.contains(&due_date.format("%Y-%m-%d").to_string()));
        
        // Without due date
        task.due_date = None;
        assert_eq!(task.due_date_text(), "No due date");
    }

    #[test]
    fn test_is_overdue() {
        let mut task = create_test_task();
        
        // Future date - not overdue
        task.due_date = Some(Utc::now() + Duration::days(1));
        assert!(!task.is_overdue());
        
        // Past date - overdue
        task.due_date = Some(Utc::now() - Duration::days(1));
        assert!(task.is_overdue());
        
        // Completed task - not overdue even if past due
        task.completed = true;
        assert!(!task.is_overdue());
        
        // No due date - not overdue
        task.due_date = None;
        task.completed = false;
        assert!(!task.is_overdue());
    }

    #[test]
    fn test_display_summary() {
        let mut task = create_test_task();
        task.id = Some(42);
        
        let summary = task.display_summary();
        assert!(summary.contains("[42]"));
        assert!(summary.contains("Test task"));
        assert!(summary.contains("MEDIUM"));
    }

    #[test]
    fn test_display_detailed() {
        let mut task = create_test_task();
        task.id = Some(42);
        
        let detailed = task.display_detailed();
        assert!(detailed.contains("Task #42:"));
        assert!(detailed.contains("Test task"));
        assert!(detailed.contains("Test description"));
        assert!(detailed.contains("MEDIUM"));
    }

    #[test]
    fn test_task_with_id() {
        let mut task = create_test_task();
        task.id = Some(123);
        
        assert_eq!(task.id, Some(123));
    }

    #[test]
    fn test_task_serialization() {
        let task = create_test_task();
        
        // Test serialization
        let json = serde_json::to_string(&task).unwrap();
        assert!(json.contains("Test task"));
        assert!(json.contains("Test description"));
        
        // Test deserialization
        let deserialized_task: Task = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized_task.title, task.title);
        assert_eq!(deserialized_task.description, task.description);
        assert_eq!(deserialized_task.priority, task.priority);
    }

    #[test]
    fn test_task_without_description() {
        let task = Task::new(
            "Simple task".to_string(),
            None,
            None,
            0,
        );
        
        assert_eq!(task.title, "Simple task");
        assert_eq!(task.description, None);
        assert_eq!(task.priority, 0);
        assert_eq!(task.completed, false);
    }

    #[test]
    fn test_high_priority_task() {
        let task = Task::new(
            "Urgent task".to_string(),
            Some("Very important".to_string()),
            Some(Utc::now() + Duration::hours(1)),
            2,
        );
        
        assert_eq!(task.priority_text(), "HIGH");
        assert_eq!(task.title, "Urgent task");
        assert!(task.due_date.is_some());
    }

    #[test]
    fn test_completed_task_status() {
        let mut task = create_test_task();
        task.completed = true;
        
        let status = task.status_text();
        assert!(status.to_string().contains("COMPLETED"));
    }

    #[test]
    fn test_pending_task_status() {
        let task = create_test_task();
        
        let status = task.status_text();
        assert!(status.to_string().contains("PENDING"));
    }
}
