use futures_core::{future::BoxFuture, stream::BoxStream};

pub type DBDriver = sqlx::Postgres;
//pub type DBDriver = sqlx::Sqlite;

#[derive(Debug)]
pub enum DBContext<'c> {
    Pool(sqlx::Pool<DBDriver>),
    Transaction(sqlx::Transaction<'c, DBDriver>),
}

#[derive(Debug)]
pub struct DBExecutor<'h, 'c> {
    pub handle: &'h mut DBContext<'c>,
}

impl<'c> DBContext<'c> {
    pub fn executor(&mut self) -> DBExecutor<'_, 'c> {
        DBExecutor { handle: self }
    }

    pub async fn commit(self) -> Result<(), sqlx::Error> {
        match self {
            DBContext::Transaction(tx) => tx.commit().await,
            _ => Ok(()),
        }
    }

    pub async fn rollback(self) -> Result<(), sqlx::Error> {
        match self {
            DBContext::Transaction(tx) => tx.rollback().await,
            _ => Ok(()),
        }
    }
}

impl<'h, 'c> sqlx::Executor<'h> for DBExecutor<'h, 'c> {
    type Database = DBDriver;

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
            DBContext::Pool(pool) => pool.fetch_many(query),
            DBContext::Transaction(tx) => tx.fetch_many(query),
        }
    }

    fn fetch_optional<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxFuture<
        'e,
        Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>,
    >
    where
        'h: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database>,
    {
        match self.handle {
            DBContext::Pool(pool) => pool.fetch_optional(query),
            DBContext::Transaction(tx) => tx.fetch_optional(query),
        }
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
    ) -> BoxFuture<
        'e,
        Result<<Self::Database as sqlx::Database>::Statement<'q>, sqlx::Error>,
    >
    where
        'h: 'e,
    {
        match self.handle {
            DBContext::Pool(pool) => pool.prepare_with(sql, parameters),
            DBContext::Transaction(tx) => tx.prepare_with(sql, parameters),
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
            DBContext::Pool(pool) => pool.describe(sql),
            DBContext::Transaction(tx) => tx.describe(sql),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use super::*;

    async fn accept_handle(
        handle: &mut DBContext<'_>,
    ) -> Result<i32, sqlx::Error> {
        let row: (i32,) =
            sqlx::query_as("SELECT 1").fetch_one(handle.executor()).await?;
        let row2: (i32,) =
            sqlx::query_as("SELECT 2").fetch_one(handle.executor()).await?;
        Ok(row.0 + row2.0)
    }

    async fn db_connect() -> Result<sqlx::Pool<DBDriver>, sqlx::Error> {
        let config = stardust_common::config::Config::test_config();
        let pool = sqlx::pool::PoolOptions::<DBDriver>::new()
            .max_connections(1)
            .connect(&config.database.url)
            .await?;
        Ok(pool)
    }

    #[tokio::test]
    async fn test_handle() {
        let pool = db_connect().await.unwrap();

        let mut ctx = DBContext::Pool(pool.clone());
        let result = accept_handle(&mut ctx).await.unwrap();
        assert_eq!(result, 3);
        ctx.rollback().await.unwrap();

        let tx = pool.begin().await.unwrap();
        let mut ctx = DBContext::Transaction(tx);
        let result = accept_handle(&mut ctx).await.unwrap();
        assert_eq!(result, 3);
        ctx.commit().await.unwrap();
    }

    #[async_trait::async_trait]
    trait Selector {
        async fn select(
            &self,
            handle: &mut DBContext<'_>,
        ) -> Result<i32, sqlx::Error>;
    }

    pub struct SelectorImpl;

    #[async_trait::async_trait]
    impl Selector for SelectorImpl {
        async fn select(
            &self,
            handle: &mut DBContext<'_>,
        ) -> Result<i32, sqlx::Error> {
            let row: (i32,) =
                sqlx::query_as("SELECT 1").fetch_one(handle.executor()).await?;
            let row2: (i32,) =
                sqlx::query_as("SELECT 2").fetch_one(handle.executor()).await?;
            Ok(row.0 + row2.0)
        }
    }

    #[tokio::test]
    async fn test_dyn_selector() {
        let pool = db_connect().await.unwrap();

        let selector: Arc<dyn Selector> = Arc::new(SelectorImpl);

        let mut ctx = DBContext::Pool(pool.clone());
        let result = selector.select(&mut ctx).await.unwrap();
        assert_eq!(result, 3);
        ctx.rollback().await.unwrap();

        let tx = pool.begin().await.unwrap();
        let mut ctx = DBContext::Transaction(tx);
        let result = selector.select(&mut ctx).await.unwrap();
        assert_eq!(result, 3);
        ctx.commit().await.unwrap();
    }
}
