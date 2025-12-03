// use std::sync::Arc;

// use stardust_db::database::Handle;

// pub struct MigrationServiceImpl<Database, OAuth2MigrationRepository, MigrationRepository> {
//     database: Database,
//     oauth2_migration_repo: Arc<OAuth2MigrationRepository>,
//     migration_repo: Arc<MigrationRepository>,
// }

// impl<Database, OAuth2MigrationRepository, MigrationRepository>
//     MigrationServiceImpl<Database, OAuth2MigrationRepository, MigrationRepository>
// {
//     pub fn new(
//         database: Database,
//         oauth2_migration_repo: Arc<OAuth2MigrationRepository>,
//         migration_repo: Arc<MigrationRepository>,
//     ) -> Self {
//         Self {
//             database,
//             oauth2_migration_repo,
//             migration_repo,
//         }
//     }
// }

// #[async_trait::async_trait]
// impl<Database, OAuth2MigrationRepository, MigrationRepository>
//     stardust_core::service::MigrationService
//     for MigrationServiceImpl<Database, OAuth2MigrationRepository, MigrationRepository>
// where
//     Database: stardust_db::database::Database + 'static,
//     OAuth2MigrationRepository:
//         for<'h> crate::repository::MigrationRepository<Handle<'h> = Database::Handle<'h>>,
//     MigrationRepository:
//         for<'h> stardust_core::repository::MigrationRepository<Handle<'h> = Database::Handle<'h>>,
// {
//     async fn migrate(&self) -> stardust::Result<()> {
//         const NAME: &str = "oauth2_server_migration";
//         let mut handle = self.database.tx_handle().await?;
//         let mut migration =
//             self.migration_repo.get_latest(&mut handle, NAME).await?.unwrap_or_default();
//         if migration.version == 0 {
//             self.oauth2_migration_repo.create_client_store(&mut handle).await?;
//             self.oauth2_migration_repo.create_authorization_store(&mut handle).await?;
//             migration.name = NAME.into();
//             migration.version = 1;
//             migration.description = "create user table".into();
//             migration = self.migration_repo.create(&mut handle, &migration).await?;
//         }
//         handle.commit().await?;
//         if migration.version == 1 {}
//         Ok(())
//     }
// }
