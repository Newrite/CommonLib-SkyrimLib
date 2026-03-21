//!
//! @file log.rs
//! @author Andrew Spaulding (Kasplat)
//! @brief Implements a logging API that creates a file in the SKSE log folder based on the
//!        name of the plugin in the version structure.
//! @bug No known bugs.
//!

use core::fmt::{Arguments, Write};
use core::ffi::CStr;
use alloc::format; // Используем аллокатор для красивого выравнивания строк

use cstd::io::File;
use core_util::{Later, RacyCell, StringBuffer, WideStringBuffer, WideStr};
use windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxA;
use windows_sys::Win32::System::Com::CoTaskMemFree;
use windows_sys::Win32::UI::Shell::{SHGetKnownFolderPath, FOLDERID_Documents};
use windows_sys::Win32::Foundation::{MAX_PATH, S_OK};

// Импорты для точного времени
use windows_sys::Win32::System::SystemInformation::GetSystemTimePreciseAsFileTime;
use windows_sys::Win32::System::Time::{FileTimeToSystemTime, SystemTimeToTzSpecificLocalTime};
use windows_sys::Win32::Foundation::{FILETIME, SYSTEMTIME};

#[doc(hidden)]
pub use windows_sys::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_ICONWARNING};

use crate::SKSEPlugin_Version;
use crate::version;

#[doc(hidden)]
pub enum LogType {
    File,
    Window(u32),
    Both(u32)
}

const BUF_SIZE: usize = 8192;
static LOG_FILE: Later<RacyCell<File>> = Later::new();

impl LogType {
    unsafe fn log(&self, msg: &CStr) -> Result<(), ()> {
        let win_res = match self {
            Self::Window(ico) | Self::Both(ico) => {
                let res = MessageBoxA(
                    core::ptr::null_mut(),
                    msg.as_ptr().cast(),
                    SKSEPlugin_Version.name.as_ptr().cast(),
                    *ico
                );
                if res == 0 { Err(()) } else { Ok(()) }
            },
            _ => Ok(())
        };

        let log_res = match self {
            Self::File | Self::Both(_) => {
                if LOG_FILE.is_init() &&
                        (*LOG_FILE.get()).write(msg.to_bytes()).is_ok() &&
                        (*LOG_FILE.get()).flush().is_ok() {
                    Ok(())
                } else {
                    Err(())
                }
            },
            _ => Ok(())
        };

        win_res.and(log_res)
    }
}

pub (in crate) fn open() {
    let mut buf: WideStringBuffer<BUF_SIZE> = WideStringBuffer::new();

    unsafe {
        assert!(BUF_SIZE > MAX_PATH as usize);
        let mut path: windows_sys::core::PWSTR = core::ptr::null_mut();

        assert!(SHGetKnownFolderPath(&FOLDERID_Documents, 0, core::ptr::null_mut(), &mut path) == S_OK);
        buf.write_w_str(WideStr::from_ptr(path)).unwrap();
        CoTaskMemFree(path.cast());
    }

    buf.write_fmt(format_args!(
        "\\My Games\\{}\\SKSE\\{}.log",
        version::current_runtime().save_folder(),
        unsafe { CStr::from_ptr(SKSEPlugin_Version.name.as_ptr()).to_str().unwrap() }
    )).unwrap();

    LOG_FILE.init(RacyCell::new(File::wopen(
        buf.as_w_str(),
        core_util::wcstr!("w+b")
    ).unwrap()));

    unsafe {
        (*LOG_FILE.get()).write(&cstd::io::UTF8_BOM).unwrap();
    }
}

