use sea_orm::sqlx::PgPool;

use crate::config::Config;

pub async fn setup_database(config: &Config) -> anyhow::Result<PgPool> {
    todo!()
}
