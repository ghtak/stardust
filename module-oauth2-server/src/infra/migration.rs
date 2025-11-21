use stardust_db::{
    database::{Database, Handle},
    internal::postgres,
};

use crate::infra::{authorization_repo, client_repo};

const NAME: &str = "oauth2_server_migration";

pub async fn migrate(database: postgres::Database) -> stardust_common::Result<()> {
    let mut handle = database.tx_handle().await?;
    let mut migration = stardust_core::migration::get_latest(&mut handle, NAME).await?;

    if migration.version == 0 {
        client_repo::create_table(&mut handle).await?;
        authorization_repo::create_table(&mut handle).await?;
        migration =
            stardust_core::migration::save(&mut handle, NAME, 1, "create oauth2 table").await?;
    }
    handle.commit().await?;

    if migration.version == 1 {}
    Ok(())
}
