use std::any::Any;
use std::collections::BTreeMap;
use std::convert::Infallible;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

pub struct Error {
    pub message: String,
    pub meta_map: BTreeMap<String, Arc<dyn Any + Send + Sync>>,
}

impl Error {

    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into(), meta_map: BTreeMap::new() }
    }

    pub fn prefix(&self, prefix: impl AsRef<str>) -> Self {
        Self::new(format!("{}: {}", prefix.as_ref(), self.message))
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub fn insert_meta<T: 'static + Send + Sync>(&mut self, key: impl Into<String>, val: T) {
        self.meta_map.insert(key.into(), Arc::new(val));
    }

    pub fn get_meta<T: 'static + Send>(&self, key: &str) -> Option<&T> {
        self.meta_map.get(key).and_then(|boxed| boxed.downcast_ref())
    }
}

impl Display for Error {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Debug for Error {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
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