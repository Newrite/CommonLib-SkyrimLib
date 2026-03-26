#![allow(non_camel_case_types)]

/// C++ `RE::PROCESS_TYPE`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PROCESS_TYPE {
    None = -1,
    High = 0,
    MiddleHigh = 1,
    MiddleLow = 2,
    Low = 3,
}

core_util::impl_enumset_type!(PROCESS_TYPE => u8);

impl TryFrom<u8> for PROCESS_TYPE {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::High,
            1 => Self::MiddleHigh,
            2 => Self::MiddleLow,
            3 => Self::Low,
            _ => return Err(()),
        })
    }
}
