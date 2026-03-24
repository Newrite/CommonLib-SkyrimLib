use core::ffi::c_char;

use crate::relocation::RelocationID;

core_util::abstract_type! { pub type BSResourceLocation; }

/// C++ `namespace RE::BSResource`
pub struct BSResource;

impl BSResource {
    crate::relocation_func! {
        pub fn register_global_path(path: *const c_char) => RelocationID::new(68480, 69833)
    }

    crate::relocation_func! {
        pub fn register_location(location: *mut BSResourceLocation, priority: u32) => RelocationID::new(68476, 69829)
    }
}
