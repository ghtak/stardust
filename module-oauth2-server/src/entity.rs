pub struct OAuth2ClientEntity {
    pub id: i64,
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
    pub grant_types: Vec<String>,
    pub auth_methods: Vec<String>,
    pub scopes: Vec<String>,
}
