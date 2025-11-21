mod error;
pub use error::*;
pub mod database;
pub mod internal;

// pub trait Context<'c>: sqlx::Executor<'c, Database = DefaultDriver> {}
// impl<'c, T> Context<'c> for T where T: sqlx::Executor<'c, Database = DefaultDriver> {}

#[cfg(test)]
mod tests {}
