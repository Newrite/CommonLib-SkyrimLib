#![allow(non_camel_case_types)]

use core::ops::{Deref, DerefMut};
use core::ptr;

use crate::re::crc::{BSTHash, generate_crc32};
use crate::re::hkReferencedObject;

/// Trait for intrusive Havok reference counting.
///
/// Required bound for `hkRefPtr<T>`.
pub trait HkRef {
    fn add_reference(&self);
    fn remove_reference(&self);
}

impl<T: AsRef<hkReferencedObject>> HkRef for T {
    #[inline(always)]
    fn add_reference(&self) {
        hkReferencedObject::add_reference(self.as_ref())
    }

    #[inline(always)]
    fn remove_reference(&self) {
        hkReferencedObject::remove_reference(self.as_ref())
    }
}

/// C++ `RE::hkRefPtr<T>`.
///
/// An intrusive reference-counted smart pointer for Havok objects. The pointed-to
/// type must support `AddReference()` / `RemoveReference()` through `HkRef`.
#[repr(transparent)]
pub struct hkRefPtr<T: HkRef> {
    _ptr: *mut T,
}

const _: () = assert!(core::mem::size_of::<hkRefPtr<hkReferencedObject>>() == 0x8);

impl<T: HkRef> hkRefPtr<T> {
    /// Creates a null `hkRefPtr`.
    #[inline(always)]
    pub const fn null() -> Self {
        Self {
            _ptr: ptr::null_mut(),
        }
    }

    /// Constructs a C++ `hkRefPtr<T>` directly into out storage owned by Rust.
    ///
    /// This is intended for ABI-safe bridges that materialize the smart pointer on the
    /// C++ side, such as `make_hkref<T>(...)` wrappers.
    #[inline(always)]
    pub unsafe fn try_construct_with(construct: impl FnOnce(*mut Self) -> bool) -> Option<Self> {
        unsafe { crate::ffi::try_construct_out_param(construct) }
    }

    /// Creates a new `hkRefPtr` from a raw pointer, incrementing the refcount.
    ///
    /// Matches C++ `hkRefPtr(Y* a_rhs)` which calls `TryAttach`.
    ///
    /// # Safety
    /// `ptr` must be null or point to a valid intrusive-refcounted `T`.
    #[inline(always)]
    pub unsafe fn new(ptr: *mut T) -> Self {
        unsafe {
            if !ptr.is_null() {
                (*ptr).add_reference();
            }
            Self { _ptr: ptr }
        }
    }

    /// Creates an `hkRefPtr` from a raw pointer without incrementing the refcount.
    ///
    /// # Safety
    /// `ptr` must be null or point to a valid `T` with an already-attached count.
    #[inline(always)]
    pub const unsafe fn from_raw(ptr: *mut T) -> Self {
        Self { _ptr: ptr }
    }

    /// Returns the raw pointer without affecting the refcount.
    #[inline(always)]
    pub const fn get(&self) -> *mut T {
        self._ptr
    }

    /// Returns true if the pointer is null.
    #[inline(always)]
    pub const fn is_null(&self) -> bool {
        self._ptr.is_null()
    }

    /// Consumes the `hkRefPtr` and returns the raw pointer without detaching.
    #[inline(always)]
    pub fn into_raw(self) -> *mut T {
        let ptr = self._ptr;
        core::mem::forget(self);
        ptr
    }

    /// Releases the current pointer and sets the smart pointer to null.
    #[inline(always)]
    pub fn reset(&mut self) {
        self.try_detach();
    }

    /// Releases the current pointer and replaces it with `ptr`,
    /// incrementing the new pointer's refcount when needed.
    ///
    /// # Safety
    /// `ptr` must be null or point to a valid intrusive-refcounted `T`.
    #[inline(always)]
    pub unsafe fn reset_to(&mut self, ptr: *mut T) {
        if self._ptr != ptr {
            self.try_detach();
            self._ptr = ptr;
            self.try_attach();
        }
    }

    #[inline(always)]
    fn try_attach(&self) {
        if !self._ptr.is_null() {
            unsafe {
                (*self._ptr).add_reference();
            }
        }
    }

    #[inline(always)]
    fn try_detach(&mut self) {
        if !self._ptr.is_null() {
            unsafe {
                (*self._ptr).remove_reference();
            }
            self._ptr = ptr::null_mut();
        }
    }
}

impl<T: HkRef> Clone for hkRefPtr<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        self.try_attach();
        Self { _ptr: self._ptr }
    }
}

impl<T: HkRef> Drop for hkRefPtr<T> {
    #[inline(always)]
    fn drop(&mut self) {
        self.try_detach();
    }
}

impl<T: HkRef> Deref for hkRefPtr<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        debug_assert!(!self._ptr.is_null());
        unsafe { &*self._ptr }
    }
}

impl<T: HkRef> DerefMut for hkRefPtr<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        debug_assert!(!self._ptr.is_null());
        unsafe { &mut *self._ptr }
    }
}

impl<T: HkRef, U: HkRef> PartialEq<hkRefPtr<U>> for hkRefPtr<T> {
    #[inline(always)]
    fn eq(&self, other: &hkRefPtr<U>) -> bool {
        self._ptr.cast::<()>() == other._ptr.cast::<()>()
    }
}

impl<T: HkRef> Eq for hkRefPtr<T> {}

impl<T: HkRef> Default for hkRefPtr<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::null()
    }
}

impl<T: HkRef> BSTHash for hkRefPtr<T> {
    #[inline(always)]
    fn bst_hash(&self) -> u32 {
        let bytes = unsafe {
            core::slice::from_raw_parts(
                core::ptr::addr_of!(self._ptr).cast::<u8>(),
                core::mem::size_of::<*mut T>(),
            )
        };
        generate_crc32(bytes)
    }
}
