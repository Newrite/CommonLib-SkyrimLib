use core::ffi::{CStr, c_char};
use core::fmt;
use core::ops::Deref;

use bytemuck::Zeroable;

use crate::re::crc::BSTHash;

/// C++ `RE::BSFixedString` (aliases: `BSFixedStringCI` in some cases)
#[repr(transparent)]
#[derive(Zeroable)]
pub struct BSFixedString {
    data: *const c_char,
}

const _: () = assert!(core::mem::size_of::<BSFixedString>() == 0x8);

impl BSFixedString {
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
        unsafe {
            crate::ffi::commonlib_bs_fixed_string_ctor8(
                core::ptr::from_mut(&mut fixed_str).cast(),
                string,
            );
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
    pub fn len(&self) -> u32 {
        unsafe { crate::ffi::commonlib_bs_fixed_string_size(core::ptr::from_ref(self).cast()) }
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
        let data = unsafe {
            crate::ffi::commonlib_bs_fixed_string_c_str(core::ptr::from_ref(self).cast())
        };
        if data.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(data) })
        }
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        match self.as_c_str() {
            Some(c_str) => c_str.to_str().unwrap_or("<invalid utf8>"),
            None => "",
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
        let mut cloned = Self::empty();
        unsafe {
            crate::ffi::commonlib_bs_fixed_string_copy(
                core::ptr::from_mut(&mut cloned).cast(),
                core::ptr::from_ref(self).cast(),
            );
        }
        cloned
    }
}

impl Drop for BSFixedString {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::commonlib_bs_fixed_string_destroy(core::ptr::from_mut(self).cast());
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

        unsafe {
            crate::ffi::commonlib_bs_fixed_string_eq(
                core::ptr::from_ref(self).cast(),
                core::ptr::from_ref(other).cast(),
            )
        }
    }
}

impl Eq for BSFixedString {}

impl BSTHash for BSFixedString {
    #[inline]
    fn bst_hash(&self) -> u32 {
        unsafe { crate::ffi::commonlib_bs_fixed_string_hash(core::ptr::from_ref(self).cast()) }
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
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            data: core::ptr::null(),
        }
    }

    pub fn new(string: *const u16) -> Self {
        let mut fixed_str = Self::empty();
        unsafe {
            crate::ffi::commonlib_bs_fixed_string_ctor16(
                core::ptr::from_mut(&mut fixed_str).cast(),
                string,
            );
        }
        fixed_str
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        unsafe { crate::ffi::commonlib_bs_fixed_string_w_size(core::ptr::from_ref(self).cast()) }
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
        let mut cloned = Self::empty();
        unsafe {
            crate::ffi::commonlib_bs_fixed_string_w_copy(
                core::ptr::from_mut(&mut cloned).cast(),
                core::ptr::from_ref(self).cast(),
            );
        }
        cloned
    }
}

impl Drop for BSFixedStringW {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::commonlib_bs_fixed_string_w_destroy(core::ptr::from_mut(self).cast());
        }
    }
}

impl PartialEq for BSFixedStringW {
    fn eq(&self, other: &Self) -> bool {
        if self.data == other.data {
            return true;
        }

        unsafe {
            crate::ffi::commonlib_bs_fixed_string_w_eq(
                core::ptr::from_ref(self).cast(),
                core::ptr::from_ref(other).cast(),
            )
        }
    }
}

impl Eq for BSFixedStringW {}

impl BSTHash for BSFixedStringW {
    #[inline]
    fn bst_hash(&self) -> u32 {
        unsafe { crate::ffi::commonlib_bs_fixed_string_w_hash(core::ptr::from_ref(self).cast()) }
    }
}
