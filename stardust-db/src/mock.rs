use std::{any::Any, collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Database {
    pool: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
}

pub enum Handle<'a> {
    Pool(&'a Database),
    Transaction(&'a Database),
}

impl Database {
    pub async fn open(
        _config: &stardust_common::config::DatabaseConfig,
    ) -> stardust_common::Result<Self> {
        Ok(Self {
            pool: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn pool(&self) -> Handle<'_> {
        Handle::Pool(&self)
    }

    pub async fn transaction(&self) -> stardust_common::Result<Handle<'_>> {
        Ok(Handle::Transaction(&self))
    }
}
