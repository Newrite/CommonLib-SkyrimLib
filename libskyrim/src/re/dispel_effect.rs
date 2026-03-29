use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_DispelEffect;
use crate::offsets::offsets_vtable::VTABLE_DispelEffect;
use crate::re::ActiveEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::DispelEffect`
#[repr(C)]
pub struct DispelEffect {
    pub base: ActiveEffect, // 00
}

const _: () = assert!(core::mem::size_of::<DispelEffect>() == 0x90);
const _: () = assert!(core::mem::offset_of!(DispelEffect, base) == 0x00);

impl RttiType for DispelEffect {
    const RTTI: VariantID = RTTI_DispelEffect;
}

inherit!(DispelEffect : ActiveEffect);

impl DispelEffect {
    pub const RTTI: VariantID = RTTI_DispelEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_DispelEffect;

    // override (ActiveEffect)
    // ~DispelEffect() override;  // 13
    // void Start() override;     // 14
}
