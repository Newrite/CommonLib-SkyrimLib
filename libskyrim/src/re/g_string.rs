#![allow(non_camel_case_types)]

use core::ffi::{CStr, c_char};
use core::sync::atomic::{AtomicI32, Ordering};

use core_util::EnumSet;

use crate::re::GMemory;
use crate::relocation::RelocationID;

/// C++ `RE::GString::HeapType`
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GStringHeapType {
    kGlobal = 0,
    kLocal = 1,
    kDynamic = 2,
    kMask = 3,
}

core_util::impl_enumset_type!(GStringHeapType => usize);

/// C++ `RE::GString::FlagConstants::FlagConstant`
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GStringFlagConstant {
    kReserveIsSizeShift = (core::mem::size_of::<usize>() * 8) - 1,
}

/// C++ `RE::GString::FlagConstants`
pub struct GStringFlagConstants;

impl GStringFlagConstants {
    pub const kReserveIsSizeShift: usize = GStringFlagConstant::kReserveIsSizeShift as usize;
}

/// C++ `RE::GString::DataDesc`
#[repr(C)]
pub struct GStringDataDesc {
    pub capacity: usize,   // 00
    pub ref_count: i32,    // 08
    pub data: [c_char; 1], // 0C
}

const _: () = assert!(core::mem::size_of::<GStringDataDesc>() == 0x10);

impl GStringDataDesc {
    pub const kFullFlag: usize = 1usize << GStringFlagConstants::kReserveIsSizeShift;

    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            capacity: 1,
            ref_count: 1,
            data: [0],
        }
    }

    #[inline(always)]
    pub fn add_ref(&mut self) {
        let counter = unsafe { &*(core::ptr::addr_of!(self.ref_count).cast::<AtomicI32>()) };
        counter.fetch_add(1, Ordering::AcqRel);
    }

    #[inline(always)]
    pub fn release(&mut self) {
        let counter = unsafe { &*(core::ptr::addr_of!(self.ref_count).cast::<AtomicI32>()) };
        if counter.fetch_sub(1, Ordering::AcqRel) == 1 {
            GMemory::free(core::ptr::from_mut(self).cast());
        }
    }

    #[inline(always)]
    pub const fn get_capacity(&self) -> usize {
        self.capacity & !Self::kFullFlag
    }

    #[inline(always)]
    pub const fn is_full(&self) -> bool {
        (self.capacity & Self::kFullFlag) != 0
    }

    #[inline(always)]
    pub fn set_full(&mut self, set: bool) {
        if set {
            self.capacity |= Self::kFullFlag;
        } else {
            self.capacity &= !Self::kFullFlag;
        }
    }

    #[inline(always)]
    pub fn data_ptr(&self) -> *const c_char {
        core::ptr::addr_of!(self.data).cast()
    }

    #[inline(always)]
    pub fn data_mut_ptr(&mut self) -> *mut c_char {
        core::ptr::addr_of_mut!(self.data).cast()
    }

    // TODO: CommonLib's `GString::DataDesc::~DataDesc()` forwards to
    // `Release()`, but `DataDesc` is a variable-sized trailing-buffer
    // allocation. Keep destruction explicit through `release()` until there is
    // a source-backed Rust owner type for those heap blocks instead of adding a
    // blanket Rust `Drop` impl that would also fire for stack values.
}

/// C++ `RE::GString::DataDescUnion`
#[repr(C)]
pub union GStringDataDescUnion {
    pub data: *mut GStringDataDesc,
    pub heap_type: EnumSet<GStringHeapType, usize>,
    pub bits: usize,
}

const _: () = assert!(core::mem::size_of::<GStringDataDescUnion>() == 0x8);

impl GStringDataDescUnion {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            data: core::ptr::null_mut(),
        }
    }
}

/// C++ `RE::GString::HashFunctor`
pub struct GStringHashFunctor;

impl GStringHashFunctor {
    #[inline(always)]
    pub fn call(&self, data: &GString) -> usize {
        GString::bernstein_hash_function(data.data().cast(), data.size(), 5381)
    }
}

/// C++ `RE::GString::NoCaseHashFunctor`
pub struct GStringNoCaseHashFunctor;

impl GStringNoCaseHashFunctor {
    #[inline(always)]
    pub fn call(&self, data: &GString) -> usize {
        GString::bernstein_hash_function_cis(data.data().cast(), data.size(), 5381)
    }
}

/// C++ `RE::GString`
#[repr(C)]
pub struct GString {
    pub data_desc: GStringDataDescUnion, // 00
}

