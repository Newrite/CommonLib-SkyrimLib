//! Core building blocks shared across multiple SDK domains.

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
