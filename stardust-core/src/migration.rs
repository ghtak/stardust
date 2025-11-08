use crate::infra;

pub async fn migrate(
    database: stardust_db::Database,
) -> stardust_common::Result<()> {
    let mut handle = database.pool();
    infra::migration_repo::create_table(&mut handle).await?;
    Ok(())
}
