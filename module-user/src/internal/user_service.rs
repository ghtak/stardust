pub struct UserServiceImpl {
    database: stardust_db::Database,
}

impl UserServiceImpl {
    pub fn new(database: stardust_db::Database) -> Self {
        Self { database }
    }
}

#[async_trait::async_trait]
impl crate::service::UserService for UserServiceImpl {
    async fn hello(&self) -> String {
        "hello".into()
    }
}
