use stardust_db::internal::postgres;

use crate::{command, entity, infra::model, query};

pub async fn create_table(handle: &mut postgres::Handle<'_>) -> stardust_common::Result<()> {
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
    handle: &mut postgres::Handle<'_>,
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

pub async fn find_clients(
    handle: &mut postgres::Handle<'_>,
    query: &crate::query::FindOAuth2ClientQuery<'_>,
) -> stardust_common::Result<Vec<entity::OAuth2ClientEntity>> {
    let mut querybuilder = sqlx::QueryBuilder::new("SELECT * FROM oauth2_client where 1 = 1 ");
    if let Some(ref client_id) = query.client_id {
        querybuilder.push(" AND client_id = ");
        querybuilder.push_bind(client_id);
    }

    querybuilder.push(" order by id desc");

    let rows = querybuilder
        .build_query_as::<model::OAuth2ClientModel>()
        .fetch_all(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;

    Ok(rows.into_iter().map(Into::into).collect())
}

pub async fn delete_client(
    handle: &mut postgres::Handle<'_>,
    command: &crate::command::DeleteOAuth2ClientCommand,
) -> stardust_common::Result<()> {
    let mut querybuilder = sqlx::QueryBuilder::new("DELETE FROM oauth2_client where id = ");
    querybuilder.push_bind(command.id);
    querybuilder.build().execute(handle.executor()).await.map_err(stardust_db::into_error)?;
    Ok(())
}

pub struct PostgresClientRepository {}

impl PostgresClientRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl crate::repository::ClientRepository for PostgresClientRepository {
    type Handle<'h> = postgres::Handle<'h>;

    async fn create_client(
        &self,
        handle: &mut Self::Handle<'_>,
        entity: &entity::OAuth2ClientEntity,
    ) -> stardust_common::Result<entity::OAuth2ClientEntity> {
        create_client(handle, entity).await
    }

    async fn find_clients(
        &self,
        handle: &mut Self::Handle<'_>,
        query: &query::FindOAuth2ClientQuery<'_>,
    ) -> stardust_common::Result<Vec<entity::OAuth2ClientEntity>> {
        find_clients(handle, query).await
    }

    async fn delete_client(
        &self,
        handle: &mut Self::Handle<'_>,
        command: &command::DeleteOAuth2ClientCommand,
    ) -> stardust_common::Result<()> {
        delete_client(handle, command).await
    }
}
