#![allow(non_camel_case_types)]

/// C++ `RE::PC_GAMEPAD_TYPE`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PC_GAMEPAD_TYPE {
    kDirectX = 0,
    kOrbis = 1,
    kTotal = 2,
}

core_util::impl_enumset_type!(PC_GAMEPAD_TYPE => u32);
