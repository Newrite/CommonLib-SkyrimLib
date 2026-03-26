//! Translation of `RE::BSTSmartPointer.h`.
//!
//! `BSTSmartPointer<T>` вЂ” intrusive reference-counted smart pointer,
//! typically used with `BSTSmartPointerIntrusiveRefCount` or `BSTSmartPointerAutoPtr`.

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::ptr;

/// Trait defining the acquire and release logic for `BSTSmartPointer`.
pub trait BSTSmartPointerManager<T> {
    /// Increments the reference count or acquires ownership.
    fn acquire(ptr: *mut T);
    /// Decrements the reference count or releases ownership.
    /// If the reference count reaches 0, this usually deletes the object.
    fn release(ptr: *mut T);
}

/// Intrusive reference count manager for `BSTSmartPointer`.
///
/// Types `T` managed by this must implement `BSTSmartPointerIntrusiveRefCountable`.
pub struct BSTSmartPointerIntrusiveRefCount;

/// Trait for types managed by `BSTSmartPointerIntrusiveRefCount`.
pub trait BSTSmartPointerIntrusiveRefCountable {
    /// Increment the reference count.
    fn bst_inc_ref(&self);
    /// Decrement the reference count. If it reaches 0, the object should be deleted.
    fn bst_dec_ref(&self) -> u32;
    /// Deletes the object. Called when `bst_dec_ref` returns 0.
    ///
    /// # Safety
    /// Must only be called when the reference count is 0.
    unsafe fn bst_delete(&self);
}

impl<T: BSTSmartPointerIntrusiveRefCountable> BSTSmartPointerManager<T>
    for BSTSmartPointerIntrusiveRefCount
{
    #[inline(always)]
    fn acquire(ptr: *mut T) {
        if !ptr.is_null() {
            unsafe { (*ptr).bst_inc_ref() };
        }
    }

    #[inline(always)]
    fn release(ptr: *mut T) {
        if !ptr.is_null() {
            unsafe {
                if (*ptr).bst_dec_ref() == 0 {
                    (*ptr).bst_delete();
                }
            }
        }
    }
}

/// Auto pointer manager for `BSTSmartPointer`.
pub struct BSTSmartPointerAutoPtr;

/// Trait for types managed by `BSTSmartPointerAutoPtr`.
pub trait BSTSmartPointerAutoDeletable {
    /// Deletes the object.
    ///
    /// # Safety
    /// Must only be called once to free the object.
    unsafe fn bst_delete(&self);
}

impl<T: BSTSmartPointerAutoDeletable> BSTSmartPointerManager<T> for BSTSmartPointerAutoPtr {
    #[inline(always)]
    fn acquire(_ptr: *mut T) {
        // Does nothing
    }

    #[inline(always)]
    fn release(ptr: *mut T) {
        if !ptr.is_null() {
            unsafe { (*ptr).bst_delete() };
        }
    }
}

/// C++ `RE::BSTSmartPointer<T, RefManager>`.
///
/// Layout: `{ _ptr: *mut T }` вЂ” 0x8 bytes (one pointer).
#[repr(C)]
pub struct BSTSmartPointer<T, M: BSTSmartPointerManager<T> = BSTSmartPointerIntrusiveRefCount> {
    _ptr: *mut T, // 00
    _marker: PhantomData<M>,
}

struct Dummy;

impl BSTSmartPointerIntrusiveRefCountable for Dummy {
    fn bst_inc_ref(&self) {}
    fn bst_dec_ref(&self) -> u32 {
        0
    }
    unsafe fn bst_delete(&self) {}
}

const _: () = assert!(
    core::mem::size_of::<BSTSmartPointer<Dummy, BSTSmartPointerIntrusiveRefCount>>() == 0x8
);

impl<T, M: BSTSmartPointerManager<T>> BSTSmartPointer<T, M> {
    /// Creates a null `BSTSmartPointer`.
    #[inline(always)]
    pub const fn null() -> Self {
        Self {
            _ptr: ptr::null_mut(),
            _marker: PhantomData,
        }
    }

    /// Constructs a C++ `BSTSmartPointer<T, M>` directly into out storage owned by Rust.
    ///
    /// This is intended for ABI-safe bridges that materialize the smart pointer on the
    /// C++ side, such as `make_smart<T>(...)` wrappers.
    #[inline(always)]
    pub unsafe fn try_construct_with(construct: impl FnOnce(*mut Self) -> bool) -> Option<Self> {
        unsafe { crate::ffi::try_construct_out_param(construct) }
    }

