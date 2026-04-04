use crate::relocation::RttiType;

use super::super::handles::{HandleFamilyTarget, ResolvableHandle, Resolved, ResolvedHandle};
use super::super::owners::NativeOwner;
use super::super::sealed;
use super::traits::{ConstRttiCastSource, MutRttiCastSource};

impl<H, O> ConstRttiCastSource for ResolvedHandle<H, O>
where
    H: ResolvableHandle,
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
    H: ResolvableHandle,
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
