use std::sync::Arc;

use crate::service;

pub struct OAuth2AuthorizationServiceImpl<H> {
    database: stardust_db::Database,
    hasher: Arc<H>,
}

impl<H> OAuth2AuthorizationServiceImpl<H>
where
    H: stardust_common::hash::Hasher,
{
    pub fn new(database: stardust_db::Database, hasher: Arc<H>) -> Self {
        Self { database, hasher }
    }
}


impl<H> service::OAuth2AuthorizationService for OAuth2AuthorizationServiceImpl<H>
where
    H: stardust_common::hash::Hasher,
{
}