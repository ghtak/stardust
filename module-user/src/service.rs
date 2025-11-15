use crate::{
    command::{LoginCommand, SignupCommand},
    entity,
};

#[async_trait::async_trait]
pub trait UserService: Sync + Send {
    async fn hello(&self) -> String;

    async fn signup(
        &self,
        command: &SignupCommand,
    ) -> stardust_common::Result<entity::UserAggregate>;

    async fn login(&self, command: &LoginCommand)
    -> stardust_common::Result<entity::UserAggregate>;
}

pub trait ApiKeyService: Sync + Send {
    fn create_apikey(&self) -> impl Future<Output = stardust_common::Result<()>> + Send;
}
