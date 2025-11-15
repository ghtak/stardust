use std::sync::Arc;

use crate::{
    entity,
    infra::{apikey_repo, user_repo},
};

const NAME: &str = "user_migration";

pub async fn migrate<US>(
    database: stardust_db::Database,
    user_service: Arc<US>,
) -> stardust_common::Result<()>
where
    US: crate::service::UserService + 'static,
{
    let mut handle = database.transaction().await?;
    let mut migration = stardust_core::migration::get_latest(&mut handle, NAME).await?;

    if migration.version == 0 {
        user_repo::create_table(&mut handle).await?;
        migration =
            stardust_core::migration::save(&mut handle, NAME, 1, "create user table").await?;
    }
    handle.commit().await?;

    if migration.version == 1 {
        tracing::info!("migration 1 begin");
        user_service
            .signup(&crate::command::SignupCommand::Provisioned {
                username: "admin".into(),
                email: "admin@stardust.io".into(),
                password: "1qaz2wsx!".into(),
                account_type: entity::AccountType::Local,
                role: entity::Role::Admin,
                status: entity::Status::Active,
            })
            .await?;
        migration =
            stardust_core::migration::save(&mut database.pool(), NAME, 2, "create admin user")
                .await?;
    }

    if migration.version == 2 {
        tracing::info!("migration 2 begin");
        apikey_repo::create_table(&mut database.pool()).await?;
        migration =
            stardust_core::migration::save(&mut database.pool(), NAME, 3, "create apikey table")
                .await?;
    }

    if migration.version == 3 {}
    Ok(())
}
