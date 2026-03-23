//!
//! @file lib.rs
//! @author Andrew Spaulding (Kasplat) / Обновлено под CommonLib-NG
//! @brief Module runtime loader and environment for libskyrim.
//!

#![no_std]
extern crate alloc;

#[macro_use]
pub mod log;
pub mod ffi;
pub mod ini;
pub mod offsets;
pub mod re;
pub mod relocation;

// Наши новые модули
pub mod runtime;
pub mod skse64;
pub mod version;

// Needed for macros
pub extern crate core;
pub extern crate core_util;

use crate::version::Version;
use core::ffi::CStr;
#[cfg(not(test))]
use core::panic::PanicInfo;
use core_util::RacyCell; // Подтягиваем наш тип версий

// Подтягиваем интерфейсы из модуля skse64
use crate::skse64::plugin_api::{PluginInfo, SkseInterface, SksePluginVersionData};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Core plugin loader
////////////////////////////////////////////////////////////////////////////////////////////////////

unsafe extern "Rust" {
    fn skse_plugin_rust_entry(skse: &SkseInterface) -> Result<(), ()>;
    pub(crate) static SKSEPlugin_Version: SksePluginVersionData;
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    crate::log::fatal_runtime(format_args!("panic: {info}"))
}

fn init_runtime_only(skse: *const SkseInterface) -> bool {
    if skse.is_null() {
        skse_fatal!(
            window,
            "SKSE initialization received a null interface pointer"
        );
        return false;
    }

    let skse = unsafe { &*skse };

    // Конвертируем u32 из FFI в нашу удобную структуру Version
    let runtime_ver = Version::from_packed(skse.runtime_version);

    if !crate::runtime::CURRENT_VERSION.is_init() {
        crate::runtime::init(runtime_ver);
    }
    true
}

