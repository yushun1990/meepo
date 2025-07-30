use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct UserAuth {
    pub user_id: String,
    pub password_hash: String,
}
