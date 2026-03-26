use crate::re::bs_core_types::FormID;
use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESMagicEffectApplyEvent`
#[repr(C)]
pub struct TESMagicEffectApplyEvent {
    pub target: NiPointer<TESObjectREFR>, // 00
    pub caster: NiPointer<TESObjectREFR>, // 08
    pub magic_effect: FormID,             // 10
    pub pad14: u32,                       // 14
}

const _: () = assert!(core::mem::size_of::<TESMagicEffectApplyEvent>() == 0x18);
