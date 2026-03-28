//! Shared pointer and reference helpers for the SDK layer.
//!
//! The long-term goal is to centralize ergonomic nullable/reference wrappers
//! here instead of keeping domain-specific pointer adapters scattered across the
//! tree.

pub use super::cast::{ConstRttiCastSource, DynamicCastExt, DynamicCastMutExt, MutRttiCastSource};
pub use super::handles::{
    CanonicalHandle, HandleFamilyTarget, HandleTarget, ResolvableHandle, Resolved, ResolvedHandle,
};
pub use super::owners::{NativeOwner, NativeOwnerCastExt, NativeOwnerFamily};
pub use super::refs::{GamePtr, GameRef};
