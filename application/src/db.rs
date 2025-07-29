use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn setup_database(
    database: crate::config::Database,
) -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(database.url.unwrap());

    if let Some(max_conns) = database.max_conns {
        opt.max_connections(max_conns);
    }

    if let Some(min_conns) = database.min_conns {
        opt.min_connections(min_conns);
    }

    if let Some(conn_timeout_millis) = database.conn_timeout_millis {
        opt.connect_timeout(Duration::from_millis(conn_timeout_millis));
    }

    if let Some(idle_timeoute_millis) = database.idle_timeout_millis {
        opt.idle_timeout(Duration::from_micros(idle_timeoute_millis));
    }

    if let Some(enable_logging) = database.enable_logging {
        opt.sqlx_logging(enable_logging);
    }

    let db = Database::connect(opt).await?;

    tracing::info!("Database connected successfully!");

    Ok(db)
}
