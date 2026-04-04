use crate::re::{
    BSTSmartPointer, BSTSmartPointerManager, GPtr, GPtrTarget, HkRef, NiPointer, NiRef, hkRefPtr,
};
use crate::relocation::RttiType;

use super::super::refs::{GamePtr, GameRef};
use super::super::sealed;
use super::traits::ConstRttiCastSource;

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
