use crate::entity;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct OAuth2ClientModel {
    pub id: i64,
    pub client_id: String,
    pub client_secret_hash: String,
    pub name: String,
    pub redirect_uris: String,
    pub grant_types: String,
    pub auth_methods: String,
    pub scopes: String,
}

pub fn split_comma(s: String) -> Vec<String> {
    s.split(',').map(|s| s.to_string()).collect()
}

impl From<OAuth2ClientModel> for entity::OAuth2ClientEntity {
    fn from(row: OAuth2ClientModel) -> Self {
        Self {
            id: row.id,
            client_id: row.client_id,
            client_secret_hash: row.client_secret_hash,
            name: row.name,
            redirect_uris: split_comma(row.redirect_uris),
            grant_types: split_comma(row.grant_types),
            auth_methods: split_comma(row.auth_methods),
            scopes: split_comma(row.scopes),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct OAuth2AuthorizationModel {
    id: i64,
    oauth2_client_id: i64,
    principal_id: i64,
    grant_type: String,
    scopes: String,
    state: String,
    auth_code_value: String,
    auth_code_issued_at: chrono::DateTime<chrono::Utc>,
    auth_code_expires_at: chrono::DateTime<chrono::Utc>,
    access_token_value: String,
    access_token_issued_at: chrono::DateTime<chrono::Utc>,
    access_token_expires_at: chrono::DateTime<chrono::Utc>,
    refresh_token_hash: String,
    refresh_token_issued_at: chrono::DateTime<chrono::Utc>,
    refresh_token_expires_at: chrono::DateTime<chrono::Utc>,
}

impl From<OAuth2AuthorizationModel> for entity::OAuth2AuthorizationEntity {
    fn from(row: OAuth2AuthorizationModel) -> Self {
        Self {
            id: row.id,
            oauth2_client_id: row.oauth2_client_id,
            principal_id: row.principal_id,
            grant_type: row.grant_type,
            scope: row.scopes,
            state: row.state,
            auth_code_value: row.auth_code_value,
            auth_code_issued_at: row.auth_code_issued_at,
            auth_code_expires_at: row.auth_code_expires_at,
            access_token_value: row.access_token_value,
            access_token_issued_at: row.access_token_issued_at,
            access_token_expires_at: row.access_token_expires_at,
            refresh_token_hash: row.refresh_token_hash,
            refresh_token_issued_at: row.refresh_token_issued_at,
            refresh_token_expires_at: row.refresh_token_expires_at,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]

pub struct OAuth2AuthorizationUserModel {
    pub authorization_id: i64,
    pub authorization_oauth2_client_id: i64,
    pub authorization_principal_id: i64,
    pub authorization_grant_type: String,
    pub authorization_scopes: String,
    pub authorization_state: String,
    pub authorization_auth_code_value: String,
    pub authorization_auth_code_issued_at: chrono::DateTime<chrono::Utc>,
    pub authorization_auth_code_expires_at: chrono::DateTime<chrono::Utc>,
    pub authorization_access_token_value: String,
    pub authorization_access_token_issued_at: chrono::DateTime<chrono::Utc>,
    pub authorization_access_token_expires_at: chrono::DateTime<chrono::Utc>,
    pub authorization_refresh_token_hash: String,
    pub authorization_refresh_token_issued_at: chrono::DateTime<chrono::Utc>,
    pub authorization_refresh_token_expires_at: chrono::DateTime<chrono::Utc>,

    pub client_id: i64,
    pub client_client_id: String,
    pub client_client_secret_hash: String,
    pub client_name: String,
    pub client_redirect_uris: String,
    pub client_grant_types: String,
    pub client_auth_methods: String,
    pub client_scopes: String,

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

impl OAuth2AuthorizationUserModel {
    pub fn authorization_entity(&self) -> entity::OAuth2AuthorizationEntity {
        entity::OAuth2AuthorizationEntity {
            id: self.authorization_id,
            oauth2_client_id: self.authorization_oauth2_client_id,
            principal_id: self.authorization_principal_id,
            grant_type: self.authorization_grant_type.clone(),
            scope: self.authorization_scopes.clone(),
            state: self.authorization_state.clone(),
            auth_code_value: self.authorization_auth_code_value.clone(),
            auth_code_issued_at: self.authorization_auth_code_issued_at,
            auth_code_expires_at: self.authorization_auth_code_expires_at,
            access_token_value: self.authorization_access_token_value.clone(),
            access_token_issued_at: self.authorization_access_token_issued_at,
            access_token_expires_at: self.authorization_access_token_expires_at,
            refresh_token_hash: self.authorization_refresh_token_hash.clone(),
            refresh_token_issued_at: self.authorization_refresh_token_issued_at,
            refresh_token_expires_at: self.authorization_refresh_token_expires_at,
        }
    }

    pub fn client_entity(&self) -> entity::OAuth2ClientEntity {
        entity::OAuth2ClientEntity {
            id: self.client_id,
            client_id: self.client_client_id.clone(),
            client_secret_hash: self.client_client_secret_hash.clone(),
            name: self.client_name.clone(),
            redirect_uris: split_comma(self.client_redirect_uris.clone()),
            grant_types: split_comma(self.client_grant_types.clone()),
            auth_methods: split_comma(self.client_auth_methods.clone()),
            scopes: split_comma(self.client_scopes.clone()),
        }
    }


    pub fn user_entity(&self) -> module_user::entity::UserEntity {
        module_user::entity::UserEntity {
            id: self.user_id,
            username: self.user_username.clone(),
            email: self.user_email.clone(),
            role: self.user_role.parse().unwrap_or(module_user::entity::Role::User),
            status: self.user_status.parse().unwrap_or(module_user::entity::Status::Inactive),
            created_at: self.user_created_at,
            updated_at: self.user_updated_at,
        }
    }

    pub fn account_entity(&self) -> module_user::entity::UserAccountEntity {
        module_user::entity::UserAccountEntity {
            uid: self.account_uid.clone(),
            user_id: self.account_user_id,
            account_type: self
                .account_account_type
                .parse()
                .unwrap_or(module_user::entity::AccountType::Local),
            password_hash: self.account_password_hash.clone(),
            created_at: self.account_created_at,
            updated_at: self.account_updated_at,
        }
    }
}
