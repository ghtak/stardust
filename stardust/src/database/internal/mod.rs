pub mod mock;
pub mod postgres;
pub mod sqlite;

use anyhow::anyhow;

pub fn into_error(err: sqlx::Error) -> crate::Error {
    match err {
        sqlx::Error::RowNotFound => crate::Error::NotFound("".into()),
        sqlx::Error::PoolTimedOut => crate::Error::Timeout,
        sqlx::Error::Io(io_err) => {
            crate::Error::Unhandled(anyhow!("sqlx io error: {:?}", io_err))
        }
        sqlx::Error::Database(ref db_err) => match db_err.code().as_deref() {
            Some("23505") => crate::Error::AlreadyExists("".into()),
            Some("23000") if db_err.message().contains("Duplicate entry") => {
                crate::Error::AlreadyExists("".into())
            }
            _ => crate::Error::Database(anyhow!("database error: {:?}", err)),
        },
        _ => crate::Error::Database(anyhow!("database error: {:?}", err)),
    }
}
