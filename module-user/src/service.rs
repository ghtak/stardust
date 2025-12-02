use stardust::With;

use crate::{command, entity, query};

#[async_trait::async_trait]
pub trait UserService: Sync + Send {
    async fn hello(&self) -> String;

    async fn signup(
        &self,
        command: &command::SignupCommand,
    ) -> stardust::Result<entity::UserAggregate>;

    async fn login(
        &self,
        command: &command::LoginCommand,
    ) -> stardust::Result<entity::UserAggregate>;
}

pub trait ApiKeyService: Sync + Send {
    fn create_apikey(
        &self,
        command: &command::CreateApiKeyCommand,
    ) -> impl Future<Output = stardust::Result<With<String, entity::ApiKeyEntity>>> + Send;

    fn find_user(
        &self,
        query: &query::FindApiKeyUserQuery<'_>,
    ) -> impl Future<Output = stardust::Result<Option<entity::ApiKeyUserAggregate>>> + Send;

    fn find_apikeys(
        &self,
        query: &query::FindApiKeysQuery,
    ) -> impl Future<Output = stardust::Result<Vec<entity::ApiKeyEntity>>> + Send;

    fn deactivate_apikey(
        &self,
        command: &command::DeactivateApiKeyCommand,
    ) -> impl Future<Output = stardust::Result<entity::ApiKeyEntity>> + Send;
}

#[async_trait::async_trait]
pub trait ApiKeyUsageTracker: Sync + Send {
    async fn track_usage(
        &self,
        apikey_id: i64,
    ) -> stardust::Result<()>;
}