use std::sync::Arc;

pub struct Container<Database, UserContainer, OAuth2ServerContainer> {
    pub config: stardust_common::config::Config,
    pub database: Database,
    pub user_container: Arc<UserContainer>,
    pub oauth2_server_container: Arc<OAuth2ServerContainer>,
}

impl<Database, UserContainer, OAuth2ServerContainer>
    Container<Database, UserContainer, OAuth2ServerContainer>
{
    pub fn new(
        config: stardust_common::config::Config,
        database: Database,
        user_container: Arc<UserContainer>,
        oauth2_server_container: Arc<OAuth2ServerContainer>,
    ) -> Self {
        Self {
            config,
            database,
            user_container,
            oauth2_server_container,
        }
    }
}
