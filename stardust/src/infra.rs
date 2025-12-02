pub mod migration {
    use crate::database::Database as _;

    #[derive(
        Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow,
    )]
    pub struct MigrationEntity {
        pub name: String,
        pub version: i32,
        pub description: String,
        pub updated_at: chrono::DateTime<chrono::Utc>,
    }

    impl Default for MigrationEntity {
        fn default() -> Self {
            Self {
                name: "".into(),
                version: 0,
                description: "".into(),
                updated_at: chrono::Utc::now(),
            }
        }
    }

    pub type Database = crate::database::internal::postgres::Database;
    pub type Handle<'h> = <Database as crate::database::Database>::Handle<'h>;

    pub async fn init(database: Database) -> crate::Result<()> {
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS stardust_migration (
            name VARCHAR(255) NOT NULL,
            version INT NOT NULL,
            description VARCHAR(255) NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL
            );"#,
        )
        .execute(database.handle().executor())
        .await
        .map_err(crate::database::internal::into_error)?;
        Ok(())
    }

    pub async fn get_latest(
        handle: &mut Handle<'_>,
        name: &str,
    ) -> crate::Result<Option<MigrationEntity>> {
        let row =
            sqlx::QueryBuilder::new("SELECT * FROM stardust_migration WHERE ")
                .push("name = ")
                .push_bind(name)
                .push(" ORDER BY version DESC LIMIT 1")
                .build_query_as::<MigrationEntity>()
                .fetch_optional(handle.executor())
                .await
                .map_err(crate::database::internal::into_error)?;
        Ok(row)
    }

    pub async fn save(
        handle: &mut Handle<'_>,
        entity: &MigrationEntity,
    ) -> crate::Result<MigrationEntity> {
        let mut builder = sqlx::QueryBuilder::new(
            "INSERT INTO stardust_migration (name, version, description, updated_at) ",
        );
        builder.push_values(std::iter::once(entity), |mut values, model| {
            values.push_bind(&model.name);
            values.push_bind(model.version);
            values.push_bind(&model.description);
            values.push_bind(model.updated_at);
        });
        builder.push(" RETURNING name, version, description, updated_at");
        let row = builder
            .build_query_as::<MigrationEntity>()
            .fetch_one(handle.executor())
            .await
            .map_err(crate::database::internal::into_error)?;
        Ok(row)
    }
}
