use crate::{command, entity, query};

#[async_trait::async_trait]
pub trait OAuth2ClientService: Sync + Send {
    async fn create_client(
        &self,
        command: &command::CreateOAuth2ClientCommand,
    ) -> stardust_common::Result<entity::OAuth2ClientEntity>;

    async fn find_clients(
        &self,
        query: &query::FindOAuth2ClientQuery<'_>,
    ) -> stardust_common::Result<Vec<entity::OAuth2ClientEntity>>;

    async fn delete_client(
        &self,
        command: &command::DeleteOAuth2ClientCommand,
    ) -> stardust_common::Result<()>;
}

#[async_trait::async_trait]
pub trait OAuth2AuthorizationService: Sync + Send {}
