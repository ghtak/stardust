pub mod migration_repo {

    use crate::migration::DatabaseHandleImpl;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
    pub struct MigrationModel {
        pub name: String,
        pub version: i32,
        pub description: String,
        pub updated_at: chrono::DateTime<chrono::Utc>,
    }

    pub async fn create_table(handle: &mut DatabaseHandleImpl<'_>) -> stardust_common::Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS stardust_migration (
                name VARCHAR(255) NOT NULL,
                version INT NOT NULL,
                description VARCHAR(255) NOT NULL,
                updated_at TIMESTAMPTZ NOT NULL
            );
            "#,
        )
        .execute(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;
        Ok(())
    }

    pub async fn create(
        handle: &mut DatabaseHandleImpl<'_>,
        model: &MigrationModel,
    ) -> stardust_common::Result<MigrationModel> {
        let mut builder = sqlx::QueryBuilder::new(
            "INSERT INTO stardust_migration (name, version, description, updated_at) ",
        );
        builder.push_values(std::iter::once(model), |mut values, model| {
            values.push_bind(&model.name);
            values.push_bind(model.version);
            values.push_bind(&model.description);
            values.push_bind(model.updated_at);
        });
        builder.push(" RETURNING name, version, description, updated_at");
        let row = builder
            .build_query_as::<MigrationModel>()
            .fetch_one(handle.executor())
            .await
            .map_err(stardust_db::into_error)?;
        Ok(row)
    }

    pub async fn get_all(
        handle: &mut DatabaseHandleImpl<'_>,
    ) -> stardust_common::Result<Vec<MigrationModel>> {
        let rows = sqlx::QueryBuilder::new("SELECT * FROM stardust_migration")
            .build_query_as::<MigrationModel>()
            .fetch_all(handle.executor())
            .await
            .map_err(stardust_db::into_error)?;
        Ok(rows)
    }

    pub async fn get_latest(
        handle: &mut DatabaseHandleImpl<'_>,
        name: &str,
    ) -> stardust_common::Result<Option<MigrationModel>> {
        let row = sqlx::QueryBuilder::new("SELECT * FROM stardust_migration WHERE ")
            .push("name = ")
            .push_bind(name)
            .push(" ORDER BY version DESC LIMIT 1")
            .build_query_as::<MigrationModel>()
            .fetch_optional(handle.executor())
            .await
            .map_err(stardust_db::into_error)?;
        Ok(row)
    }
}
