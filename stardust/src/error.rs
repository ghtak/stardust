use std::borrow::Cow;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // [system level error]
    // anyhow::Context를 사용하여 "Config load failed" 등의 메시지를 붙입니다.
    //#[error("internal error: {0:?}")]
    #[error("internal error: {0:#?}")]
    Unhandled(#[from] anyhow::Error),

    #[error("database error: {0:#?}")]
    Database(anyhow::Error),

    // [client level error]
    #[error("invalid parameter: {0}")]
    InvalidParameter(Cow<'static, str>),

    // [business level error]
    #[error("illegal state: {0}")]
    IllegalState(Cow<'static, str>),

    #[error("already exists: {0}")]
    AlreadyExists(Cow<'static, str>),

    #[error("not found: {0}")]
    NotFound(Cow<'static, str>),

    #[error("timeout")]
    Timeout,

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cow_borrowed() {
        let error = Error::InvalidParameter("foo".into());
        match error {
            Error::InvalidParameter(msg) => {
                assert_eq!(msg, "foo");
                assert!(matches!(msg, Cow::Borrowed(_)));
            }
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn test_cow_owned() {
        let owned_string = String::from("bar");
        let error = Error::InvalidParameter(Cow::Owned(owned_string.clone()));
        match error {
            Error::InvalidParameter(msg) => {
                assert_eq!(msg, owned_string);
                assert!(matches!(msg, Cow::Owned(_)));
            }
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn test_anyhow_context() {
        let base_error =
            std::io::Error::new(std::io::ErrorKind::Other, "base io error");
        let anyhow_error = anyhow::Error::new(base_error)
            .context("additional context")
            .context("one more");
        let error = Error::Unhandled(anyhow_error);
        print!("{}\n", error);
    }
}
