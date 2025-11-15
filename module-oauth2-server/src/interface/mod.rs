pub mod dto;
pub mod http;
pub mod container;

pub trait ServiceProvider: Sync + Send {}
