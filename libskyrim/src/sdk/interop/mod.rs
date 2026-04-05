//! Inter-plugin and external API interoperability helpers.
//!
//! This domain covers the recurring ways Skyrim plugins talk to each other:
//!
//! - SKSE messaging listeners and request/response protocols in [`messaging`]
//! - DLL-export/API negotiation in [`external_api`]
//!
//! The current `external_api` surface intentionally supports several real
//! plugin patterns:
//!
//! - `RequestPluginAPI`-style versioned interface loaders
//! - messaging-driven interface negotiation
//! - flat exported symbol lookup
//! - callback/subscriber registration over flat exported functions
//!
//! Reach for this module whenever your plugin depends on another plugin's API
//! or wants to expose one of its own in a reusable shape.

pub mod external_api;
pub mod messaging;
