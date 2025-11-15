use crate::{command, entity, service::OAuth2ClientService};

pub struct OAuth2ClientServiceImpl {}

impl OAuth2ClientServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl OAuth2ClientService for OAuth2ClientServiceImpl {
    async fn create_client(
        &self,
        _command: &command::CreateOAuth2ClientCommand,
    ) -> stardust_common::Result<entity::OAuth2ClientEntity> {
        unimplemented!()
    }
}
