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
            status: model.status.parse().unwrap_or(crate::entity::Status::Inactive),
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
            account_type: model.account_type.parse().unwrap_or(crate::entity::AccountType::Local),
            password_hash: model.password_hash,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct UserAggregateModel {
    pub user_id: i64,
    pub user_username: String,
    pub user_email: String,
    pub user_role: String,
    pub user_status: String,
    pub user_created_at: chrono::DateTime<chrono::Utc>,
    pub user_updated_at: chrono::DateTime<chrono::Utc>,

    pub account_uid: String,
    pub account_user_id: i64,
    pub account_account_type: String,
    pub account_password_hash: String,
    pub account_created_at: chrono::DateTime<chrono::Utc>,
    pub account_updated_at: chrono::DateTime<chrono::Utc>,
}

impl UserAggregateModel {
    pub fn user_entity(&self) -> crate::entity::UserEntity {
        crate::entity::UserEntity {
            id: self.user_id,
            username: self.user_username.clone(),
            email: self.user_email.clone(),
            role: self.user_role.parse().unwrap_or(crate::entity::Role::User),
            status: self.user_status.parse().unwrap_or(crate::entity::Status::Inactive),
            created_at: self.user_created_at,
            updated_at: self.user_updated_at,
        }
    }

    pub fn account_entity(&self) -> crate::entity::UserAccountEntity {
        crate::entity::UserAccountEntity {
            uid: self.account_uid.clone(),
            user_id: self.account_user_id,
            account_type: self
                .account_account_type
                .parse()
                .unwrap_or(crate::entity::AccountType::Local),
            password_hash: self.account_password_hash.clone(),
            created_at: self.account_created_at,
            updated_at: self.account_updated_at,
        }
    }
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
    pub apikey_user_id: i64,
    pub apikey_key_hash: String,
    pub apikey_prefix: String,
    pub apikey_description: String,
    pub apikey_created_at: chrono::DateTime<chrono::Utc>,
    pub apikey_updated_at: chrono::DateTime<chrono::Utc>,
    pub apikey_last_used_at: chrono::DateTime<chrono::Utc>,
    pub apikey_deactivated_at: Option<chrono::DateTime<chrono::Utc>>,

    pub user_id: i64,
    pub user_username: String,
    pub user_email: String,
    pub user_role: String,
    pub user_status: String,
    pub user_created_at: chrono::DateTime<chrono::Utc>,
    pub user_updated_at: chrono::DateTime<chrono::Utc>,

    pub account_uid: String,
    pub account_user_id: i64,
    pub account_account_type: String,
    pub account_password_hash: String,
    pub account_created_at: chrono::DateTime<chrono::Utc>,
    pub account_updated_at: chrono::DateTime<chrono::Utc>,
}

impl ApiKeyUserModel {
    pub fn apikey_entity(&self) -> crate::entity::ApiKeyEntity {
        crate::entity::ApiKeyEntity {
            id: self.apikey_id,
            user_id: self.apikey_user_id,
            key_hash: self.apikey_key_hash.clone(),
            prefix: self.apikey_prefix.clone(),
            description: self.apikey_description.clone(),
            created_at: self.apikey_created_at,
            updated_at: self.apikey_updated_at,
            last_used_at: self.apikey_last_used_at,
            deactivated_at: self.apikey_deactivated_at,
        }
    }

    pub fn user_entity(&self) -> crate::entity::UserEntity {
        crate::entity::UserEntity {
            id: self.user_id,
            username: self.user_username.clone(),
            email: self.user_email.clone(),
            role: self.user_role.parse().unwrap_or(crate::entity::Role::User),
            status: self.user_status.parse().unwrap_or(crate::entity::Status::Inactive),
            created_at: self.user_created_at,
            updated_at: self.user_updated_at,
        }
    }

    pub fn account_entity(&self) -> crate::entity::UserAccountEntity {
        crate::entity::UserAccountEntity {
            uid: self.account_uid.clone(),
            user_id: self.account_user_id,
            account_type: self
                .account_account_type
                .parse()
                .unwrap_or(crate::entity::AccountType::Local),
            password_hash: self.account_password_hash.clone(),
            created_at: self.account_created_at,
            updated_at: self.account_updated_at,
        }
    }
}
