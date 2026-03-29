use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ScriptedRefEffect;
use crate::offsets::offsets_vtable::VTABLE_ScriptedRefEffect;
use crate::re::ScriptEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ScriptedRefEffect`
#[repr(C)]
pub struct ScriptedRefEffect {
    pub base: ScriptEffect, // 00
}

const _: () = assert!(core::mem::size_of::<ScriptedRefEffect>() == 0xA0);
const _: () = assert!(core::mem::offset_of!(ScriptedRefEffect, base) == 0x00);

impl RttiType for ScriptedRefEffect {
    const RTTI: VariantID = RTTI_ScriptedRefEffect;
}

inherit!(ScriptedRefEffect : ScriptEffect);

impl ScriptedRefEffect {
    pub const RTTI: VariantID = RTTI_ScriptedRefEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ScriptedRefEffect;

    // override (ActiveEffect)
    // ~ScriptedRefEffect() override;  // 13
}
