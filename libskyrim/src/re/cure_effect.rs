use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_CureEffect;
use crate::offsets::offsets_vtable::VTABLE_CureEffect;
use crate::re::{ActiveEffect, EffectArchetype, magic_system::SpellType};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::CureEffect`
#[repr(C)]
pub struct CureEffect {
    pub base: ActiveEffect,            // 00
    pub spell_type: SpellType,         // 90
    pub archetype_id: EffectArchetype, // 94
}

const _: () = assert!(core::mem::size_of::<CureEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(CureEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(CureEffect, spell_type) == 0x90);
const _: () = assert!(core::mem::offset_of!(CureEffect, archetype_id) == 0x94);

impl RttiType for CureEffect {
    const RTTI: VariantID = RTTI_CureEffect;
}

inherit!(CureEffect : ActiveEffect);

impl CureEffect {
    pub const RTTI: VariantID = RTTI_CureEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_CureEffect;

    // override (ActiveEffect)
    // ~CureEffect() override;  // 13
    // void Start() override;   // 14
}
