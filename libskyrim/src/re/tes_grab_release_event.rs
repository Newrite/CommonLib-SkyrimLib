use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESGrabReleaseEvent`
#[repr(C)]
pub struct TESGrabReleaseEvent {
    pub ref_: NiPointer<TESObjectREFR>, // 00
    pub grabbed: bool,                  // 08
    pub pad09: u8,                      // 09
    pub pad0a: u16,                     // 0A
    pub pad0c: u32,                     // 0C
}

const _: () = assert!(core::mem::size_of::<TESGrabReleaseEvent>() == 0x10);
