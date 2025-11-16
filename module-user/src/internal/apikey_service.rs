use std::sync::Arc;

use stardust_common::With;

use crate::{command, entity, infra::apikey_repo, query, service::ApiKeyService};

pub struct ApikeyServiceImpl<Hasher> {
    database: stardust_db::Database,
    hasher: Arc<Hasher>,
}

impl<Hasher> ApikeyServiceImpl<Hasher>
where
    Hasher: stardust_common::hash::Hasher,
{
    pub fn new(database: stardust_db::Database, hasher: Arc<Hasher>) -> Self {
        Self { database, hasher }
    }
}

impl<Hasher> ApiKeyService for ApikeyServiceImpl<Hasher>
where
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
        let entity = apikey_repo::create_apikey(&mut self.database.pool(), &entity).await?;
        stardust_core::audit(entity.user_id, "apikey.created", serde_json::json!(entity));
        Ok(With {
            inner: key,
            related: entity,
        })
    }

    async fn find_user(
        &self,
        query: &query::FindApiKeyUserQuery<'_>,
    ) -> stardust_common::Result<Option<entity::UserAggregate>> {
        return apikey_repo::find_user(&mut self.database.pool(), &query).await;
    }

    async fn find_apikeys(
        &self,
        query: &query::FindApiKeysQuery,
    ) -> stardust_common::Result<Vec<entity::ApiKeyEntity>> {
        return apikey_repo::find_apikeys(&mut self.database.pool(), &query).await;
    }

    async fn deactivate_apikey(
        &self,
        command: &command::DeactivateApiKeyCommand,
    ) -> stardust_common::Result<entity::ApiKeyEntity> {
        let result = apikey_repo::get_apikey(&mut self.database.pool(), command.apikey_id).await?;
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
        let key = apikey_repo::save_apikey(&mut self.database.pool(), &key).await?;
        stardust_core::audit(key.user_id, "apikey.deactivated", serde_json::json!(key));
        Ok(key)
    }
}
