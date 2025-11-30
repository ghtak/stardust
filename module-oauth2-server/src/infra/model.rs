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
    config: serde_json::Value,
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
            config: row.config,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]

pub struct OAuth2AuthorizationUserModel {
    #[sqlx(json, rename = "client_json")]
    pub client: OAuth2ClientModel,

    #[sqlx(json, rename = "user_json")]
    pub user: module_user::infra::model::UserModel,

    #[sqlx(json, rename = "authorization_json")]
    pub authorization: OAuth2AuthorizationModel,
}
