use crate::app::*;
use std::sync::Arc;

pub struct AppContainer {
    pub config: stardust_common::config::Config,
    pub database: DatabaseImpl,
    pub user_service: Arc<UserServiceImpl>,
    pub apikey_service: Arc<ApikeyServiceImpl>,

    pub oauth2_client_service: Arc<OAuth2ClientServiceImpl>,
    pub oauth2_authorization_service: Arc<OAuth2AuthorizationServiceImpl>,
}

impl AppContainer {
    pub async fn build(
        configs: stardust_common::config::Config,
    ) -> stardust_common::Result<Arc<Self>> {
        let database = DatabaseImpl::new(&configs.database).await.unwrap();
        let hasher = Arc::new(HasherImpl::default());

        let password_hasher = Arc::new(PasswordHasherImpl::default());
        let user_repo = Arc::new(UserRepositoryImpl::new());
        let user_service = Arc::new(UserServiceImpl::new(
            database.clone(),
            user_repo.clone(),
            password_hasher.clone(),
        ));

        let apikey_repo = Arc::new(ApiKeyRepositoryImpl::new());
        let apikey_usage_tracker = ApiKeyUsageTrackerImpl::new(database.clone());
        let apikey_service = Arc::new(ApikeyServiceImpl::new(
            database.clone(),
            apikey_repo.clone(),
            apikey_usage_tracker.clone(),
            hasher.clone(),
        ));

        let oauth2_client_repo = Arc::new(OAuth2ClientRepositoryImpl::new());
        let oauth2_client_service = Arc::new(OAuth2ClientServiceImpl::new(
            database.clone(),
            oauth2_client_repo.clone(),
            hasher.clone(),
        ));

        let oauth2_authorization_repo = Arc::new(OAuth2AuthorizationRepositoryImpl::new());
        let oauth2_authorization_service = Arc::new(OAuth2AuthorizationServiceImpl::new(
            database.clone(),
            oauth2_authorization_repo.clone(),
            oauth2_client_service.clone(),
            hasher.clone(),
        ));
        Ok(Arc::new(Self {
            config: configs,
            database,
            user_service,
            apikey_service,
            oauth2_client_service,
            oauth2_authorization_service,
        }))
    }
}

impl module_user::Container for AppContainer {
    type UserService = UserServiceImpl;
    type ApiKeyService = ApikeyServiceImpl;

    fn user_service(&self) -> Arc<Self::UserService> {
        self.user_service.clone()
    }
    fn apikey_service(&self) -> Arc<Self::ApiKeyService> {
        self.apikey_service.clone()
    }
}

impl module_oauth2_server::Container for AppContainer {
    type OAuth2ClientService = OAuth2ClientServiceImpl;
    type OAuth2AuthorizationService = OAuth2AuthorizationServiceImpl;

    fn oauth2_client_service(&self) -> Arc<Self::OAuth2ClientService> {
        self.oauth2_client_service.clone()
    }
    fn oauth2_authorization_service(&self) -> Arc<Self::OAuth2AuthorizationService> {
        self.oauth2_authorization_service.clone()
    }
}
