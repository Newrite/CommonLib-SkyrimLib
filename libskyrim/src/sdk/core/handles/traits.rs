use crate::relocation::RttiType;

use super::super::owners::{NativeOwner, NativeOwnerFamily};
use super::super::sealed;
use super::resolved::{Resolved, ResolvedHandle};

/// Trait for native handle types that can resolve into a native owner.
pub trait ResolvableHandle: Copy + sealed::Sealed {
    type Target;
    type Owner: NativeOwner<Target = Self::Target>;

    fn is_null(self) -> bool;

    fn resolve_owned(self) -> Option<Self::Owner>;

    #[inline(always)]
    fn resolve(self) -> Option<ResolvedHandle<Self>>
    where
        Self: Sized,
    {
        ResolvedHandle::from_handle(self)
    }

    #[inline(always)]
    fn resolve_cast<U>(
        self,
    ) -> Option<ResolvedHandle<Self, <Self::Owner as NativeOwnerFamily<U>>::Owner>>
    where
        Self: Sized,
        Self::Target: RttiType,
        U: RttiType,
        Self::Owner: Clone + NativeOwnerFamily<U>,
    {
        self.resolve()?.try_cast::<U>()
    }
}

/// Trait for canonical Bethesda handles that can be reconstructed from a raw
/// target pointer.
pub trait CanonicalHandle: ResolvableHandle + sealed::Sealed {
    fn from_target_ptr(ptr: *mut Self::Target) -> Self;
}

/// Trait for engine object types that have a canonical Bethesda handle family.
pub trait HandleTarget: sealed::Sealed + Sized {
    type Handle: ResolvableHandle<Target = Self>;

    fn canonical_handle(ptr: *mut Self) -> Self::Handle;

    #[inline(always)]
    fn resolve_handle(ptr: *mut Self) -> Option<ResolvedHandle<Self::Handle>> {
        if ptr.is_null() {
            return None;
        }

        Self::canonical_handle(ptr).resolve()
    }
}

/// Trait for runtime object types that belong to one of the supported
/// Bethesda handle families.
///
/// This is intentionally wider than `HandleTarget`: family members such as
/// `Character`, `Hazard`, or `ArrowProjectile` can map themselves onto a
/// canonical root object family without pretending to own a bespoke handle
/// type.
pub trait HandleFamilyTarget: sealed::Sealed + RttiType + Sized {
    type CanonicalTarget: HandleTarget<Handle = Self::Handle> + RttiType;
    type Handle: CanonicalHandle<Target = Self::CanonicalTarget>;

    /// Convert a pointer to `Self` into the canonical root pointer used by the
    /// corresponding Bethesda handle family.
    fn to_canonical_ptr(ptr: *mut Self) -> *mut Self::CanonicalTarget;

    #[inline(always)]
    fn canonical_handle(ptr: *mut Self) -> Self::Handle {
        Self::Handle::from_target_ptr(Self::to_canonical_ptr(ptr))
    }

    #[inline(always)]
    fn resolve(ptr: *mut Self) -> Option<Resolved<Self>> {
        if ptr.is_null() {
            return None;
        }

        Resolved::from_ptr(ptr)
    }
}

#[macro_export]
macro_rules! impl_handle_family_target {
    ($ty:path => $canonical:path, $handle:path) => {
        impl $crate::sdk::core::HandleFamilyTarget for $ty {
            type CanonicalTarget = $canonical;
            type Handle = $handle;

            #[inline(always)]
            fn to_canonical_ptr(ptr: *mut Self) -> *mut Self::CanonicalTarget {
                ptr.cast::<$canonical>()
            }
        }
    };
}
