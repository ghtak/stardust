use std::sync::Arc;

pub struct Container<US> {
    pub config: stardust_common::config::Config,
    pub database: stardust_db::Database,
    user_service: Arc<US>,
}

impl<US> Container<US>
where
    US: module_user::service::UserService,
{
    pub fn new(
        config: stardust_common::config::Config,
        database: stardust_db::Database,
        user_service: Arc<US>,
    ) -> Self {
        Self {
            config,
            database,
            user_service,
        }
    }
}

impl<US> module_user::interface::UserServiceProvider for Container<US>
where
    US: module_user::service::UserService,
{
    type UserService = US;

    fn user_service(&self) -> Arc<Self::UserService> {
        self.user_service.clone()
    }
}
