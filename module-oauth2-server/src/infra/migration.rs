use stardust::database::{Database, Handle};

pub async fn migrate(
    database: stardust::infra::migration::Database,
) -> stardust::Result<()> {
    const NAME: &str = "oauth2_server_migration";

    let mut migration =
        stardust::infra::migration::get_latest(&mut database.handle(), NAME)
            .await?
            .unwrap_or_default();
    if migration.version == 0 {
        let mut handle = database.tx_handle().await?;
        sqlx::query(
            r#" CREATE TABLE IF NOT EXISTS oauth2_client (
                id BIGSERIAL PRIMARY KEY,
                client_id VARCHAR(255) UNIQUE NOT NULL,
                client_secret_hash VARCHAR(255) NOT NULL,
                name VARCHAR(255) NOT NULL,
                redirect_uris TEXT NOT NULL,
                grant_types TEXT NOT NULL,
                auth_methods TEXT NOT NULL,
                scopes TEXT NOT NULL,
                token_settings JSONB NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            ); "#,
        )
        .execute(handle.executor())
        .await
        .map_err(stardust::database::internal::into_error)?;

        sqlx::query(
             r#" CREATE TABLE IF NOT EXISTS oauth2_authorization (
                id BIGSERIAL PRIMARY KEY,
                oauth2_client_id BIGSERIAL NOT NULL,
                principal_id BIGINT NOT NULL,
                grant_type VARCHAR(50) NOT NULL,
                scopes TEXT,
                state VARCHAR(255),
                auth_code_value VARCHAR(255),
                auth_code_issued_at TIMESTAMPTZ,
                auth_code_expires_at TIMESTAMPTZ,
                access_token_value VARCHAR(255),
                access_token_issued_at TIMESTAMPTZ,
                access_token_expires_at TIMESTAMPTZ,
                refresh_token_hash VARCHAR(255),
                refresh_token_issued_at TIMESTAMPTZ,
                refresh_token_expires_at TIMESTAMPTZ,
                config JSONB NOT NULL,
                CONSTRAINT fk_oauth2_client_id FOREIGN KEY (oauth2_client_id) REFERENCES oauth2_client(id)
            ); "#,
        )
        .execute(handle.executor())
        .await
        .map_err(stardust::database::internal::into_error)?;

        migration.name = NAME.into();
        migration.version = 1;
        migration.description =
            "create oauth2_client and oauth2_authorization tables".into();
        migration =
            stardust::infra::migration::save(&mut handle, &migration).await?;
        handle.commit().await?;
    }
    if migration.version == 1 {}
    Ok(())
}
