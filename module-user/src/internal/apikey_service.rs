use std::sync::Arc;

use stardust_common::With;

use crate::{command, entity, query, service::ApiKeyService};

pub struct ApikeyServiceImpl<Database, ApiKeyRepo, Tracker, Hasher> {
    database: Database,
    apikey_repo: Arc<ApiKeyRepo>,
    tracker: Arc<Tracker>,
    hasher: Arc<Hasher>,
}

impl<Database, ApiKeyRepo, Tracker, Hasher> ApikeyServiceImpl<Database, ApiKeyRepo, Tracker, Hasher>
where
    Database: stardust_db::database::Database,
    ApiKeyRepo: for<'h> crate::repository::ApiKeyRepository<Handle<'h> = Database::Handle<'h>>,
    Tracker: crate::service::ApiKeyUsageTracker,
    Hasher: stardust_common::hash::Hasher,
{
    pub fn new(
        database: Database,
        apikey_repo: Arc<ApiKeyRepo>,
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

impl<Database, ApiKeyRepo, Tracker, Hasher> ApiKeyService
    for ApikeyServiceImpl<Database, ApiKeyRepo, Tracker, Hasher>
where
    Database: stardust_db::database::Database + 'static,
    ApiKeyRepo: for<'h> crate::repository::ApiKeyRepository<Handle<'h> = Database::Handle<'h>>,
    Tracker: crate::service::ApiKeyUsageTracker,
    Hasher: stardust_common::hash::Hasher,
{
    async fn create_apikey(
        &self,
        command: &command::CreateApiKeyCommand,
    ) -> stardust_common::Result<With<String, entity::ApiKeyEntity>> {
        let key = stardust_common::utils::generate_uid();
        let key_hash = self.hasher.hash(&key)?;
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
        let entity = self.apikey_repo.create_apikey(&mut self.database.handle(), &entity).await?;
        stardust_core::audit(entity.user_id, "apikey.created", serde_json::json!(entity));
        Ok(With {
            inner: key,
            related: entity,
        })
    }

    async fn find_user(
        &self,
        query: &query::FindApiKeyUserQuery<'_>,
    ) -> stardust_common::Result<Option<entity::ApiKeyUserAggregate>> {
        let result = self.apikey_repo.find_user(&mut self.database.handle(), &query).await;
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
    ) -> stardust_common::Result<Vec<entity::ApiKeyEntity>> {
        return self.apikey_repo.find_apikeys(&mut self.database.handle(), &query).await;
    }

    async fn deactivate_apikey(
        &self,
        command: &command::DeactivateApiKeyCommand,
    ) -> stardust_common::Result<entity::ApiKeyEntity> {
        let result =
            self.apikey_repo.get_apikey(&mut self.database.handle(), command.apikey_id).await?;
        if result.is_none() {
            return Err(stardust_common::Error::NotFound);
        }
        let mut key = result.unwrap();
        if key.user_id != command.request_user_id {
            return Err(stardust_common::Error::Forbidden);
        }
        if key.deactivated_at.is_some() {
            return Err(stardust_common::Error::Forbidden);
        }
        key.deactivated_at = Some(chrono::Utc::now());
        let key = self.apikey_repo.save_apikey(&mut self.database.handle(), &key).await?;
        stardust_core::audit(key.user_id, "apikey.deactivated", serde_json::json!(key));
        Ok(key)
    }
}
