use super::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub trait ResultExt<T> {

    fn error_message_prefixed(self, prefix: impl AsRef<str>) -> Result<T>;

    fn error_path_prefixed(self, prefix: impl AsRef<str>) -> Result<T>;

    fn alter_error_code(self, code: u16) -> Result<T>;
}

impl<T> ResultExt<T> for std::result::Result<T, Error> {

    fn error_message_prefixed(self, prefix: impl AsRef<str>) -> Self {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.message_prefixed(prefix)),
        }
    }

    fn error_path_prefixed(self, prefix: impl AsRef<str>) -> Self {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.path_prefixed(prefix)),
        }
    }

    fn alter_error_code(self, code: u16) -> Self {
        match self {
            Ok(t) => Ok(t),
            Err(mut e) => Err({
                e.code = code;
                e
            }),
        }
    }
}
