use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ScriptEffect;
use crate::offsets::offsets_vtable::VTABLE_ScriptEffect;
use crate::re::{ActiveEffect, Script, ScriptLocals};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ScriptEffect`
#[repr(C)]
pub struct ScriptEffect {
    pub base: ActiveEffect,               // 00
    pub script: *mut Script,              // 90
    pub effect_locals: *mut ScriptLocals, // 98
}

const _: () = assert!(core::mem::size_of::<ScriptEffect>() == 0xA0);
const _: () = assert!(core::mem::offset_of!(ScriptEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ScriptEffect, script) == 0x90);
const _: () = assert!(core::mem::offset_of!(ScriptEffect, effect_locals) == 0x98);

impl RttiType for ScriptEffect {
    const RTTI: VariantID = RTTI_ScriptEffect;
}

inherit!(ScriptEffect : ActiveEffect);

impl ScriptEffect {
    pub const RTTI: VariantID = RTTI_ScriptEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ScriptEffect;

    // override (ActiveEffect)
    // void Update(float) override;               // 04
    // void SaveGame(BGSSaveFormBuffer*) override;  // 08
    // void LoadGame(BGSLoadFormBuffer*) override;  // 09
    // void ClearTargetImpl() override;           // 12
    // ~ScriptEffect() override;                  // 13
    // void Start() override;                     // 14
    // void Finish() override;                    // 15
}
