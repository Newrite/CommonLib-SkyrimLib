use crate::re::{AITimeStamp, BGSWorldLocation};

/// C++ `RE::CombatSearchLocation`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CombatSearchLocation {
    pub loc: BGSWorldLocation,  // 00
    pub timestamp: AITimeStamp, // 18
    pub unk1c: f32,             // 1C
}

const _: () = assert!(core::mem::size_of::<CombatSearchLocation>() == 0x20);
const _: () = assert!(core::mem::offset_of!(CombatSearchLocation, loc) == 0x00);
const _: () = assert!(core::mem::offset_of!(CombatSearchLocation, timestamp) == 0x18);
const _: () = assert!(core::mem::offset_of!(CombatSearchLocation, unk1c) == 0x1C);