    /// Creates a new `BSTSmartPointer` from a raw pointer, acquiring the reference.
    /// Matches C++ `BSTSmartPointer(Y* a_rhs)`.
    ///
    /// # Safety
    /// `ptr` must be null or point to a valid `T`.
    #[inline(always)]
    pub unsafe fn new(ptr: *mut T) -> Self {
        if !ptr.is_null() {
            M::acquire(ptr);
        }
        Self {
            _ptr: ptr,
            _marker: PhantomData,
        }
    }

    /// Creates a `BSTSmartPointer` from a raw pointer **without** acquiring the reference.
    /// Use this when you've already incremented the refcount or are taking ownership
    /// from C++ code that has already attached.
    ///
    /// # Safety
    /// `ptr` must be null or point to a valid `T` with an already-incremented refcount.
    #[inline(always)]
    pub const unsafe fn from_raw(ptr: *mut T) -> Self {
        Self {
            _ptr: ptr,
            _marker: PhantomData,
        }
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

    /// Consumes the `BSTSmartPointer` and returns the raw pointer without
    /// releasing the reference. Caller takes ownership.
    #[inline(always)]
    pub fn into_raw(self) -> *mut T {
        let ptr = self._ptr;
        core::mem::forget(self);
        ptr
    }

    /// Releases the current pointer and sets to null.
    /// Matches C++ `reset()`.
    #[inline(always)]
    pub fn reset(&mut self) {
        self.try_detach();
    }

    /// Releases the current pointer and replaces it with `ptr`,
    /// acquiring the new pointer's reference.
    /// Matches C++ `reset(Y* a_ptr)`.
    ///
    /// # Safety
    /// `ptr` must be null or point to a valid `T`.
    #[inline(always)]
    pub unsafe fn reset_to(&mut self, ptr: *mut T) {
        if self._ptr != ptr {
            self.try_detach();
            self._ptr = ptr;
            self.try_attach();
        }
    }

    // в”Ђв”Ђ Private helpers (match C++ TryAttach/TryDetach) в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

    #[inline(always)]
    fn try_attach(&self) {
        if !self._ptr.is_null() {
            M::acquire(self._ptr);
        }
    }

    #[inline(always)]
    fn try_detach(&mut self) {
        if !self._ptr.is_null() {
            M::release(self._ptr);
            self._ptr = ptr::null_mut();
        }
    }
}

impl<T, M: BSTSmartPointerManager<T>> Clone for BSTSmartPointer<T, M> {
    /// Cloning acquires the reference (matches C++ copy constructor).
    #[inline(always)]
    fn clone(&self) -> Self {
        self.try_attach();
        Self {
            _ptr: self._ptr,
            _marker: PhantomData,
        }
    }
}

impl<T, M: BSTSmartPointerManager<T>> Drop for BSTSmartPointer<T, M> {
    /// Dropping releases the reference (matches C++ destructor).
    #[inline(always)]
    fn drop(&mut self) {
        self.try_detach();
    }
}

impl<T, M: BSTSmartPointerManager<T>> Deref for BSTSmartPointer<T, M> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        debug_assert!(!self._ptr.is_null());
        unsafe { &*self._ptr }
    }
}

impl<T, M: BSTSmartPointerManager<T>> DerefMut for BSTSmartPointer<T, M> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        debug_assert!(!self._ptr.is_null());
        unsafe { &mut *self._ptr }
    }
}

impl<T, M: BSTSmartPointerManager<T>> PartialEq for BSTSmartPointer<T, M> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self._ptr == other._ptr
    }
}

impl<T, M: BSTSmartPointerManager<T>> Eq for BSTSmartPointer<T, M> {}

impl<T, M: BSTSmartPointerManager<T>> Default for BSTSmartPointer<T, M> {
    #[inline(always)]
    fn default() -> Self {
        Self::null()
    }
}

/// C++ `RE::BSTAutoPointer<T>`.
pub type BSTAutoPointer<T> = BSTSmartPointer<T, BSTSmartPointerAutoPtr>;

impl BSTSmartPointerAutoDeletable for Dummy {
    unsafe fn bst_delete(&self) {}
}

const _: () = assert!(core::mem::size_of::<BSTAutoPointer<Dummy>>() == 0x8);
