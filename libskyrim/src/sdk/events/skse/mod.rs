//! SKSE-owned event helper domains.
//!
//! The SKSE layer exposes two distinct event systems:
//!
//! - dispatcher-backed `BSTEventSource<T>` values such as `ActionEvent`
//! - plugin messaging listeners through `MessagingInterface`
//!
//! These stay separate here because they have different ownership, lifetime,
//! and unregister semantics.

pub mod dispatchers;
pub mod messages;
