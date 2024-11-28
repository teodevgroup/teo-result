use std::ffi::NulError;
use crate::Error;

impl From<NulError> for Error {
    fn from(value: NulError) -> Self {
        Self::new(format!("{}", value))
    }
}