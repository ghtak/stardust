use std::sync::Arc;

pub struct Container<US> {
    user_service: Arc<US>,
}

impl<US> Container<US>
where
    US: module_user::service::UserService,
{
    pub fn new(user_service: Arc<US>) -> Self {
        Self { user_service }
    }
}

impl<US> module_user::interface::UserServiceProvider for Container<US>
where
    US: module_user::service::UserService,
{
    type UserService = US;

    fn user_service(&self) -> Arc<Self::UserService> {
        self.user_service.clone()
    }
}
