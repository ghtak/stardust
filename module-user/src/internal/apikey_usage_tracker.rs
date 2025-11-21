use std::sync::Arc;

use stardust_db::{database::Database, internal::postgres};

use crate::{infra::apikey_repo, service::ApiKeyUsageTracker};

pub struct ImmediateUsageTracker {
    database: postgres::Database,
}

impl ImmediateUsageTracker {
    pub fn new(database: postgres::Database) -> Arc<Self> {
        Arc::new(Self { database })
    }
}

#[async_trait::async_trait]
impl ApiKeyUsageTracker for ImmediateUsageTracker {
    async fn track_usage(&self, apikey_id: i64) -> stardust_common::Result<()> {
        if let Err(e) = apikey_repo::update_last_used_at(
            &mut self.database.handle(),
            apikey_id,
            chrono::Utc::now(),
        )
        .await
        {
            tracing::warn!(
                "Failed to update last_used_at for apikey {}: {}",
                apikey_id,
                e
            );
            Err(e)
        } else {
            Ok(())
        }
    }
}
