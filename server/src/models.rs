use uuid::Uuid;
use chrono::{NaiveDateTime};

/// The Project entity represents a container for related tasks.
#[derive(Debug, Clone)]
pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// The Task entity represents a single actionable unit of work.
#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    // Foreign Key linking to the parent Project.
    pub project_id: Uuid, 
    pub title: String,
    pub description: Option<String>,
    // Domain Status must adhere to the lifecycle defined in CONTEXT.md
    pub status: TaskStatus, 
    pub due_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,      // To Do: Initial state.
    InProgress,   // Active work phase.
    Completed,    // Terminal state, requires resolution_note for finality.
}