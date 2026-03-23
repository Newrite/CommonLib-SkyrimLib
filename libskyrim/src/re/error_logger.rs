/// C++ `RE::BSScript::ErrorLogger::Severity`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Info = 0,
    Warning = 1,
    Error = 2,
    Fatal = 3,
}

/// C++ `RE::BSScript::ErrorLogger::PerThreadErrorCounts`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PerThreadErrorCounts {
    pub fatal_count: u32,   // 00
    pub error_count: u32,   // 04
    pub warning_count: u32, // 08
}

const _: () = assert!(core::mem::size_of::<PerThreadErrorCounts>() == 0xC);

// AUTO-STUB: Full translation pending
// TODO: VERIFY - replace with full translation when layout is needed
core_util::abstract_type! { pub type ErrorLogger; }
