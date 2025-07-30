use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> DatabaseConnection {
        self.db.clone()
    }
}
