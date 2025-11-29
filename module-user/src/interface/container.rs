use std::sync::Arc;

pub trait ServiceContainer: Sync + Send {
    type UserService: crate::service::UserService;
    type ApiKeyService: crate::service::ApiKeyService;

    fn user_service(&self) -> Arc<Self::UserService>;
    fn apikey_service(&self) -> Arc<Self::ApiKeyService>;
}


pub struct Container<US, AKS> {
    pub user_service: Arc<US>,
    pub apikey_service: Arc<AKS>,
}

impl<US, AKS> Container<US, AKS>
where
    US: crate::service::UserService,
    AKS: crate::service::ApiKeyService,
{
    pub fn new(user_service: Arc<US>, apikey_service: Arc<AKS>) -> Self {
        Self {
            user_service,
            apikey_service,
        }
    }
}

impl<US, AKS> ServiceContainer for Container<US, AKS>
where
    US: crate::service::UserService,
    AKS: crate::service::ApiKeyService,
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
