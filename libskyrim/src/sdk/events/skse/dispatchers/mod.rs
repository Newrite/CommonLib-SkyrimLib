//! High-level helpers for SKSE dispatcher-backed `BSTEventSource<T>`.
//!
//! This domain is intended for events retrieved through the SKSE API storage,
//! for example `ModCallbackEvent`, `CameraEvent`, `CrosshairRefEvent`,
//! `ActionEvent`, and `NiNodeUpdateEvent`.
//!
//! Compared with [`super::messages`], these subscriptions behave like ordinary
//! event sinks: they attach to dispatcher-backed `BSTEventSource<T>` values and
//! are represented by sink/registration objects instead of install-once
//! listeners.
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::re::ActionEvent;
//! use libskyrim::sdk::events::{EventFlow, skse::dispatchers};
//!
//! fn install_action_listener() {
//!     let _listener = dispatchers::subscribe(|_event: &ActionEvent| EventFlow::Continue);
//! }
//! ```

mod api;
mod registry;

pub use api::{prepend, subscribe};
pub use registry::DispatcherEvent;
