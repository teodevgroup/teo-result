use cookie::ParseError;
use crate::Error;

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Error::new(value.to_string())
    }
}
