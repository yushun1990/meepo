use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct PgConfig {
    pub connection: String,
    pub max_conns: Option<u32>,
    pub min_conns: Option<u32>,
    pub conn_timeout_millis: Option<u64>,
    pub idle_timeout_millis: Option<u64>,
    pub enable_logging: Option<bool>,
}

#[allow(async_fn_in_trait)]
pub trait Postgres {
    fn config(&self) -> PgConfig;
    async fn setup(&self) -> anyhow::Result<DatabaseConnection>;
}

impl Postgres for PgConfig {
    fn config(&self) -> PgConfig {
        self.clone()
    }

    async fn setup(&self) -> anyhow::Result<DatabaseConnection> {
        let mut opt = ConnectOptions::new(self.connection.clone());

        if let Some(max_conns) = self.max_conns.clone() {
            opt.max_connections(max_conns);
        }

        if let Some(min_conns) = self.min_conns.clone() {
            opt.min_connections(min_conns);
        }

        if let Some(conn_timeout_millis) = self.conn_timeout_millis.clone() {
            opt.connect_timeout(Duration::from_millis(conn_timeout_millis));
        }

        if let Some(idle_timeout_millis) = self.idle_timeout_millis.clone() {
            opt.idle_timeout(Duration::from_millis(idle_timeout_millis));
        }

        if let Some(enable_logging) = self.enable_logging.clone() {
            opt.sqlx_logging(enable_logging);
        }

        let db = Database::connect(opt).await?;

        tracing::info!("PostgreSQL connected!");

        Ok(db)
    }
}
