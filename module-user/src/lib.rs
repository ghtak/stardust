use std::sync::Arc;

pub mod command;
pub mod entity;
pub mod infra;
pub mod interface;
pub mod internal;
pub mod query;
pub mod repository;
pub mod service;

pub trait Container: Sync + Send {
    type UserService: service::UserService;
    type ApiKeyService: service::ApiKeyService;

    fn user_service(&self) -> Arc<Self::UserService>;
    fn apikey_service(&self) -> Arc<Self::ApiKeyService>;
}
