#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OAuth2ClientEntity {
    pub id: i64,
    pub name: String,
    pub client_id: String,
    pub client_secret_hash: String,
    pub redirect_uris: Vec<String>,
    pub grant_types: Vec<String>,
    pub auth_methods: Vec<String>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OAuth2AuthorizationEntity {
    pub id: i64,
    pub oauth2_client_id: i64,
    pub principal_id: i64,
    pub grant_type: String,
    pub scope: String,
    pub state: String,
    pub auth_code_value: String,
    pub auth_code_issued_at: chrono::DateTime<chrono::Utc>,
    pub auth_code_expires_at: chrono::DateTime<chrono::Utc>,
    pub access_token_value: String,
    pub access_token_issued_at: chrono::DateTime<chrono::Utc>,
    pub access_token_expires_at: chrono::DateTime<chrono::Utc>,
    pub refresh_token_hash: String,
    pub refresh_token_issued_at: chrono::DateTime<chrono::Utc>,
    pub refresh_token_expires_at: chrono::DateTime<chrono::Utc>,
    pub config: serde_json::Value,
}

impl OAuth2AuthorizationEntity {
    pub fn new(
        client_id: i64,
        principal_id: i64,
        scope: String,
        state: String,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: 0,
            oauth2_client_id: client_id,
            principal_id: principal_id,
            scope: scope,
            state: state,
            grant_type: "authorization_code".to_owned(),
            auth_code_value: stardust::utils::generate_uid(),
            auth_code_issued_at: now,
            auth_code_expires_at: now + chrono::Duration::minutes(10),
            access_token_value: "".to_owned(),
            access_token_issued_at: now,
            access_token_expires_at: now,
            refresh_token_hash: "".to_owned(),
            refresh_token_issued_at: now,
            refresh_token_expires_at: now,
            config: serde_json::json!({}),
        }
    }

    pub fn issue_token(
        &mut self,
        access_token: String,
        refresh_token_hash: String,
    ) {
        let now = chrono::Utc::now();
        self.auth_code_expires_at = now;
        self.access_token_value = access_token;
        self.access_token_issued_at = now;
        self.access_token_expires_at = now + chrono::Duration::days(1);
        self.refresh_token_hash = refresh_token_hash;
        self.refresh_token_issued_at = now;
        self.refresh_token_expires_at = now + chrono::Duration::days(30);
    }

    pub fn refresh_token(&mut self, access_token: String) {
        let now = chrono::Utc::now();
        self.access_token_value = access_token;
        self.access_token_issued_at = now;
        self.access_token_expires_at = now + chrono::Duration::days(1);
        self.refresh_token_expires_at = now + chrono::Duration::days(30);
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OAuth2Token {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub token_type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OAuthUserAggregate {
    pub user: module_user::entity::UserEntity,
    pub client: OAuth2ClientEntity,
    pub authorization: OAuth2AuthorizationEntity,
}
