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
