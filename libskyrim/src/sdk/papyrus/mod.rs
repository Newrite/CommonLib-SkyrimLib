//! High-level Papyrus authoring layer.
//!
//! This module intentionally supports three different authoring styles, because
//! real SKSE plugins use all three:
//!
//! - direct script binding through [`PapyrusScript`], [`Registry`], and
//!   [`register_script`]
//! - richer module/class binding through [`PapyrusModule`],
//!   [`ModuleRegistry`], and `papyrus_module!`
//! - persistent `RegisterFor...`-style event sets through
//!   [`PapyrusEventRegistry`], [`PapyrusTargetedEventRegistry`],
//!   [`register_event_set`], and the event-module macros
//!
//! Choose the lightest layer that matches your plugin:
//!
//! - one script name plus a few functions:
//!   [`PapyrusScript`] / [`register_script`]
//! - multiple classes or a more declarative module surface:
//!   [`PapyrusModule`] / `papyrus_module!`
//! - installed persistent event registries that survive save/load/revert:
//!   the [`runtime`] and [`events`] layers
//!
//! Callback-side wrappers such as [`GameRef`], [`GamePtr`], [`PapyrusRef`],
//! [`Context`], and [`LatentContext`] keep plugin-facing signatures ergonomic
//! without hiding the underlying engine contracts.
//!
//! Decision guide:
//!
//! - start with [`PapyrusScript`] when the plugin still looks like
//!   classic `Bind(VM*) -> RegisterFunction(...)`
//! - move to [`PapyrusModule`] when registration should stay grouped by
//!   subsystem or class instead of by one script type
//! - add [`PapyrusEventRegistry`] or [`PapyrusTargetedEventRegistry`] when
//!   Papyrus owns registrations that must survive save/load/revert/delete
//! - use [`crate::sdk::plugin::task`] together with this module when callbacks
//!   should enqueue gameplay work rather than mutate the game directly inside
//!   the Papyrus callback
//!
//! It is normal for one plugin to use more than one Papyrus layer at once:
//! for example, a direct script binding for utility calls plus a persistent
//! event registry for `RegisterFor...` style subscriptions.

pub mod context;
pub mod events;
pub mod macros;
pub mod registry;
pub mod runtime;
pub mod types;

pub use crate::skse::RegistrationEventArgs;
pub use context::{Context, LatentContext};
pub use events::{
    PapyrusEventCollection, PapyrusEventLoadStatus, PapyrusEventRecord, PapyrusEventRegistry,
    PapyrusPersistentEventRegistry, PapyrusTargetedEventRegistry, PapyrusUniqueEventRegistry,
    form_delete_registries, load_record, revert_registries, save_registries,
};
pub use macros::*;
pub use registry::{
    ClassRegistry, FunctionOptions, ModuleRegistry, NamedClassRegistry, PapyrusClass,
    PapyrusModule, PapyrusScript, Registry, register, register_module, register_script,
};
pub use runtime::{
    PapyrusEventAccessError, PapyrusEventRegistrationError, PapyrusEventRuntimeError,
    PapyrusEventSet, PapyrusEventUniqueId, has_registered_event_set, last_runtime_error,
    register_event_set, registered_event_set_unique_id, take_last_runtime_error,
    unregister_event_set, with_registered_event_set, with_registered_event_set_mut,
    with_registered_events, with_registered_events_mut,
};
pub use types::{
    GamePtr, GameRef, PapyrusMethodFunctionSignature, PapyrusMethodLatentFunctionSignature,
    PapyrusMethodLongFunctionSignature, PapyrusRef, PapyrusStaticFunctionSignature,
    PapyrusStaticLatentFunctionSignature, PapyrusStaticLongFunctionSignature, UserPapyrusBase,
    UserPapyrusParameter,
};
