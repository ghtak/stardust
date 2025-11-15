use std::sync::Arc;

use crate::service::ApiKeyService;

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
    async fn create_apikey(&self) -> stardust_common::Result<()> {
        unimplemented!()
    }
}
