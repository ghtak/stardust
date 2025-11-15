use std::sync::Arc;

pub struct Container<US, AKS> {
    pub config: stardust_common::config::Config,
    pub database: stardust_db::Database,
    user_service: Arc<US>,
    apikey_service: Arc<AKS>,
}

impl<US, AKS> Container<US, AKS>
where
    US: module_user::service::UserService,
    AKS: module_user::service::ApiKeyService,
{
    pub fn new(
        config: stardust_common::config::Config,
        database: stardust_db::Database,
        user_service: Arc<US>,
        apikey_service: Arc<AKS>,
    ) -> Self {
        Self {
            config,
            database,
            user_service,
            apikey_service,
        }
    }
}

impl<US, AKS> module_user::interface::ServiceProvider for Container<US, AKS>
where
    US: module_user::service::UserService,
    AKS: module_user::service::ApiKeyService,
{
    type UserService = US;
    type ApiKeyService = AKS;

    fn user_service(&self) -> Arc<Self::UserService> {
        self.user_service.clone()
    }

    fn apikey_service(&self) -> Arc<Self::ApiKeyService> {
        self.apikey_service.clone()
    }
}
