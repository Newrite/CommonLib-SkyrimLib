use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_FrenzyEffect;
use crate::offsets::offsets_vtable::VTABLE_FrenzyEffect;
use crate::re::TargetValueModifierEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::FrenzyEffect`
#[repr(C)]
pub struct FrenzyEffect {
    pub base: TargetValueModifierEffect, // 00
}

const _: () = assert!(core::mem::size_of::<FrenzyEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(FrenzyEffect, base) == 0x00);

impl RttiType for FrenzyEffect {
    const RTTI: VariantID = RTTI_FrenzyEffect;
}

inherit!(FrenzyEffect : TargetValueModifierEffect);

impl FrenzyEffect {
    pub const RTTI: VariantID = RTTI_FrenzyEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FrenzyEffect;

    // override (TargetValueModifierEffect)
    // ~FrenzyEffect() override;           // 13
    // void Start() override;              // 14
    // void Finish() override;             // 15
    // float GetTargetValue() const override;  // 21
}