const _: () = assert!(core::mem::size_of::<GString>() == 0x8);
const _: () = assert!(core::mem::offset_of!(GString, data_desc) == 0x0);

impl Default for GString {
    #[inline(always)]
    fn default() -> Self {
        Self::from_str("")
    }
}

impl Clone for GString {
    #[inline(always)]
    fn clone(&self) -> Self {
        let mut out = Self {
            data_desc: GStringDataDescUnion::new(),
        };
        let desc = self.get_desc();
        out.set_desc(desc);
        if !desc.is_null() {
            unsafe {
                (*desc).add_ref();
            }
        }
        out
    }
}

impl Drop for GString {
    #[inline(always)]
    fn drop(&mut self) {
        let desc = self.get_desc();
        if !desc.is_null() {
            unsafe {
                (*desc).release();
            }
        }
    }
}

impl PartialEq for GString {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size() && self.as_bytes() == other.as_bytes()
    }
}

impl Eq for GString {}

impl core::fmt::Debug for GString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GString")
            .field("data", &self.as_str().ok())
            .field("size", &self.size())
            .finish()
    }
}

impl GString {
    crate::relocation_func! {
        fn ctor_impl(this: *mut GString, value: *const c_char) -> *mut GString => RelocationID::new(80446, 82562)
    }

    #[inline(always)]
    pub fn from_str(value: &str) -> Self {
        let mut out = Self {
            data_desc: GStringDataDescUnion::new(),
        };
        out.assign_str(value);
        out
    }

    #[inline(always)]
    pub unsafe fn from_c_str(value: *const c_char) -> Self {
        let mut out = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            Self::ctor_impl(out.as_mut_ptr(), value);
            out.assume_init()
        }
    }

    #[inline(always)]
    pub unsafe fn ctor(&mut self, value: *const c_char) -> *mut Self {
        Self::ctor_impl(self, value)
    }

    #[inline(always)]
    pub fn assign_str(&mut self, rhs: &str) {
        let desc = self.get_desc();
        if !desc.is_null() {
            unsafe {
                (*desc).release();
            }
        }

        let len = rhs.len();
        let mem_size = len
            + core::mem::size_of::<usize>()
            + core::mem::size_of::<i32>()
            + core::mem::size_of::<c_char>();
        let desc = GMemory::alloc_aligned(mem_size, core::mem::align_of::<GStringDataDesc>())
            .cast::<GStringDataDesc>();
        unsafe {
            (*desc).capacity = len;
            (*desc).set_full(true);
            (*desc).ref_count = 1;
            core::ptr::copy_nonoverlapping(
                rhs.as_ptr().cast::<c_char>(),
                (*desc).data_mut_ptr(),
                len,
            );
            *(*desc).data_mut_ptr().add(len) = 0;
        }
        self.set_desc(desc);
    }

    #[inline(always)]
    pub fn heap_type(&self) -> GStringHeapType {
        match unsafe { self.data_desc.bits } & (GStringHeapType::kMask as usize) {
            0 => GStringHeapType::kGlobal,
            1 => GStringHeapType::kLocal,
            2 => GStringHeapType::kDynamic,
            _ => GStringHeapType::kMask,
        }
    }

    #[inline(always)]
    pub fn get_desc(&self) -> *mut GStringDataDesc {
        (unsafe { self.data_desc.bits } & !(GStringHeapType::kMask as usize))
            as *mut GStringDataDesc
    }

    #[inline(always)]
    pub fn set_desc(&mut self, desc: *mut GStringDataDesc) {
        let heap_type = self.heap_type() as usize;
        self.data_desc = GStringDataDescUnion {
            bits: (desc as usize) | heap_type,
        };
    }

    #[inline(always)]
    pub fn front(&self) -> c_char {
        self[0]
    }

    #[inline(always)]
    pub fn back(&self) -> c_char {
        self[self.size() - 1]
    }

    #[inline(always)]
    pub fn data(&self) -> *const c_char {
        let desc = self.get_desc();
        if desc.is_null() {
            c"".as_ptr()
        } else {
            unsafe { (*desc).data_ptr() }
        }
    }

    #[inline(always)]
    pub fn data_mut(&mut self) -> *mut c_char {
        let desc = self.get_desc();
        if desc.is_null() {
            c"".as_ptr().cast_mut()
        } else {
            unsafe { (*desc).data_mut_ptr() }
        }
    }

    #[inline(always)]
    pub fn c_str(&self) -> *const c_char {
        self.data()
    }

    #[inline(always)]
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.c_str()) }
    }

    #[inline(always)]
    pub fn as_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(self.as_bytes())
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.data().cast::<u8>(), self.size()) }
    }

    #[inline(always)]
    pub fn empty(&self) -> bool {
        self.size() == 0
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        let desc = self.get_desc();
        if desc.is_null() {
            0
        } else if unsafe { (*desc).is_full() } {
            unsafe { (*desc).capacity }
        } else {
            self.as_c_str().to_bytes().len()
        }
    }

    #[inline(always)]
    pub fn length(&self) -> usize {
        self.size()
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        let desc = self.get_desc();
        if !desc.is_null() {
            unsafe {
                *(*desc).data_mut_ptr() = 0;
            }
        }
    }

    #[inline(always)]
    pub fn bernstein_hash_function(
        data_in: *const core::ffi::c_void,
        size: usize,
        seed: usize,
    ) -> usize {
        debug_assert!(!data_in.is_null());
        let data = data_in.cast::<u8>();
        let mut hash = seed;
        let mut remaining = size;
        while remaining != 0 {
            let byte = unsafe { *data.add(remaining - 1) } as usize;
            hash = byte ^ (33usize.wrapping_mul(hash));
            remaining -= 1;
        }
        hash
    }

    #[inline(always)]
    pub fn bernstein_hash_function_cis(
        data_in: *const core::ffi::c_void,
        size: usize,
        seed: usize,
    ) -> usize {
        debug_assert!(!data_in.is_null());
        let data = data_in.cast::<u8>();
        let mut hash = seed;
        let mut remaining = size;
        while remaining != 0 {
            let mut byte = unsafe { *data.add(remaining - 1) };
            if byte.wrapping_sub(b'A') <= 25 {
                byte = byte.wrapping_add(32);
            }
            hash = (byte as usize) ^ (33usize.wrapping_mul(hash));
            remaining -= 1;
        }
        hash
    }
}

