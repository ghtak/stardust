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

impl OAuth2AuthorizeRequest{
    pub fn as_verify_command(&self) -> command::OAuth2VerifyCommand<'_> {
        command::OAuth2VerifyCommand {
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

