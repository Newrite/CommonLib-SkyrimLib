//!
//! @file lib.rs
//! @author Andrew Spaulding (Kasplat) / Updated for CommonLib-NG
//! @brief Module runtime loader and environment for libskyrim.
//!

#![no_std]

extern crate alloc;
extern crate self as libskyrim;
#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "std", feature = "panic-handler"))]
compile_error!("`libskyrim` features `std` and `panic-handler` are mutually exclusive.");

pub mod ffi;
pub mod ini;
pub mod offsets;
pub mod re;
pub mod relocation;
pub mod rex;
pub mod runtime;
pub mod safety;
pub mod sdk;
pub mod skse;
pub mod version;

pub extern crate core;
pub extern crate core_util;

#[cfg(not(test))]
use crate::skse::PluginInfo;
use crate::skse::{LoadInterface, PluginDeclaration};

#[cfg(not(test))]
unsafe extern "Rust" {
    fn skse_plugin_rust_entry(skse: &LoadInterface) -> Result<(), ()>;
    pub(crate) static SKSEPlugin_Version: PluginDeclaration;
}

#[cfg(test)]
unsafe fn skse_plugin_rust_entry(_skse: &LoadInterface) -> Result<(), ()> {
    Ok(())
}

#[cfg(test)]
#[allow(non_upper_case_globals)]
pub(crate) static SKSEPlugin_Version: PluginDeclaration =
    PluginDeclaration::new(crate::skse::PluginDeclarationInfo {
        version: crate::skse::PluginDeclarationVersionNumber::new(0, 0, 0, 0),
        name: crate::skse::PluginDeclarationString::from_str("libskyrim-tests"),
        author: crate::skse::PluginDeclarationString::from_str("libskyrim"),
        support_email: crate::skse::PluginDeclarationString::from_str(""),
        struct_compatibility: crate::skse::StructCompatibility::Independent,
        runtime_compatibility: crate::skse::RuntimeCompatibility::new(),
        minimum_skse_version: crate::skse::PluginDeclarationVersionNumber(0),
    });

#[cfg(not(test))]
pub fn panic_runtime(info: &core::panic::PanicInfo<'_>) -> ! {
    crate::skse::log::fatal_runtime(format_args!("panic: {info}"))
}

#[cfg(all(not(test), feature = "panic-handler"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    panic_runtime(info)
}

#[cfg(not(test))]
#[unsafe(no_mangle)]
pub unsafe extern "system" fn SKSEPlugin_Query(
    skse: *const LoadInterface,
    info: *mut PluginInfo,
) -> bool {
    unsafe { crate::skse::loader::query(skse, info) }
}

#[cfg(not(test))]
#[unsafe(no_mangle)]
pub unsafe extern "system" fn SKSEPlugin_Load(skse: *const LoadInterface) -> bool {
    unsafe { crate::skse::loader::load(skse) }
}
