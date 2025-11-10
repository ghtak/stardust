#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct UserModel {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub role: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserModel> for crate::entity::UserEntity {
    fn from(model: UserModel) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            role: model.role.parse().unwrap_or(crate::entity::Role::User),
            status: model
                .status
                .parse()
                .unwrap_or(crate::entity::Status::Inactive),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
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

impl From<UserAccountModel> for crate::entity::UserAccountEntity {
    fn from(model: UserAccountModel) -> Self {
        Self {
            uid: model.uid,
            user_id: model.user_id,
            account_type: model
                .account_type
                .parse()
                .unwrap_or(crate::entity::AccountType::Local),
            password_hash: model.password_hash,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
