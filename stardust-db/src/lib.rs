mod db;
pub use db::*;
mod error;
pub use error::*;
mod handle;
pub use handle::*;
mod with;
pub use with::*;
pub mod mock;

// pub trait Context<'c>: sqlx::Executor<'c, Database = DefaultDriver> {}
// impl<'c, T> Context<'c> for T where T: sqlx::Executor<'c, Database = DefaultDriver> {}

#[cfg(test)]
mod tests {}
