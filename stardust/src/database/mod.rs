pub mod internal;

pub trait Handle: Sync + Send {
    fn commit(
        self,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send;
    fn rollback(
        self,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send;
}

pub trait Database: Sync + Send {
    type Handle<'h>: Handle
    where
        Self: 'h;

    fn handle(&self) -> Self::Handle<'_>;

    fn tx_handle(
        &self,
    ) -> impl std::future::Future<Output = crate::Result<Self::Handle<'_>>> + Send;
}


// pub trait Context<'c>: sqlx::Executor<'c, Database = DefaultDriver> {}
// impl<'c, T> Context<'c> for T where T: sqlx::Executor<'c, Database = DefaultDriver> {}