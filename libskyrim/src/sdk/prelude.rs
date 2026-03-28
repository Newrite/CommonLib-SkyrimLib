//! Small, intentionally conservative prelude for common `sdk` entry points.
//!
//! This module should stay compact. It exists to cover frequent plugin author
//! workflows without dragging large advanced surfaces into scope by default.

pub use crate::sdk::core::{
    DynamicCastExt, DynamicCastMutExt, GameLifecyclePhase, GamePtr, GameRef, HandleFamilyTarget,
    HandleTarget, LifecyclePhase, NativeOwner, NativeOwnerCastExt, PluginLifecyclePhase,
    ResolvableHandle, Resolved, ResolvedHandle,
};
pub use crate::sdk::events::{EventFlow, EventSourceExt};
pub use crate::sdk::papyrus::{
    ClassRegistry, Context, FunctionOptions, LatentContext, ModuleRegistry, NamedClassRegistry,
    PapyrusClass, PapyrusModule, Registry, register, register_module,
};
pub use crate::sdk::plugin::{LogLevel, add_task, add_ui_task};
