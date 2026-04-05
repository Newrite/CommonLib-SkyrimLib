//! Core building blocks shared across multiple SDK domains.
//!
//! This module is the shared vocabulary layer for the rest of the SDK. Most
//! plugin authors do not build directly on every submodule here, but almost
//! every higher-level domain uses these types to express identity, ownership,
//! casting, traversal, and lifecycle timing.
//!
//! Start here when you need one of the repeated engine-facing patterns that
//! show up across gameplay, UI, Papyrus, hooks, or interop code:
//!
//! - [`refs`] for trusted stable engine pointers through [`GameRef`] and
//!   [`GamePtr`]
//! - [`handles`] for cross-frame identity and retained runtime resolution
//! - [`owners`] for Bethesda-native smart-pointer families
//! - [`cast`] for RTTI-backed dynamic casts
//! - [`containers`] for contiguous native traversal/snapshot helpers
//! - [`phase`] for gameplay/HUD safety gates and runtime timing state
//!
//! The re-exported items in `sdk::core` are the intended front door. Reach into
//! submodules only when you need a narrower trait or helper family.
//!
//! Typical workflow:
//!
//! ```rust,ignore
//! use libskyrim::sdk::core::{
//!     GamePtr, ResolvableHandle, is_safe_for_gameplay, snapshot_runtime_phase,
//! };
//! use libskyrim::re::Actor;
//!
//! fn resolve_actor_if_safe(handle: impl ResolvableHandle<Target = Actor>) -> Option<GamePtr<Actor>> {
//!     let phase = snapshot_runtime_phase();
//!     if !phase.is_safe_for_gameplay() || !is_safe_for_gameplay() {
//!         return None;
//!     }
//!
//!     handle.resolve().map(|resolved| resolved.ptr())
//! }
//! ```
//!
//! Prefer importing from `sdk::core` directly when a subsystem needs only this
//! shared vocabulary. Reach for [`crate::sdk::prelude`] when the module also
//! wants the common event, Papyrus, or task helpers that most gameplay plugins
//! use together.

pub(crate) mod sealed {
    pub trait Sealed {}
}

pub mod cast;
pub mod containers;
pub mod error;
pub mod handles;
pub mod owners;
pub mod phase;
pub mod ptr;
pub mod refs;

pub use cast::{ConstRttiCastSource, DynamicCastExt, DynamicCastMutExt, MutRttiCastSource};
pub use containers::{
    ContiguousSequence, ContiguousSequenceIterationOptions, contiguous_sequence_bounds,
    contiguous_sequence_bounds_named, for_each_contiguous_sequence,
    for_each_contiguous_sequence_named, snapshot_contiguous_cloned,
    snapshot_contiguous_cloned_named, snapshot_contiguous_copied, snapshot_contiguous_copied_named,
};
pub use handles::{
    CanonicalHandle, HandleFamilyTarget, HandleTarget, ResolvableHandle, Resolved, ResolvedHandle,
};
pub use owners::{NativeOwner, NativeOwnerCastExt, NativeOwnerFamily};
pub use phase::{
    GameLifecyclePhase, LifecyclePhase, PluginLifecyclePhase, RuntimePhaseBlocker,
    RuntimePhaseSnapshot, allows_hud_widgets, gameplay_phase_blocker, hud_widget_phase_blocker,
    is_loading_or_fading, is_safe_for_gameplay, is_unsafe_for_gameplay, should_defer_gameplay_work,
    snapshot_runtime_phase,
};
pub use refs::{GamePtr, GameRef};
