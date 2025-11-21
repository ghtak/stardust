use stardust_db::{Handle, internal::postgres};

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
    handle: &mut postgres::Handle<'_>,
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
    handle: &mut postgres::Handle<'_>,
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
    handle: &mut postgres::Handle<'_>,
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
    handle: &mut postgres::Handle<'_>,
    query: &query::FindOAuth2UserQuery<'_>,
) -> stardust_common::Result<Option<entity::OAuthUserAggregate>> {
    let mut builder = sqlx::QueryBuilder::new(
        r#"
        select
            row_to_json(c) as client_json,
            row_to_json(u) as user_json
        from oauth2_authorization oa
        left join stardust_user u on oa.principal_id = u.id
        left join oauth2_client c on oa.oauth2_client_id = c.id
        where oa.access_token_value =
    "#,
    );
    builder.push_bind(query.access_token);

    let row = builder
        .build_query_as::<model::OAuth2AuthorizationUserModel>()
        .fetch_optional(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;

    let Some(row) = row else {
        return Ok(None);
    };

    Ok(Some(entity::OAuthUserAggregate {
        client: row.client.into(),
        user: row.user.into(),
    }))
}

pub struct PostgresAuthorizationRepository {}

impl PostgresAuthorizationRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl crate::repository::AuthorizationRepository for PostgresAuthorizationRepository {
    type Handle<'h> = stardust_db::internal::postgres::Handle<'h>;

    async fn create_table(&self, handle: &mut Self::Handle<'_>) -> stardust_common::Result<()> {
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

    async fn create_authorization(
        &self,
        handle: &mut Self::Handle<'_>,
        entity: &crate::entity::OAuth2AuthorizationEntity,
    ) -> stardust_common::Result<crate::entity::OAuth2AuthorizationEntity> {
        create_authorization(handle, entity).await
    }

    async fn find_authorization(
        &self,
        handle: &mut Self::Handle<'_>,
        query: &query::FindOAuth2AuthorizationQuery<'_>,
    ) -> stardust_common::Result<Option<entity::OAuth2AuthorizationEntity>> {
        find_authorization(handle, query).await
    }

    async fn save_authorization(
        &self,
        handle: &mut Self::Handle<'_>,
        entity: &entity::OAuth2AuthorizationEntity,
    ) -> stardust_common::Result<entity::OAuth2AuthorizationEntity> {
        save_authorization(handle, entity).await
    }

    async fn find_user(
        &self,
        handle: &mut Self::Handle<'_>,
        query: &query::FindOAuth2UserQuery<'_>,
    ) -> stardust_common::Result<Option<entity::OAuthUserAggregate>> {
        find_user(handle, query).await
    }
}
