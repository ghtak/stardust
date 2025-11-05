pub mod context;
pub mod db;
pub mod error;

// pub trait Context<'c>: sqlx::Executor<'c, Database = DBDriver> {}
// impl<'c, T> Context<'c> for T where T: sqlx::Executor<'c, Database = DBDriver> {}

#[cfg(test)]
mod tests {}
