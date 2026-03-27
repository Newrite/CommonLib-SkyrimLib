//!
//! @file lib.rs
//! @author Andrew Spaulding (Kasplat) / Updated for CommonLib-NG
//! @brief Module runtime loader and environment for libskyrim.
//!

#![no_std]

extern crate alloc;

pub mod ffi;
pub mod ini;
pub mod offsets;
pub mod re;
pub mod relocation;
pub mod rex;
pub mod runtime;
pub mod skse;
pub mod version;

pub extern crate core;
pub extern crate core_util;

use crate::skse::{LoadInterface, PluginDeclaration, PluginInfo};

unsafe extern "Rust" {
    fn skse_plugin_rust_entry(skse: &LoadInterface) -> Result<(), ()>;
    pub(crate) static SKSEPlugin_Version: PluginDeclaration;
}

#[cfg(not(test))]
pub fn panic_runtime(info: &core::panic::PanicInfo<'_>) -> ! {
    crate::skse::log::fatal_runtime(format_args!("panic: {info}"))
}

#[cfg(all(not(test), feature = "panic-handler"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    panic_runtime(info)
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn SKSEPlugin_Query(
    skse: *const LoadInterface,
    info: *mut PluginInfo,
) -> bool {
    unsafe { crate::skse::loader::query(skse, info) }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn SKSEPlugin_Load(skse: *const LoadInterface) -> bool {
    unsafe { crate::skse::loader::load(skse) }
}
