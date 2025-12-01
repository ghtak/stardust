use std::sync::Arc;

pub mod command;
pub mod entity;
pub mod infra;
pub mod interface;
pub mod internal;
pub mod query;
pub mod repository;
pub mod service;

pub trait Container: module_user::Container + Sync + Send {
    type OAuth2ClientService: crate::service::OAuth2ClientService;
    type OAuth2AuthorizationService: crate::service::OAuth2AuthorizationService;

    fn oauth2_client_service(&self) -> Arc<Self::OAuth2ClientService>;
    fn oauth2_authorization_service(&self) -> Arc<Self::OAuth2AuthorizationService>;
}