fn init_full(skse: *const SkseInterface) -> bool {
    static DONE: RacyCell<bool> = RacyCell::new(false);
    if unsafe { *DONE.get() } {
        return true;
    }

    if !init_runtime_only(skse) {
        return false;
    }

    log::open();

    let skse = unsafe { &*skse };

    if skse.is_editor != 0 {
        return false;
    }

    unsafe {
        plugin_api::PLUGIN_HANDLE.init((skse.get_plugin_handle)());
    }
    if !plugin_api::init_listener(skse) {
        return false;
    }

    unsafe {
        *DONE.get() = true;
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn SKSEPlugin_Query(
    skse: *const SkseInterface,
    info: *mut PluginInfo,
) -> bool {
    if !init_runtime_only(skse) {
        return false;
    }
    if info.is_null() {
        skse_fatal!(
            window,
            "SKSEPlugin_Query received a null PluginInfo pointer"
        );
        return false;
    }

    unsafe {
        *info = PluginInfo {
            info_version: PluginInfo::VERSION,
            name: SKSEPlugin_Version.name.as_ptr(),
            version: SKSEPlugin_Version.plugin_version, // Передаем чистый u32
        };
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn SKSEPlugin_Load(skse: *const SkseInterface) -> bool {
    if !init_full(skse) {
        return false;
    }

    unsafe {
        crate::ffi::init_commonlib(skse as *const core::ffi::c_void);
    }

    let skse = unsafe { &*skse };

    let game_ver = Version::from_packed(skse.runtime_version);
    let skse_ver = Version::from_packed(skse.skse_version);

    skse_message!(
        "{} v{}\nRunning on Skyrim SE/AE {}, SKSE {}",
        unsafe { CStr::from_ptr(SKSEPlugin_Version.name.as_ptr()) }
            .to_str()
            .unwrap_or("Unknown"),
        unsafe { Version::from_packed(SKSEPlugin_Version.plugin_version) },
        game_ver,
        skse_ver
    );

    if let Ok(_) = unsafe { skse_plugin_rust_entry(skse) } {
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

pub mod task {
    use crate::ffi;
    use alloc::boxed::Box;
    use core::ffi::c_void;

    /// Внутренний обработчик, который вызывается из C++
    /// Он распаковывает замыкание из void* и исполняет его.
    extern "C" fn task_runner<F: FnOnce()>(data: *mut c_void) {
        // Восстанавливаем Box из сырого указателя.
        // В конце области видимости он автоматически очистит память (Dispose)!
        let closure = unsafe { Box::from_raw(data as *mut F) };
        closure();
    }

    /// Добавляет задачу в главную очередь игры (выполняется в основном потоке).
    pub fn add_task<F>(f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Упаковываем замыкание в Box и превращаем в сырой указатель (чтобы Rust его не удалил)
        let data = Box::into_raw(Box::new(f)) as *mut c_void;
        unsafe {
            ffi::commonlib_add_task(task_runner::<F>, data);
        }
    }

    /// Добавляет задачу в очередь UI (безопасно для работы со Scaleform/меню).
    pub fn add_ui_task<F>(f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let data = Box::into_raw(Box::new(f)) as *mut c_void;
        unsafe {
            ffi::commonlib_add_ui_task(task_runner::<F>, data);
        }
    }
}

pub mod plugin_api {
    use alloc::vec::Vec;
    use core::ffi::c_char;

    pub use crate::skse64::plugin_api::*;
    use crate::version::Version;
    use core_util::{Later, RacyCell};

    const VEC_INIT: Vec<fn(&Message)> = Vec::new();
    static SKSE_HANDLERS: RacyCell<[Vec<fn(&Message)>; Message::SKSE_MAX]> =
        RacyCell::new([VEC_INIT; Message::SKSE_MAX]);

    pub(crate) static PLUGIN_HANDLE: Later<PluginHandle> = Later::new();

    pub(crate) fn init_listener(skse: &SkseInterface) -> bool {
        unsafe {
            let msg_if =
                (skse.query_interface)(InterfaceId::Messaging) as *mut SkseMessagingInterface;
            if msg_if.is_null() {
                crate::skse_fatal!(window, "Failed to acquire the SKSE messaging interface");
                return false;
            }

            if !((*msg_if).register_listener)(
                handle(),
                "SKSE\0".as_bytes().as_ptr() as *const c_char,
                skse_listener,
            ) {
                crate::skse_fatal!(window, "Failed to register the SKSE messaging listener");
                return false;
            }
        }
        true
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
            #[unsafe(no_mangle)]
            pub static SKSEPlugin_Version: $crate::plugin_api::SksePluginVersionData =
            $crate::plugin_api::SksePluginVersionData {
                data_version: $crate::plugin_api::SksePluginVersionData::VERSION,
                // Упаковываем версию в u32 с помощью метода .pack()
                plugin_version: $crate::version::Version::new(
                    $crate::plugin_api::unsigned_from_str($crate::core::env!("CARGO_PKG_VERSION_MAJOR")) as u16,
                    $crate::plugin_api::unsigned_from_str($crate::core::env!("CARGO_PKG_VERSION_MINOR")) as u16,
                    $crate::plugin_api::unsigned_from_str($crate::core::env!("CARGO_PKG_VERSION_PATCH")) as u16,
                    0
                ).pack(),
                name: $crate::plugin_api::make_str($crate::core::env!("CARGO_CRATE_NAME")),
                author: $crate::plugin_api::make_str($author),
                support_email: $crate::plugin_api::make_str($email),
                version_indep_ex: $vix,
                version_indep: $vi,
                compat_versions: $crate::plugin_api::make_vers(&[$($compat),*]),
                se_version_required: 0
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

    // Возвращаем массив u32 для идеального совпадения с C ABI
    #[doc(hidden)]
    pub const fn make_vers<const N: usize>(v: &[Version]) -> [u32; N] {
        let mut ret = [0; N];
        assert!(v.len() <= (N - 1), "Too many compatible versions!");
        let mut i = 0;
        while i < v.len() {
            ret[i] = v[i].pack(); // Пакуем в u32
            i += 1;
        }
        ret
    }

    unsafe extern "system" fn skse_listener(msg: *mut Message) {
        let Some(msg) = (unsafe { msg.as_ref() }) else {
            crate::skse_fatal!(window, "SKSE delivered a null messaging payload");
            return;
        };
        if msg.msg_type >= Message::SKSE_MAX as u32 {
            return;
        }
        for callback in unsafe { (*SKSE_HANDLERS.get())[msg.msg_type as usize].iter() } {
            callback(msg);
        }
    }
}
