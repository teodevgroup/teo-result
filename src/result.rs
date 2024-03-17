use super::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub trait ResultExt<T> {

    fn error_message_prefix(self, prefix: impl Into<String>) -> Result<T>;

    fn error_path_prefix(self, prefix: impl Into<String>) -> Result<T>;
}

impl<T> ResultExt<T> for std::result::Result<T, Error> {

    fn error_message_prefix(self, prefix: impl Into<String>) -> Self {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.message_prefixed(prefix)),
        }
    }

    fn error_path_prefix(self, prefix: impl Into<String>) -> Self {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.message_prefixed(prefix)),
        }
    }
}

pub trait IntoTeoResult<T> {
    fn into_teo_result(self) -> Result<T>;
}

impl<T> IntoTeoResult<T> for std::io::Result<T> {
    fn into_teo_result(self) -> Result<T> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(Error::new(e.to_string())),
        }
    }
}