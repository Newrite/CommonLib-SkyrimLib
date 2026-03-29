use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_NightEyeEffect;
use crate::offsets::offsets_vtable::VTABLE_NightEyeEffect;
use crate::re::ValueModifierEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::NightEyeEffect`
#[repr(C)]
pub struct NightEyeEffect {
    pub base: ValueModifierEffect, // 00
}

const _: () = assert!(core::mem::size_of::<NightEyeEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(NightEyeEffect, base) == 0x00);

impl RttiType for NightEyeEffect {
    const RTTI: VariantID = RTTI_NightEyeEffect;
}

inherit!(NightEyeEffect : ValueModifierEffect);

impl NightEyeEffect {
    pub const RTTI: VariantID = RTTI_NightEyeEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NightEyeEffect;

    // override (ActiveEffect)
    // void Update(float) override;  // 04
    // ~NightEyeEffect() override;   // 13
    // void Start() override;        // 14
    // void Finish() override;       // 15
}
