use crate::relocation::{RttiType, skyrim_cast, skyrim_cast_const};

use super::super::sealed;

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
