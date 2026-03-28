//! High-level Papyrus-facing Rust types and conversion wrappers.

pub use crate::sdk::core::{GamePtr, GameRef};
pub use crate::skse::papyrus::{
    PapyrusMethodFunctionSignature, PapyrusMethodLatentFunctionSignature,
    PapyrusMethodLongFunctionSignature, PapyrusRef, PapyrusStaticFunctionSignature,
    PapyrusStaticLatentFunctionSignature, PapyrusStaticLongFunctionSignature, UserPapyrusBase,
    UserPapyrusParameter,
};