#[doc(hidden)]
pub fn write(
    log_type: LogType,
    file: &str,
    line: u32,
    args: Arguments<'_>
) {
    let mut buf = StringBuffer::<BUF_SIZE>::new();

    let file_name = file.rsplit('\\').next().unwrap_or(file);
    let file_name = file_name.rsplit('/').next().unwrap_or(file_name);

    // 1. Получаем высокоточное UTC время (FILETIME)
    let mut ft: FILETIME = unsafe { core::mem::zeroed() };
    unsafe { GetSystemTimePreciseAsFileTime(&mut ft) };

    // Микросекунды достаем напрямую из UTC, так как смещение часового пояса не влияет на доли секунд
    let combined = ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64);
    let microseconds = (combined / 10) % 1_000_000;

    // 2. Конвертируем FILETIME (UTC) в SYSTEMTIME (UTC)
    let mut st_utc: SYSTEMTIME = unsafe { core::mem::zeroed() };
    unsafe { FileTimeToSystemTime(&ft, &mut st_utc) };

    // 3. Конвертируем SYSTEMTIME (UTC) в локальное время (с учетом летнего/зимнего времени)
    let mut st: SYSTEMTIME = unsafe { core::mem::zeroed() };
    unsafe { SystemTimeToTzSpecificLocalTime(core::ptr::null(), &st_utc, &mut st) };

    // Формируем блок [Файл:Строка]
    let loc_str = format!("[{}:{}]", file_name, line);
    let plugin_name = unsafe { CStr::from_ptr(SKSEPlugin_Version.name.as_ptr()).to_str().unwrap_or("Unknown") };

    // Идеальное форматирование!
    buf.write_fmt(format_args!(
        "[{}] [{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}] {:<25} ",
        plugin_name,
        st.wYear, st.wMonth, st.wDay, st.wHour, st.wMinute, st.wSecond, microseconds,
        loc_str
    )).unwrap();

    buf.write_fmt(args).unwrap();
    buf.write_str("\n").unwrap();

    unsafe {
        log_type.log(buf.as_c_str()).unwrap();
    }
}

#[doc(hidden)]
pub fn fatal(
    log_type: LogType,
    file: &str,
    line: u32,
    args: Arguments<'_>
) {
    let mut buf = StringBuffer::<BUF_SIZE>::new();

    let file_name = file.rsplit('\\').next().unwrap_or(file);
    let file_name = file_name.rsplit('/').next().unwrap_or(file_name);
    let loc_str = format!("[{}:{}]", file_name, line);
    let plugin_name = unsafe { CStr::from_ptr(SKSEPlugin_Version.name.as_ptr()).to_str().unwrap_or("Unknown") };

    unsafe {
        if buf.write_fmt(format_args!("[{}] [FATAL ERROR] {:<25} ", plugin_name, loc_str)).is_err() ||
           buf.write_fmt(args).is_err() ||
           buf.write_str("\n").is_err() {
            let _ = log_type.log(core_util::cstr!("The plugin encountered an unknown fatal error.\n"));
        } else {
            let _ = log_type.log(buf.as_c_str());
        }
    }
}

// ── МАКРОСЫ ──────────────────────────────────────────────────────────────────
// Изменил `( $($fmt:expr),* )` на `( $($arg:tt)* )`.
// Это стандартный синтаксис Rust, который позволяет писать `skse_message!("HP: {}", 100)`.
// Также они теперь прозрачно пробрасывают текущий файл и строку.

#[macro_export]
macro_rules! skse_message {
    ( $($arg:tt)* ) => {
        $crate::log::write(
            $crate::log::LogType::File,
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    };
}

#[macro_export]
macro_rules! skse_warning {
    ( window, $($arg:tt)* ) => {
        $crate::log::write(
            $crate::log::LogType::Window($crate::log::MB_ICONWARNING),
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    };
    ( log, $($arg:tt)* ) => {
        $crate::log::write(
            $crate::log::LogType::File,
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    };
    ( $($arg:tt)* ) => {
        $crate::log::write(
            $crate::log::LogType::Both($crate::log::MB_ICONWARNING),
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    };
}

#[macro_export]
macro_rules! skse_fatal {
    ( window, $($arg:tt)* ) => {
        $crate::log::fatal(
            $crate::log::LogType::Window($crate::log::MB_ICONERROR),
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    };
    ( log, $($arg:tt)* ) => {
        $crate::log::fatal(
            $crate::log::LogType::File,
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    };
    ( $($arg:tt)* ) => {
        $crate::log::fatal(
            $crate::log::LogType::Both($crate::log::MB_ICONERROR),
            $crate::core::file!(),
            $crate::core::line!(),
            $crate::core::format_args!($($arg)*)
        );
    };
}
