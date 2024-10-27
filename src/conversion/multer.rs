use crate::Error;

impl From<multer::Error> for Error {
    fn from(value: multer::Error) -> Self {
        Error::invalid_request_message(format!("multipart/form-data error: {}", value.to_string()))
    }
}