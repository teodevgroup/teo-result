use crate::Error;

impl From<std::io::Error> for Error {

    fn from(value: std::io::Error) -> Self {
        Self::new(format!("{}", value))
    }
}