use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESEnterBleedoutEvent`
#[repr(C)]
pub struct TESEnterBleedoutEvent {
    pub actor: NiPointer<TESObjectREFR>, // 00
}

const _: () = assert!(core::mem::size_of::<TESEnterBleedoutEvent>() == 0x08);
