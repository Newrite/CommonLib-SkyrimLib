//!
//! @file lib.rs
//! @author Andrew Spaulding (Kasplat) / Обновлено под CommonLib-NG
//! @brief Module runtime loader and environment for libskyrim.
//!

#![no_std]
extern crate alloc;

#[macro_use]
pub mod log;
pub mod ini;
pub mod ffi;
pub mod relocation;
pub mod offsets;
pub mod re;

// Наши новые модули
pub mod version;
pub mod runtime;
pub mod skse64;

// Needed for macros
pub extern crate core;
pub extern crate core_util;

use core::ffi::CStr;
use core_util::RacyCell;

// Подтягиваем интерфейсы из модуля skse64
use crate::skse64::plugin_api::{PluginInfo, SkseInterface, SksePluginVersionData};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Core plugin loader
////////////////////////////////////////////////////////////////////////////////////////////////////

extern "Rust" {
    /// Точка входа в ваш пользовательский плагин (skse-hello-rust)
    fn skse_plugin_rust_entry(skse: &SkseInterface) -> Result<(), ()>;

    /// Генерируется макросом plugin_version_data!
    pub (in crate) static SKSEPlugin_Version: SksePluginVersionData;
}

unsafe fn init_runtime_only(skse: *const SkseInterface) -> bool {
    if skse.is_null() { return false; }

    if let Some(runtime_ver) = (*skse).runtime_version {
        if !crate::runtime::CURRENT_VERSION.is_init() {
            crate::runtime::init(runtime_ver);
        }
        true
    } else {
        false
    }
}

unsafe fn init_full(skse: *const SkseInterface) -> bool {
    static DONE: RacyCell<bool> = RacyCell::new(false);
    if *DONE.get() { return true; }

    if !init_runtime_only(skse) { return false; }

    log::open(); // Логи открываем только при загрузке

    if (*skse).is_editor != 0 { return false; }

    plugin_api::PLUGIN_HANDLE.init(((*skse).get_plugin_handle)());

    // ВАЖНО: Инициализируем сообщения ТОЛЬКО ЗДЕСЬ
    plugin_api::init_listener(skse.as_ref().unwrap());

    *DONE.get() = true;
    true
}

/// Общая инициализация (вызывается из Query и Load)
unsafe fn init_skse(skse: *const SkseInterface) -> bool {
    static DO_ONCE: RacyCell<Option<bool>> = RacyCell::new(None);
    if let Some(ret) = *DO_ONCE.get() {
        return ret;
    }

    if skse.is_null() { return false; }

    // СНАЧАЛА инициализируем рантайм, так как логи могут зависеть от него (пути к файлам)
    if let Some(runtime_ver) = (*skse).runtime_version {
        // Проверяем, не инициализировано ли уже (на всякий случай)
        if !crate::runtime::CURRENT_VERSION.is_init() {
            crate::runtime::init(runtime_ver);
        }
    } else {
        return false;
    }

    // Теперь открываем логи (теперь они точно знают, в какую папку писать)
    log::open();

    if (*skse).is_editor != 0 {
        *DO_ONCE.get() = Some(false);
        return false;
    }

    plugin_api::PLUGIN_HANDLE.init(((*skse).get_plugin_handle)());
    plugin_api::init_listener(skse.as_ref().unwrap());

    *DO_ONCE.get() = Some(true);
    true
}

#[no_mangle]
pub unsafe extern "system" fn SKSEPlugin_Query(
    skse: *const SkseInterface,
    info: *mut PluginInfo
) -> bool {
    if !init_runtime_only(skse) { return false; }
    assert!(!info.is_null());

    *info = PluginInfo {
        info_version: PluginInfo::VERSION,
        name: SKSEPlugin_Version.name.as_ptr(),
        version: Some(SKSEPlugin_Version.plugin_version)
    };

    // CommonLib-NG плагины поддерживают все версии, поэтому просто возвращаем true
    skse_message!("Plugin query complete.");
    true
}

