pub trait Handle: Sync + Send {
    fn commit(self) -> impl std::future::Future<Output = stardust_common::Result<()>> + Send;
    fn rollback(self) -> impl std::future::Future<Output = stardust_common::Result<()>> + Send;
}

pub trait Database: Sync + Send {
    type Handle<'h>: Handle
    where
        Self: 'h;

    fn handle(&self) -> Self::Handle<'_>;

    fn tx_handle(
        &self,
    ) -> impl std::future::Future<Output = stardust_common::Result<Self::Handle<'_>>> + Send;
}
