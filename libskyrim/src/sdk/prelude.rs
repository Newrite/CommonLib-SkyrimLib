//! Small, intentionally conservative prelude for common `sdk` entry points.
//!
//! This module should stay compact. It exists to cover frequent plugin author
//! workflows without dragging large advanced surfaces into scope by default.
//!
//! The prelude is aimed at the "ordinary plugin layer":
//!
//! - common pointer / handle / cast vocabulary from [`crate::sdk::core`]
//! - event helpers from [`crate::sdk::events`]
//! - the most common Papyrus authoring entry points
//! - basic background/UI task queue helpers
//!
//! It intentionally does *not* pull in large gameplay, UI runtime, interop, or
//! advanced engine-facing surfaces. Those stay opt-in so modules can be honest
//! about which domain they depend on.
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::prelude::*;
//! use libskyrim::re::StaticFunctionTag;
//!
//! fn install_papyrus() {
//!     struct ExampleScript;
//!
//!     impl PapyrusScript for ExampleScript {
//!         const NAME: &'static str = "Example:Plugin";
//!
//!         fn register(registry: &mut Registry<'_>) {
//!             registry.register_static_function(
//!                 "Ping",
//!                 |_: &mut StaticFunctionTag| true,
//!             );
//!         }
//!     }
//!
//!     let _ = register_script::<ExampleScript>();
//!     add_task(|| {});
//! }
//! ```

pub use crate::sdk::core::{
    DynamicCastExt, DynamicCastMutExt, GameLifecyclePhase, GamePtr, GameRef, HandleFamilyTarget,
    HandleTarget, LifecyclePhase, NativeOwner, NativeOwnerCastExt, PluginLifecyclePhase,
    ResolvableHandle, Resolved, ResolvedHandle,
};
pub use crate::sdk::events::{EventFlow, EventSourceExt};
pub use crate::sdk::papyrus::{
    ClassRegistry, Context, FunctionOptions, LatentContext, ModuleRegistry, NamedClassRegistry,
    PapyrusClass, PapyrusModule, PapyrusScript, Registry, register, register_module,
    register_script,
};
pub use crate::sdk::plugin::{LogLevel, add_task, add_ui_task};
