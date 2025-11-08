use stardust_core::infra::migration_repo::MigrationModel;

pub async fn migrate(
    database: stardust_db::Database,
) -> stardust_common::Result<()> {
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

    if migration.version == 1 {
        tracing::info!("migration 1 done");
    }

    handle.commit().await?;
    Ok(())
}
