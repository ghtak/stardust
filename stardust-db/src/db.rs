use crate::context::{DBContext, DBDriver};

pub struct Database {
    pool: sqlx::Pool<DBDriver>,
}

impl Database {
    pub async fn open(
        config: &stardust_common::config::DatabaseConfig,
    ) -> stardust_common::Result<Self> {
        let pool = sqlx::pool::PoolOptions::<DBDriver>::new()
            .max_connections(config.pool_size)
            .connect(&config.url)
            .await
            .map_err(crate::error::map_err)?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> DBContext<'_> {
        DBContext::Pool(self.pool.clone())
    }

    pub async fn transaction(&self) -> stardust_common::Result<DBContext<'_>> {
        let tx = self.pool.begin().await.map_err(crate::error::map_err)?;
        Ok(DBContext::Transaction(tx))
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
