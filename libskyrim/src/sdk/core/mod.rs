//! Core building blocks shared across multiple SDK domains.

pub(crate) mod sealed {
    pub trait Sealed {}
}

pub mod cast;
pub mod error;
pub mod handles;
pub mod owners;
pub mod phase;
pub mod ptr;
pub mod refs;

pub use cast::{ConstRttiCastSource, DynamicCastExt, DynamicCastMutExt, MutRttiCastSource};
pub use handles::{
    CanonicalHandle, HandleFamilyTarget, HandleTarget, ResolvableHandle, Resolved, ResolvedHandle,
};
pub use owners::{NativeOwner, NativeOwnerCastExt, NativeOwnerFamily};
pub use phase::{GameLifecyclePhase, LifecyclePhase, PluginLifecyclePhase};
pub use refs::{GameRef, GameRefMut};
