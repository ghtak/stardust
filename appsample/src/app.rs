pub mod dev_env {

    pub type HasherImpl = stardust_common::hash::DummyHasher;
    pub type PasswordHasherImpl = stardust_common::hash::DummyHasher;

    pub type DatabaseImpl = stardust_db::internal::postgres::Database;
    pub type UserRepositoryImpl = module_user::infra::user_repo::PostgresUserRepository;

    pub type UserServiceImpl = module_user::internal::UserServiceImpl<
        DatabaseImpl,
        UserRepositoryImpl,
        PasswordHasherImpl,
    >;

    pub type ApiKeyUsageTrackerImpl = module_user::internal::ImmediateUsageTracker;

    pub type ApiKeyRepositoryImpl = module_user::infra::apikey_repo::PostgresApiKeyRepository;
    pub type ApikeyServiceImpl = module_user::internal::ApikeyServiceImpl<
        DatabaseImpl,
        ApiKeyRepositoryImpl,
        ApiKeyUsageTrackerImpl,
        HasherImpl,
    >;

    pub type UserContainerImpl =
        module_user::interface::container::Container<UserServiceImpl, ApikeyServiceImpl>;

    pub type OAuth2ClientRepositoryImpl =
        module_oauth2_server::infra::client_repo::PostgresClientRepository;

    pub type OAuth2ClientServiceImpl = module_oauth2_server::internal::OAuth2ClientServiceImpl<
        DatabaseImpl,
        OAuth2ClientRepositoryImpl,
        HasherImpl,
    >;

    pub type OAuth2AuthorizationRepositoryImpl =
        module_oauth2_server::infra::authorization_repo::PostgresAuthorizationRepository;

    pub type OAuth2AuthorizationServiceImpl =
        module_oauth2_server::internal::OAuth2AuthorizationServiceImpl<
            DatabaseImpl,
            OAuth2AuthorizationRepositoryImpl,
            OAuth2ClientServiceImpl,
            HasherImpl,
        >;

    pub type OAuth2ServerContainerImpl = module_oauth2_server::interface::container::Container<
        UserContainerImpl,
        OAuth2ClientServiceImpl,
        OAuth2AuthorizationServiceImpl,
    >;

    pub type MigrationRepositoryImpl =
        stardust_core::infra::migration_repo::PostgresMigrationRepository;

    pub type UserMigrationRepositoryImpl =
        module_user::infra::migration_repo::PostgresMigrationRepository;
    pub type UserMigrationServiceImpl = module_user::internal::MigrationServiceImpl<
        DatabaseImpl,
        UserMigrationRepositoryImpl,
        UserServiceImpl,
        MigrationRepositoryImpl,
    >;

    pub type OAuth2MigrationRepositoryImpl =
        module_oauth2_server::infra::migration_repo::PostgresMigrationRepository;
    pub type OAuth2MigrationServiceImpl = module_oauth2_server::internal::MigrationServiceImpl<
        DatabaseImpl,
        OAuth2MigrationRepositoryImpl,
        MigrationRepositoryImpl,
    >;

    pub type Container =
        crate::container::Container<DatabaseImpl, UserContainerImpl, OAuth2ServerContainerImpl>;
}

use std::sync::Arc;

pub use dev_env::*;

impl Container {
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

        let user_container = Arc::new(UserContainerImpl::new(
            user_service.clone(),
            apikey_service.clone(),
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

        let oauth2_server_container = Arc::new(OAuth2ServerContainerImpl::new(
            user_container.clone(),
            oauth2_client_service.clone(),
            oauth2_authorization_service.clone(),
        ));

        let ct = Container::new(configs, database, user_container, oauth2_server_container);
        Ok(Arc::new(ct))
    }
}
