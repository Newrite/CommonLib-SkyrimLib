use crate::re::bs_core_types::FormID;
use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESSpellCastEvent`
#[repr(C)]
pub struct TESSpellCastEvent {
    pub object: NiPointer<TESObjectREFR>, // 00
    pub spell: FormID,                    // 08
    pub pad0c: u32,                       // 0C
}

const _: () = assert!(core::mem::size_of::<TESSpellCastEvent>() == 0x10);
