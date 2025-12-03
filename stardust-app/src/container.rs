mod env_dev {

    pub type Hasher = stardust::hash::NoOpHasher;
    pub type PasswordHasher = stardust::hash::NoOpHasher;
    pub type Database = stardust::database::internal::postgres::Database;

    pub type UserRepository =
        module_user::infra::user_repository::PostgresUserRepository;
    pub type UserService = module_user::internal::UserServiceImpl<
        Database,
        UserRepository,
        PasswordHasher,
    >;

    pub type ApiKeyUsageTracker = module_user::internal::ImmediateUsageTracker;
    pub type ApiKeyRepository =
        module_user::infra::apikey_repository::PostgresApiKeyRepository;
    pub type ApiKeyService = module_user::internal::ApiKeyServiceImpl<
        Database,
        ApiKeyRepository,
        ApiKeyUsageTracker,
        Hasher,
    >;

    pub type OAuth2ClientRepository =
        module_oauth2_server::infra::client_repository::PostgresClientRepository;
    pub type OAuth2ClientService =
        module_oauth2_server::internal::OAuth2ClientServiceImpl<
            Database,
            OAuth2ClientRepository,
            Hasher,
        >;

    pub type OAuth2AuthorizationRepository =
        module_oauth2_server::infra::authorization_repository::PostgresAuthorizationRepository;
    pub type OAuth2AuthorizationService =
        module_oauth2_server::internal::OAuth2AuthorizationServiceImpl<
            Database,
            OAuth2AuthorizationRepository,
            OAuth2ClientService,
            Hasher,
        >;
}

use env_dev::*;
use std::sync::Arc;

pub struct UserModule {
    pub user_service: Arc<UserService>,
    pub apikey_service: Arc<ApiKeyService>,
}

impl UserModule {
    pub fn new(
        database: Database,
        password_hasher: Arc<PasswordHasher>,
        hasher: Arc<Hasher>,
    ) -> Self {
        let user_repo = Arc::new(UserRepository::new());
        let user_service = Arc::new(UserService::new(
            database.clone(),
            user_repo.clone(),
            password_hasher.clone(),
        ));

        let apikey_repo = Arc::new(ApiKeyRepository::new());
        let apikey_usage_tracker = ApiKeyUsageTracker::new(database.clone());
        let apikey_service = Arc::new(ApiKeyService::new(
            database.clone(),
            apikey_repo.clone(),
            apikey_usage_tracker.clone(),
            hasher.clone(),
        ));
        Self {
            user_service,
            apikey_service,
        }
    }
}

pub struct OAuth2ServerModule {
    pub oauth2_client_service: Arc<OAuth2ClientService>,
    pub oauth2_authorization_service: Arc<OAuth2AuthorizationService>,
}

impl OAuth2ServerModule {
    pub fn new(database: Database, hasher: Arc<Hasher>) -> Self {
        let oauth2_client_repo = Arc::new(OAuth2ClientRepository::new());
        let oauth2_client_service = Arc::new(OAuth2ClientService::new(
            database.clone(),
            oauth2_client_repo.clone(),
            hasher.clone(),
        ));

        let oauth2_authorization_repo =
            Arc::new(OAuth2AuthorizationRepository::new());
        let oauth2_authorization_service =
            Arc::new(OAuth2AuthorizationService::new(
                database.clone(),
                oauth2_authorization_repo.clone(),
                oauth2_client_service.clone(),
                hasher.clone(),
            ));
        Self {
            oauth2_client_service,
            oauth2_authorization_service,
        }
    }
}

pub struct Container {
    pub database: Database,
    pub user_module: UserModule,
    pub oauth2_server_module: OAuth2ServerModule,
}

impl Container {
    pub async fn build(
        configs: stardust::config::Config,
    ) -> stardust::Result<Arc<Self>> {
        let database = Database::new(&configs.database).await.unwrap();
        let hasher = Arc::new(Hasher::default());
        let password_hasher = Arc::new(PasswordHasher::default());

        let user_module = UserModule::new(
            database.clone(),
            password_hasher.clone(),
            hasher.clone(),
        );

        let oauth2_server_module =
            OAuth2ServerModule::new(database.clone(), hasher.clone());

        Ok(Arc::new(Self {
            database,
            user_module,
            oauth2_server_module,
        }))
    }
}

impl module_user::Container for Container {
    type UserService = UserService;
    type ApiKeyService = ApiKeyService;

    fn user_service(&self) -> Arc<Self::UserService> {
        self.user_module.user_service.clone()
    }
    fn apikey_service(&self) -> Arc<Self::ApiKeyService> {
        self.user_module.apikey_service.clone()
    }
}

impl module_oauth2_server::Container for Container {
    type OAuth2ClientService = OAuth2ClientService;
    type OAuth2AuthorizationService = OAuth2AuthorizationService;

    fn oauth2_client_service(&self) -> Arc<Self::OAuth2ClientService> {
        self.oauth2_server_module.oauth2_client_service.clone()
    }
    fn oauth2_authorization_service(
        &self,
    ) -> Arc<Self::OAuth2AuthorizationService> {
        self.oauth2_server_module.oauth2_authorization_service.clone()
    }
}
