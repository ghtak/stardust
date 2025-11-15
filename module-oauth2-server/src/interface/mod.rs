use std::sync::Arc;

use crate::service::OAuth2ClientService;

pub mod container;
pub mod dto;
pub mod http;

pub trait ServiceProvider: Sync + Send {
    type OAuth2ClientService: OAuth2ClientService;

    fn oauth2_client_service(&self) -> Arc<Self::OAuth2ClientService>;
}
