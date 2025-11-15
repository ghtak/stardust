use crate::entity;

#[derive(sqlx::FromRow)]
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