use crate::relocation::{RttiType, skyrim_cast};

use super::super::sealed;

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
        if self.is_null() {
            return None;
        }

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
