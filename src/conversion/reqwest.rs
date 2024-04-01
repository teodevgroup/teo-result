use crate::Error;

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::new(value.to_string())
    }
}