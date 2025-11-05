pub fn map_err(err: sqlx::Error) -> stardust_common::Error {
    stardust_common::Error::DatabaseError(err.into())
}
