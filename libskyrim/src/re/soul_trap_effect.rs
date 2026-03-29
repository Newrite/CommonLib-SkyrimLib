use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_SoulTrapEffect;
use crate::offsets::offsets_vtable::VTABLE_SoulTrapEffect;
use crate::re::ActiveEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::SoulTrapEffect`
#[repr(C)]
pub struct SoulTrapEffect {
    pub base: ActiveEffect, // 00
}

const _: () = assert!(core::mem::size_of::<SoulTrapEffect>() == 0x90);
const _: () = assert!(core::mem::offset_of!(SoulTrapEffect, base) == 0x00);

impl RttiType for SoulTrapEffect {
    const RTTI: VariantID = RTTI_SoulTrapEffect;
}

inherit!(SoulTrapEffect : ActiveEffect);

impl SoulTrapEffect {
    pub const RTTI: VariantID = RTTI_SoulTrapEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SoulTrapEffect;

    // override (ActiveEffect)
    // void Update(float) override;  // 04
    // ~SoulTrapEffect() override;   // 13
    // void Finish() override;       // 15
}
