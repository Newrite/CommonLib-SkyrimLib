use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_OpenEffect;
use crate::offsets::offsets_vtable::VTABLE_OpenEffect;
use crate::re::ActiveEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::OpenEffect`
#[repr(C)]
pub struct OpenEffect {
    pub base: ActiveEffect, // 00
}

const _: () = assert!(core::mem::size_of::<OpenEffect>() == 0x90);
const _: () = assert!(core::mem::offset_of!(OpenEffect, base) == 0x00);

impl RttiType for OpenEffect {
    const RTTI: VariantID = RTTI_OpenEffect;
}

inherit!(OpenEffect : ActiveEffect);

impl OpenEffect {
    pub const RTTI: VariantID = RTTI_OpenEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_OpenEffect;

    // override (ActiveEffect)
    // ~OpenEffect() override;  // 13
    // void Start() override;   // 14
}
