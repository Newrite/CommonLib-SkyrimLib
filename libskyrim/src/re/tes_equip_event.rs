use crate::re::bs_core_types::FormID;
use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESEquipEvent`
#[repr(C)]
pub struct TESEquipEvent {
    pub actor: NiPointer<TESObjectREFR>, // 00
    pub base_object: FormID,             // 08
    pub original_refr: FormID,           // 0C
    pub unique_id: u16,                  // 10
    pub equipped: bool,                  // 12
    pub pad13: u8,                       // 13
    pub pad14: u32,                      // 14
}

const _: () = assert!(core::mem::size_of::<TESEquipEvent>() == 0x18);
