use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESActiveEffectApplyRemoveEvent`
#[repr(C)]
pub struct TESActiveEffectApplyRemoveEvent {
    pub caster: NiPointer<TESObjectREFR>, // 00
    pub target: NiPointer<TESObjectREFR>, // 08
    pub active_effect_unique_id: u16,     // 10
    pub is_applied: bool,                 // 12
    pub pad13: u8,                        // 13
    pub pad14: u32,                       // 14
}

const _: () = assert!(core::mem::size_of::<TESActiveEffectApplyRemoveEvent>() == 0x18);
