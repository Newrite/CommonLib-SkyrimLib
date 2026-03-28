use alloc::ffi::CString;
use core::ffi::{CStr, c_char};

use crate::relocation::RelocationID;

core_util::abstract_type! { pub type BSResourceLocation; }

/// C++ `namespace RE::BSResource`
pub struct BSResource;

impl BSResource {
    crate::relocation_func! {
        pub fn register_global_path(path: *const c_char) => RelocationID::new(68480, 69833)
    }

    #[inline(always)]
    pub fn register_global_path_c_str(path: &CStr) {
        Self::register_global_path(path.as_ptr())
    }

    #[inline]
    pub fn register_global_path_str(path: &str) -> Result<(), alloc::ffi::NulError> {
        let path = CString::new(path)?;
        Self::register_global_path(path.as_ptr());
        Ok(())
    }

    crate::relocation_func! {
        pub fn register_location(location: *mut BSResourceLocation, priority: u32) => RelocationID::new(68476, 69829)
    }
}
