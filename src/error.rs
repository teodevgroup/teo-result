use std::any::Any;
use std::convert::Infallible;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use indexmap::IndexMap;

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub prefixes: Option<Vec<String>>,
    pub title: Option<String>,
    pub code: Option<u16>,
    pub errors: Option<IndexMap<String, String>>,
    pub platform_native_object: Option<Arc<dyn Any + Send + Sync>>,
}

impl Error {

    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            prefixes: None,
            title: None,
            code: None,
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