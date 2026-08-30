use crate::models::{Task, TaskStatus, Uuid};

#[tokio::test]
async fn test_successful_task_completion() {
    // 1. Setup: Initial State - Task is created and in progress.
    let initial_task = Task { 
        id: Uuid::new_v4(), // Unique ID for this specific task instance.
        project_id: Uuid::new_v4(), 
        title: "Implement Task Lifecycle".to_string(),
        description: None, 
        status: TaskStatus::InProgress, // Starting from the active phase.
        due_date: None, 
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    // 2. Action: The transition attempt to complete the task.
    let update_payload = Task { 
        // The desired end state.
        status: TaskStatus::Completed, 
        title: "Implement Task Lifecycle".to_string(), // Retaining context
        description: None, 
        // The critical business rule enforcement field.
        resolution_note: Some("Task was successfully completed and reviewed.".to_string()), 
        // Other fields are updated by the database service.
    };

    let handler = TaskHandlers::new(/* Mocked DB */);
    // Attempt the transition via the service layer.
    let result = handler.update_task(/* project_id, task_id */, update_payload).await;

    // Assertion: The domain rule passed.
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_completion_without_note_fails() {
    // 1. Setup: Trying to mark a task complete without fulfilling the auditability requirement.
    let update_payload = Task { 
        status: TaskStatus::Completed, // The intended transition.
        // Omission of the required field is the flaw being tested.
        resolution_note: None, 
    };

    let handler = TaskHandlers::new(/* Mocked DB */);
    // Execute the attempt.
    let result = handler.update_task(/* task_id */, update_payload).await;

    // Assertion: The business rule successfully blocked the invalid transition.
    assert!(result.is_err());
}