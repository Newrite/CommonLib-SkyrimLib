use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESOpenCloseEvent`
#[repr(C)]
pub struct TESOpenCloseEvent {
    pub ref_: NiPointer<TESObjectREFR>,       // 00
    pub active_ref: NiPointer<TESObjectREFR>, // 08
    pub opened: bool,                         // 10
    pub pad11: u8,                            // 11
    pub pad12: u16,                           // 12
    pub pad14: u32,                           // 14
}

const _: () = assert!(core::mem::size_of::<TESOpenCloseEvent>() == 0x18);
