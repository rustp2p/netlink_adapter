use crate::RUNTIME;
use robusta_jni::jni::errors::{Error, Result};
use std::sync::Arc;
use tokio::runtime::Runtime;

pub(crate) fn convert_jni_result<T>(rs: Result<T>) -> Result<Option<T>> {
    match rs {
        Ok(rs) => Ok(Some(rs)),
        Err(e) => match e {
            Error::NullPtr(_) | Error::NullDeref(_) => Ok(None),
            e => Err(e)?,
        },
    }
}

pub(crate) fn async_runtime() -> Result<Arc<Runtime>> {
    match RUNTIME.get() {
        None => Err(Error::NullPtr("not initialize runtime")),
        Some(runtime) => Ok(runtime.clone()),
    }
}
