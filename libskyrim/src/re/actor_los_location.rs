#![allow(non_camel_case_types)]

/// C++ `RE::ACTOR_LOS_LOCATION`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ACTOR_LOS_LOCATION {
    None = 0,
    Eye = 1,
    Head = 2,
    Torso = 3,
    Feet = 4,

    Total = 5,
}
