use core::ffi::c_char;

use crate::re::error_logger::Severity;

/// C++ `RE::BSScript::LogEvent`
#[repr(C)]
pub struct LogEvent {
    pub error_msg: *const c_char, // 00
    pub severity: Severity,       // 08
    pub pad0c: u32,               // 0C
}

const _: () = assert!(core::mem::size_of::<LogEvent>() == 0x10);
const _: () = assert!(core::mem::offset_of!(LogEvent, error_msg) == 0x00);
const _: () = assert!(core::mem::offset_of!(LogEvent, severity) == 0x08);
