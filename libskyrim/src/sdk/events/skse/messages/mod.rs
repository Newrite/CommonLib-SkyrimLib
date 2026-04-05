//! High-level helpers for SKSE plugin messaging and lifecycle listeners.
//!
//! This domain remains distinct from ordinary `BSTEventSource<T>` subscriptions
//! because `MessagingInterface::RegisterListener` is install-once style and
//! does not offer the same RAII removal semantics as engine event sinks.
//!
//! Most plugins only need one of two entry styles:
//!
//! - [`on`] / [`on_sender`] when listening to one concrete [`MessageKind`]
//! - [`on_plugin_phase`] / [`on_game_lifecycle`] when the plugin wants the
//!   higher-level SDK lifecycle vocabulary from [`crate::sdk::core::phase`]
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::core::{GameLifecyclePhase, PluginLifecyclePhase};
//! use libskyrim::sdk::events::skse::messages;
//!
//! fn install_lifecycle_hooks() {
//!     let _plugin = messages::on_plugin_phase(PluginLifecyclePhase::DataLoaded, |_message| {
//!         // late bootstrap
//!     });
//!
//!     let _game = messages::on_game_lifecycle(GameLifecyclePhase::PostLoadGame, |_message| {
//!         // save-dependent refresh
//!     });
//! }
//! ```

mod listen;
mod types;

pub use listen::{
    on, on_filtered, on_game_lifecycle, on_game_lifecycle_filtered, on_game_lifecycle_raw,
    on_game_lifecycle_sender, on_game_lifecycle_sender_str, on_lifecycle, on_lifecycle_filtered,
    on_lifecycle_raw, on_lifecycle_sender, on_lifecycle_sender_str, on_plugin_phase,
    on_plugin_phase_filtered, on_plugin_phase_raw, on_plugin_phase_sender,
    on_plugin_phase_sender_str, on_raw, on_sender, on_sender_str,
};
pub use types::{MessageKind, MessageListener, MessageRef, TypedMessageRef, TypedMessageSliceRef};

#[cfg(test)]
mod tests;
