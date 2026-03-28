#![allow(non_camel_case_types)]

use core::ops::{Deref, DerefMut};
use core::ptr;

/// Trait for intrusive Scaleform reference counting used by `GPtr<T>`.
pub trait GPtrTarget {
    fn gptr_add_ref(&self);
    fn gptr_release(&self);
}

/// C++ `RE::GPtr<T>`.
///
/// An intrusive Scaleform reference-counted smart pointer. Constructing from a
/// raw pointer and cloning both attach via `AddRef()`, while dropping or
/// resetting detaches via `Release()`.
#[repr(transparent)]
#[derive(Debug, Hash)]
pub struct GPtr<T: GPtrTarget> {
    _ptr: *mut T, // 00
}

struct Dummy;

impl GPtrTarget for Dummy {
    #[inline(always)]
    fn gptr_add_ref(&self) {}

    #[inline(always)]
    fn gptr_release(&self) {}
}

const _: () = assert!(core::mem::size_of::<GPtr<Dummy>>() == 0x8);
const _: () = assert!(core::mem::offset_of!(GPtr<Dummy>, _ptr) == 0x0);

impl<T: GPtrTarget> GPtr<T> {
    /// Creates a null `GPtr`.
    #[inline(always)]
    pub const fn null() -> Self {
        Self {
            _ptr: ptr::null_mut(),
        }
    }

    /// Constructs a C++ `GPtr<T>` directly into caller-provided out storage.
    ///
    /// This is the Rust-side half of an ABI-safe bridge that materializes the
    /// smart pointer on the C++ side.
    #[inline(always)]
    pub unsafe fn try_construct_with(construct: impl FnOnce(*mut Self) -> bool) -> Option<Self> {
        unsafe { crate::ffi::try_construct_out_param(construct) }
    }

    // TODO: CommonLib's `GPtr.h` also exposes converting constructors and
    // assignments between `GPtr<Y>` and `GPtr<T>` when `Y*` is convertible to
    // `T*`. Keep the current same-type Rust surface until there is a
    // source-backed generic trait bound that expresses that pointer
    // convertibility honestly.
    // TODO: CommonLib also exposes `make_gptr<T>(Args&&...)` in `GPtr.h`.
    // Keep using `try_construct_with(...)` and per-type constructor bridges
    // until there is an ABI-safe generic factory path for arbitrary C++ `T`.

    /// Creates a new `GPtr` from a raw pointer, attaching via `AddRef()`.
    ///
    /// Matches C++ `GPtr(Y* a_rhs)`.
    ///
    /// # Safety
    /// `ptr` must be null or point to a valid intrusive-refcounted `T`.
    #[inline(always)]
    pub unsafe fn new(ptr: *mut T) -> Self {
        if !ptr.is_null() {
            unsafe {
                (*ptr).gptr_add_ref();
            }
        }

        Self { _ptr: ptr }
    }

    /// Creates a `GPtr` from a raw pointer without attaching.
    ///
    /// # Safety
    /// `ptr` must be null or point to a valid `T` with an already-attached
    /// reference count.
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

    /// Returns the raw pointer without affecting the refcount.
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self._ptr
    }

    #[inline(always)]
    pub unsafe fn as_ref(&self) -> Option<&T> {
        unsafe { self._ptr.as_ref() }
    }

    #[inline(always)]
    pub unsafe fn as_mut(&mut self) -> Option<&mut T> {
        unsafe { self._ptr.as_mut() }
    }

    /// Consumes the smart pointer and returns the raw pointer without
    /// detaching. Caller takes ownership of the attached reference.
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

    /// Releases the current pointer and replaces it with `ptr`, attaching the
    /// new pointee when needed.
    ///
    /// Matches C++ `reset(Y* a_ptr)`.
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
                (*self._ptr).gptr_add_ref();
            }
        }
    }

    #[inline(always)]
    fn try_detach(&mut self) {
        if !self._ptr.is_null() {
            unsafe {
                (*self._ptr).gptr_release();
            }
            self._ptr = ptr::null_mut();
        }
    }
}

impl<T: GPtrTarget> Clone for GPtr<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        self.try_attach();
        Self { _ptr: self._ptr }
    }
}

impl<T: GPtrTarget> Drop for GPtr<T> {
    #[inline(always)]
    fn drop(&mut self) {
        self.try_detach();
    }
}

impl<T: GPtrTarget> Deref for GPtr<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        debug_assert!(!self._ptr.is_null());
        unsafe { &*self._ptr }
    }
}

impl<T: GPtrTarget> DerefMut for GPtr<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        debug_assert!(!self._ptr.is_null());
        unsafe { &mut *self._ptr }
    }
}

impl<T: GPtrTarget, U: GPtrTarget> PartialEq<GPtr<U>> for GPtr<T> {
    #[inline(always)]
    fn eq(&self, other: &GPtr<U>) -> bool {
        self._ptr.cast::<()>() == other._ptr.cast::<()>()
    }
}

impl<T: GPtrTarget> Eq for GPtr<T> {}

impl<T: GPtrTarget> Default for GPtr<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::null()
    }
}

impl<T: GPtrTarget> From<&GPtr<T>> for bool {
    #[inline(always)]
    fn from(value: &GPtr<T>) -> Self {
        !value.is_null()
    }
}
