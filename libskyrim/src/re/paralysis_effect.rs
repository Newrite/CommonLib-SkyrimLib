use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ParalysisEffect;
use crate::offsets::offsets_vtable::VTABLE_ParalysisEffect;
use crate::re::ValueModifierEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ParalysisEffect`
#[repr(C)]
pub struct ParalysisEffect {
    pub base: ValueModifierEffect, // 00
}

const _: () = assert!(core::mem::size_of::<ParalysisEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(ParalysisEffect, base) == 0x00);

impl RttiType for ParalysisEffect {
    const RTTI: VariantID = RTTI_ParalysisEffect;
}

inherit!(ParalysisEffect : ValueModifierEffect);

impl ParalysisEffect {
    pub const RTTI: VariantID = RTTI_ParalysisEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ParalysisEffect;

    // override (ValueModifierEffect)
    // void Update(float) override;  // 04
    // ~ParalysisEffect() override;  // 13
    // void Start() override;        // 14
    // void Finish() override;       // 15
}
