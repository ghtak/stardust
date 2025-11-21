use std::sync::Arc;

pub struct Container<UserContainer, OAuth2ServerContainer> {
    pub config: stardust_common::config::Config,
    pub database: stardust_db::Database,
    pub pgdb: stardust_db::internal::postgres::Database,
    pub user_container: Arc<UserContainer>,
    pub oauth2_server_container: Arc<OAuth2ServerContainer>,
}

impl<UserContainer, OAuth2ServerContainer> Container<UserContainer, OAuth2ServerContainer> {
    pub fn new(
        config: stardust_common::config::Config,
        database: stardust_db::Database,
        user_container: Arc<UserContainer>,
        oauth2_server_container: Arc<OAuth2ServerContainer>,
        pgdb: stardust_db::internal::postgres::Database
    ) -> Self {
        Self {
            config,
            database,
            user_container,
            oauth2_server_container,
            pgdb
        }
    }
}
