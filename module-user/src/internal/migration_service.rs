use std::sync::Arc;

use stardust_db::database::Handle;

use crate::entity;

pub struct MigrationServiceImpl<DB, US, UMR, MR> {
    database: DB,
    user_service: Arc<US>,
    user_migration_repo: Arc<UMR>,
    migration_repo: Arc<MR>,
}

impl<DB, US, UMR, MR> MigrationServiceImpl<DB, US, UMR, MR> {
    pub fn new(
        database: DB,
        user_service: Arc<US>,
        user_migration_repo: Arc<UMR>,
        migration_repo: Arc<MR>,
    ) -> Self {
        Self {
            database,
            user_service,
            user_migration_repo,
            migration_repo,
        }
    }
}

#[async_trait::async_trait]
impl<DB, US, UMR, MR> stardust_core::service::MigrationService for MigrationServiceImpl<DB, US, UMR, MR>
where
    DB: stardust_db::database::Database + 'static,
    US: crate::service::UserService + 'static,
    UMR: for<'h> crate::repository::MigrationRepository<Handle<'h> = DB::Handle<'h>>,
    MR: for<'h> stardust_core::repository::MigrationRepository<Handle<'h> = DB::Handle<'h>>,
{
    async fn migrate(&self) -> stardust_common::Result<()> {
        const NAME: &str = "user_migration";
        let mut handle = self.database.tx_handle().await?;
        let mut migration =
            self.migration_repo.get_latest(&mut handle, NAME).await?.unwrap_or_default();
        if migration.version == 0 {
            self.user_migration_repo.create_user_store(&mut handle).await?;
            migration.name = NAME.into();
            migration.version = 1;
            migration.description = "create user table".into();
            migration = self.migration_repo.create(&mut handle, &migration).await?;
        }
        handle.commit().await?;

        if migration.version == 1 {
            tracing::info!("migration 1 begin");
            self.user_service
                .signup(&crate::command::SignupCommand::Provisioned {
                    username: "admin".into(),
                    email: "admin@stardust.io".into(),
                    password: "1qaz2wsx!".into(),
                    account_type: entity::AccountType::Local,
                    role: entity::Role::Admin,
                    status: entity::Status::Active,
                })
                .await?;

            migration.version = 2;
            migration.description = "create admin user".into();
            migration = self.migration_repo.create(&mut self.database.handle(), &migration).await?;
        }

        if migration.version == 2 {
            self.user_migration_repo.create_apikey_store(&mut self.database.handle()).await?;
            migration.version = 3;
            migration.description = "create apikey table".into();
            migration = self.migration_repo.create(&mut self.database.handle(), &migration).await?;
        }
        if migration.version == 3 {}
        Ok(())
    }
}
