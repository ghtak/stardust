pub struct PostgresMigrationRepository{}

impl PostgresMigrationRepository{
    pub fn new() -> Self{
        Self{}
    }
}

#[async_trait::async_trait]
impl crate::repository::MigrationRepository for PostgresMigrationRepository{
    type Handle<'h> = stardust_db::internal::postgres::Handle<'h>;

    async fn create_sample_store(
        &self,
        handle: &mut Self::Handle<'_>,
    ) -> stardust_common::Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sample_store (
                id BIGSERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#,
        )
        .execute(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;
        Ok(())
    }
}