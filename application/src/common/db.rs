use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn setup_database(
    database: super::config::Database,
) -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(database.url.unwrap());

    if database.max_conns > 0 {
        opt.max_connections(database.max_conns);
    }

    if database.min_conns > 0 {
        opt.min_connections(database.min_conns);
    }

    if database.conn_timeout_millis > 0 {
        opt.connect_timeout(Duration::from_millis(database.conn_timeout_millis));
    }

    if database.idle_timeout_millis > 0 {
        opt.idle_timeout(Duration::from_micros(database.idle_timeout_millis));
    }

    if let Some(enable_logging) = database.enable_logging {
        opt.sqlx_logging(enable_logging);
    }

    let db = Database::connect(opt).await?;

    tracing::info!("Database connected successfully!");

    Ok(db)
}
