use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_CloakEffect;
use crate::offsets::offsets_vtable::VTABLE_CloakEffect;
use crate::re::ActiveEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::CloakEffect`
#[repr(C)]
pub struct CloakEffect {
    pub base: ActiveEffect,     // 00
    pub next_target_check: u64, // 90
}

const _: () = assert!(core::mem::size_of::<CloakEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(CloakEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(CloakEffect, next_target_check) == 0x90);

impl RttiType for CloakEffect {
    const RTTI: VariantID = RTTI_CloakEffect;
}

inherit!(CloakEffect : ActiveEffect);

impl CloakEffect {
    pub const RTTI: VariantID = RTTI_CloakEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_CloakEffect;

    // override (ActiveEffect)
    // void Update(float) override;  // 04
    // ~CloakEffect() override;      // 13
}
