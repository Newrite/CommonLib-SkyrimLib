//! Dynamic cast helpers shared across SDK domains.

use crate::re::{
    BSTSmartPointer, BSTSmartPointerManager, GPtr, GPtrTarget, HkRef, NiPointer, NiRef, hkRefPtr,
};
use crate::relocation::{RttiType, skyrim_cast, skyrim_cast_const};

use super::handles::{HandleFamilyTarget, Resolved, ResolvedHandle};
use super::owners::NativeOwner;
use super::refs::{GamePtr, GameRef};
use super::sealed;

/// Read-only cast source backed by Skyrim RTTI.
pub trait ConstRttiCastSource: sealed::Sealed {
    type Source: RttiType;

    fn raw_const_source_ptr(&self) -> *const Self::Source;
}

/// Mutable cast source backed by Skyrim RTTI.
pub trait MutRttiCastSource: ConstRttiCastSource {
    fn raw_mut_source_ptr(&mut self) -> *mut Self::Source;
}

/// Shared read-only RTTI cast helpers.
pub trait DynamicCastExt: ConstRttiCastSource {
    #[inline(always)]
    fn cast_const<Target>(&self) -> *const Target
    where
        Target: RttiType,
    {
        unsafe { skyrim_cast_const::<Self::Source, Target>(self.raw_const_source_ptr()) }
    }

    #[inline(always)]
    fn can_cast<Target>(&self) -> bool
    where
        Target: RttiType,
    {
        !self.cast_const::<Target>().is_null()
    }

    #[inline(always)]
    fn try_cast_ref<Target>(&self) -> Option<&Target>
    where
        Target: RttiType,
    {
        unsafe { self.cast_const::<Target>().as_ref() }
    }
}

impl<T> DynamicCastExt for T where T: ConstRttiCastSource {}

/// Shared mutable RTTI cast helpers for sources that can honestly provide
/// mutable access to the pointee.
pub trait DynamicCastMutExt: MutRttiCastSource {
    #[inline(always)]
    fn cast_raw<Target>(&mut self) -> *mut Target
    where
        Target: RttiType,
    {
        unsafe { skyrim_cast::<Self::Source, Target>(self.raw_mut_source_ptr()) }
    }

    #[inline(always)]
    fn try_cast_mut<Target>(&mut self) -> Option<&mut Target>
    where
        Target: RttiType,
    {
        unsafe { self.cast_raw::<Target>().as_mut() }
    }
}

impl<T> DynamicCastMutExt for T where T: MutRttiCastSource {}

impl<T> sealed::Sealed for T where T: RttiType {}

impl<T> ConstRttiCastSource for T
where
    T: RttiType,
{
    type Source = T;

    #[inline(always)]
    fn raw_const_source_ptr(&self) -> *const Self::Source {
        self as *const T
    }
}

impl<T> MutRttiCastSource for T
where
    T: RttiType,
{
    #[inline(always)]
    fn raw_mut_source_ptr(&mut self) -> *mut Self::Source {
        self as *mut T
    }
}

impl<T> sealed::Sealed for GameRef<T> where T: RttiType {}

impl<T> ConstRttiCastSource for GameRef<T>
where
    T: RttiType,
{
    type Source = T;

    #[inline(always)]
    fn raw_const_source_ptr(&self) -> *const Self::Source {
        self.as_ptr().cast_const()
    }
}

impl<T> sealed::Sealed for GamePtr<T> where T: RttiType {}

impl<T> ConstRttiCastSource for GamePtr<T>
where
    T: RttiType,
{
    type Source = T;

    #[inline(always)]
    fn raw_const_source_ptr(&self) -> *const Self::Source {
        self.as_ptr()
    }
}

impl<T> ConstRttiCastSource for NiPointer<T>
where
    T: NiRef + RttiType,
{
    type Source = T;

    #[inline(always)]
    fn raw_const_source_ptr(&self) -> *const Self::Source {
        self.get().cast_const()
    }
}

impl<T> ConstRttiCastSource for GPtr<T>
where
    T: GPtrTarget + RttiType,
{
    type Source = T;

    #[inline(always)]
    fn raw_const_source_ptr(&self) -> *const Self::Source {
        self.get().cast_const()
    }
}

impl<T, M> ConstRttiCastSource for BSTSmartPointer<T, M>
where
    T: RttiType,
    M: BSTSmartPointerManager<T>,
{
    type Source = T;

    #[inline(always)]
    fn raw_const_source_ptr(&self) -> *const Self::Source {
        self.get().cast_const()
    }
}

impl<T> ConstRttiCastSource for hkRefPtr<T>
where
    T: HkRef + RttiType,
{
    type Source = T;

    #[inline(always)]
    fn raw_const_source_ptr(&self) -> *const Self::Source {
        self.get().cast_const()
    }
}

impl<H, O> ConstRttiCastSource for ResolvedHandle<H, O>
where
    H: super::handles::ResolvableHandle,
    O: NativeOwner,
    O::Target: RttiType,
{
    type Source = O::Target;

    #[inline(always)]
    fn raw_const_source_ptr(&self) -> *const Self::Source {
        self.as_ptr().cast_const()
    }
}

impl<H, O> sealed::Sealed for ResolvedHandle<H, O>
where
    H: super::handles::ResolvableHandle,
    O: NativeOwner,
    O::Target: RttiType,
{
}

impl<T> ConstRttiCastSource for Resolved<T>
where
    T: HandleFamilyTarget,
{
    type Source = T;

    #[inline(always)]
    fn raw_const_source_ptr(&self) -> *const Self::Source {
        self.as_ptr().cast_const()
    }
}

impl<T> MutRttiCastSource for Resolved<T>
where
    T: HandleFamilyTarget,
{
    #[inline(always)]
    fn raw_mut_source_ptr(&mut self) -> *mut Self::Source {
        self.as_ptr()
    }
}

impl<T> sealed::Sealed for Resolved<T> where T: HandleFamilyTarget {}
