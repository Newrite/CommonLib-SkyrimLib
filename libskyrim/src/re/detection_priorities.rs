#![allow(non_camel_case_types)]

/// C++ `RE::DETECTION_PRIORITY`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DETECTION_PRIORITY {
    None = 0,
    VeryLow = 1,
    Low = 2,
    Normal = 3,
    High = 4,
    Critical = 5,
}
