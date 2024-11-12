use robusta_jni::jni::errors::{Error, Result};

pub(crate) fn convert_jni_result<T>(rs: Result<T>) -> Result<Option<T>> {
    match rs {
        Ok(rs) => Ok(Some(rs)),
        Err(e) => match e {
            Error::NullPtr(_) | Error::NullDeref(_) => Ok(None),
            e => Err(e)?,
        },
    }
}
