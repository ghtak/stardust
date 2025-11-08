use crate::handle::{Handle, DefaultDriver};

#[derive(Debug,Clone)]
pub struct Database {
    pool: sqlx::Pool<DefaultDriver>,
}

impl Database {
    pub async fn open(
        config: &stardust_common::config::DatabaseConfig,
    ) -> stardust_common::Result<Self> {
        let pool = sqlx::pool::PoolOptions::<DefaultDriver>::new()
            .max_connections(config.pool_size)
            .connect(&config.url)
            .await
            .map_err(crate::into_error)?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> Handle<'_> {
        Handle::Pool(self.pool.clone())
    }

    pub async fn transaction(&self) -> stardust_common::Result<Handle<'_>> {
        let tx = self.pool.begin().await.map_err(crate::into_error)?;
        Ok(Handle::Transaction(tx))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    async fn db_connect() -> Result<Database, sqlx::Error> {
        let config = stardust_common::config::Config::test_config();
        let db = Database::open(&config.database).await.unwrap();
        Ok(db)
    }

    #[tokio::test]
    async fn test_db() {
        let db = db_connect().await.unwrap();
        let mut ctx = db.pool();
        let row: (i32,) =
            sqlx::query_as("SELECT 1").fetch_one(ctx.executor()).await.unwrap();
        assert_eq!(row.0, 1);
    }
}
