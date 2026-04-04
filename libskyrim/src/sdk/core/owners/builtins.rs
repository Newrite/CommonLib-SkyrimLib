use crate::re::{
    BSTSmartPointer, BSTSmartPointerManager, GPtr, GPtrTarget, HkRef, NiPointer, NiRef, hkRefPtr,
};

use super::super::sealed;
use super::traits::{NativeOwner, NativeOwnerFamily};

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
