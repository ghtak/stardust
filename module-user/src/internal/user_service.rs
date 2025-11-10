use std::sync::Arc;

use crate::command::SignupCommand;

pub struct UserServiceImpl<Hasher> {
    database: stardust_db::Database,
    hasher: Arc<Hasher>,
}

impl<Hasher> UserServiceImpl<Hasher>
where
    Hasher: stardust_common::hash::Hasher,
{
    pub fn new(database: stardust_db::Database, hasher: Arc<Hasher>) -> Self {
        Self { database, hasher }
    }
}

#[async_trait::async_trait]
impl<Hasher> crate::service::UserService for UserServiceImpl<Hasher>
where
    Hasher: stardust_common::hash::Hasher,
{
    async fn hello(&self) -> String {
        "hello".into()
    }

    async fn signup(
        &self,
        command: &SignupCommand,
    ) -> stardust_common::Result<()> {
        Ok(())
    }
}
