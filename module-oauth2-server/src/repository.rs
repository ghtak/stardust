use crate::{command, entity, query};

#[async_trait::async_trait]
pub trait ClientRepository: Sync + Send {
    type Handle<'h>;

    async fn create_client(
        &self,
        handle: &mut Self::Handle<'_>,
        entity: &entity::OAuth2ClientEntity,
    ) -> stardust::Result<entity::OAuth2ClientEntity>;

    async fn find_clients(
        &self,
        handle: &mut Self::Handle<'_>,
        query: &query::FindOAuth2ClientQuery<'_>,
    ) -> stardust::Result<Vec<entity::OAuth2ClientEntity>>;

    async fn delete_client(
        &self,
        handle: &mut Self::Handle<'_>,
        command: &command::DeleteOAuth2ClientCommand,
    ) -> stardust::Result<()>;
}

#[async_trait::async_trait]
pub trait AuthorizationRepository: Sync + Send {
    type Handle<'h>;

    async fn create_authorization(
        &self,
        handle: &mut Self::Handle<'_>,
        entity: &crate::entity::OAuth2AuthorizationEntity,
    ) -> stardust::Result<crate::entity::OAuth2AuthorizationEntity>;

    async fn find_authorization(
        &self,
        handle: &mut Self::Handle<'_>,
        query: &query::FindOAuth2AuthorizationQuery<'_>,
    ) -> stardust::Result<Option<entity::OAuth2AuthorizationEntity>>;

    async fn save_authorization(
        &self,
        handle: &mut Self::Handle<'_>,
        entity: &entity::OAuth2AuthorizationEntity,
    ) -> stardust::Result<entity::OAuth2AuthorizationEntity>;

    async fn find_user(
        &self,
        handle: &mut Self::Handle<'_>,
        query: &query::FindOAuth2UserQuery<'_>,
    ) -> stardust::Result<Option<entity::OAuthUserAggregate>>;
}

// #[async_trait::async_trait]
// pub trait MigrationRepository: Sync + Send {
//     type Handle<'h>;

//     async fn create_client_store(&self, handle: &mut Self::Handle<'_>)
//     -> stardust::Result<()>;

//     async fn create_authorization_store(
//         &self,
//         handle: &mut Self::Handle<'_>,
//     ) -> stardust::Result<()>;
// }
