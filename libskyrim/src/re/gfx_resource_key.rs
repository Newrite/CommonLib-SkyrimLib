#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_void};

/// C++ `RE::GFxResourceKey::KeyType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxResourceKeyKeyType {
    kNone = 0,
    kUnique = 1,
    kFile = 2,
    kGradient = 3,
    kSubImage = 4,
}

/// C++ `RE::GFxResourceKey::KeyInterface`
#[repr(C)]
pub struct GFxResourceKeyKeyInterface {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<GFxResourceKeyKeyInterface>() == 0x8);

impl GFxResourceKeyKeyInterface {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_ADD_REF: usize = 0x01; pub fn add_ref(data: *mut c_void) }
    crate::virtual_method! { pub const VFUNC_RELEASE: usize = 0x02; pub fn release(data: *mut c_void) }
    crate::virtual_method! { pub const VFUNC_GET_KEY_TYPE: usize = 0x03; pub fn get_key_type(data: *mut c_void) -> GFxResourceKeyKeyType }
    crate::virtual_method! { pub const VFUNC_GET_HASH_CODE: usize = 0x04; pub fn get_hash_code(data: *mut c_void) -> usize }
    crate::virtual_method! { pub const VFUNC_KEY_EQUALS: usize = 0x05; pub fn key_equals(data: *mut c_void, other: &GFxResourceKey) -> bool }
    crate::virtual_method! { pub const VFUNC_GET_FILE_URL: usize = 0x06; pub fn get_file_url(data: *mut c_void) -> *const c_char }
}

/// C++ `RE::GFxResourceKey`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GFxResourceKey {
    pub key_interface: *mut GFxResourceKeyKeyInterface, // 00
    pub key_data: *mut c_void,                          // 08
}

const _: () = assert!(core::mem::size_of::<GFxResourceKey>() == 0x10);
const _: () = assert!(core::mem::offset_of!(GFxResourceKey, key_interface) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxResourceKey, key_data) == 0x8);

impl Default for GFxResourceKey {
    #[inline(always)]
    fn default() -> Self {
        Self {
            key_interface: core::ptr::null_mut(),
            key_data: core::ptr::null_mut(),
        }
    }
}
