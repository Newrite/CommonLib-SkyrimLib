use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_RallyEffect;
use crate::offsets::offsets_vtable::VTABLE_RallyEffect;
use crate::re::TargetValueModifierEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::RallyEffect`
#[repr(C)]
pub struct RallyEffect {
    pub base: TargetValueModifierEffect, // 00
}

const _: () = assert!(core::mem::size_of::<RallyEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(RallyEffect, base) == 0x00);

impl RttiType for RallyEffect {
    const RTTI: VariantID = RTTI_RallyEffect;
}

inherit!(RallyEffect : TargetValueModifierEffect);

impl RallyEffect {
    pub const RTTI: VariantID = RTTI_RallyEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_RallyEffect;

    // override (ActiveEffect)
    // ~RallyEffect() override;  // 13

    // override (TargetValueModifierEffect)
    // float GetTargetValue() const override;  // 21
}
