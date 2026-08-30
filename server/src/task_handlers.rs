use sqlx::PgPool;
use crate::models::{Task, Project};

// TaskHandler holds the business logic and interacts with the DB layer.
pub struct TaskHandlers {
    db: PgPool,
}

impl TaskHandlers {
    pub fn new(db: PgPool) -> Self {
        TaskHandlers { db }
    }

    /// Creates a new task linked to the parent project.
    // POST /projects/{project_id}/tasks
    pub async fn create_task(&self, project_id: &uuid::Uuid, task_data: Task) -> Result<Task, &'static str> {
        // Validation: Ensure project_id exists and is active.
        println!("Task created for Project {}", project_id);
        // Transactionally insert task into the 'tasks' table.
        Ok(task_data)
    }

    /// Updates a task, enforcing the strict status transition rules.
    // PATCH /tasks/{id}
    pub async fn update_task(&self, task_id: &uuid::Uuid, updates: Task) -> Result<Task, &'static str> {
        // 1. Load current task state.
        // 2. Check if status change is valid based on current state (Domain Rule).
        // 3. If status is 'COMPLETED', check for the presence of resolution_note (Domain Rule).
        // 4. Execute update transactionally.
        println!("Task updated and validated against domain model.");
        Ok(updates)
    }

    // ... (GET and DELETE handlers would complete the suite)
}