//! High-level messaging composition over `SKSE::MessagingInterface`.
//!
//! This layer should support ergonomic listener registration while preserving
//! access to the low-level messaging surface for advanced plugins.

pub use crate::skse::{Message, PluginHandle, plugin_handle, register_listener};
