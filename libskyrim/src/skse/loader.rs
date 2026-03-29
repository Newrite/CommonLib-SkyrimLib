use crate::version::Version;
use crate::{SKSEPlugin_Version, skse_plugin_rust_entry};
use core_util::RacyCell;

use super::{LoadInterface, PluginInfo};

fn init_runtime_only(skse: *const LoadInterface) -> bool {
    if skse.is_null() {
        crate::skse_fatal!(
            window,
            "SKSE initialization received a null interface pointer"
        );
        return false;
    }

    let skse = unsafe { &*skse };
    let runtime_version = Version::from_packed(skse.runtime_version);

    if !crate::runtime::CURRENT_VERSION.is_init() {
        crate::runtime::init(runtime_version);
    }

    true
}

fn init_full(skse: *const LoadInterface) -> bool {
    static INITIALIZED: RacyCell<bool> = RacyCell::new(false);
    if unsafe { *INITIALIZED.get() } {
        return true;
    }

    if !init_runtime_only(skse) {
        return false;
    }

    let skse = unsafe { &*skse };

    unsafe {
        crate::skse::init(skse);
    }

    crate::skse::log::open();

    if skse.is_editor != 0 {
        return false;
    }

    crate::skse::initialize_plugin_handle(unsafe { (skse.get_plugin_handle)() });
    if !crate::skse::initialize_messaging_listener(skse) {
        return false;
    }

    unsafe {
        *INITIALIZED.get() = true;
    }
    true
}

#[inline(always)]
fn plugin_name() -> &'static str {
    unsafe {
        core::ffi::CStr::from_ptr(SKSEPlugin_Version.get_name_ptr())
            .to_str()
            .unwrap_or("Unknown")
    }
}

pub unsafe fn query(skse: *const LoadInterface, info: *mut PluginInfo) -> bool {
    if !init_runtime_only(skse) {
        return false;
    }
    if info.is_null() {
        crate::skse_fatal!(
            window,
            "SKSEPlugin_Query received a null PluginInfo pointer"
        );
        return false;
    }

    unsafe {
        *info = PluginInfo {
            info_version: PluginInfo::VERSION,
            name: SKSEPlugin_Version.get_name_ptr(),
            version: SKSEPlugin_Version.get_version().pack(),
        };
    }

    true
}

pub unsafe fn load(skse: *const LoadInterface) -> bool {
    if !init_full(skse) {
        return false;
    }

    crate::skse::crash::install_panic_hook();

    let skse = unsafe { &*skse };
    let game_version = Version::from_packed(skse.runtime_version);
    let skse_version = Version::from_packed(skse.skse_version);

    crate::skse_message!(
        "{} v{}\nRunning on Skyrim SE/AE {}, SKSE {}",
        plugin_name(),
        unsafe { SKSEPlugin_Version.get_version() },
        game_version,
        skse_version
    );

    crate::skse::crash::guard("SKSEPlugin_Load -> skse_plugin_rust_entry", || unsafe {
        skse_plugin_rust_entry(skse)
    })
    .is_ok()
}
