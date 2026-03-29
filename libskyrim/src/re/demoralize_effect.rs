use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_DemoralizeEffect;
use crate::offsets::offsets_vtable::VTABLE_DemoralizeEffect;
use crate::re::TargetValueModifierEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::DemoralizeEffect`
#[repr(C)]
pub struct DemoralizeEffect {
    pub base: TargetValueModifierEffect, // 00
}

const _: () = assert!(core::mem::size_of::<DemoralizeEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(DemoralizeEffect, base) == 0x00);

impl RttiType for DemoralizeEffect {
    const RTTI: VariantID = RTTI_DemoralizeEffect;
}

inherit!(DemoralizeEffect : TargetValueModifierEffect);

impl DemoralizeEffect {
    pub const RTTI: VariantID = RTTI_DemoralizeEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_DemoralizeEffect;

    // override (TargetValueModifierEffect)
    // 13 ~DemoralizeEffect
    // 21 GetTargetValue
}
