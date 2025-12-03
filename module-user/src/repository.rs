use crate::{entity, query};

#[async_trait::async_trait]
pub trait UserRepository: Sync + Send {
    type Handle<'h>;

    async fn create_user(
        &self,
        handle: &mut Self::Handle<'_>,
        user_entity: &entity::UserEntity,
    ) -> stardust::Result<entity::UserEntity>;

    async fn create_user_account(
        &self,
        handle: &mut Self::Handle<'_>,
        user_account_entity: &entity::UserAccountEntity,
    ) -> stardust::Result<entity::UserAccountEntity>;

    async fn find_user(
        &self,
        handle: &mut Self::Handle<'_>,
        query: &query::FindUserQuery<'_>,
    ) -> stardust::Result<Option<entity::UserEntity>>;

    async fn find_user_accounts(
        &self,
        handle: &mut Self::Handle<'_>,
        user_id: i64,
    ) -> stardust::Result<Vec<crate::entity::UserAccountEntity>>;

    async fn find_user_aggregate(
        &self,
        handle: &mut Self::Handle<'_>,
        query: &crate::query::FindUserQuery<'_>,
    ) -> stardust::Result<Option<entity::UserAggregate>>;

    async fn save_user_account(
        &self,
        handle: &mut Self::Handle<'_>,
        user_account_entity: &entity::UserAccountEntity,
    ) -> stardust::Result<entity::UserAccountEntity>;
}

#[async_trait::async_trait]
pub trait ApiKeyRepository: Sync + Send {
    type Handle<'h>;

    async fn create_apikey(
        &self,
        handle: &mut Self::Handle<'_>,
        entity: &entity::ApiKeyEntity,
    ) -> stardust::Result<entity::ApiKeyEntity>;

    async fn find_user(
        &self,
        handle: &mut Self::Handle<'_>,
        query: &query::FindApiKeyUserQuery<'_>,
    ) -> stardust::Result<Option<entity::ApiKeyUserAggregate>>;

    async fn find_apikeys(
        &self,
        handle: &mut Self::Handle<'_>,
        q: &query::FindApiKeysQuery,
    ) -> stardust::Result<Vec<entity::ApiKeyEntity>>;

    async fn get_apikey(
        &self,
        handle: &mut Self::Handle<'_>,
        id: i64,
    ) -> stardust::Result<Option<entity::ApiKeyEntity>>;

    async fn save_apikey(
        &self,
        handle: &mut Self::Handle<'_>,
        entity: &entity::ApiKeyEntity,
    ) -> stardust::Result<entity::ApiKeyEntity>;

    async fn update_last_used_at(
        &self,
        handle: &mut Self::Handle<'_>,
        id: i64,
        last_used_at: chrono::DateTime<chrono::Utc>,
    ) -> stardust::Result<()>;
}
