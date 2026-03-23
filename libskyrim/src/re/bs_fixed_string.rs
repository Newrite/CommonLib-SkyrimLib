use crate::relocation::RelocationID;
use core::ffi::{CStr, c_char};
use core::fmt;
use core::ops::Deref;

use bytemuck::Zeroable;

use crate::re::bs_string_pool::BSStringPoolEntry;
use crate::re::crc::{BSTHash, generate_crc32};
use crate::relocation_func;

/// C++ `RE::BSFixedString` (aliases: `BSFixedStringCI` in some cases)
#[repr(transparent)]
#[derive(Zeroable)]
pub struct BSFixedString {
    data: *const c_char,
}

const _: () = assert!(core::mem::size_of::<BSFixedString>() == 0x8);

impl BSFixedString {
    // C++ `RE::BSFixedString::ctor8`
    relocation_func! {
        pub fn ctor8(this: *mut BSFixedString, string: *const c_char) -> *mut BSFixedString => RelocationID::new(67819, 69161)
    }

    /// Creates an empty string (pointer is null)
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            data: core::ptr::null(),
        }
    }

    /// Creates a new `BSFixedString` from a C string pointer.
    pub fn new(string: *const c_char) -> Self {
        let mut fixed_str = Self::empty();
        if !string.is_null() {
            Self::ctor8(&mut fixed_str as *mut _, string);
        }
        fixed_str
    }

    /// Creates a new `BSFixedString` from a Rust string slice.
    /// This requires allocating a temporary CStr, so use `cstr!` macro when possible
    /// and pass it to `new()`.
    pub fn from_str(string: &str) -> Self {
        let c_str = alloc::ffi::CString::new(string).unwrap();
        Self::new(c_str.as_ptr())
    }

    #[inline(always)]
    fn get_proxy(&self) -> *const BSStringPoolEntry {
        if self.data.is_null() {
            core::ptr::null()
        } else {
            // The Entry header sits exactly one unit (sizeof(BSStringPoolEntry) = 0x18)
            // behind the actual string data pointer in memory.
            unsafe { (self.data as *const BSStringPoolEntry).sub(1) }
        }
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        let proxy = self.get_proxy();
        if proxy.is_null() {
            0
        } else {
            unsafe { (*proxy).length() }
        }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.data.is_null() || self.len() == 0
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const c_char {
        self.data
    }

    #[inline(always)]
    pub fn as_c_str(&self) -> Option<&CStr> {
        if self.data.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(self.data) })
        }
    }
}

impl Default for BSFixedString {
    fn default() -> Self {
        Self::empty()
    }
}

impl Clone for BSFixedString {
    fn clone(&self) -> Self {
        let proxy = self.get_proxy();
        if !proxy.is_null() {
            unsafe { (*proxy).acquire() };
        }
        Self { data: self.data }
    }
}

impl Drop for BSFixedString {
    fn drop(&mut self) {
        if !self.data.is_null() {
            BSStringPoolEntry::release8(self.data);
            self.data = core::ptr::null();
        }
    }
}

impl Deref for BSFixedString {
    type Target = CStr;

    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            // Empty string literal safely returned as CStr fallback.
            unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") }
        } else {
            unsafe { CStr::from_ptr(self.data) }
        }
    }
}

impl PartialEq for BSFixedString {
    fn eq(&self, other: &Self) -> bool {
        if self.data == other.data {
            return true;
        }
        if self.is_empty() && other.is_empty() {
            return true;
        }
        false // Since they are interned, pointer equality implies string equality.
    }
}

impl Eq for BSFixedString {}

impl BSTHash for BSFixedString {
    #[inline]
    fn bst_hash(&self) -> u32 {
        let bytes = unsafe {
            core::slice::from_raw_parts(
                &self.data as *const *const c_char as *const u8,
                core::mem::size_of::<*const c_char>(),
            )
        };
        generate_crc32(bytes)
    }
}

impl fmt::Debug for BSFixedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(c_str) = self.as_c_str() {
            write!(f, "{}", c_str.to_string_lossy())
        } else {
            write!(f, "BSFixedString(null)")
        }
    }
}

impl fmt::Display for BSFixedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(c_str) = self.as_c_str() {
            write!(f, "{}", c_str.to_string_lossy())
        } else {
            Ok(())
        }
    }
}

/// C++ `RE::BSFixedStringW`
#[repr(transparent)]
#[derive(Zeroable)]
pub struct BSFixedStringW {
    data: *const u16,
}

const _: () = assert!(core::mem::size_of::<BSFixedStringW>() == 0x8);

impl BSFixedStringW {
    relocation_func! {
        pub fn ctor16(this: *mut BSFixedStringW, string: *const u16) -> *mut BSFixedStringW => RelocationID::new(67834, 69176)
    }

    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            data: core::ptr::null(),
        }
    }

    pub fn new(string: *const u16) -> Self {
        let mut fixed_str = Self::empty();
        if !string.is_null() {
            Self::ctor16(&mut fixed_str as *mut _, string);
        }
        fixed_str
    }

    #[inline(always)]
    fn get_proxy(&self) -> *const BSStringPoolEntry {
        if self.data.is_null() {
            core::ptr::null()
        } else {
            unsafe { (self.data as *const BSStringPoolEntry).sub(1) }
        }
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        let proxy = self.get_proxy();
        if proxy.is_null() {
            0
        } else {
            unsafe { (*proxy).length() }
        }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.data.is_null() || self.len() == 0
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const u16 {
        self.data
    }
}

impl Default for BSFixedStringW {
    fn default() -> Self {
        Self::empty()
    }
}

impl Clone for BSFixedStringW {
    fn clone(&self) -> Self {
        let proxy = self.get_proxy();
        if !proxy.is_null() {
            unsafe { (*proxy).acquire() };
        }
        Self { data: self.data }
    }
}

impl Drop for BSFixedStringW {
    fn drop(&mut self) {
        if !self.data.is_null() {
            BSStringPoolEntry::release16(self.data);
            self.data = core::ptr::null();
        }
    }
}

impl PartialEq for BSFixedStringW {
    fn eq(&self, other: &Self) -> bool {
        if self.data == other.data {
            return true;
        }
        if self.is_empty() && other.is_empty() {
            return true;
        }
        false
    }
}

impl Eq for BSFixedStringW {}
