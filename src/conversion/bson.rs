use bson::oid::Error as OidError;
use crate::Error;

impl From<OidError> for Error {
    fn from(value: OidError) -> Self {
        Self::new(value.to_string())
    }
}
