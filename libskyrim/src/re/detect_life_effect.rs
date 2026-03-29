use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_DetectLifeEffect;
use crate::offsets::offsets_vtable::VTABLE_DetectLifeEffect;
use crate::re::ActiveEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::DetectLifeEffect`
#[repr(C)]
pub struct DetectLifeEffect {
    pub base: ActiveEffect, // 00
}

const _: () = assert!(core::mem::size_of::<DetectLifeEffect>() == 0x90);
const _: () = assert!(core::mem::offset_of!(DetectLifeEffect, base) == 0x00);

impl RttiType for DetectLifeEffect {
    const RTTI: VariantID = RTTI_DetectLifeEffect;
}

inherit!(DetectLifeEffect : ActiveEffect);

impl DetectLifeEffect {
    pub const RTTI: VariantID = RTTI_DetectLifeEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_DetectLifeEffect;

    // override (ActiveEffect)
    // void Update(float) override;                    // 04
    // bool ShouldDispelOnDeath() const override;      // 10
    // bool GetAllowMultipleCastingSourceStacking() override;  // 11
    // ~DetectLifeEffect() override;                   // 13
    // bool CheckCustomSkillUseConditions() const override;  // 17
}
