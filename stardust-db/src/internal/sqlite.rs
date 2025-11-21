use futures_core::{future::BoxFuture, stream::BoxStream};

pub type DefaultDriver = sqlx::Sqlite;

#[derive(Debug)]
pub enum Handle<'c> {
    Pool(sqlx::Pool<DefaultDriver>),
    Transaction(sqlx::Transaction<'c, DefaultDriver>),
}

pub struct Database {
    pub pool: sqlx::Pool<DefaultDriver>,
}

impl Database {
    pub async fn new(
        config: &stardust_common::config::DatabaseConfig,
    ) -> stardust_common::Result<Self> {
        let pool = sqlx::pool::PoolOptions::<DefaultDriver>::new()
            .max_connections(config.pool_size)
            .connect(&config.url)
            .await
            .map_err(crate::error::into_error)?;
        Ok(Self { pool })
    }
}

impl crate::database::Database for Database {
    type Handle<'h>
        = Handle<'h>
    where
        Self: 'h;

    fn handle(&self) -> Self::Handle<'_> {
        Handle::Pool(self.pool.clone())
    }

    async fn tx_handle(&self) -> stardust_common::Result<Self::Handle<'_>> {
        let tx = self.pool.begin().await.map_err(crate::error::into_error)?;
        Ok(Handle::Transaction(tx))
    }
}

#[derive(Debug)]
pub struct Executor<'h, 'c> {
    pub handle: &'h mut Handle<'c>,
}

impl<'c> Handle<'c> {
    pub fn executor(&mut self) -> Executor<'_, 'c> {
        Executor { handle: self }
    }
}

impl<'c> crate::database::Handle for Handle<'c> {
    async fn commit(self) -> Result<(), stardust_common::Error> {
        match self {
            Handle::Transaction(tx) => tx.commit().await.map_err(crate::into_error),
            _ => Ok(()),
        }
    }

    async fn rollback(self) -> Result<(), stardust_common::Error> {
        match self {
            Handle::Transaction(tx) => tx.rollback().await.map_err(crate::into_error),
            _ => Ok(()),
        }
    }
}

impl<'h, 'c> sqlx::Executor<'h> for Executor<'h, 'c> {
    type Database = DefaultDriver;

    fn fetch_many<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxStream<
        'e,
        Result<
            sqlx::Either<
                <Self::Database as sqlx::Database>::QueryResult,
                <Self::Database as sqlx::Database>::Row,
            >,
            sqlx::Error,
        >,
    >
    where
        'h: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database>,
    {
        match self.handle {
            Handle::Pool(pool) => pool.fetch_many(query),
            Handle::Transaction(tx) => tx.fetch_many(query),
        }
    }

    fn fetch_optional<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxFuture<'e, Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>>
    where
        'h: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database>,
    {
        match self.handle {
            Handle::Pool(pool) => pool.fetch_optional(query),
            Handle::Transaction(tx) => tx.fetch_optional(query),
        }
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
    ) -> BoxFuture<'e, Result<<Self::Database as sqlx::Database>::Statement<'q>, sqlx::Error>>
    where
        'h: 'e,
    {
        match self.handle {
            Handle::Pool(pool) => pool.prepare_with(sql, parameters),
            Handle::Transaction(tx) => tx.prepare_with(sql, parameters),
        }
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'h: 'e,
    {
        match self.handle {
            Handle::Pool(pool) => pool.describe(sql),
            Handle::Transaction(tx) => tx.describe(sql),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use stardust_common::config::DatabaseConfig;

    use crate::database::Database;
    use crate::database::Handle;



    async fn accept_handle(handle: &mut super::Handle<'_>) -> Result<i32, sqlx::Error> {
        let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(handle.executor()).await?;
        let row2: (i32,) = sqlx::query_as("SELECT 2").fetch_one(handle.executor()).await?;
        Ok(row.0 + row2.0)
    }

    async fn db_connect() -> stardust_common::Result<super::Database> {
        let config = DatabaseConfig{
            url: "sqlite::memory:".into(),
            pool_size: 1,
        };
        Ok(super::Database::new(&config).await?)
    }

    #[tokio::test]
    async fn test_handle() {
        let db = db_connect().await.unwrap();

        let mut ctx = db.handle();
        let result = accept_handle(&mut ctx).await.unwrap();
        assert_eq!(result, 3);
        ctx.rollback().await.unwrap();

        let mut ctx = db.tx_handle().await.unwrap();
        let result = accept_handle(&mut ctx).await.unwrap();
        assert_eq!(result, 3);
        ctx.commit().await.unwrap();
    }

    #[async_trait::async_trait]
    trait Selector {
        async fn select(&self, handle: &mut super::Handle<'_>) -> Result<i32, sqlx::Error>;
    }

    pub struct SelectorImpl;

    #[async_trait::async_trait]
    impl Selector for SelectorImpl {
        async fn select(&self, handle: &mut super::Handle<'_>) -> Result<i32, sqlx::Error> {
            let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(handle.executor()).await?;
            let row2: (i32,) = sqlx::query_as("SELECT 2").fetch_one(handle.executor()).await?;
            Ok(row.0 + row2.0)
        }
    }

    #[tokio::test]
    async fn test_dyn_selector() {
        let db = db_connect().await.unwrap();

        let selector: Arc<dyn Selector> = Arc::new(SelectorImpl);

        let mut ctx = db.handle();
        let result = selector.select(&mut ctx).await.unwrap();
        assert_eq!(result, 3);
        ctx.rollback().await.unwrap();

        let mut ctx = db.tx_handle().await.unwrap();
        let result = selector.select(&mut ctx).await.unwrap();
        assert_eq!(result, 3);
        ctx.commit().await.unwrap();
    }
}
