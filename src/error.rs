use std::any::Any;
use std::borrow::Cow;
use std::convert::Infallible;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use indexmap::IndexMap;

#[derive(Debug)]
pub struct Error {
    pub code: u16,
    pub message: String,
    pub errors: Option<IndexMap<String, String>>,
    pub platform_native_object: Option<Arc<dyn Any + Send + Sync>>,
}

impl Error {

    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: 500,
            errors: None,
            platform_native_object: None,
        }
    }

    pub fn new_with_code(message: impl Into<String>, code: u16) -> Self {
        Self {
            message: message.into(),
            prefixes: None,
            title: None,
            code: Some(code),
            errors: None,
            platform_native_object: None,
        }
    }

    pub fn new_with_code_title(message: impl Into<String>, code: u16, title: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            prefixes: None,
            title: Some(title.into()),
            code: Some(code),
            errors: None,
            platform_native_object: None,
        }
    }

    pub fn new_with_code_title_errors(message: impl Into<String>, code: u16, title: impl Into<String>, errors: IndexMap<String, String>) -> Self {
        Self {
            message: message.into(),
            prefixes: None,
            title: Some(title.into()),
            code: Some(code),
            errors: Some(errors),
            platform_native_object: None,
        }
    }

    pub fn new_with_title_errors(message: impl Into<String>, title: impl Into<String>, errors: IndexMap<String, String>) -> Self {
        Self {
            message: message.into(),
            prefixes: None,
            title: Some(title.into()),
            code: None,
            errors: Some(errors),
            platform_native_object: None,
        }
    }

    pub fn prefixed(&self, prefix: impl Into<String>) -> Self {
        Self {
            message: self.message.clone(),
            prefixes: {
                let mut original = self.prefixes.clone().unwrap_or(vec![]);
                original.insert(0, prefix.into());
                Some(original)
            },
            title: self.title.clone(),
            code: self.code.clone(),
            errors: self.errors.clone(),
            platform_native_object: self.platform_native_object.clone(),
        }
    }

    pub fn title(&self) -> &str {
        self.title.as_ref().map_or("InternalServerError", AsRef::as_ref)
    }

    pub fn message(&self) -> String {
        if let Some(prefixes) = &self.prefixes {
            let mut result = "".to_owned();
            for prefix in prefixes {
                result += prefix.as_str();
                result += ": ";
            }
            result += self.message.as_str();
            result
        } else {
            self.message.clone()
        }
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

}

impl Display for Error {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message().as_str())
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