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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct UserAggregateModel {
    #[sqlx(json, rename = "user_json")]
    pub user: UserModel,

    #[sqlx(json, rename = "account_json")]
    pub account: UserAccountModel,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct ApiKeyModel {
    pub id: i64,
    pub user_id: i64,
    pub key_hash: String,
    pub prefix: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub last_used_at: chrono::DateTime<chrono::Utc>,
    pub deactivated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<ApiKeyModel> for crate::entity::ApiKeyEntity {
    fn from(model: ApiKeyModel) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            key_hash: model.key_hash,
            prefix: model.prefix,
            description: model.description,
            created_at: model.created_at,
            updated_at: model.updated_at,
            last_used_at: model.last_used_at,
            deactivated_at: model.deactivated_at,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct ApiKeyUserModel {
    pub apikey_id: i64,

    #[sqlx(flatten)]
    pub user: UserModel,
}
