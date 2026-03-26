use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESDeathEvent`
#[repr(C)]
pub struct TESDeathEvent {
    pub actor_dying: NiPointer<TESObjectREFR>,  // 00
    pub actor_killer: NiPointer<TESObjectREFR>, // 08
    pub dead: bool,                             // 10
    pub pad11: u8,                              // 11
    pub pad12: u16,                             // 12
    pub pad14: u32,                             // 14
}

const _: () = assert!(core::mem::size_of::<TESDeathEvent>() == 0x18);
