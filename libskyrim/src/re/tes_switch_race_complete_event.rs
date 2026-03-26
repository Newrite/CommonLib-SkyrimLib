use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESSwitchRaceCompleteEvent`
#[repr(C)]
pub struct TESSwitchRaceCompleteEvent {
    pub subject: NiPointer<TESObjectREFR>, // 00
}

const _: () = assert!(core::mem::size_of::<TESSwitchRaceCompleteEvent>() == 0x08);
