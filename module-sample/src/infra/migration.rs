use stardust::database::{Database, Handle};

pub async fn migrate(
    database: stardust::infra::migration::Database,
) -> stardust::Result<()> {
    const NAME: &str = "sample_migration";
    let mut handle = database.tx_handle().await?;
    let mut migration =
        stardust::infra::migration::get_latest(&mut handle, NAME)
            .await?
            .unwrap_or_default();
    if migration.version == 0 {
        sqlx::query(
            r#" CREATE TABLE IF NOT EXISTS sample_store (
                id BIGSERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            ); "#,
        )
        .execute(handle.executor())
        .await
        .map_err(stardust::database::internal::into_error)?;

        migration.name = NAME.into();
        migration.version = 1;
        migration.description = "create sample_store table".into();
        migration =
            stardust::infra::migration::save(&mut handle, &migration).await?;
    }
    handle.commit().await?;
    if migration.version == 1 {}
    Ok(())
}
