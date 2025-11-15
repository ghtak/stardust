pub async fn create_table(handle: &mut stardust_db::Handle<'_>) -> stardust_common::Result<()> {
    sqlx::query(
        r#"
            create table if not exists stardust_apikey (
                id BIGSERIAL PRIMARY KEY,
                user_id BIGINT not null,
                key_hash varchar(255) not null,
                prefix varchar(255) not null,
                description varchar(255) not null,
                created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                last_used_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                deactivated_at TIMESTAMPTZ
            );
        "#,
    )
    .execute(handle.executor())
    .await
    .map_err(stardust_db::into_error)?;
    Ok(())
}
