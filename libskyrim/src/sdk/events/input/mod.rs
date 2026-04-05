//! Ergonomic wrappers over `BSInputDeviceManager` and `InputEvent*` chains.
//!
//! This module is the usual entry point when a plugin wants to observe or
//! inspect Skyrim's live input stream without working against raw
//! `InputEvent*` linked lists directly.
//!
//! The layer is intentionally split in two:
//!
//! - [`subscribe`] / [`prepend`] install one input-chain sink on
//!   `BSInputDeviceManager`
//! - [`InputEvents`] and its iterators provide borrowed traversal/filtering
//!   over the delivered chain
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::events::{EventFlow, input};
//!
//! fn install_input_listener() {
//!     let _subscription = input::subscribe(|events| {
//!         if events.buttons().any(|button| button.is_down()) {
//!             // react to at least one pressed button in this chain
//!         }
//!         EventFlow::Continue
//!     });
//! }
//! ```

mod api;
mod chain;

pub use api::{prepend, subscribe};
pub use chain::{InputEventIter, InputEventIterMut, InputEvents};

#[cfg(test)]
mod tests;
