use std::sync::Arc;

use stardust_db::database::Handle;

pub struct MigrationServiceImpl<Database, SampleMigrationRepo, MigrationRepo> {
    database: Database,
    sample_migration_repo: Arc<SampleMigrationRepo>,
    migration_repo: Arc<MigrationRepo>,
}

impl<Database, SampleMigrationRepo, MigrationRepo>
    MigrationServiceImpl<Database, SampleMigrationRepo, MigrationRepo>
{
    pub fn new(
        database: Database,
        sample_migration_repo: Arc<SampleMigrationRepo>,
        migration_repo: Arc<MigrationRepo>,
    ) -> Self {
        Self {
            database,
            sample_migration_repo,
            migration_repo,
        }
    }
}

#[async_trait::async_trait]
impl<Database, SampleMigrationRepo, MigrationRepo> stardust_core::service::MigrationService
    for MigrationServiceImpl<Database, SampleMigrationRepo, MigrationRepo>
where
    Database: stardust_db::database::Database + 'static,
    SampleMigrationRepo:
        for<'h> crate::repository::MigrationRepository<Handle<'h> = Database::Handle<'h>>,
    MigrationRepo:
        for<'h> stardust_core::repository::MigrationRepository<Handle<'h> = Database::Handle<'h>>,
{
    async fn migrate(&self) -> stardust_common::Result<()> {
        const NAME: &str = "oauth2_server_migration";
        let mut handle = self.database.tx_handle().await?;
        let mut migration =
            self.migration_repo.get_latest(&mut handle, NAME).await?.unwrap_or_default();
        if migration.version == 0 {
            self.sample_migration_repo.create_sample_store(&mut handle).await?;

            migration.name = NAME.into();
            migration.version = 1;
            migration.description = "create user table".into();
            migration = self.migration_repo.create(&mut handle, &migration).await?;
        }
        handle.commit().await?;
        if migration.version == 1 {}
        Ok(())
    }
}
