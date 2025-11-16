use std::sync::Arc;

use crate::{command, entity, infra::client_repo, query, service::OAuth2ClientService};

pub struct OAuth2ClientServiceImpl<H> {
    database: stardust_db::Database,
    hasher: Arc<H>,
}

impl<H> OAuth2ClientServiceImpl<H>
where
    H: stardust_common::hash::Hasher,
{
    pub fn new(database: stardust_db::Database, hasher: Arc<H>) -> Self {
        Self { database, hasher }
    }
}

#[async_trait::async_trait]
impl<H> OAuth2ClientService for OAuth2ClientServiceImpl<H>
where
    H: stardust_common::hash::Hasher,
{
    async fn create_client(
        &self,
        command: &command::CreateOAuth2ClientCommand,
    ) -> stardust_common::Result<entity::OAuth2ClientEntity> {
        let client_secret_hash = self.hasher.hash(&command.client_secret)?;
        let entity = entity::OAuth2ClientEntity {
            id: 0,
            name: command.name.clone(),
            client_id: command.client_id.clone(),
            client_secret_hash: client_secret_hash,
            redirect_uris: command.redirect_uris.clone(),
            grant_types: command.grant_types.clone(),
            auth_methods: command.auth_methods.clone(),
            scopes: command.scopes.clone(),
        };
        let entity = client_repo::create_client(&mut self.database.pool(), &entity).await?;
        Ok(entity)
    }

    async fn find_clients(
        &self,
        query: &query::FindOAuth2ClientQuery<'_>,
    ) -> stardust_common::Result<Vec<entity::OAuth2ClientEntity>>{
        client_repo::find_clients(&mut self.database.pool(), &query).await
    }

    async fn delete_client(
        &self,
        command: &command::DeleteOAuth2ClientCommand,
    ) -> stardust_common::Result<()>
    {
        client_repo::delete_client(&mut self.database.pool(), &command).await
    }
}
