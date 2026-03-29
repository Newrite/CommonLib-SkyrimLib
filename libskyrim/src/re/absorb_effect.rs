use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_AbsorbEffect;
use crate::offsets::offsets_vtable::VTABLE_AbsorbEffect;
use crate::re::ValueModifierEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::AbsorbEffect`
#[repr(C)]
pub struct AbsorbEffect {
    pub base: ValueModifierEffect, // 00
}

const _: () = assert!(core::mem::size_of::<AbsorbEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(AbsorbEffect, base) == 0x00);

impl RttiType for AbsorbEffect {
    const RTTI: VariantID = RTTI_AbsorbEffect;
}

inherit!(AbsorbEffect : ValueModifierEffect);

impl AbsorbEffect {
    pub const RTTI: VariantID = RTTI_AbsorbEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_AbsorbEffect;

    // override (ValueModifierEffect)
    // 13 ~AbsorbEffect
    // 1B ModifyOnStart
    // 1D ModifyOnUpdate
    // 1F ModifyOnFinish
}
