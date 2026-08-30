use sqlx::SqlitePool;
use crate::models::{Project, Task};

pub struct DbService {
    pool: SqlitePool,
}

impl DbService {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(DbService { pool })
    }

    // Placeholder for future methods: find_project, save_task, etc.
}