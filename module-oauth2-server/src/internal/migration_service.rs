use std::sync::Arc;

use stardust_db::database::Handle;

pub struct MigrationServiceImpl<DB, OMR, MR> {
    database: DB,
    oauth2_migration_repo: Arc<OMR>,
    migration_repo: Arc<MR>,
}

impl<DB, OMR, MR> MigrationServiceImpl<DB, OMR, MR> {
    pub fn new(database: DB, oauth2_migration_repo: Arc<OMR>, migration_repo: Arc<MR>) -> Self {
        Self {
            database,
            oauth2_migration_repo,
            migration_repo,
        }
    }
}

#[async_trait::async_trait]
impl<DB, OMR, MR> stardust_core::service::MigrationService for MigrationServiceImpl<DB, OMR, MR>
where
    DB: stardust_db::database::Database + 'static,
    OMR: for<'h> crate::repository::MigrationRepository<Handle<'h> = DB::Handle<'h>>,
    MR: for<'h> stardust_core::repository::MigrationRepository<Handle<'h> = DB::Handle<'h>>,
{
    async fn migrate(&self) -> stardust_common::Result<()> {
        const NAME: &str = "oauth2_server_migration";
        let mut handle = self.database.tx_handle().await?;
        let mut migration =
            self.migration_repo.get_latest(&mut handle, NAME).await?.unwrap_or_default();
        if migration.version == 0 {
            self.oauth2_migration_repo.create_client_store(&mut handle).await?;
            self.oauth2_migration_repo.create_authorization_store(&mut handle).await?;
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
