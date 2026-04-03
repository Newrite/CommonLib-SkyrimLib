//! High-level Papyrus authoring layer.

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
    PapyrusModule, Registry, register, register_module,
};
pub use runtime::{
    PapyrusEventAccessError, PapyrusEventRegistrationError, PapyrusEventRuntimeError,
    PapyrusEventSet, PapyrusEventUniqueId, is_registered, last_error, register_event_set,
    registered_unique_id, take_last_error, unregister_event_set, with_event_set,
    with_event_set_mut, with_events, with_events_mut,
};
pub use types::{
    GamePtr, GameRef, PapyrusMethodFunctionSignature, PapyrusMethodLatentFunctionSignature,
    PapyrusMethodLongFunctionSignature, PapyrusRef, PapyrusStaticFunctionSignature,
    PapyrusStaticLatentFunctionSignature, PapyrusStaticLongFunctionSignature, UserPapyrusBase,
    UserPapyrusParameter,
};
