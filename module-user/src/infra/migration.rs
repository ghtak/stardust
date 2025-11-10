use std::sync::Arc;

use stardust_core::infra::migration_repo::MigrationModel;

use crate::entity;

pub async fn migrate<US>(
    database: stardust_db::Database,
    user_service: Arc<US>,
) -> stardust_common::Result<()>
where
    US: crate::service::UserService + 'static,
{
    let mut handle = database.transaction().await?;
    const NAME: &str = "user_migration";
    let mut migration =
        stardust_core::infra::migration_repo::get_latest(&mut handle, NAME)
            .await?
            .unwrap_or(MigrationModel {
                name: NAME.into(),
                version: 0,
                description: "".into(),
                updated_at: chrono::Utc::now(),
            });

    if migration.version == 0 {
        crate::infra::user_repo::create_table(&mut handle).await?;
        migration = stardust_core::infra::migration_repo::create(
            &mut handle,
            &MigrationModel {
                name: NAME.into(),
                version: 1,
                description: "create user table".into(),
                updated_at: chrono::Utc::now(),
            },
        )
        .await?;
    }
    handle.commit().await?;

    if migration.version == 1 {
        tracing::info!("migration 1 begin");
        user_service
            .signup(&crate::command::SignupCommand::Provisioned {
                username: "admin".into(),
                email: "admin@stardust.io".into(),
                password: "1qaz2wsx!".into(),
                account_type: entity::AccountType::Provisioned,
                role: entity::Role::Admin,
                status: entity::Status::Active,
            })
            .await?;
        migration = stardust_core::infra::migration_repo::create(
            &mut database.pool(),
            &MigrationModel {
                name: NAME.into(),
                version: 2,
                description: "create default admin user".into(),
                updated_at: chrono::Utc::now(),
            },
        )
        .await?;
    }

    if migration.version == 2 {}

    Ok(())
}
