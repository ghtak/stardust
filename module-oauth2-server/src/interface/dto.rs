use crate::command;


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
