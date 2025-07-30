use application::common::error::AppError;
use sea_orm::DatabaseConnection;

#[allow(unused)]
pub struct Hello {
    db: DatabaseConnection,
}

impl Hello {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn hello(&self) -> Result<String, AppError> {
        Ok("Hello Edge!".to_string())
    }
}
