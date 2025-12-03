use crate::{command, entity};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateOAuth2ClientRequest {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
    pub grant_types: Vec<String>,
    pub auth_methods: Vec<String>,
    pub scopes: Vec<String>,
}

impl From<CreateOAuth2ClientRequest> for command::CreateOAuth2ClientCommand {
    fn from(value: CreateOAuth2ClientRequest) -> Self {
        command::CreateOAuth2ClientCommand {
            name: value.name,
            client_id: value.client_id,
            client_secret: value.client_secret,
            redirect_uris: value.redirect_uris,
            grant_types: value.grant_types,
            auth_methods: value.auth_methods,
            scopes: value.scopes,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OAuth2ClientDto {
    pub id: i64,
    pub name: String,
    pub client_id: String,
    pub redirect_uris: Vec<String>,
    pub grant_types: Vec<String>,
    pub auth_methods: Vec<String>,
    pub scopes: Vec<String>,
}

impl From<entity::OAuth2ClientEntity> for OAuth2ClientDto {
    fn from(value: entity::OAuth2ClientEntity) -> Self {
        OAuth2ClientDto {
            id: value.id,
            name: value.name,
            client_id: value.client_id,
            redirect_uris: value.redirect_uris,
            grant_types: value.grant_types,
            auth_methods: value.auth_methods,
            scopes: value.scopes,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OAuth2AuthorizeRequest {
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: String,
    pub state: String,
}

impl OAuth2AuthorizeRequest {
    pub fn as_verify_command(
        &self,
    ) -> command::VerifyOAuth2AuthorizationCommand<'_> {
        command::VerifyOAuth2AuthorizationCommand {
            response_type: &self.response_type,
            client_id: &self.client_id,
            redirect_uri: &self.redirect_uri,
            scope: &self.scope,
            state: &self.state,
        }
    }

    pub fn as_params(self) -> String {
        let params = [
            ("response_type", self.response_type.as_str()),
            ("client_id", self.client_id.as_str()),
            ("redirect_uri", self.redirect_uri.as_str()),
            ("scope", self.scope.as_str()),
            ("state", self.state.as_str()),
        ];
        params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OAuth2TokenRequest {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub code: Option<String>,
    pub refresh_token: Option<String>,
}

impl OAuth2TokenRequest {
    pub fn as_command(&self) -> command::TokenCommand<'_> {
        command::TokenCommand {
            grant_type: self.grant_type.as_str(),
            client_id: self.client_id.as_str(),
            client_secret: self.client_secret.as_str(),
            redirect_uri: self.redirect_uri.as_str(),
            code: self.code.as_deref(),
            refresh_token: self.refresh_token.as_deref(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OAuth2TokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub token_type: String,
}

impl From<entity::OAuth2Token> for OAuth2TokenResponse {
    fn from(value: entity::OAuth2Token) -> Self {
        Self {
            access_token: value.access_token,
            expires_in: value.expires_in,
            refresh_token: value.refresh_token,
            scope: value.scope,
            token_type: value.token_type,
        }
    }
}
