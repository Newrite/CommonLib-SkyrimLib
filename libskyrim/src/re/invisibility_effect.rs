use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_InvisibilityEffect;
use crate::offsets::offsets_vtable::VTABLE_InvisibilityEffect;
use crate::re::ValueModifierEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::InvisibilityEffect`
#[repr(C)]
pub struct InvisibilityEffect {
    pub base: ValueModifierEffect, // 00
}

const _: () = assert!(core::mem::size_of::<InvisibilityEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(InvisibilityEffect, base) == 0x00);

impl RttiType for InvisibilityEffect {
    const RTTI: VariantID = RTTI_InvisibilityEffect;
}

inherit!(InvisibilityEffect : ValueModifierEffect);

impl InvisibilityEffect {
    pub const RTTI: VariantID = RTTI_InvisibilityEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_InvisibilityEffect;

    // override (ActiveEffect)
    // void Update(float) override;                        // 04
    // void FinishLoadGame(BGSLoadFormBuffer*) override;  // 0A
    // void Revert(BGSLoadFormBuffer*) override;          // 0B
    // ~InvisibilityEffect() override;                    // 13
    // void Start() override;                             // 14
    // void Finish() override;                            // 15
}
