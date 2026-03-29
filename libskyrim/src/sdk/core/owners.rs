//! Native smart-pointer traits shared across SDK domains.

use crate::re::{
    BSTSmartPointer, BSTSmartPointerManager, GPtr, GPtrTarget, HkRef, NiPointer, NiRef, hkRefPtr,
};
use crate::relocation::{RttiType, skyrim_cast};

use super::sealed;

/// Small shared surface over native Bethesda smart-pointer families.
pub trait NativeOwner: sealed::Sealed {
    type Target;

    /// # Safety
    /// `raw` must already carry the ownership/reference semantics expected by
    /// the concrete native owner type.
    unsafe fn from_raw(raw: *mut Self::Target) -> Self;

    fn as_ptr(&self) -> *mut Self::Target;

    fn into_raw(self) -> *mut Self::Target;

    #[inline(always)]
    fn is_null(&self) -> bool {
        self.as_ptr().is_null()
    }

    #[inline(always)]
    fn as_ref(&self) -> Option<&Self::Target> {
        unsafe { self.as_ptr().as_ref() }
    }

    #[inline(always)]
    fn as_mut(&mut self) -> Option<&mut Self::Target> {
        unsafe { self.as_ptr().as_mut() }
    }
}

/// Family relationship for native owner types that can rebind to a different
/// pointee while preserving the same ownership semantics.
pub trait NativeOwnerFamily<U>: NativeOwner + sealed::Sealed {
    type Owner: NativeOwner<Target = U>;
}

/// High-level cast helpers for native smart-pointer families.
pub trait NativeOwnerCastExt: NativeOwner {
    #[inline(always)]
    fn try_cast_owner<U>(&self) -> Option<<Self as NativeOwnerFamily<U>>::Owner>
    where
        Self: Clone + NativeOwnerFamily<U>,
        Self::Target: RttiType,
        U: RttiType,
    {
        let raw = self.clone().into_raw();
        let casted = unsafe { skyrim_cast::<Self::Target, U>(raw) };
        if casted.is_null() {
            unsafe { drop(Self::from_raw(raw)) };
            None
        } else {
            Some(unsafe {
                <<Self as NativeOwnerFamily<U>>::Owner as NativeOwner>::from_raw(casted)
            })
        }
    }
}

impl<T: NativeOwner> NativeOwnerCastExt for T {}

impl<T> sealed::Sealed for NiPointer<T> where T: NiRef {}

impl<T> NativeOwner for NiPointer<T>
where
    T: NiRef,
{
    type Target = T;

    #[inline(always)]
    unsafe fn from_raw(raw: *mut Self::Target) -> Self {
        unsafe { NiPointer::from_raw(raw) }
    }

    #[inline(always)]
    fn as_ptr(&self) -> *mut Self::Target {
        self.get()
    }

    #[inline(always)]
    fn into_raw(self) -> *mut Self::Target {
        NiPointer::into_raw(self)
    }
}

impl<T, U> NativeOwnerFamily<U> for NiPointer<T>
where
    T: NiRef,
    U: NiRef,
{
    type Owner = NiPointer<U>;
}

impl<T> sealed::Sealed for GPtr<T> where T: GPtrTarget {}

impl<T> NativeOwner for GPtr<T>
where
    T: GPtrTarget,
{
    type Target = T;

    #[inline(always)]
    unsafe fn from_raw(raw: *mut Self::Target) -> Self {
        unsafe { GPtr::from_raw(raw) }
    }

    #[inline(always)]
    fn as_ptr(&self) -> *mut Self::Target {
        self.get()
    }

    #[inline(always)]
    fn into_raw(self) -> *mut Self::Target {
        GPtr::into_raw(self)
    }
}

impl<T, U> NativeOwnerFamily<U> for GPtr<T>
where
    T: GPtrTarget,
    U: GPtrTarget,
{
    type Owner = GPtr<U>;
}

impl<T, M> sealed::Sealed for BSTSmartPointer<T, M> where M: BSTSmartPointerManager<T> {}

impl<T, M> NativeOwner for BSTSmartPointer<T, M>
where
    M: BSTSmartPointerManager<T>,
{
    type Target = T;

    #[inline(always)]
    unsafe fn from_raw(raw: *mut Self::Target) -> Self {
        unsafe { BSTSmartPointer::from_raw(raw) }
    }

    #[inline(always)]
    fn as_ptr(&self) -> *mut Self::Target {
        self.get()
    }

    #[inline(always)]
    fn into_raw(self) -> *mut Self::Target {
        BSTSmartPointer::into_raw(self)
    }
}

impl<T, U, M> NativeOwnerFamily<U> for BSTSmartPointer<T, M>
where
    M: BSTSmartPointerManager<T> + BSTSmartPointerManager<U>,
{
    type Owner = BSTSmartPointer<U, M>;
}

impl<T> sealed::Sealed for hkRefPtr<T> where T: HkRef {}

impl<T> NativeOwner for hkRefPtr<T>
where
    T: HkRef,
{
    type Target = T;

    #[inline(always)]
    unsafe fn from_raw(raw: *mut Self::Target) -> Self {
        unsafe { hkRefPtr::from_raw(raw) }
    }

    #[inline(always)]
    fn as_ptr(&self) -> *mut Self::Target {
        self.get()
    }

    #[inline(always)]
    fn into_raw(self) -> *mut Self::Target {
        hkRefPtr::into_raw(self)
    }
}

impl<T, U> NativeOwnerFamily<U> for hkRefPtr<T>
where
    T: HkRef,
    U: HkRef,
{
    type Owner = hkRefPtr<U>;
}
