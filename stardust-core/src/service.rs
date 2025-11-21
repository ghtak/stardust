#[async_trait::async_trait]
pub trait MigrationService {
    async fn migrate(&self) -> stardust_common::Result<()>;
}