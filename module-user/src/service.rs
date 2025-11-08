#[async_trait::async_trait]
pub trait UserService: Sync + Send {
    async fn hello(&self) -> String;
}
