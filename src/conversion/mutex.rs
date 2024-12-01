use std::sync::{MutexGuard, PoisonError};
use crate::Error;

impl<T> From<PoisonError<MutexGuard<'_, T>>> for Error {
    fn from(value: PoisonError<MutexGuard<T>>) -> Self {
        Self::new(format!("{}", value))
    }
}
