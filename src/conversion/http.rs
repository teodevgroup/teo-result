use http::header::{InvalidHeaderName, InvalidHeaderValue, ToStrError};
use http::uri::InvalidUri;
use crate::Error;

impl From<ToStrError> for Error {
    fn from(_: ToStrError) -> Self {
        Error::new("Failed to parse header value")
    }
}

impl From<InvalidHeaderName> for Error {
    fn from(_: InvalidHeaderName) -> Self {
        Error::new("Invalid header name")
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(_: InvalidHeaderValue) -> Self {
        Error::new("Invalid header value")
    }
}

impl From<InvalidUri> for Error {
    fn from(_: InvalidUri) -> Self {
        Error::new("Invalid URI")
    }
}