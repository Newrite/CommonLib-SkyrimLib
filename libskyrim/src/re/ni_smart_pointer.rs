//! Translation of `RE::NiSmartPointer.h`.
//!
//! `NiPointer<T>` — an intrusive reference-counted smart pointer.
//! `T` must implement the `NiRef` trait (providing `inc_ref`/`dec_ref`).
//!
//! This is the Skyrim engine's equivalent of `std::shared_ptr`, but with
//! intrusive reference counting (the refcount lives inside the pointed-to object).

use core::ops::{Deref, DerefMut};
use core::ptr;

use crate::re::ni_ref_object::NiRef;
use crate::re::TESObjectWEAP;

/// C++ `RE::NiPointer<T>`.
///
/// An intrusive reference-counted smart pointer. The pointed-to type must
/// implement `NiRef` (e.g., `NiRefObject`, `BSHandleRefObject`).
///
/// Layout: `{ _ptr: *mut T }` — 0x8 bytes (one pointer).
#[repr(transparent)]
#[derive(Debug)]
pub struct NiPointer<T: NiRef> {
    _ptr: *mut T,
}

const _: () = assert!(core::mem::size_of::<NiPointer<crate::re::ni_ref_object::NiRefObject>>() == 0x8);

impl<T: NiRef> NiPointer<T> {
    /// Creates a null `NiPointer`.
    #[inline(always)]
    pub const fn null() -> Self {
        Self { _ptr: ptr::null_mut() }
    }

    /// Creates a new `NiPointer` from a raw pointer, incrementing the refcount.
    /// Matches C++ `NiPointer(T* a_rhs)` which calls `TryAttach`.
    ///
    /// # Safety
    /// `ptr` must be null or point to a valid `T` with a live refcount.
    #[inline(always)]
    pub unsafe fn new(ptr: *mut T) -> Self {
        if !ptr.is_null() {
            (*ptr).inc_ref();
        }
        Self { _ptr: ptr }
    }

    /// Creates a `NiPointer` from a raw pointer **without** incrementing the refcount.
    /// Use this when you've already incremented the refcount or are taking ownership
    /// from C++ code that has already attached.
    ///
    /// # Safety
    /// `ptr` must be null or point to a valid `T` with an already-incremented refcount.
    #[inline(always)]
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self { _ptr: ptr }
    }

    /// Returns the raw pointer without affecting the refcount.
    #[inline(always)]
    pub fn get(&self) -> *mut T {
        self._ptr
    }

    /// Returns true if the pointer is null.
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self._ptr.is_null()
    }

    /// Consumes the `NiPointer` and returns the raw pointer without
    /// decrementing the refcount. Caller takes ownership.
    #[inline(always)]
    pub fn into_raw(self) -> *mut T {
        let ptr = self._ptr;
        core::mem::forget(self);
        ptr
    }

    /// Releases the current pointer (decrementing refcount) and sets to null.
    /// Matches C++ `reset()`.
    pub fn reset(&mut self) {
        self.try_detach();
    }

    /// Releases the current pointer and replaces it with `ptr`,
    /// incrementing the new pointer's refcount.
    /// Matches C++ `reset(T* a_ptr)`.
    ///
    /// # Safety
    /// `ptr` must be null or point to a valid `T`.
    pub unsafe fn reset_to(&mut self, ptr: *mut T) {
        if self._ptr != ptr {
            self.try_detach();
            self._ptr = ptr;
            self.try_attach();
        }
    }

    // ── Private helpers (match C++ TryAttach/TryDetach) ──────────────────

    #[inline(always)]
    fn try_attach(&self) {
        if !self._ptr.is_null() {
            unsafe { (*self._ptr).inc_ref(); }
        }
    }

    #[inline(always)]
    fn try_detach(&mut self) {
        if !self._ptr.is_null() {
            unsafe { (*self._ptr).dec_ref(); }
            self._ptr = ptr::null_mut();
        }
    }
}

impl<T: NiRef> Clone for NiPointer<T> {
    /// Cloning increments the refcount (matches C++ copy constructor).
    fn clone(&self) -> Self {
        self.try_attach();
        Self { _ptr: self._ptr }
    }
}

impl<T: NiRef> Drop for NiPointer<T> {
    /// Dropping decrements the refcount (matches C++ destructor).
    fn drop(&mut self) {
        self.try_detach();
    }
}

impl<T: NiRef> Deref for NiPointer<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        debug_assert!(!self._ptr.is_null());
        unsafe { &*self._ptr }
    }
}

impl<T: NiRef> DerefMut for NiPointer<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        debug_assert!(!self._ptr.is_null());
        unsafe { &mut *self._ptr }
    }
}

impl<T: NiRef> PartialEq for NiPointer<T> {
    fn eq(&self, other: &Self) -> bool {
        self._ptr == other._ptr
    }
}

impl<T: NiRef> Eq for NiPointer<T> {}

impl<T: NiRef> Default for NiPointer<T> {
    fn default() -> Self {
        Self::null()
    }
}
