use crate::{command, entity, query};

#[async_trait::async_trait]
pub trait ClientRepository: Sync + Send {
    type Handle<'h>;

    async fn create_table(&self, handle: &mut Self::Handle<'_>) -> stardust_common::Result<()>;

    async fn create_client(
        &self,
        handle: &mut Self::Handle<'_>,
        entity: &entity::OAuth2ClientEntity,
    ) -> stardust_common::Result<entity::OAuth2ClientEntity>;

    async fn find_clients(
        &self,
        handle: &mut Self::Handle<'_>,
        query: &query::FindOAuth2ClientQuery<'_>,
    ) -> stardust_common::Result<Vec<entity::OAuth2ClientEntity>>;

    async fn delete_client(
        &self,
        handle: &mut Self::Handle<'_>,
        command: &command::DeleteOAuth2ClientCommand,
    ) -> stardust_common::Result<()>;
}

#[async_trait::async_trait]
pub trait AuthorizationRepository: Sync + Send {
    type Handle<'h>;

    async fn create_table(&self, handle: &mut Self::Handle<'_>) -> stardust_common::Result<()>;

    async fn create_authorization(
        &self,
        handle: &mut Self::Handle<'_>,
        entity: &crate::entity::OAuth2AuthorizationEntity,
    ) -> stardust_common::Result<crate::entity::OAuth2AuthorizationEntity>;

    async fn find_authorization(
        &self,
        handle: &mut Self::Handle<'_>,
        query: &query::FindOAuth2AuthorizationQuery<'_>,
    ) -> stardust_common::Result<Option<entity::OAuth2AuthorizationEntity>>;

    async fn save_authorization(
        &self,
        handle: &mut Self::Handle<'_>,
        entity: &entity::OAuth2AuthorizationEntity,
    ) -> stardust_common::Result<entity::OAuth2AuthorizationEntity>;

    async fn find_user(
        &self,
        handle: &mut Self::Handle<'_>,
        query: &query::FindOAuth2UserQuery<'_>,
    ) -> stardust_common::Result<Option<entity::OAuthUserAggregate>>;
}
