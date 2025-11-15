use crate::{
    command, entity,
    infra::model,
};

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

pub async fn find_user(
    handle: &mut stardust_db::Handle<'_>,
    command: &command::FindApiKeyUserCommand,
) -> stardust_common::Result<Option<entity::UserAggregate>> {
    let mut builder = sqlx::QueryBuilder::new(
        r#"
        SELECT (u.*) as "inner!: UserModel", (ua.*) as "related!: UserAccountModel"
        FROM stardust_user u
        LEFT JOIN stardust_user_account ua ON u.id = ua.user_id
        WHERE u.id IN
    "#,
    );
    builder
        .push("(SELECT user_id from stardust_apikey where key_hash = ")
        .push_bind(&command.key_hash)
        .push(" AND deactivated_at IS NULL)");

    let rows = builder
        .build_query_as::<stardust_db::With<model::UserModel, model::UserAccountModel>>()
        .fetch_all(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;

    if rows.is_empty() {
        return Ok(None);
    }

    let mut aggregate: Option<entity::UserAggregate> = None;
    for r in rows {
        let agg = aggregate.get_or_insert_with(|| entity::UserAggregate {
            user: r.inner.into(),
            accounts: Vec::new(),
        });

        agg.accounts.push(r.related.into());
    }

    Ok(aggregate)
}
