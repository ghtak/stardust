use stardust_db::Handle;

use crate::{entity, infra::model};

pub async fn create_table(handle: &mut Handle<'_>) -> stardust_common::Result<()> {
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS oauth2_client (
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
            );
        "#,
    )
    .execute(handle.executor())
    .await
    .map_err(stardust_db::into_error)?;
    Ok(())
}

pub async fn create_client(
    handle: &mut Handle<'_>,
    entity: &entity::OAuth2ClientEntity,
) -> stardust_common::Result<entity::OAuth2ClientEntity> {
    let mut querybuilder = sqlx::QueryBuilder::new(
        r#"INSERT INTO oauth2_client (client_id, client_secret_hash, name,
            redirect_uris, grant_types, auth_methods, scopes, token_settings) "#,
    );
    querybuilder.push_values(std::iter::once(entity), |mut values, item| {
        values
            .push_bind(&item.client_id)
            .push_bind(&item.client_secret_hash)
            .push_bind(&item.name)
            .push_bind(item.redirect_uris.join(","))
            .push_bind(item.grant_types.join(","))
            .push_bind(item.auth_methods.join(","))
            .push_bind(item.scopes.join(","))
            .push_bind(serde_json::json!({}));
    });
    querybuilder.push(" RETURNING *");

    let row = querybuilder
        .build_query_as::<model::OAuth2ClientModel>()
        .fetch_one(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;

    Ok(row.into())
}
