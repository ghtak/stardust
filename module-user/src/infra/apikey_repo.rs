use crate::entity;

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

pub async fn create_apikey(
    handle: &mut stardust_db::Handle<'_>,
    entity: &entity::ApiKeyEntity,
) -> stardust_common::Result<entity::ApiKeyEntity> {
    let mut builder = sqlx::QueryBuilder::new(
        r#"INSERT INTO stardust_apikey (user_id, key_hash, prefix, description,
        created_at, updated_at) "#,
    );
    builder.push_values(std::iter::once(entity), |mut values, model| {
        values.push_bind(&model.user_id);
        values.push_bind(&model.key_hash);
        values.push_bind(&model.prefix);
        values.push_bind(&model.description);
        values.push_bind(model.created_at);
        values.push_bind(model.updated_at);
    });
    builder.push(
        r#" RETURNING id, user_id, key_hash, prefix, description,
        created_at, updated_at, last_used_at, deactivated_at"#,
    );
    let row = builder
        .build_query_as::<crate::infra::model::ApiKeyModel>()
        .fetch_one(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;
    Ok(row.into())
}
