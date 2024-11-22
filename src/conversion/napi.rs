use std::sync::Arc;
use napi::Error;
use crate::error::ErrorSerializable;

fn build_from_error_serializable(value: Error, error_serializable: ErrorSerializable) -> crate::Error {
    crate::Error {
        code: error_serializable.code,
        message: error_serializable.message.to_string(),
        errors: if let Some(object) = error_serializable.errors.as_object() {
            Some(object.iter().map(|(k, v)| (k.to_string(), v.as_str().unwrap().to_string())).collect())
        } else {
            None
        },
        platform_native_object: Some(Arc::new(value)),
    }
}

impl From<Error> for crate::Error {
    fn from(value: Error) -> Self {
        if value.status.as_ref() == "GenericFailure" && (value.reason.starts_with("TeoError: ") || value.reason.starts_with("Error: TeoError: ")) {
            let json_string = if let Some(string) = value.reason.strip_prefix("Error: TeoError: ") {
                string
            } else {
                value.reason.strip_prefix("TeoError: ").unwrap()
            };
            let error_serializable: Result<ErrorSerializable, serde_json::Error> = serde_json::from_str(json_string);
            match error_serializable {
                Ok(error_serializable) => {
                    build_from_error_serializable(value, error_serializable)
                }
                Err(_) => {
                    let mut result = crate::Error::new(value.reason.as_str());
                    result.assign_platform_native_object(value);
                    result
                }
            }
        } else {
            let mut error = crate::Error::new(value.reason.as_str());
            error.assign_platform_native_object(value);
            error
        }
    }
}

impl From<crate::Error> for Error {
    fn from(value: crate::Error) -> Self {
        if let Some(napi_error) = value.platform_native_object::<Error>() {
            // contains one native error, use it
            napi_error.clone()
        } else {
            let message = format!("TeoError: {}", ErrorSerializable::error_string(&value));
            Error::new(napi::Status::GenericFailure, message)
        }
    }
}