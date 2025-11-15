use std::sync::Arc;

use stardust_common::With;

use crate::{command, entity, infra::apikey_repo, service::ApiKeyService};

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
        Ok(With {
            inner: key,
            related: entity,
        })
    }

    async fn find_user(
            &self,
            command: &command::FindApiKeyUserCommand,
        ) -> stardust_common::Result<Option<entity::UserAggregate>> {
        return apikey_repo::find_user(&mut self.database.pool(), command).await;
    }
}
