use std::borrow::Cow;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid parameter: {0}")]
    InvalidParameter(Cow<'static, str>),

    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error),

    #[error("Not found")]
    NotFound,

    #[error("Timeout")]
    Timeout,

    #[error("Already exists")]
    AlreadyExists,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("duplicate entry {0:?}")]
    Duplicate(Option<String>),

    #[error("Io Error {0}")]
    IoError(#[from] std::io::Error),

    #[error("Load error: {0}")]
    LoadError(anyhow::Error),

    #[error("Parse error: {0}")]
    ParseError(anyhow::Error),

    #[error("Database error: {0}")]
    DatabaseError(anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cow_borrowed() {
        let error = Error::InvalidParameter("foo".into());
        //let borrowed = matches!(error, Error::InvalidParameter(Cow::Borrowed(_)));
        let borrowed = match error {
            Error::InvalidParameter(Cow::Borrowed(_)) => true,
            _ => false,
        };
        assert!(borrowed)
    }

    #[test]
    fn test_anyhow() {
        let err = anyhow::anyhow!("Invalid AccountType: {}", "params");
        println!("{}", err.to_string());
    }
}
