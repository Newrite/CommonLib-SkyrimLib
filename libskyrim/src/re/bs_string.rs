//! Translation of `RE::BSString.h`.
//!
//! Contains:
//! - `DynamicMemoryManagementPol` (heap-backed allocator policy)
//! - `FixedLengthMemoryManagementPol` (inline fixed buffer policy)
//! - `BSStringT` (templated string type)
//! - `BSString` alias (`BSStringT<char, -1, DynamicMemoryManagementPol>`)
//! - `BSStaticStringT<N>` alias

use core::cmp::min;
use core::ffi::{c_char, c_void};
use core::ptr;

/// Allocator policy used by `BSStringT`.
pub trait BSStringAllocator<const N: usize>: Clone {
    fn allocate(&mut self, num: u32) -> *mut c_char;
    fn deallocate(&mut self, ptr: *mut c_char);
}

/// C++ `RE::DynamicMemoryManagementPol<T, N>`.
///
/// Zero-sized allocator that allocates via engine heap (`RE::malloc` / `RE::free`).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DynamicMemoryManagementPol<const N: usize>;

impl<const N: usize> BSStringAllocator<N> for DynamicMemoryManagementPol<N> {
    #[inline]
    fn allocate(&mut self, num: u32) -> *mut c_char {
        if num as usize > N {
            return ptr::null_mut();
        }

        let size = num as usize;
        let mem = unsafe { crate::ffi::commonlib_malloc(size) as *mut c_char };
        if mem.is_null() {
            return ptr::null_mut();
        }

        unsafe {
            ptr::write_bytes(mem as *mut u8, 0, size);
        }
        mem
    }

    #[inline]
    fn deallocate(&mut self, ptr: *mut c_char) {
        if !ptr.is_null() {
            unsafe {
                crate::ffi::commonlib_free(ptr as *mut c_void);
            }
        }
    }
}

/// C++ `RE::FixedLengthMemoryManagementPol<T, N>` for `char`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FixedLengthMemoryManagementPol<const N: usize> {
    buffer: [c_char; N], // 00
}

impl<const N: usize> Default for FixedLengthMemoryManagementPol<N> {
    #[inline(always)]
    fn default() -> Self {
        Self { buffer: [0; N] }
    }
}

impl<const N: usize> BSStringAllocator<N> for FixedLengthMemoryManagementPol<N> {
    #[inline]
    fn allocate(&mut self, num: u32) -> *mut c_char {
        if num as usize > N {
            ptr::null_mut()
        } else {
            self.buffer.as_mut_ptr()
        }
    }

    #[inline(always)]
    fn deallocate(&mut self, _ptr: *mut c_char) {}
}

/// C++ `RE::BSStringT<char, N, Allocator>`.
#[repr(C)]
pub struct BSStringT<const N: usize, A: BSStringAllocator<N>> {
    allocator: A,      // 00 (zero-sized for dynamic policy)
    data: *mut c_char, // 00 / sizeof(allocator)
    size: u16,         // 08
    capacity: u16,     // 0A
    pad0c: u32,        // 0C
}

impl<const N: usize, A: BSStringAllocator<N> + Default> BSStringT<N, A> {
    #[inline]
    pub fn new() -> Self {
        let mut s = Self {
            allocator: A::default(),
            data: ptr::null_mut(),
            size: 0,
            capacity: 0,
            pad0c: 0,
        };
        s.clear();
        s
    }

    #[inline]
    pub fn from_c_str(rhs: *const c_char) -> Self {
        let mut s = Self::new();
        let _ = s.set_c_str(rhs, 0);
        s
    }

    #[inline]
    pub fn from_str(rhs: &str) -> Self {
        let mut s = Self::new();
        let _ = s.set_str(rhs);
        s
    }
}

impl<const N: usize, A: BSStringAllocator<N>> BSStringT<N, A> {
    const EMPTY: [c_char; 1] = [0];

    #[inline(always)]
    fn empty_mut_ptr() -> *mut c_char {
        static mut EMPTY: [c_char; 1] = [0];
        unsafe { core::ptr::addr_of_mut!(EMPTY[0]) }
    }

    #[inline(always)]
    const fn max_len() -> u16 {
        N as u16
    }

    #[inline(always)]
    pub fn data(&self) -> *const c_char {
        if self.data.is_null() {
            Self::EMPTY.as_ptr()
        } else {
            self.data
        }
    }

    #[inline(always)]
    pub fn data_mut(&mut self) -> *mut c_char {
        if self.data.is_null() {
            Self::empty_mut_ptr()
        } else {
            self.data
        }
    }

