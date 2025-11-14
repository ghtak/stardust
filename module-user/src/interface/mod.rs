pub mod dto;
pub mod http;
pub mod user;

use std::sync::Arc;

use crate::service::UserService;

pub trait UserServiceProvider:
    Sync + Send
{
    type UserService: UserService;

    fn user_service(&self) -> Arc<Self::UserService>;
}
