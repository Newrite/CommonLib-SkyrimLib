use alloc::ffi::CString;
use core::ffi::{CStr, c_char};

use crate::re::BSResourceFileID;
use crate::relocation::RelocationID;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BSResourceID {
    pub base: BSResourceFileID, // 0x00
    pub dir: u32,               // 0x08
}

const _: () = assert!(core::mem::size_of::<BSResourceID>() == 0xC);

impl BSResourceID {
    crate::relocation_func! {
        pub fn generate_from_path(&mut self, a_path: *const c_char) => RelocationID::new(68635, 69979)
    }

    #[inline(always)]
    pub fn generate_from_path_c_str(&mut self, path: &CStr) {
        self.generate_from_path(path.as_ptr())
    }

    #[inline]
    pub fn generate_from_path_str(&mut self, path: &str) -> Result<(), alloc::ffi::NulError> {
        let path = CString::new(path)?;
        self.generate_from_path(path.as_ptr());
        Ok(())
    }
}
