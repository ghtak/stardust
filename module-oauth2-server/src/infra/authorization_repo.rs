use stardust_db::Handle;

use crate::infra::model;

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

pub async fn create_authorization(
    handle: &mut Handle<'_>,
    entity: &crate::entity::OAuth2AuthorizationEntity,
) -> stardust_common::Result<crate::entity::OAuth2AuthorizationEntity> {
    let mut builder = sqlx::QueryBuilder::new(
        r#"INSERT INTO oauth2_authorization
            (oauth2_client_id, principal_id, grant_type, scopes, state,
            auth_code_value, auth_code_issued_at, auth_code_expires_at,
            access_token_value, access_token_issued_at, access_token_expires_at,
            refresh_token_hash, refresh_token_issued_at, refresh_token_expires_at) "#,
    );
    builder.push_values(std::iter::once(entity), |mut values, v| {
        values
            .push_bind(v.oauth2_client_id)
            .push_bind(v.principal_id)
            .push_bind(&v.grant_type)
            .push_bind(&v.scope)
            .push_bind(&v.state)
            .push_bind(&v.auth_code_value)
            .push_bind(v.auth_code_issued_at)
            .push_bind(v.auth_code_expires_at)
            .push_bind(&v.access_token_value)
            .push_bind(v.access_token_issued_at)
            .push_bind(v.access_token_expires_at)
            .push_bind(&v.refresh_token_hash)
            .push_bind(v.refresh_token_issued_at)
            .push_bind(v.refresh_token_expires_at);
    });
    builder.push(r#" RETURNING *"#);

    let row = builder
        .build_query_as::<model::OAuth2AuthorizationModel>()
        .fetch_one(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;
    Ok(row.into())
}
