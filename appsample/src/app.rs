pub mod dev_env {

    pub type HasherImpl = stardust_common::hash::DummyHasher;
    pub type PasswordHasherImpl = stardust_common::hash::DummyHasher;

    pub type DatabaseImpl = stardust_db::internal::postgres::Database;
    pub type UserRepositoryImpl = module_user::infra::user_repository::PostgresUserRepository;

    pub type UserServiceImpl = module_user::internal::UserServiceImpl<
        DatabaseImpl,
        UserRepositoryImpl,
        PasswordHasherImpl,
    >;

    pub type ApiKeyUsageTrackerImpl = module_user::internal::ImmediateUsageTracker;

    pub type ApiKeyRepositoryImpl = module_user::infra::apikey_repository::PostgresApiKeyRepository;
    pub type ApikeyServiceImpl = module_user::internal::ApikeyServiceImpl<
        DatabaseImpl,
        ApiKeyRepositoryImpl,
        ApiKeyUsageTrackerImpl,
        HasherImpl,
    >;

    pub type OAuth2ClientRepositoryImpl =
        module_oauth2_server::infra::client_repository::PostgresClientRepository;

    pub type OAuth2ClientServiceImpl = module_oauth2_server::internal::OAuth2ClientServiceImpl<
        DatabaseImpl,
        OAuth2ClientRepositoryImpl,
        HasherImpl,
    >;

    pub type OAuth2AuthorizationRepositoryImpl =
        module_oauth2_server::infra::authorization_repository::PostgresAuthorizationRepository;

    pub type OAuth2AuthorizationServiceImpl =
        module_oauth2_server::internal::OAuth2AuthorizationServiceImpl<
            DatabaseImpl,
            OAuth2AuthorizationRepositoryImpl,
            OAuth2ClientServiceImpl,
            HasherImpl,
        >;

    pub type MigrationRepositoryImpl =
        stardust_core::infra::migration_repo::PostgresMigrationRepository;

    pub type UserMigrationRepositoryImpl =
        module_user::infra::migration_repository::PostgresMigrationRepository;
    pub type UserMigrationServiceImpl = module_user::internal::MigrationServiceImpl<
        DatabaseImpl,
        UserMigrationRepositoryImpl,
        UserServiceImpl,
        MigrationRepositoryImpl,
    >;

    pub type OAuth2MigrationRepositoryImpl =
        module_oauth2_server::infra::migration_repository::PostgresMigrationRepository;
    pub type OAuth2MigrationServiceImpl = module_oauth2_server::internal::MigrationServiceImpl<
        DatabaseImpl,
        OAuth2MigrationRepositoryImpl,
        MigrationRepositoryImpl,
    >;
}

pub use dev_env::*;
