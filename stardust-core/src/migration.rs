use stardust_db::{database::Database, internal::postgres};

use crate::infra;

pub async fn migrate(database: postgres::Database) -> stardust_common::Result<()> {
    let mut handle = database.handle();
    infra::migration_repo::create_table(&mut handle).await?;
    Ok(())
}

pub async fn get_latest(
    handle: &mut postgres::Handle<'_>,
    name: &str,
) -> stardust_common::Result<infra::migration_repo::MigrationModel> {
    let result = infra::migration_repo::get_latest(handle, name).await?;
    Ok(result.unwrap_or(infra::migration_repo::MigrationModel {
        name: name.into(),
        version: 0,
        description: "".into(),
        updated_at: chrono::Utc::now(),
    }))
}

pub async fn save(
    handle: &mut postgres::Handle<'_>,
    name: &str,
    version: i32,
    description: &str,
) -> stardust_common::Result<infra::migration_repo::MigrationModel> {
    infra::migration_repo::create(
        handle,
        &infra::migration_repo::MigrationModel {
            name: name.into(),
            version,
            description: description.into(),
            updated_at: chrono::Utc::now(),
        },
    )
    .await
}
