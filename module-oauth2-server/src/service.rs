use crate::{command, entity};

#[async_trait::async_trait]
pub trait OAuth2ClientService : Sync + Send{
    async fn create_client(
        &self,
        command: &command::CreateOAuth2ClientCommand,
    ) -> stardust_common::Result<entity::OAuth2ClientEntity>;
}