    #[inline(always)]
    pub fn c_str(&self) -> *const c_char {
        self.data()
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        core_util::ptr_to_str(self.c_str())
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn len(&self) -> u16 {
        if self.size != Self::max_len() {
            self.size
        } else {
            unsafe { c_strlen(self.data()) as u16 }
        }
    }

    #[inline(always)]
    pub fn length(&self) -> u16 {
        self.len()
    }

    #[inline]
    pub fn clear(&mut self) {
        let _ = self.set_c_str(Self::EMPTY.as_ptr(), 0);
    }

    #[inline]
    pub fn set_c_str(&mut self, src: *const c_char, src_len: u32) -> bool {
        let src = if src.is_null() {
            Self::EMPTY.as_ptr()
        } else {
            src
        };

        if self.data == src as *mut c_char {
            return true;
        }

        let len = if src_len == 0 {
            unsafe { c_strlen(src) }
        } else {
            src_len as usize
        };

        self.set_bytes_raw(src as *const u8, len)
    }

    #[inline]
    pub fn set_str(&mut self, src: &str) -> bool {
        self.set_bytes_raw(src.as_ptr(), src.len())
    }

    #[inline]
    fn set_bytes_raw(&mut self, src: *const u8, src_len: usize) -> bool {
        let max = Self::max_len() as usize;
        let new_size = min(src_len, max) as u16;
        let copy_len = new_size as usize;
        let needed = copy_len.saturating_add(1);

        if needed <= self.capacity as usize {
            if self.data.is_null() {
                return false;
            }
            unsafe {
                ptr::copy_nonoverlapping(src, self.data as *mut u8, copy_len);
                *self.data.add(copy_len) = 0;
            }
            self.size = new_size;
            return true;
        }

        if needed > u32::MAX as usize {
            return false;
        }

        let new_data = self.allocator.allocate(needed as u32);
        if new_data.is_null() {
            return false;
        }

        unsafe {
            ptr::copy_nonoverlapping(src, new_data as *mut u8, copy_len);
            *new_data.add(copy_len) = 0;
        }

        if !self.data.is_null() {
            self.allocator.deallocate(self.data);
        }

        self.data = new_data;
        self.size = new_size;
        self.capacity = min(needed, max) as u16;
        true
    }
}

impl<const N: usize, A: BSStringAllocator<N> + Default> Default for BSStringT<N, A> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, A: BSStringAllocator<N>> Drop for BSStringT<N, A> {
    fn drop(&mut self) {
        self.allocator.deallocate(self.data);
        self.data = ptr::null_mut();
        self.size = 0;
        self.capacity = 0;
    }
}

impl<const N: usize, A: BSStringAllocator<N>> Clone for BSStringT<N, A> {
    fn clone(&self) -> Self {
        let mut out = Self {
            allocator: self.allocator.clone(),
            data: ptr::null_mut(),
            size: 0,
            capacity: 0,
            pad0c: 0,
        };
        let _ = out.set_c_str(self.c_str(), self.len() as u32);
        out
    }
}

impl<const N: usize, A: BSStringAllocator<N>> PartialEq for BSStringT<N, A> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { eq_c_str_icase(self.c_str(), other.c_str()) }
    }
}

impl<const N: usize, A: BSStringAllocator<N>> Eq for BSStringT<N, A> {}

impl<const N: usize, A: BSStringAllocator<N>> PartialEq<&str> for BSStringT<N, A> {
    fn eq(&self, other: &&str) -> bool {
        eq_strn_icase_cstr(other.as_bytes(), self.c_str())
    }
}

impl<const N: usize, A: BSStringAllocator<N>> PartialEq<BSStringT<N, A>> for &str {
    fn eq(&self, other: &BSStringT<N, A>) -> bool {
        eq_strn_icase_cstr(self.as_bytes(), other.c_str())
    }
}

pub type BSString =
    BSStringT<{ u32::MAX as usize }, DynamicMemoryManagementPol<{ u32::MAX as usize }>>;

/// C++ `RE::BSStaticStringT<N>`.
pub type BSStaticStringT<const N: usize> = BSStringT<N, FixedLengthMemoryManagementPol<N>>;

const _: () = assert!(core::mem::size_of::<BSString>() == 0x10);

#[inline]
unsafe fn c_strlen(mut s: *const c_char) -> usize {
    unsafe {
        let mut len = 0usize;
        while *s != 0 {
            s = s.add(1);
            len += 1;
        }
        len
    }
}

#[inline(always)]
const fn lower_ascii(b: u8) -> u8 {
    if b >= b'A' && b <= b'Z' {
        b + (b'a' - b'A')
    } else {
        b
    }
}

#[inline]
unsafe fn eq_c_str_icase(mut lhs: *const c_char, mut rhs: *const c_char) -> bool {
    unsafe {
        loop {
            let l = *lhs as u8;
            let r = *rhs as u8;
            if lower_ascii(l) != lower_ascii(r) {
                return false;
            }
            if l == 0 {
                return true;
            }
            lhs = lhs.add(1);
            rhs = rhs.add(1);
        }
    }
}

#[cfg(test)]
#[inline]
fn eq_str_icase_cstr(lhs: &[u8], rhs: *const c_char) -> bool {
    if rhs.is_null() {
        return lhs.is_empty();
    }

    let mut i = 0usize;
    loop {
        let r = unsafe { *rhs.add(i) as u8 };
        if i == lhs.len() {
            return r == 0;
        }
        if lower_ascii(lhs[i]) != lower_ascii(r) {
            return false;
        }
        i += 1;
    }
}

#[inline]
fn eq_strn_icase_cstr(lhs: &[u8], rhs: *const c_char) -> bool {
    if rhs.is_null() {
        return lhs.is_empty();
    }

    let mut i = 0usize;
    while i < lhs.len() {
        let r = unsafe { *rhs.add(i) as u8 };
        if r == 0 || lower_ascii(lhs[i]) != lower_ascii(r) {
            return false;
        }
        i += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{eq_str_icase_cstr, eq_strn_icase_cstr};
    use core::ffi::c_char;

    #[test]
    fn c_str_equality_requires_full_match() {
        let rhs = [b'a' as c_char, b'b' as c_char, b'c' as c_char, 0];
        assert!(eq_str_icase_cstr(b"abc", rhs.as_ptr()));
        assert!(!eq_str_icase_cstr(b"ab", rhs.as_ptr()));
    }

    #[test]
    fn string_view_equality_matches_cpp_prefix_semantics() {
        let rhs = [
            b'a' as c_char,
            b'b' as c_char,
            b'c' as c_char,
            b'd' as c_char,
            0,
        ];
        let short_rhs = [b'a' as c_char, b'b' as c_char, 0];

        assert!(eq_strn_icase_cstr(b"abc", rhs.as_ptr()));
        assert!(!eq_strn_icase_cstr(b"abc", short_rhs.as_ptr()));
    }
}
