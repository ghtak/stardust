#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct MigrationModel {
    pub name: String,
    pub version: i32,
    pub description: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Default for MigrationModel {
    fn default() -> Self {
        Self {
            name: "".into(),
            version: 0,
            description: "".into(),
            updated_at: chrono::Utc::now(),
        }
    }
}

#[async_trait::async_trait]
pub trait MigrationRepository: Sync + Send {
    type Handle<'h>;

    async fn create_table(&self, handle: &mut Self::Handle<'_>) -> stardust_common::Result<()>;

    async fn create(
        &self,
        handle: &mut Self::Handle<'_>,
        model: &MigrationModel,
    ) -> stardust_common::Result<MigrationModel>;

    async fn get_all(
        &self,
        handle: &mut Self::Handle<'_>,
    ) -> stardust_common::Result<Vec<MigrationModel>>;

    async fn get_latest(
        &self,
        handle: &mut Self::Handle<'_>,
        name: &str,
    ) -> stardust_common::Result<Option<MigrationModel>>;
}
