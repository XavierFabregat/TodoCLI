use crate::models::Task;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result as SqliteResult};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &std::path::Path) -> SqliteResult<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    pub fn init(&self) -> SqliteResult<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                due_date TEXT,
                priority INTEGER DEFAULT 1,
                completed BOOLEAN DEFAULT FALSE,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_task(&self, task: &Task) -> SqliteResult<i32> {
        let due_date_str = task.due_date.map(|d| d.to_rfc3339());

        self.conn.execute(
            "INSERT INTO tasks (title, description, due_date, priority, completed, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                task.title,
                task.description,
                due_date_str,
                task.priority,
                task.completed,
                task.created_at.to_rfc3339(),
                task.updated_at.to_rfc3339(),
            ],
        )?;

        Ok(self.conn.last_insert_rowid() as i32)
    }

    pub fn get_all_tasks(
        &self,
        include_completed: bool,
        priority_filter: Option<i32>,
    ) -> SqliteResult<Vec<Task>> {
        let mut query = String::from(
            "SELECT id, title, description, due_date, priority, completed, created_at, updated_at 
             FROM tasks",
        );

        let mut conditions = Vec::new();
        if !include_completed {
            conditions.push("completed = FALSE".to_string());
        }
        if let Some(priority) = priority_filter {
            conditions.push(format!("priority = {}", priority));
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(" ORDER BY priority DESC, created_at ASC");

        let mut stmt = self.conn.prepare(&query)?;
        let task_iter = stmt.query_map([], |row| {
            let due_date_str: Option<String> = row.get(3)?;
            let due_date = due_date_str
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc));

            Ok(Task {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                description: row.get(2)?,
                due_date,
                priority: row.get(4)?,
                completed: row.get(5)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?;

        task_iter.collect()
    }

    pub fn get_task_by_id(&self, id: i32) -> SqliteResult<Option<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, due_date, priority, completed, created_at, updated_at 
             FROM tasks WHERE id = ?",
        )?;

        let mut task_iter = stmt.query_map([id], |row| {
            let due_date_str: Option<String> = row.get(3)?;
            let due_date = due_date_str
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc));

            Ok(Task {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                description: row.get(2)?,
                due_date,
                priority: row.get(4)?,
                completed: row.get(5)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?;

        task_iter.next().transpose()
    }

    pub fn update_task(&self, id: i32, task: &Task) -> SqliteResult<()> {
        let due_date_str = task.due_date.map(|d| d.to_rfc3339());

        self.conn.execute(
            "UPDATE tasks 
             SET title = ?1, description = ?2, due_date = ?3, priority = ?4, 
                 completed = ?5, updated_at = ?6
             WHERE id = ?7",
            params![
                task.title,
                task.description,
                due_date_str,
                task.priority,
                task.completed,
                Utc::now().to_rfc3339(),
                id,
            ],
        )?;
        Ok(())
    }

    pub fn delete_task(&self, id: i32) -> SqliteResult<()> {
        self.conn.execute("DELETE FROM tasks WHERE id = ?", [id])?;
        Ok(())
    }

    pub fn complete_task(&self, id: i32) -> SqliteResult<()> {
        self.conn.execute(
            "UPDATE tasks SET completed = TRUE, updated_at = ? WHERE id = ?",
            params![Utc::now().to_rfc3339(), id],
        )?;
        Ok(())
    }

    pub fn task_exists(&self, id: i32) -> SqliteResult<bool> {
        let count: i32 =
            self.conn
                .query_row("SELECT COUNT(*) FROM tasks WHERE id = ?", [id], |row| {
                    row.get(0)
                })?;
        Ok(count > 0)
    }
}
