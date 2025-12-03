use std::sync::Arc;

use crate::{command, entity, query, service::ApiKeyService};

pub struct ApiKeyServiceImpl<Database, ApiKeyRepository, Tracker, Hasher> {
    database: Database,
    apikey_repo: Arc<ApiKeyRepository>,
    tracker: Arc<Tracker>,
    hasher: Arc<Hasher>,
}

impl<Database, ApiKeyRepository, Tracker, Hasher>
    ApiKeyServiceImpl<Database, ApiKeyRepository, Tracker, Hasher>
where
    Database: stardust::database::Database,
    ApiKeyRepository: for<'h> crate::repository::ApiKeyRepository<
            Handle<'h> = Database::Handle<'h>,
        >,
    Tracker: crate::service::ApiKeyUsageTracker,
    Hasher: stardust::hash::Hasher,
{
    pub fn new(
        database: Database,
        apikey_repo: Arc<ApiKeyRepository>,
        tracker: Arc<Tracker>,
        hasher: Arc<Hasher>,
    ) -> Self {
        Self {
            database,
            apikey_repo,
            tracker,
            hasher,
        }
    }
}

impl<Database, ApiKeyRepository, Tracker, Hasher> ApiKeyService
    for ApiKeyServiceImpl<Database, ApiKeyRepository, Tracker, Hasher>
where
    Database: stardust::database::Database + 'static,
    ApiKeyRepository: for<'h> crate::repository::ApiKeyRepository<
            Handle<'h> = Database::Handle<'h>,
        >,
    Tracker: crate::service::ApiKeyUsageTracker,
    Hasher: stardust::hash::Hasher,
{
    async fn create_apikey(
        &self,
        command: &command::CreateApiKeyCommand,
    ) -> stardust::Result<entity::ApiKeyWithSecret> {
        let key = stardust::utils::generate_uid();
        let key_hash = self.hasher.hash(&key).await?;
        let now = chrono::Utc::now();
        let entity = entity::ApiKeyEntity {
            id: 0,
            user_id: command.user_id,
            key_hash: key_hash,
            prefix: key[..8].to_string(),
            description: command.description.clone(),
            created_at: now,
            updated_at: now,
            last_used_at: now,
            deactivated_at: None,
        };
        let entity = self
            .apikey_repo
            .create_apikey(&mut self.database.handle(), &entity)
            .await?;
        // stardust_core::audit(entity.user_id, "apikey.created", serde_json::json!(entity));
        Ok(entity::ApiKeyWithSecret {
            secret: key,
            apikey: entity,
        })
    }

    async fn find_user(
        &self,
        query: &query::FindApiKeyUserQuery<'_>,
    ) -> stardust::Result<Option<entity::ApiKeyUserAggregate>> {
        let key_hash = self.hasher.hash(&query.key_hash).await?;
        let result = self
            .apikey_repo
            .find_user(
                &mut self.database.handle(),
                &query::FindApiKeyUserQuery {
                    key_hash: &key_hash,
                },
            )
            .await;
        match result {
            Ok(Some(ref user)) => {
                self.tracker.track_usage(user.apikey_id).await?;
            }
            _ => {}
        }
        result
    }

    async fn find_apikeys(
        &self,
        query: &query::FindApiKeysQuery,
    ) -> stardust::Result<Vec<entity::ApiKeyEntity>> {
        return self
            .apikey_repo
            .find_apikeys(&mut self.database.handle(), &query)
            .await;
    }

    async fn deactivate_apikey(
        &self,
        command: &command::DeactivateApiKeyCommand,
    ) -> stardust::Result<entity::ApiKeyEntity> {
        let result = self
            .apikey_repo
            .get_apikey(&mut self.database.handle(), command.apikey_id)
            .await?;
        if result.is_none() {
            return Err(stardust::Error::NotFound("".into()));
        }
        let mut key = result.unwrap();
        if key.user_id != command.request_user_id {
            return Err(stardust::Error::Forbidden);
        }
        if key.deactivated_at.is_some() {
            return Err(stardust::Error::Forbidden);
        }
        key.deactivated_at = Some(chrono::Utc::now());
        let key = self
            .apikey_repo
            .save_apikey(&mut self.database.handle(), &key)
            .await?;
        // stardust_core::audit(key.user_id, "apikey.deactivated", serde_json::json!(key));
        Ok(key)
    }
}
