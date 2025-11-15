use stardust_common::With;

use crate::{command, entity};

#[async_trait::async_trait]
pub trait UserService: Sync + Send {
    async fn hello(&self) -> String;

    async fn signup(
        &self,
        command: &command::SignupCommand,
    ) -> stardust_common::Result<entity::UserAggregate>;

    async fn login(
        &self,
        command: &command::LoginCommand,
    ) -> stardust_common::Result<entity::UserAggregate>;
}

pub trait ApiKeyService: Sync + Send {
    fn create_apikey(
        &self,
        command: &command::CreateApiKeyCommand,
    ) -> impl Future<Output = stardust_common::Result<With<String, entity::ApiKeyEntity>>> + Send;
}