#[no_mangle]
pub unsafe extern "system" fn SKSEPlugin_Load(skse: *const SkseInterface) -> bool {
    // Тут уже инициализируем логи, сообщения и остальное
    if !init_full(skse) { return false; }

    crate::ffi::init_commonlib(skse as *const core::ffi::c_void);

    let game_ver = (*skse).runtime_version.unwrap();
    let skse_ver = (*skse).skse_version.unwrap();

    skse_message!(
        "{} v{}\nRunning on Skyrim SE/AE {}, SKSE {}",
        CStr::from_ptr(SKSEPlugin_Version.name.as_ptr()).to_str().unwrap_or("Unknown"),
        SKSEPlugin_Version.plugin_version,
        game_ver,
        skse_ver
    );

    if let Ok(_) = skse_plugin_rust_entry(skse.as_ref().unwrap()) {
        true
    } else {
        false
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Вспомогательные модули библиотеки
////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod trampoline {
    pub fn alloc_trampoline(size: usize) {
        unsafe {
            crate::ffi::commonlib_alloc_trampoline(size);
        }
    }
}

pub mod plugin_api {
    use core::ffi::c_char;
    use alloc::vec::Vec;

    // Подтягиваем типы из skse64
    pub use crate::skse64::plugin_api::*;
    use core_util::{Later, RacyCell};
    use crate::version::Version;

    const VEC_INIT: Vec<fn(&Message)> = Vec::new();
    static SKSE_HANDLERS: RacyCell<[Vec<fn(&Message)>; Message::SKSE_MAX]> = RacyCell::new([VEC_INIT; Message::SKSE_MAX]);

    pub (in crate) static PLUGIN_HANDLE: Later<PluginHandle> = Later::new();

    pub (in crate) fn init_listener(skse: &SkseInterface) {
        unsafe {
            let msg_if = (skse.query_interface)(InterfaceId::Messaging) as *mut SkseMessagingInterface;
            ((*msg_if).register_listener)(
                handle(),
                "SKSE\0".as_bytes().as_ptr() as *const c_char,
                skse_listener
            );
        }
    }

    pub fn register_listener(msg_type: u32, callback: fn(&Message)) {
        assert!(msg_type < Message::SKSE_MAX as u32);
        unsafe {
            (*SKSE_HANDLERS.get())[msg_type as usize].push(callback);
        }
    }

    pub fn handle() -> PluginHandle {
        *PLUGIN_HANDLE
    }

    #[macro_export]
    macro_rules! plugin_version_data {
        (
            author: $author:literal,
            email: $email:literal,
            version_indep_ex: $vix:expr,
            version_indep: $vi:expr,
            compat_versions: [ $($compat:expr),* ]
        ) => {
            #[no_mangle]
            pub static SKSEPlugin_Version: $crate::plugin_api::SksePluginVersionData =
            $crate::plugin_api::SksePluginVersionData {
                data_version: $crate::plugin_api::SksePluginVersionData::VERSION,
                // Используем наш новый Version
                plugin_version: $crate::version::Version::new(
                    $crate::plugin_api::unsigned_from_str($crate::core::env!("CARGO_PKG_VERSION_MAJOR")) as u16,
                    $crate::plugin_api::unsigned_from_str($crate::core::env!("CARGO_PKG_VERSION_MINOR")) as u16,
                    $crate::plugin_api::unsigned_from_str($crate::core::env!("CARGO_PKG_VERSION_PATCH")) as u16,
                    0 // Build number
                ),
                name: $crate::plugin_api::make_str($crate::core::env!("CARGO_CRATE_NAME")),
                author: $crate::plugin_api::make_str($author),
                support_email: $crate::plugin_api::make_str($email),
                version_indep_ex: $vix,
                version_indep: $vi,
                compat_versions: $crate::plugin_api::make_vers(&[$($compat),*]),
                se_version_required: None
            };
        };
    }
    pub use plugin_version_data;

    #[doc(hidden)]
    pub const fn unsigned_from_str(s: &str) -> u32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut res = 0;
        while i < s.len() {
            assert!(b'0' <= s[i] && s[i] <= b'9');
            res *= 10;
            res += (s[i] - b'0') as u32;
            i += 1;
        }
        res
    }

    #[doc(hidden)]
    pub const fn make_str<const N: usize>(s: &str) -> [c_char; N] {
        let mut ret: [c_char; N] = [0; N];
        let s = s.as_bytes();
        assert!(s.len() <= (N - 1), "Cannot fit string in C array!");
        let mut i = 0;
        while i < s.len() {
            ret[i] = s[i] as i8;
            i += 1;
        }
        ret
    }

    #[doc(hidden)]
    pub const fn make_vers<const N: usize>(v: &[Version]) -> [Option<Version>; N] {
        let mut ret = [None; N];
        assert!(v.len() <= (N - 1), "Too many compatible versions!");
        let mut i = 0;
        while i < v.len() {
            ret[i] = Some(v[i]);
            i += 1;
        }
        ret
    }

    unsafe extern "system" fn skse_listener(msg: *mut Message) {
        let msg = msg.as_ref().unwrap();
        if msg.msg_type >= Message::SKSE_MAX as u32 { return; }
        for callback in (*SKSE_HANDLERS.get())[msg.msg_type as usize].iter() {
            callback(msg);
        }
    }
}