//! High-level Papyrus authoring layer.

pub mod context;
pub mod macros;
pub mod registry;
pub mod types;

pub use context::{Context, LatentContext};
pub use macros::*;
pub use registry::{
    ClassRegistry, FunctionOptions, ModuleRegistry, NamedClassRegistry, PapyrusClass,
    PapyrusModule, Registry, register, register_module,
};
pub use types::{
    GameRef, GameRefMut, PapyrusMethodFunctionSignature, PapyrusMethodLatentFunctionSignature,
    PapyrusMethodLongFunctionSignature, PapyrusRef, PapyrusStaticFunctionSignature,
    PapyrusStaticLatentFunctionSignature, PapyrusStaticLongFunctionSignature, UserPapyrusBase,
    UserPapyrusParameter,
};
