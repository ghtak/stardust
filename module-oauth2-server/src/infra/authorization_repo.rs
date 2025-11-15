use stardust_db::Handle;

pub async fn create_table(handle: &mut Handle<'_>) -> stardust_common::Result<()> {
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS oauth2_authorization (
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
                CONSTRAINT fk_oauth2_client_id FOREIGN KEY (oauth2_client_id) REFERENCES oauth2_client(id)
            );
        "#,
    )
    .execute(handle.executor())
    .await
    .map_err(stardust_db::into_error)?;
    Ok(())
}
