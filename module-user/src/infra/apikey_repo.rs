use stardust_db::internal::postgres;

use crate::{entity, infra::model, query};

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
    handle: &mut postgres::Handle<'_>,
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
    handle: &mut postgres::Handle<'_>,
    query: &query::FindApiKeyUserQuery<'_>,
) -> stardust_common::Result<Option<entity::ApiKeyUserAggregate>> {
    let mut builder = sqlx::QueryBuilder::new(
        r#"
        select
            sa.id as apikey_id,
            u.*
        from stardust_apikey sa
        left join stardust_user u on sa.user_id  = u.id
        left join stardust_user_account ua on u.id = ua.user_id
        where sa.key_hash =
    "#,
    );
    builder.push_bind(query.key_hash).push(" AND deactivated_at IS NULL");
    let row = builder
        .build_query_as::<model::ApiKeyUserModel>()
        .fetch_optional(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;

    let Some(row) = row else {
        return Ok(None);
    };

    Ok(Some(entity::ApiKeyUserAggregate {
        apikey_id: row.apikey_id,
        user: row.user.into(),
    }))
}

pub async fn find_apikeys(
    handle: &mut postgres::Handle<'_>,
    q: &query::FindApiKeysQuery,
) -> stardust_common::Result<Vec<entity::ApiKeyEntity>> {
    let mut builder = sqlx::QueryBuilder::new("SELECT * FROM stardust_apikey WHERE user_id = ");
    builder.push_bind(q.user_id);

    let rows = builder
        .build_query_as::<model::ApiKeyModel>()
        .fetch_all(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;

    return Ok(rows.into_iter().map(Into::into).collect());
}

pub async fn get_apikey(
    handle: &mut postgres::Handle<'_>,
    id: i64,
) -> stardust_common::Result<Option<entity::ApiKeyEntity>> {
    let mut builder = sqlx::QueryBuilder::new("SELECT * FROM stardust_apikey WHERE id = ");
    builder.push_bind(id);

    let row = builder
        .build_query_as::<model::ApiKeyModel>()
        .fetch_optional(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;

    Ok(row.map(Into::into))
}

pub async fn save_apikey(
    handle: &mut postgres::Handle<'_>,
    entity: &entity::ApiKeyEntity,
) -> stardust_common::Result<entity::ApiKeyEntity> {
    let mut builder = sqlx::QueryBuilder::new("UPDATE stardust_apikey SET ");

    builder.push("description = ");
    builder.push_bind(&entity.description);
    builder.push(", updated_at = ");
    builder.push_bind(entity.updated_at);

    if let Some(ref deactivated_at) = entity.deactivated_at {
        builder.push(", deactivated_at = ");
        builder.push_bind(deactivated_at);
    }

    builder.push(" WHERE id = ");
    builder.push_bind(entity.id);
    builder.push(" RETURNING *");

    let row = builder
        .build_query_as::<model::ApiKeyModel>()
        .fetch_one(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;

    Ok(row.into())
}

pub async fn update_last_used_at(
    handle: &mut postgres::Handle<'_>,
    id: i64,
    last_used_at: chrono::DateTime<chrono::Utc>,
) -> stardust_common::Result<()> {
    sqlx::QueryBuilder::new("UPDATE stardust_apikey SET ")
        .push("last_used_at = ")
        .push_bind(last_used_at)
        .push(", updated_at = ")
        .push_bind(last_used_at)
        .push(" WHERE id = ")
        .push_bind(id)
        .build()
        .execute(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;
    Ok(())
}

pub struct PostgresApiKeyRepository {}

impl PostgresApiKeyRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl crate::repository::ApiKeyRepository for PostgresApiKeyRepository {
    type Handle<'h> = stardust_db::internal::postgres::Handle<'h>;

    async fn create_table(
        &self,
        handle: &mut Self::Handle<'_>,
    ) -> stardust_common::Result<()> {
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

    async fn create_apikey(
        &self,
        handle: &mut Self::Handle<'_>,
        entity: &entity::ApiKeyEntity,
    ) -> stardust_common::Result<entity::ApiKeyEntity> {
        create_apikey(handle, entity).await
    }

    async fn find_user(
        &self,
        handle: &mut Self::Handle<'_>,
        query: &query::FindApiKeyUserQuery<'_>,
    ) -> stardust_common::Result<Option<entity::ApiKeyUserAggregate>> {
        find_user(handle, query).await
    }

    async fn find_apikeys(
        &self,
        handle: &mut Self::Handle<'_>,
        q: &query::FindApiKeysQuery,
    ) -> stardust_common::Result<Vec<entity::ApiKeyEntity>> {
        find_apikeys(handle, q).await
    }

    async fn get_apikey(
        &self,
        handle: &mut Self::Handle<'_>,
        id: i64,
    ) -> stardust_common::Result<Option<entity::ApiKeyEntity>> {
        get_apikey(handle, id).await
    }

    async fn save_apikey(
        &self,
        handle: &mut Self::Handle<'_>,
        entity: &entity::ApiKeyEntity,
    ) -> stardust_common::Result<entity::ApiKeyEntity> {
        save_apikey(handle, entity).await
    }

    async fn update_last_used_at(
        &self,
        handle: &mut Self::Handle<'_>,
        id: i64,
        last_used_at: chrono::DateTime<chrono::Utc>,
    ) -> stardust_common::Result<()> {
        update_last_used_at(handle, id, last_used_at).await
    }
}
