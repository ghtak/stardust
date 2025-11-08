use crate::entity::{Role, Status};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct UserModel {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub role: Role,
    pub status: Status,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]

pub struct UserAccountModel {
    pub uid: String,
    pub user_id: i64,
    pub account_type: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
