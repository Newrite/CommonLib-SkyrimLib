use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_LockEffect;
use crate::offsets::offsets_vtable::VTABLE_LockEffect;
use crate::re::ActiveEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::LockEffect`
#[repr(C)]
pub struct LockEffect {
    pub base: ActiveEffect, // 00
}

const _: () = assert!(core::mem::size_of::<LockEffect>() == 0x90);
const _: () = assert!(core::mem::offset_of!(LockEffect, base) == 0x00);

impl RttiType for LockEffect {
    const RTTI: VariantID = RTTI_LockEffect;
}

inherit!(LockEffect : ActiveEffect);

impl LockEffect {
    pub const RTTI: VariantID = RTTI_LockEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_LockEffect;

    // override (ActiveEffect)
    // ~LockEffect() override;  // 13
    // void Start() override;   // 14
}
