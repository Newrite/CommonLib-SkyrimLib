//! High-level messaging composition over `SKSE::MessagingInterface`.
//!
//! This layer should support ergonomic listener registration while preserving
//! access to the low-level messaging surface for advanced plugins.

pub use crate::sdk::core::{GameLifecyclePhase, LifecyclePhase, PluginLifecyclePhase};
pub use crate::sdk::events::skse::messages::{
    MessageKind, MessageListener, MessageRef, TypedMessageRef, TypedMessageSliceRef, on,
    on_filtered, on_game_lifecycle, on_game_lifecycle_filtered, on_game_lifecycle_raw,
    on_game_lifecycle_sender, on_game_lifecycle_sender_str, on_lifecycle, on_lifecycle_filtered,
    on_lifecycle_raw, on_lifecycle_sender, on_lifecycle_sender_str, on_plugin_phase,
    on_plugin_phase_filtered, on_plugin_phase_raw, on_plugin_phase_sender,
    on_plugin_phase_sender_str, on_raw, on_sender, on_sender_str,
};
pub use crate::skse::{
    Message, PluginHandle, plugin_handle, register_listener, register_listener_dyn,
};
