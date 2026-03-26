use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESMoveAttachDetachEvent`
#[repr(C)]
pub struct TESMoveAttachDetachEvent {
    pub moved_ref: NiPointer<TESObjectREFR>, // 00
    pub is_cell_attached: bool,              // 08
    pub pad09: u8,                           // 09
    pub pad0a: u16,                          // 0A
    pub pad0c: u32,                          // 0C
}

const _: () = assert!(core::mem::size_of::<TESMoveAttachDetachEvent>() == 0x10);
