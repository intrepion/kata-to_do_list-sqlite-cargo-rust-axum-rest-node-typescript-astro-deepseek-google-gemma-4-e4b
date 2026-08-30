use axum::{routing::post, Router};
// Import the fully implemented handlers and services
mod models;
mod db_service;
mod project_handlers;
mod task_handlers;

use models::{Project, Task};
use db_service::DbService;
use project_handlers::ProjectHandlers;
use task_handlers::TaskHandlers;

#[tokio::main]
async fn main() {
    // 1. Initialize the database connection pool (using the schema we created)
    let db_pool = // Pool setup logic goes here...

    // 2. Initialize services and handlers
    let db_service = DbService(/* pool connection details */);
    let project_handlers = ProjectHandlers::new(db_service.clone());
    let task_handlers = TaskHandlers::new(db_service);

    // 3. Define routes (API contract)
    let app = Router::new()
        // Project Routes
        .route("/projects", post(project_handlers.create_project)) // POST /projects
        // Task Routes (nested under a project ID)
        .route("/projects/:id/tasks", post(task_handlers.create_project_tasks)); // POST /projects/{id}/tasks
        // ... GET, PATCH routes following suit

    // 4. Start the server
    println!("Server listening on port 8080");
}