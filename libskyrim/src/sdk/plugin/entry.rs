//! High-level Rust plugin entry helpers.
//!
//! This module is intended to wrap the most common `LoadInterface` setup paths
//! without replacing the low-level exported ABI in `libskyrim::skse::loader`.
//!
//! Decision guide:
//!
//! - call [`init`] when the plugin wants the normal SDK/bootstrap path without
//!   forcing log initialization
//! - call [`init_with_log`] when bootstrap should also explicitly opt in or out
//!   of the logging setup path
//! - call [`alloc_trampoline`] after initialization when the plugin owns manual
//!   hooks that need trampoline storage
//!
//! Typical `SKSEPluginLoad` flow:
//!
//! 1. receive [`LoadInterface`]
//! 2. call [`init`] or [`init_with_log`]
//! 3. optionally call [`alloc_trampoline`]
//! 4. move on to `sdk::plugin::lifecycle`, `sdk::plugin::config`, and later
//!    installation code
//!
//! This module deliberately stays tiny: once bootstrap is complete, most
//! plugin code should move upward into `sdk::plugin::lifecycle`,
//! `sdk::plugin::task`, `sdk::plugin::config`, or `sdk::hooks`.

pub use crate::skse::{LoadInterface, PluginHandle, PluginInfo};

/// Initialize the SKSE runtime services for the current plugin load.
///
/// This is the normal entry helper when the plugin wants the SDK/bootstrap path
/// but does not need to override logging behavior explicitly.
#[inline(always)]
pub fn init(load_interface: &LoadInterface) {
    unsafe {
        crate::skse::init(load_interface);
    }
}

/// Initialize the SKSE runtime services and explicitly choose whether to
/// install SDK logging support.
///
/// Use `log = false` when the plugin wants to keep bootstrap minimal or owns
/// its logging setup separately.
#[inline(always)]
pub fn init_with_log(load_interface: &LoadInterface, log: bool) {
    unsafe {
        crate::skse::init_with_log(load_interface, log);
    }
}

/// Reserve trampoline storage for manual hook installation.
///
/// Call this during plugin load before installing manual trampoline/thunk-based
/// hooks. Pure attribute-hook or message-only plugins often do not need it.
#[inline(always)]
pub fn alloc_trampoline(size: usize) {
    crate::skse::alloc_trampoline(size);
}
