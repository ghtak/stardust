use std::sync::Arc;

use crate::service::{OAuth2AuthorizationService, OAuth2ClientService};

pub mod container;
pub mod dto;
pub mod extract;
pub mod http;

pub trait ServiceProvider: module_user::interface::ServiceProvider + Sync + Send {
    type OAuth2ClientService: OAuth2ClientService;
    type OAuth2AuthorizationService: OAuth2AuthorizationService;

    fn oauth2_client_service(&self) -> Arc<Self::OAuth2ClientService>;
    fn oauth2_authorization_service(&self) -> Arc<Self::OAuth2AuthorizationService>;
}
