pub fn map_err(err: sqlx::Error) -> stardust_common::Error {
    match err {
        sqlx::Error::RowNotFound => stardust_common::Error::NotFound,
        sqlx::Error::PoolTimedOut => stardust_common::Error::Timeout,
        sqlx::Error::Io(io_err) => stardust_common::Error::IoError(io_err),
        sqlx::Error::Database(ref db_err) => match db_err.code().as_deref() {
            Some("23505") => stardust_common::Error::AlreadyExists,
            Some("23000") if db_err.message().contains("Duplicate entry") => {
                stardust_common::Error::AlreadyExists
            }
            _ => stardust_common::Error::DatabaseError(err.into()),
        },
        _ => stardust_common::Error::DatabaseError(err.into()),
    }
}
