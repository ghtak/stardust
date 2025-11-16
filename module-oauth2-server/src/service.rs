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

    async fn verify(
        &self,
        command: &command::VerifyOAuth2ClientCommand<'_>,
    ) -> stardust_common::Result<()>;
}

#[async_trait::async_trait]
pub trait OAuth2AuthorizationService: Sync + Send {
    async fn verify(
        &self,
        command: &command::VerifyOAuth2AuthorizationCommand<'_>,
    ) -> stardust_common::Result<entity::OAuth2ClientEntity>;

    async fn authorize(
        &self,
        command: &command::AuthorizeOAuth2Command<'_>,
    ) -> stardust_common::Result<entity::OAuth2AuthorizationEntity>;

    async fn token(
        &self,
        command: &command::TokenCommand<'_>,
    ) -> stardust_common::Result<entity::OAuth2Token>;

    async fn find_user(
        &self,
        query: &query::FindOAuth2UserQuery<'_>,
    ) -> stardust_common::Result<Option<entity::OAuthUserAggregate>>;
}
