use std::convert::Infallible;
use std::fmt::{Debug, Display, Formatter};

pub struct Error {
    pub message: String,
}

impl Error {

    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }

    pub fn prefix(&self, prefix: impl AsRef<str>) -> Self {
        Self::new(format!("{}: {}", prefix.as_ref(), self.message))
    }
}

impl Display for Error {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Debug for Error {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error { }

impl From<std::io::Error> for Error {

    fn from(value: std::io::Error) -> Self {
        Self::new(value.to_string())
    }
}

impl From<Infallible> for Error {
    fn from(_value: Infallible) -> Self {
        Self::new("infallible")
    }
}