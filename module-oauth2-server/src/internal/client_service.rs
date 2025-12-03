use std::{borrow::Cow, sync::Arc};

use crate::{command, entity, query, service::OAuth2ClientService};

pub struct OAuth2ClientServiceImpl<Database, ClientRepository, Hasher> {
    database: Database,
    client_repo: Arc<ClientRepository>,
    hasher: Arc<Hasher>,
}

impl<Database, ClientRepository, Hasher>
    OAuth2ClientServiceImpl<Database, ClientRepository, Hasher>
{
    pub fn new(
        database: Database,
        client_repo: Arc<ClientRepository>,
        hasher: Arc<Hasher>,
    ) -> Self {
        Self {
            database,
            client_repo,
            hasher,
        }
    }
}

#[async_trait::async_trait]
impl<Database, ClientRepository, Hasher> OAuth2ClientService
    for OAuth2ClientServiceImpl<Database, ClientRepository, Hasher>
where
    Database: stardust::database::Database + 'static,
    ClientRepository: for<'h> crate::repository::ClientRepository<
            Handle<'h> = Database::Handle<'h>,
        >,
    Hasher: stardust::hash::Hasher,
{
    async fn create_client(
        &self,
        command: &command::CreateOAuth2ClientCommand,
    ) -> stardust::Result<entity::OAuth2ClientEntity> {
        let client_secret_hash =
            self.hasher.hash(&command.client_secret).await?;
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
        let entity = self
            .client_repo
            .create_client(&mut self.database.handle(), &entity)
            .await?;
        Ok(entity)
    }

    async fn find_clients(
        &self,
        query: &query::FindOAuth2ClientQuery<'_>,
    ) -> stardust::Result<Vec<entity::OAuth2ClientEntity>> {
        self.client_repo.find_clients(&mut self.database.handle(), &query).await
    }

    async fn delete_client(
        &self,
        command: &command::DeleteOAuth2ClientCommand,
    ) -> stardust::Result<()> {
        self.client_repo
            .delete_client(&mut self.database.handle(), &command)
            .await
    }

    async fn verify(
        &self,
        command: &command::VerifyOAuth2ClientCommand<'_>,
    ) -> stardust::Result<()> {
        let clients = self
            .find_clients(&query::FindOAuth2ClientQuery {
                client_id: Some(command.client_id),
            })
            .await?;

        if clients.len() == 0 {
            return Err(stardust::Error::NotFound(Cow::Owned(
                command.client_id.to_owned(),
            )));
        }

        let client = clients.first().unwrap();
        let result = self
            .hasher
            .verify(&command.client_secret, &client.client_secret_hash)
            .await?;
        if !result {
            return Err(stardust::Error::InvalidParameter(
                "Invalid client secret".into(),
            ));
        }
        Ok(())
    }
}