impl core::ops::Index<usize> for GString {
    type Output = c_char;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        let desc = self.get_desc();
        debug_assert!(!desc.is_null());
        unsafe { &*(*desc).data_ptr().add(index) }
    }
}

impl core::ops::IndexMut<usize> for GString {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let desc = self.get_desc();
        debug_assert!(!desc.is_null());
        unsafe { &mut *(*desc).data_mut_ptr().add(index) }
    }
}

impl AsRef<GString> for GString {
    #[inline(always)]
    fn as_ref(&self) -> &GString {
        self
    }
}

impl AsMut<GString> for GString {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut GString {
        self
    }
}

pub trait GStringExt: AsRef<GString> + AsMut<GString> {
    #[inline(always)]
    unsafe fn ctor(&mut self, value: *const c_char) -> *mut GString {
        unsafe { self.as_mut().ctor(value) }
    }

    #[inline(always)]
    fn assign_str(&mut self, rhs: &str) {
        self.as_mut().assign_str(rhs);
    }

    #[inline(always)]
    fn heap_type(&self) -> GStringHeapType {
        self.as_ref().heap_type()
    }

    #[inline(always)]
    fn get_desc(&self) -> *mut GStringDataDesc {
        self.as_ref().get_desc()
    }

    #[inline(always)]
    fn front(&self) -> c_char {
        self.as_ref().front()
    }

    #[inline(always)]
    fn back(&self) -> c_char {
        self.as_ref().back()
    }

    #[inline(always)]
    fn data(&self) -> *const c_char {
        self.as_ref().data()
    }

    #[inline(always)]
    fn data_mut(&mut self) -> *mut c_char {
        self.as_mut().data_mut()
    }

    #[inline(always)]
    fn c_str(&self) -> *const c_char {
        self.as_ref().c_str()
    }

    #[inline(always)]
    fn as_c_str(&self) -> &CStr {
        self.as_ref().as_c_str()
    }

    #[inline(always)]
    fn as_str(&self) -> Result<&str, core::str::Utf8Error> {
        self.as_ref().as_str()
    }

    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        self.as_ref().as_bytes()
    }

    #[inline(always)]
    fn empty(&self) -> bool {
        self.as_ref().empty()
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.as_ref().size()
    }

    #[inline(always)]
    fn length(&self) -> usize {
        self.as_ref().length()
    }

    #[inline(always)]
    fn clear(&mut self) {
        self.as_mut().clear();
    }
}

impl<T> GStringExt for T where T: AsRef<GString> + AsMut<GString> {}
