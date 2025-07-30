use std::sync::Arc;

use sea_orm::{DatabaseConnection, prelude::async_trait};

use crate::common::error::AppError;

#[async_trait::async_trait]
pub trait HelloService: Send + Sync {
    fn create_service(db: DatabaseConnection) -> Arc<dyn HelloService>
    where
        Self: Sized;

    fn hello(&self) -> Result<String, AppError>;
}
