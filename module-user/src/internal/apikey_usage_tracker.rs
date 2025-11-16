use std::sync::Arc;

use crate::{infra::apikey_repo, service::ApiKeyUsageTracker};

pub struct ImmediateUsageTracker {
    database: stardust_db::Database,
}

impl ImmediateUsageTracker {
    pub fn new(database: stardust_db::Database) -> Arc<Self> {
        Arc::new(Self { database })
    }
}

#[async_trait::async_trait]
impl ApiKeyUsageTracker for ImmediateUsageTracker {
    async fn track_usage(&self, apikey_id: i64) -> stardust_common::Result<()> {
        if let Err(e) = apikey_repo::update_last_used_at(
            &mut self.database.pool(),
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
