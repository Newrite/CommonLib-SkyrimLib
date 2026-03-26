#![allow(non_camel_case_types)]

/// C++ `RE::hkMultiThreadCheck::AccessType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkMultiThreadCheckAccessType {
    Ignore = 0,
    ReadOnly = 1,
    ReadWrite = 2,
}

/// C++ `RE::hkMultiThreadCheck::ReadMode`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkMultiThreadCheckReadMode {
    ThisObjOnly = 0,
    Recursive = 1,
}

/// C++ `RE::hkMultiThreadCheck::MarkedState`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkMultiThreadCheckMarkedState {
    ReadOnly = 0xFFFFFFE1,
    ReadOnlySelfOnly = 0xFFFFFFC1,
    UnMarked = 0xFFFFFFF1,
}

/// C++ `RE::hkMultiThreadCheck`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hkMultiThreadCheck {
    pub thread_id: u32,      // 00
    pub stack_trace_id: i32, // 04
    pub mark_count: u16,     // 08
    pub mark_bit_stack: u16, // 0A
}

const _: () = assert!(core::mem::size_of::<hkMultiThreadCheck>() == 0x0C);
