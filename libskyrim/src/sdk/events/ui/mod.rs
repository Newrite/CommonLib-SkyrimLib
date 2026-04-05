//! High-level helpers over `UI`-owned `BSTEventSource<T>` registrations.
//!
//! This domain is intentionally separate from gameplay and SKSE event
//! dispatchers because the owner, lifecycle, and registration timing differ.
//!
//! Use this when the plugin wants menu, HUD, cursor, or UI-state notifications
//! exposed through the engine's `UI` singleton.
//!
//! Compared with neighboring event domains:
//!
//! - prefer `ui` over [`crate::sdk::events::game`] when the source is menu/UI
//!   state rather than gameplay state
//! - prefer `ui` over [`crate::sdk::events::source`] when the plugin does not
//!   already own a raw `BSTEventSource<T>` pointer
//! - prefer `ui` over [`crate::sdk::events::skse::messages`] when the event is
//!   an engine-owned UI source rather than an inter-plugin lifecycle message

mod api;

pub use api::{prepend, subscribe};
