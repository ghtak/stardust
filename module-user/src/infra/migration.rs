use std::sync::Arc;

use stardust::database::Database;

use crate::service::UserService;

pub async fn migrate<C>(
    database: stardust::infra::migration::Database,
    container: Arc<C>,
) -> stardust::Result<()>
where
    C: crate::Container + 'static,
{
    const NAME: &str = "user_migration";
    let mut handle = database.handle();
    let mut migration =
        stardust::infra::migration::get_latest(&mut handle, NAME)
            .await?
            .unwrap_or_default();
    if migration.version == 0 {
        sqlx::query(
            r#"create table if not exists stardust_user (
                id BIGSERIAL PRIMARY KEY,
                username varchar(255) not null,
                email varchar(255) not null,
                role varchar(255) not null,
                status varchar(255) not null,
                created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            );"#,
        )
        .execute(handle.executor())
        .await
        .map_err(stardust::database::internal::into_error)?;

        sqlx::query(
            r#"create table if not exists stardust_user_account (
                uid varchar(255) primary key,
                user_id BIGINT not null,
                account_type varchar(255) not null,
                password_hash varchar(255) not null,
                created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            );"#,
        )
        .execute(handle.executor())
        .await
        .map_err(stardust::database::internal::into_error)?;

        sqlx::query(
            r#"create table if not exists stardust_apikey (
                id BIGSERIAL PRIMARY KEY,
                user_id BIGINT not null,
                key_hash varchar(255) not null,
                prefix varchar(255) not null,
                description varchar(255) not null,
                created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                last_used_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                deactivated_at TIMESTAMPTZ
            );"#,
        )
        .execute(handle.executor())
        .await
        .map_err(stardust::database::internal::into_error)?;

        migration.name = NAME.into();
        migration.version = 1;
        migration.description = "create user/apikey table".into();
        migration =
            stardust::infra::migration::save(&mut handle, &migration).await?;
    }

    if migration.version == 1 {
        tracing::info!("migration 1 begin");
        container
            .user_service()
            .signup(&crate::command::SignupCommand::Provisioned {
                username: "admin".into(),
                email: "admin@stardust.io".into(),
                password: "1qaz2wsx!".into(),
                account_type: crate::entity::AccountType::Local,
                role: crate::entity::Role::Admin,
                status: crate::entity::Status::Active,
            })
            .await?;
        migration.name = NAME.into();
        migration.version = 2;
        migration.description = "add admin user".into();
        migration = stardust::infra::migration::save(
            &mut database.handle(),
            &migration,
        )
        .await?;
    }

    if migration.version == 2 {
        // No more migrations yet
    }

    Ok(())
}
