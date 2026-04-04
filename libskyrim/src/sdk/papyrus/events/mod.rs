//! High-level Papyrus event registration helpers.
//!
//! This module builds a more domain-oriented SDK surface on top of
//! `skse::RegistrationSet*`:
//!
//! - event-name-first registries for form / alias / active-effect listeners
//! - optional persistent record metadata for save/load/revert flows
//! - grouped helper functions for plugin serialization callbacks
//! - queued dispatch helpers for "register now, run later" event delivery

mod collection;
mod registry;
mod targeted;
mod types;

pub use crate::skse::RegistrationEventArgs;
pub use collection::*;
pub use registry::*;
pub use targeted::*;
pub use types::*;

#[cfg(test)]
mod tests;
