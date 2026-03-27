//! High-level Rust plugin entry helpers.
//!
//! This module is intended to wrap the most common `LoadInterface` setup paths
//! without replacing the low-level exported ABI in `libskyrim::skse::loader`.

pub use crate::skse::{LoadInterface, PluginHandle, PluginInfo};

#[inline(always)]
pub fn init(load_interface: &LoadInterface) {
    unsafe {
        crate::skse::init(load_interface);
    }
}

#[inline(always)]
pub fn init_with_log(load_interface: &LoadInterface, log: bool) {
    unsafe {
        crate::skse::init_with_log(load_interface, log);
    }
}

#[inline(always)]
pub fn alloc_trampoline(size: usize) {
    crate::skse::alloc_trampoline(size);
}
