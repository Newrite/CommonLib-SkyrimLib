use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESResetEvent`
#[repr(C)]
pub struct TESResetEvent {
    pub object: NiPointer<TESObjectREFR>, // 00
}

const _: () = assert!(core::mem::size_of::<TESResetEvent>() == 0x08);
