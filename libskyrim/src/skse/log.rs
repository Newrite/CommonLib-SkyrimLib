//!
//! @file log.rs
//! @author Andrew Spaulding (Kasplat)
//! @brief Implements a logging API that creates a file in the SKSE log folder based on the
//!        name of the plugin in the version structure.
//! @bug No known bugs.
//!

use alloc::format;
use core::ffi::CStr;
use core::fmt::{Arguments, Write};

use core_util::{Later, RacyCell, StringBuffer, WideStr, WideStringBuffer};
use cstd::io::File;
use windows_sys::Win32::Foundation::{MAX_PATH, S_OK};
use windows_sys::Win32::System::Com::CoTaskMemFree;
use windows_sys::Win32::System::Diagnostics::Debug::RaiseException;
use windows_sys::Win32::System::Threading::{GetCurrentProcess, TerminateProcess};
use windows_sys::Win32::UI::Shell::{FOLDERID_Documents, SHGetKnownFolderPath};
use windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxA;

use windows_sys::Win32::Foundation::{FILETIME, SYSTEMTIME};
use windows_sys::Win32::System::SystemInformation::GetSystemTimePreciseAsFileTime;
use windows_sys::Win32::System::Time::{FileTimeToSystemTime, SystemTimeToTzSpecificLocalTime};

#[doc(hidden)]
pub use windows_sys::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_ICONWARNING};

use crate::SKSEPlugin_Version;
use crate::runtime::CURRENT_VERSION;

#[doc(hidden)]
pub enum LogType {
    File,
    Window(u32),
    Both(u32),
}

const BUF_SIZE: usize = 8192;
static LOG_FILE: Later<RacyCell<File>> = Later::new();

impl LogType {
    fn log(&self, msg: &CStr) -> Result<(), ()> {
        let win_res = match self {
            Self::Window(ico) | Self::Both(ico) => {
                let res = unsafe {
                    MessageBoxA(
                        core::ptr::null_mut(),
                        msg.as_ptr().cast(),
                        SKSEPlugin_Version.name.as_ptr().cast(),
                        *ico,
                    )
                };
                if res == 0 { Err(()) } else { Ok(()) }
            }
            _ => Ok(()),
        };

        let log_res = match self {
            Self::File | Self::Both(_) => {
                if LOG_FILE.is_init()
                    && unsafe { (*LOG_FILE.get()).write(msg.to_bytes()).is_ok() }
                    && unsafe { (*LOG_FILE.get()).flush().is_ok() }
                {
                    Ok(())
                } else {
                    Err(())
                }
            }
            _ => Ok(()),
        };

        win_res.and(log_res)
    }
}

pub(crate) fn open() {
    let mut buf: WideStringBuffer<BUF_SIZE> = WideStringBuffer::new();

    unsafe {
        assert!(BUF_SIZE > MAX_PATH as usize);
        let mut path: windows_sys::core::PWSTR = core::ptr::null_mut();

        assert!(
            SHGetKnownFolderPath(&FOLDERID_Documents, 0, core::ptr::null_mut(), &mut path) == S_OK
        );
        buf.write_w_str(WideStr::from_ptr(path)).unwrap();
        CoTaskMemFree(path.cast());
    }

    buf.write_fmt(format_args!(
        "\\My Games\\{}\\SKSE\\{}.log",
        (*CURRENT_VERSION).save_folder(),
        unsafe {
            CStr::from_ptr(SKSEPlugin_Version.name.as_ptr())
                .to_str()
                .unwrap()
        }
    ))
    .unwrap();

    let mode_storage = core_util::create_utf16_string::<4>("w+b");
    let mode = core_util::WideStr::from_slice(&mode_storage);
    LOG_FILE.init(RacyCell::new(File::wopen(buf.as_w_str(), mode).unwrap()));

    unsafe {
        (*LOG_FILE.get()).write(&cstd::io::UTF8_BOM).unwrap();
    }
}

#[doc(hidden)]
pub fn write(log_type: LogType, file: &str, line: u32, args: Arguments<'_>) {
    let mut buf = StringBuffer::<BUF_SIZE>::new();

    let file_name = file.rsplit('\\').next().unwrap_or(file);
    let file_name = file_name.rsplit('/').next().unwrap_or(file_name);

    let mut ft: FILETIME = unsafe { core::mem::zeroed() };
    unsafe { GetSystemTimePreciseAsFileTime(&mut ft) };

    let combined = ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64);
    let microseconds = (combined / 10) % 1_000_000;

    let mut st_utc: SYSTEMTIME = unsafe { core::mem::zeroed() };
    unsafe { FileTimeToSystemTime(&ft, &mut st_utc) };

    let mut st: SYSTEMTIME = unsafe { core::mem::zeroed() };
    unsafe { SystemTimeToTzSpecificLocalTime(core::ptr::null(), &st_utc, &mut st) };

    let loc_str = format!("[{}:{}]", file_name, line);
    let plugin_name = unsafe {
        CStr::from_ptr(SKSEPlugin_Version.name.as_ptr())
            .to_str()
            .unwrap_or("Unknown")
    };

    buf.write_fmt(format_args!(
        "[{}] [{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}] {:<25} ",
        plugin_name,
        st.wYear,
        st.wMonth,
        st.wDay,
        st.wHour,
        st.wMinute,
        st.wSecond,
        microseconds,
        loc_str
    ))
    .unwrap();

    buf.write_fmt(args).unwrap();
    buf.write_str("\n").unwrap();

    log_type.log(buf.as_c_str()).unwrap();
}

#[doc(hidden)]
pub fn fatal(log_type: LogType, file: &str, line: u32, args: Arguments<'_>) {
    let mut buf = StringBuffer::<BUF_SIZE>::new();

    let file_name = file.rsplit('\\').next().unwrap_or(file);
    let file_name = file_name.rsplit('/').next().unwrap_or(file_name);
    let loc_str = format!("[{}:{}]", file_name, line);
    let plugin_name = unsafe {
        CStr::from_ptr(SKSEPlugin_Version.name.as_ptr())
            .to_str()
            .unwrap_or("Unknown")
    };

    if buf
        .write_fmt(format_args!(
            "[{}] [FATAL ERROR] {:<25} ",
            plugin_name, loc_str
        ))
        .is_err()
        || buf.write_fmt(args).is_err()
        || buf.write_str("\n").is_err()
    {
        let _ = log_type.log(core_util::cstr!(
            "The plugin encountered an unknown fatal error.\n"
        ));
    } else {
        let _ = log_type.log(buf.as_c_str());
    }
}

#[track_caller]
pub fn fatal_runtime(args: Arguments<'_>) -> ! {
    let location = core::panic::Location::caller();
    fatal(
        LogType::Both(MB_ICONERROR),
        location.file(),
        location.line(),
        args,
    );

    unsafe {
        RaiseException(0xE000_0001u32, 0x1, 0, core::ptr::null());
        TerminateProcess(GetCurrentProcess(), 0xE000_0001u32);
    }

    loop {
        core::hint::spin_loop();
    }
}

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
