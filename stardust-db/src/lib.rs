pub mod db;
pub mod error;
pub mod handle;

// pub trait Context<'c>: sqlx::Executor<'c, Database = DefaultDriver> {}
// impl<'c, T> Context<'c> for T where T: sqlx::Executor<'c, Database = DefaultDriver> {}

#[cfg(test)]
mod tests {}
