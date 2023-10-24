use super::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub trait ResultExt<T> {

    fn err_prefix(self, prefix: impl AsRef<str>) -> Result<T>;
}

impl<T> ResultExt<T> for std::result::Result<T, Error> {

    fn err_prefix(self, prefix: impl AsRef<str>) -> Self {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.prefix(prefix)),
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