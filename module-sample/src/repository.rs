#[async_trait::async_trait]
pub trait MigrationRepository: Sync + Send {
    type Handle<'h>;

    async fn create_sample_store(
        &self,
        handle: &mut Self::Handle<'_>,
    ) -> stardust::Result<()>;
}
