use module_user::entity::UserEntity;

pub struct CreateOAuth2ClientCommand {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
    pub grant_types: Vec<String>,
    pub auth_methods: Vec<String>,
    pub scopes: Vec<String>,
}

pub struct DeleteOAuth2ClientCommand {
    pub id: i64,
}

pub struct VerifyOAuth2ClientCommand<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
}

pub struct VerifyOAuth2AuthorizationCommand<'a> {
    pub response_type: &'a str,
    pub client_id: &'a str,
    pub redirect_uri: &'a str,
    pub scope: &'a str,
    pub state: &'a str,
}

pub struct AuthorizeOAuth2Command<'a> {
    pub principal: &'a UserEntity,
    pub verify_command: &'a VerifyOAuth2AuthorizationCommand<'a>,
}

pub struct TokenCommand<'a> {
    pub grant_type: &'a str,
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub redirect_uri: &'a str,
    pub code: Option<&'a str>,
    pub refresh_token: Option<&'a str>,
}
