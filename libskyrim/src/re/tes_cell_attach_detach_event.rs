use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESCellAttachDetachEvent`
#[repr(C)]
pub struct TESCellAttachDetachEvent {
    pub reference: NiPointer<TESObjectREFR>, // 00
    pub attached: bool,                      // 08
    pub pad09: u8,                           // 09
    pub pad0a: u16,                          // 0A
    pub pad0c: u32,                          // 0C
}

const _: () = assert!(core::mem::size_of::<TESCellAttachDetachEvent>() == 0x10);
