use crate::re::BSFixedString;

/// C++ `RE::TESTrackedStatsEvent`
#[repr(C)]
pub struct TESTrackedStatsEvent {
    pub stat: BSFixedString, // 00
    pub value: i32,          // 08
    pub pad0c: u32,          // 0C
}

const _: () = assert!(core::mem::size_of::<TESTrackedStatsEvent>() == 0x10);
