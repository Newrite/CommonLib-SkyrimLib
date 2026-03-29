use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_DarknessEffect;
use crate::offsets::offsets_vtable::VTABLE_DarknessEffect;
use crate::re::ValueModifierEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::DarknessEffect`
#[repr(C)]
pub struct DarknessEffect {
    pub base: ValueModifierEffect, // 00
}

const _: () = assert!(core::mem::size_of::<DarknessEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(DarknessEffect, base) == 0x00);

impl RttiType for DarknessEffect {
    const RTTI: VariantID = RTTI_DarknessEffect;
}

inherit!(DarknessEffect : ValueModifierEffect);

impl DarknessEffect {
    pub const RTTI: VariantID = RTTI_DarknessEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_DarknessEffect;

    // override (ActiveEffect)
    // 13 ~DarknessEffect
    // 14 Start
    // 15 Finish
}
