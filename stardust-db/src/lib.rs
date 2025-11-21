mod db;
pub use db::*;
mod error;
pub use error::*;
mod handle;
pub use handle::*;
mod with;
pub use with::*;
pub mod mock;
pub mod database;
pub mod internal;

// pub trait Context<'c>: sqlx::Executor<'c, Database = DefaultDriver> {}
// impl<'c, T> Context<'c> for T where T: sqlx::Executor<'c, Database = DefaultDriver> {}

#[cfg(test)]
mod tests {
    // pub trait DB {
    //     type Handle<'h>
    //     where
    //         Self: 'h;

    //     fn pool(&self) -> Self::Handle<'_>;
    //     fn transaction(
    //         &self,
    //     ) -> impl std::future::Future<Output = stardust_common::Result<Self::Handle<'_>>> + Send;
    // }

    // use tokio::sync::Mutex;

    // use crate::{handle};
    // use std::{any::Any, collections::HashMap, marker::PhantomData, sync::Arc};

    // struct PostgresDB {
    //     pool: sqlx::Pool<sqlx::Postgres>,
    // }

    // impl PostgresDB {
    //     pub async fn open(
    //         config: &stardust_common::config::DatabaseConfig,
    //     ) -> stardust_common::Result<Self> {
    //         let pool = sqlx::pool::PoolOptions::<sqlx::Postgres>::new()
    //             .max_connections(config.pool_size)
    //             .connect(&config.url)
    //             .await
    //             .map_err(crate::into_error)?;
    //         Ok(Self { pool })
    //     }
    // }

    // impl DB for PostgresDB {
    //     type Handle<'h>
    //         = handle::Handle<'h>
    //     where
    //         Self: 'h;

    //     fn pool(&self) -> Self::Handle<'_> {
    //         Self::Handle::Pool(self.pool.clone())
    //     }

    //     fn transaction(
    //         &self,
    //     ) -> impl std::future::Future<Output = stardust_common::Result<Self::Handle<'_>>> + Send
    //     {
    //         async {
    //             let tx = self.pool.begin().await.map_err(crate::into_error)?;
    //             Ok(Self::Handle::Transaction(tx))
    //         }
    //     }
    // }

    // pub struct MemoryDB {
    //     pub _pool: Arc<Mutex<HashMap<String, Box<dyn Any + Send>>>>,
    // }

    // impl MemoryDB {
    //     pub async fn open(
    //         _config: &stardust_common::config::DatabaseConfig,
    //     ) -> stardust_common::Result<Self> {
    //         Ok(Self {
    //             _pool: Arc::new(Mutex::new(HashMap::new())),
    //         })
    //     }
    // }

    // pub enum MemoryDBHandle<'a> {
    //     Pool(&'a MemoryDB),
    //     Transaction(&'a MemoryDB),
    // }

    // impl DB for MemoryDB {
    //     type Handle<'h>
    //         = MemoryDBHandle<'h>
    //     where
    //         Self: 'h;

    //     fn pool(&self) -> Self::Handle<'_> {
    //         Self::Handle::Pool(&self)
    //     }

    //     fn transaction(
    //         &self,
    //     ) -> impl std::future::Future<Output = stardust_common::Result<Self::Handle<'_>>> + Send {
    //         async move{
    //             // 실제로는 트랜잭션 상태를 관리하는 로직이 필요하겠지만,
    //             // 여기서는 &self를 그대로 반환하여 컴파일을 통과시킵니다.
    //             Ok(Self::Handle::Transaction(&self))
    //         }
    //     }
    // }

    // trait TestRepo {
    //     type Handle<'h>;

    //     fn test<'h>(
    //         &self,
    //         handle: &mut Self::Handle<'h>,
    //     ) -> impl std::future::Future<Output = stardust_common::Result<()>> + Send;
    // }

    // pub struct TestRepoImpl<DBI: DB> {
    //     marker: PhantomData<DBI>,
    // }

    // impl<DBI> TestRepo for TestRepoImpl<DBI>
    // where
    //     DBI: DB + Sync + 'static,
    // {
    //     type Handle<'h> = DBI::Handle<'h>;

    //     fn test<'c>(
    //         &self,
    //         _handle: &mut Self::Handle<'c>,
    //     ) -> impl std::future::Future<Output = stardust_common::Result<()>> + Send {
    //         async { Ok(()) }
    //     }
    // }

    // #[tokio::test]
    // async fn test_repo() {
    //     let config = stardust_common::config::Config::test_config();
    //     let db = PostgresDB::open(&config.database).await.unwrap();
    //     let mut ctx = db.pool();
    //     let repo = TestRepoImpl::<PostgresDB> {
    //         marker: PhantomData,
    //     };
    //     let result = repo.test(&mut ctx).await;
    //     assert!(result.is_ok());

    //     let memdb = MemoryDB::open(&config.database).await.unwrap();
    //     let mut mem_ctx = memdb.pool();
    //     let mem_repo = TestRepoImpl::<MemoryDB> {
    //         marker: PhantomData,
    //     };
    //     let result = mem_repo.test(&mut mem_ctx).await;
    //     assert!(result.is_ok());
    // }
}
