use std::convert::Infallible;
use crate::Error;

impl From<Infallible> for Error {
    fn from(value: Infallible) -> Self {
        Self::new(format!("{}", value))
    }
}
