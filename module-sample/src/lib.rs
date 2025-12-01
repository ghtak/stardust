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
    type SampleService: crate::service::SampleService;

    fn sample_service(&self) -> Arc<Self::SampleService>;
}
