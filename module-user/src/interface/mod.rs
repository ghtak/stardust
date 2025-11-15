pub mod dto;
pub mod http;
pub mod user;
pub mod container;

use std::sync::Arc;

use crate::service::{ApiKeyService, UserService};

pub trait ServiceProvider:
    Sync + Send
{
    type UserService: UserService;
    type ApiKeyService: ApiKeyService;

    fn user_service(&self) -> Arc<Self::UserService>;
    fn apikey_service(&self) -> Arc<Self::ApiKeyService>;
}
