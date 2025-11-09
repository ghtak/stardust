use std::sync::Arc;

use crate::service::UserService;

pub mod http;

pub trait UserServiceProvider: Sync + Send {
    type UserService: UserService;

    fn user_service(&self) -> Arc<Self::UserService>;
}
