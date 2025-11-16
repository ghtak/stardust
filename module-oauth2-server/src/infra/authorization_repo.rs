use stardust_db::Handle;

use crate::{
    entity,
    infra::model::{self, OAuth2AuthorizationModel},
    query,
};

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

pub async fn find_authorization(
    handle: &mut Handle<'_>,
    query: &query::FindOAuth2AuthorizationQuery<'_>,
) -> stardust_common::Result<Option<entity::OAuth2AuthorizationEntity>> {
    let mut builder = sqlx::QueryBuilder::new(r#"SELECT * FROM oauth2_authorization WHERE 1=1 "#);
    if let Some(auth_code_value) = &query.auth_code_value {
        builder.push(" AND auth_code_value = ");
        builder.push_bind(auth_code_value);
    }

    if let Some(refresh_token_hash) = &query.refresh_token_hash {
        builder.push(" AND refresh_token_hash = ");
        builder.push_bind(refresh_token_hash);
    }

    let row = builder
        .build_query_as::<model::OAuth2AuthorizationModel>()
        .fetch_optional(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;
    Ok(row.map(Into::into))
}

pub async fn save_authorization(
    handle: &mut Handle<'_>,
    entity: &entity::OAuth2AuthorizationEntity,
) -> stardust_common::Result<entity::OAuth2AuthorizationEntity> {
    let mut builder = sqlx::QueryBuilder::new("UPDATE oauth2_authorization SET ");
    builder
        .push(" auth_code_expires_at = ")
        .push_bind(entity.auth_code_expires_at)
        .push(", access_token_value = ")
        .push_bind(&entity.access_token_value)
        .push(", access_token_issued_at = ")
        .push_bind(entity.access_token_issued_at)
        .push(", access_token_expires_at = ")
        .push_bind(entity.access_token_expires_at)
        .push(", refresh_token_hash = ")
        .push_bind(&entity.refresh_token_hash)
        .push(", refresh_token_issued_at = ")
        .push_bind(entity.refresh_token_issued_at)
        .push(", refresh_token_expires_at = ")
        .push_bind(entity.refresh_token_expires_at)
        .push(" WHERE id = ")
        .push_bind(entity.id)
        .push(" RETURNING * ");
    let row = builder
        .build_query_as::<OAuth2AuthorizationModel>()
        .fetch_one(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;
    Ok(row.into())
}

pub async fn find_user(
    handle: &mut Handle<'_>,
    query: &query::FindOAuth2UserQuery<'_>,
) -> stardust_common::Result<Option<entity::OAuthUserAggregate>> {
    let mut builder = sqlx::QueryBuilder::new(
        r#"
        select
            oa.id as authorization_id,
            oa.oauth2_client_id as authorization_oauth2_client_id,
            oa.principal_id as authorization_principal_id,
            oa.grant_type as authorization_grant_type,
            oa.scopes as authorization_scopes,
            oa.state as authorization_state,
            oa.auth_code_value as authorization_auth_code_value,
            oa.auth_code_issued_at as authorization_auth_code_issued_at,
            oa.auth_code_expires_at as authorization_auth_code_expires_at,
            oa.access_token_value as authorization_access_token_value,
            oa.access_token_issued_at as authorization_access_token_issued_at,
            oa.access_token_expires_at as authorization_access_token_expires_at,
            oa.refresh_token_hash as authorization_refresh_token_hash,
            oa.refresh_token_issued_at as authorization_refresh_token_issued_at,
            oa.refresh_token_expires_at as authorization_refresh_token_expires_at,

            c.id as client_id,
            c.client_id as client_client_id,
            c.client_secret_hash as client_client_secret_hash,
            c.name as client_name,
            c.redirect_uris as client_redirect_uris,
            c.grant_types as client_grant_types,
            c.auth_methods as client_auth_methods,
            c.scopes as client_scopes,
            c.token_settings as client_token_settings,

            u.id as user_id,
            u.username as user_username,
            u.email as user_email,
            u.role as user_role,
            u.status as user_status,
            u.created_at as user_created_at,
            u.updated_at as user_updated_at,

            ua.uid as account_uid,
            ua.user_id as account_user_id,
            ua.account_type as account_account_type,
            ua.password_hash as account_password_hash,
            ua.created_at as account_created_at,
            ua.updated_at as account_updated_at

        from oauth2_authorization oa
        left join stardust_user u on oa.principal_id = u.id
        left join stardust_user_account ua on oa.principal_id = ua.user_id
        left join oauth2_client c on oa.oauth2_client_id = c.id
        where oa.access_token_value =
    "#,
    );
    builder.push_bind(query.access_token);

    let rows = builder
        .build_query_as::<model::OAuth2AuthorizationUserModel>()
        .fetch_all(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;

    if rows.is_empty() {
        return Ok(None);
    }

    let mut authorization: Option<entity::OAuth2AuthorizationEntity> = None;
    let mut client: Option<entity::OAuth2ClientEntity> = None;
    let mut user_aggregate: Option<module_user::entity::UserAggregate> = None;

    for r in rows {
        if authorization.is_none() {
            authorization = Some(r.authorization_entity());
        }
        if client.is_none() {
            client = Some(r.client_entity());
        }
        let agg = user_aggregate.get_or_insert_with(|| module_user::entity::UserAggregate {
            user: r.user_entity(),
            accounts: Vec::new(),
        });
        agg.accounts.push(r.account_entity());
    }
    if authorization.is_none() {
        return Ok(None);
    }
    Ok(Some(entity::OAuthUserAggregate {
        client: client.unwrap(),
        user: user_aggregate.unwrap(),
        authorization: authorization.unwrap(),
    }))
}
