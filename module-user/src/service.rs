use crate::command::SignupCommand;

#[async_trait::async_trait]
pub trait UserService: Sync + Send {
    async fn hello(&self) -> String;

    async fn signup(&self, command: &SignupCommand) -> stardust_common::Result<()>;
}
