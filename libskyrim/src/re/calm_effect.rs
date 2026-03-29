use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_CalmEffect;
use crate::offsets::offsets_vtable::VTABLE_CalmEffect;
use crate::re::TargetValueModifierEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::CalmEffect`
#[repr(C)]
pub struct CalmEffect {
    pub base: TargetValueModifierEffect, // 00
}

const _: () = assert!(core::mem::size_of::<CalmEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(CalmEffect, base) == 0x00);

impl RttiType for CalmEffect {
    const RTTI: VariantID = RTTI_CalmEffect;
}

inherit!(CalmEffect : TargetValueModifierEffect);

impl CalmEffect {
    pub const RTTI: VariantID = RTTI_CalmEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_CalmEffect;

    // override (TargetValueModifierEffect)
    // 13 ~CalmEffect
    // 14 Start
    // 21 GetTargetValue
}
