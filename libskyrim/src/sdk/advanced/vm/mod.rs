//! Advanced script VM and BSScript helpers.
//!
//! This layer is intentionally lower-level than `sdk::papyrus`: it focuses on
//! live VM/runtime operations such as bound-object lookup, property access, and
//! immediate method/static dispatch.

mod access;
mod binding;
mod dispatch;
mod shared;
mod types;

pub use access::*;
pub use binding::*;
pub use dispatch::*;
pub use types::*;

#[cfg(test)]
mod tests;
