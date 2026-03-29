use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_SlowTimeEffect;
use crate::offsets::offsets_vtable::VTABLE_SlowTimeEffect;
use crate::re::{BSSoundHandle, ScriptEffect};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::SlowTimeEffect`
#[repr(C)]
pub struct SlowTimeEffect {
    pub base: ScriptEffect,          // 00
    pub active_sound: BSSoundHandle, // A0
}

const _: () = assert!(core::mem::size_of::<SlowTimeEffect>() == 0xB0);
const _: () = assert!(core::mem::offset_of!(SlowTimeEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(SlowTimeEffect, active_sound) == 0xA0);

impl RttiType for SlowTimeEffect {
    const RTTI: VariantID = RTTI_SlowTimeEffect;
}

inherit!(SlowTimeEffect : ScriptEffect);

impl SlowTimeEffect {
    pub const RTTI: VariantID = RTTI_SlowTimeEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SlowTimeEffect;

    // override (ActiveEffect)
    // ~SlowTimeEffect() override;  // 13
    // void Start() override;       // 14
    // void Finish() override;      // 15
}
