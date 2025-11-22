pub mod dev_env {

    pub type HasherImpl = stardust_common::hash::DummyHasher;

    pub type DatabaseImpl = stardust_db::internal::postgres::Database;
    pub type UserRepositoryImpl = module_user::infra::user_repo::PostgresUserRepository;

    pub type UserServiceImpl =
        module_user::internal::UserServiceImpl<DatabaseImpl, UserRepositoryImpl, HasherImpl>;

    pub type ApiKeyUsageTrackerImpl = module_user::internal::ImmediateUsageTracker;

    pub type ApiKeyRepositoryImpl = module_user::infra::apikey_repo::PostgresApiKeyRepository;
    pub type ApikeyServiceImpl = module_user::internal::ApikeyServiceImpl<
        DatabaseImpl,
        ApiKeyRepositoryImpl,
        ApiKeyUsageTrackerImpl,
        HasherImpl,
    >;

    pub type UserContaierImpl =
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
        UserContaierImpl,
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
        crate::container::Container<DatabaseImpl, UserContaierImpl, OAuth2ServerContainerImpl>;
}

pub use dev_env::*;
