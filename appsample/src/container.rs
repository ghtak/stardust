use std::sync::Arc;

pub struct Container<UserContainer> {
    pub config: stardust_common::config::Config,
    pub database: stardust_db::Database,
    pub user_container: Arc<UserContainer>,
}

impl<UserContainer> Container<UserContainer>
{
    pub fn new(
        config: stardust_common::config::Config,
        database: stardust_db::Database,
        user_container: Arc<UserContainer>,
    ) -> Self {
        Self {
            config,
            database,
            user_container,
        }
    }
}
