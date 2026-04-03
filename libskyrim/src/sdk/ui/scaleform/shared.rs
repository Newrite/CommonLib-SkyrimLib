use alloc::ffi::{CString, NulError};
use core::ffi::CStr;

#[inline(always)]
pub(super) fn with_cstring<R>(
    value: &str,
    _caller: &'static str,
    f: impl FnOnce(&CStr) -> R,
) -> Result<R, NulError> {
    match CString::new(value) {
        Ok(value) => Ok(f(&value)),
        Err(err) => {
            crate::defensive_sdk_warn!("{} rejected input with interior NUL", _caller);
            Err(err)
        }
    }
}
