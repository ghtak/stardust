use module_user::entity::UserAggregate;

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

pub struct OAuth2VerifyCommand<'a> {
    pub response_type: &'a str,
    pub client_id: &'a str,
    pub redirect_uri: &'a str,
    pub scope: &'a str,
    pub state: &'a str,
}

pub struct OAuth2AuthorizeCommand<'a> {
    pub principal: &'a UserAggregate,
    pub verify_command: &'a OAuth2VerifyCommand<'a>,
}