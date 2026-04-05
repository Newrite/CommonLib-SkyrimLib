//! High-level messaging composition over `SKSE::MessagingInterface`.
//!
//! This layer should support ergonomic listener registration while preserving
//! access to the low-level messaging surface for advanced plugins.
//!
//! Reach for this module when plugin code genuinely cares about messaging
//! transport details:
//!
//! - sender filtering
//! - raw message kinds
//! - low-level payload access
//! - listener registration that is not purely lifecycle-oriented
//!
//! Decision guide:
//!
//! - use `sdk::plugin::lifecycle` when the plugin only cares about familiar
//!   phases like `PostLoad`, `DataLoaded`, `SaveGame`, or `PostLoadGame`
//! - use this module when the plugin cares about who sent the message, wants
//!   filtered/raw listeners, or needs a lower-level `MessageKind`/payload view
//! - use `sdk::interop::messaging` when the plugin is building a reusable
//!   plugin-to-plugin request/response protocol rather than just reacting to
//!   SKSE lifecycle notifications
//!
//! Common flow:
//!
//! 1. install one typed or raw listener with `on*`
//! 2. inspect sender or payload through [`MessageRef`]
//! 3. hand heavier work off through `sdk::plugin::task` when the callback
//!    should stay lightweight

/// Lifecycle-phase aliases commonly used by messaging listeners.
///
/// These are re-exported here so plugin code can stay within the `sdk::plugin`
/// namespace when it only needs to classify incoming SKSE lifecycle messages.
pub use crate::sdk::core::{GameLifecyclePhase, LifecyclePhase, PluginLifecyclePhase};

/// High-level listener helpers layered over `SKSE::MessagingInterface`.
///
/// The `on_*` family is the usual starting point. Reach for the typed
/// `MessageRef` wrappers when the plugin needs sender filtering or payload
/// inspection but does not want to interact with the raw SKSE API directly.
pub use crate::sdk::events::skse::messages::{
    MessageKind, MessageListener, MessageRef, TypedMessageRef, TypedMessageSliceRef, on,
    on_filtered, on_game_lifecycle, on_game_lifecycle_filtered, on_game_lifecycle_raw,
    on_game_lifecycle_sender, on_game_lifecycle_sender_str, on_lifecycle, on_lifecycle_filtered,
    on_lifecycle_raw, on_lifecycle_sender, on_lifecycle_sender_str, on_plugin_phase,
    on_plugin_phase_filtered, on_plugin_phase_raw, on_plugin_phase_sender,
    on_plugin_phase_sender_str, on_raw, on_sender, on_sender_str,
};

/// Low-level SKSE messaging registration and transport primitives.
///
/// These remain available for plugins that need to bridge existing raw SKSE
/// code or install listeners outside the higher-level `on_*` helper family.
pub use crate::skse::{
    Message, PluginHandle, plugin_handle, register_listener, register_listener_dyn,
};
