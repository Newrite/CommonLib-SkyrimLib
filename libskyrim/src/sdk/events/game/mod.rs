//! High-level helpers over `ScriptEventSourceHolder`-style gameplay events.
//!
//! Use this domain when the plugin wants ordinary gameplay event sinks such as
//! hit, animation, sleep, quest, or actor-action style notifications that are
//! owned by the engine's global `ScriptEventSourceHolder`.
//!
//! Compared with neighboring event domains:
//!
//! - prefer `game` over [`crate::sdk::events::ui`] when the source is gameplay
//!   state rather than UI state
//! - prefer `game` over [`crate::sdk::events::source`] when the plugin does
//!   not already own a concrete `BSTEventSource<T>` pointer
//! - prefer `game` over [`crate::sdk::events::skse::dispatchers`] when the
//!   event family comes from engine gameplay systems rather than SKSE
//!   dispatchers

mod api;

pub use api::{prepend, subscribe};
