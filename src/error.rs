use std::any::Any;
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use indexmap::{IndexMap, indexmap};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug)]
pub struct Error {
    pub code: u16,
    pub message: String,
    pub errors: Option<IndexMap<String, String>>,
    pub platform_native_object: Option<Arc<dyn Any + Send + Sync>>,
}

#[derive(Serialize)]
pub struct ErrorSerializable<'a> {
    pub code: u16,
    pub message: &'a str,
    pub errors: Value,
}

impl Error {

    pub fn new(message: impl Into<String>) -> Self {
        Self {
            code: 500,
            message: message.into(),
            errors: None,
            platform_native_object: None,
        }
    }

    pub fn new_with_code(message: impl Into<String>, code: u16) -> Self {
        Self {
            code,
            message: message.into(),
            errors: None,
            platform_native_object: None,
        }
    }

    pub fn new_with_code_errors(message: impl Into<String>, code: u16, errors: IndexMap<String, String>) -> Self {
        Self {
            code,
            message: message.into(),
            errors: Some(errors),
            platform_native_object: None,
        }
    }

    pub fn new_pathed(message: impl Into<String>, code: u16, key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            errors: Some(indexmap! { key.into() => value.into() }),
            platform_native_object: None,
        }
    }

    pub fn message_prefixed(&self, prefix: impl AsRef<str>) -> Self {
        Self {
            code: self.code,
            message: if self.errors.is_some() {
                self.message.clone()
            } else {
                format!("{}: {}", prefix.as_ref(), self.message())
            },
            errors: if let Some(errors) = self.errors.as_ref() {
                Some(errors.iter().map(|(k, v)| (k.clone(), format!("{}: {}", prefix.as_ref(), v))).collect())
            } else {
                None
            },
            platform_native_object: self.platform_native_object.clone(),
        }
    }

    pub fn path_prefixed(&self, prefix: impl AsRef<str>) -> Self {
        Self {
            code: self.code,
            message: self.message.clone(),
            errors: if let Some(errors) = self.errors.as_ref() {
                Some(errors.iter().map(|(k, v)| (format!("{}.{}", prefix.as_ref(), k), v.clone())).collect())
            } else {
                None
            },
            platform_native_object: self.platform_native_object.clone(),
        }
    }

    pub fn map_path<F>(&self, mapper: F) -> Self where F: Fn(&str) -> String {
        Self {
            code: self.code,
            message: self.message.clone(),
            errors: if let Some(errors) = self.errors.as_ref() {
                Some(errors.iter().map(|(k, v)| (mapper(k.as_str()), v.clone())).collect())
            } else {
                None
            },
            platform_native_object: self.platform_native_object.clone(),
        }
    }

    pub fn code(&self) -> u16 {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub fn errors(&self) -> Option<&IndexMap<String, String>> {
        self.errors.as_ref()
    }

    pub fn assign_platform_native_object<T: 'static + Send + Sync>(&mut self, val: T) {
        self.platform_native_object = Some(Arc::new(val));
    }

    pub fn platform_native_object<T: 'static + Send>(&self) -> Option<&T> {
        self.platform_native_object.as_ref().map(|boxed| boxed.downcast_ref()).flatten()
    }

    pub fn inferred_title(&self) -> Cow<'static, str> {
        match self.code {
            100 => Cow::Borrowed("Continue"),
            101 => Cow::Borrowed("SwitchingProtocols"),
            102 => Cow::Borrowed("Processing"),
            103 => Cow::Borrowed("EarlyHints"),
            200 => Cow::Borrowed("OK"),
            201 => Cow::Borrowed("Created"),
            202 => Cow::Borrowed("Accepted"),
            203 => Cow::Borrowed("NonAuthoritativeInformation"),
            204 => Cow::Borrowed("NoContent"),
            205 => Cow::Borrowed("ResetContent"),
            206 => Cow::Borrowed("PartialContent"),
            207 => Cow::Borrowed("MultiStatus"),
            208 => Cow::Borrowed("AlreadyReported"),
            226 => Cow::Borrowed("IMUsed"),
            300 => Cow::Borrowed("MultipleChoices"),
            301 => Cow::Borrowed("MovedPermanently"),
            302 => Cow::Borrowed("Found"),
            303 => Cow::Borrowed("SeeOther"),
            304 => Cow::Borrowed("NotModified"),
            307 => Cow::Borrowed("TemporaryRedirect"),
            308 => Cow::Borrowed("PermanentRedirect"),
            400 => Cow::Borrowed("BadRequest"),
            401 => Cow::Borrowed("Unauthorized"),
            402 => Cow::Borrowed("PaymentRequired"),
            403 => Cow::Borrowed("Forbidden"),
            404 => Cow::Borrowed("NotFound"),
            405 => Cow::Borrowed("MethodNotAllowed"),
            406 => Cow::Borrowed("NotAcceptable"),
            407 => Cow::Borrowed("ProxyAuthenticationRequired"),
            408 => Cow::Borrowed("RequestTimeout"),
            409 => Cow::Borrowed("Conflict"),
            410 => Cow::Borrowed("Gone"),
            411 => Cow::Borrowed("LengthRequired"),
            412 => Cow::Borrowed("PreconditionFailed"),
            413 => Cow::Borrowed("PayloadTooLarge"),
            414 => Cow::Borrowed("URITooLong"),
            415 => Cow::Borrowed("UnsupportedMediaType"),
            416 => Cow::Borrowed("RangeNotSatisfiable"),
            417 => Cow::Borrowed("ExpectationFailed"),
            418 => Cow::Borrowed("ImATeapot"),
            421 => Cow::Borrowed("MisdirectedRequest"),
            422 => Cow::Borrowed("UnprocessableContent"),
            423 => Cow::Borrowed("Locked"),
            424 => Cow::Borrowed("FailedDependency"),
            425 => Cow::Borrowed("TooEarly"),
            426 => Cow::Borrowed("UpgradeRequired"),
            428 => Cow::Borrowed("PreconditionRequired"),
            429 => Cow::Borrowed("TooManyRequests"),
            431 => Cow::Borrowed("RequestHeaderFieldsTooLarge"),
            451 => Cow::Borrowed("UnavailableForLegalReasons"),
            500 => Cow::Borrowed("InternalServerError"),
            501 => Cow::Borrowed("NotImplemented"),
            502 => Cow::Borrowed("BadGateway"),
            503 => Cow::Borrowed("ServiceUnavailable"),
            504 => Cow::Borrowed("GatewayTimeout"),
            505 => Cow::Borrowed("HTTPVersionNotSupported"),
            506 => Cow::Borrowed("VariantAlsoNegotiates"),
            507 => Cow::Borrowed("InsufficientStorage"),
            508 => Cow::Borrowed("LoopDetected"),
            510 => Cow::Borrowed("NotExtended"),
            511 => Cow::Borrowed("NetworkAuthenticationRequired"),
            _ => Cow::Owned(format!("ServerError({})", self.code)),
        }
    }

    pub fn not_found() -> Self {
        Self {
            code: 404,
            message: "not found".to_owned(),
            errors: None,
            platform_native_object: None,
        }
    }

    pub fn invalid_request() -> Self {
        Self {
            code: 400,
            message: "value is invalid".to_owned(),
            errors: None,
            platform_native_object: None,
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            code: 500,
            message: "internal server error".to_owned(),
            errors: None,
            platform_native_object: None,
        }
    }

    pub fn unauthorized() -> Self {
        Self {
            code: 401,
            message: "unauthorized".to_owned(),
            errors: None,
            platform_native_object: None,
        }
    }
}

impl Display for Error {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let error_serializable = ErrorSerializable {
            code: self.code,
            message: self.message(),
            errors: if let Some(errors) = self.errors() {
                Value::Object(errors.iter().map(|(k, v)| (k.to_string(), Value::String(v.to_string()))).collect())
            } else {
                Value::Null
            },
        };
        let serialized = serde_json::to_string(&error_serializable).unwrap();
        f.write_str(&format!("teo_result::Error: {}", serialized))
    }
}

impl std::error::Error for Error { }
