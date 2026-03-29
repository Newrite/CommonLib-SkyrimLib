use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_VampireLordEffect;
use crate::offsets::offsets_vtable::VTABLE_VampireLordEffect;
use crate::re::ActiveEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::VampireLordEffect`
#[repr(C)]
pub struct VampireLordEffect {
    pub base: ActiveEffect, // 00
}

const _: () = assert!(core::mem::size_of::<VampireLordEffect>() == 0x90);
const _: () = assert!(core::mem::offset_of!(VampireLordEffect, base) == 0x00);

impl RttiType for VampireLordEffect {
    const RTTI: VariantID = RTTI_VampireLordEffect;
}

inherit!(VampireLordEffect : ActiveEffect);

impl VampireLordEffect {
    pub const RTTI: VariantID = RTTI_VampireLordEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_VampireLordEffect;

    // override (ActiveEffect)
    // ~VampireLordEffect() override;  // 13
    // void Start() override;          // 14
    // void Finish() override;         // 15
}
