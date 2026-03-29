use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_WerewolfEffect;
use crate::offsets::offsets_vtable::VTABLE_WerewolfEffect;
use crate::re::ActiveEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::WerewolfEffect`
#[repr(C)]
pub struct WerewolfEffect {
    pub base: ActiveEffect, // 00
}

const _: () = assert!(core::mem::size_of::<WerewolfEffect>() == 0x90);
const _: () = assert!(core::mem::offset_of!(WerewolfEffect, base) == 0x00);

impl RttiType for WerewolfEffect {
    const RTTI: VariantID = RTTI_WerewolfEffect;
}

inherit!(WerewolfEffect : ActiveEffect);

impl WerewolfEffect {
    pub const RTTI: VariantID = RTTI_WerewolfEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_WerewolfEffect;

    // override (ActiveEffect)
    // ~WerewolfEffect() override;  // 13
    // void Start() override;       // 14
    // void Finish() override;      // 15
}
