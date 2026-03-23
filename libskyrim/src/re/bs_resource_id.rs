use core::ffi::c_char;

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
}
