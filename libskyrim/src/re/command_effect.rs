use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_CommandEffect;
use crate::offsets::offsets_vtable::VTABLE_CommandEffect;
use crate::re::{ActiveEffect, ActorHandle};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::CommandEffect`
#[repr(C)]
pub struct CommandEffect {
    pub base: ActiveEffect,           // 00
    pub commanded_actor: ActorHandle, // 90
    pub pad94: u32,                   // 94
}

const _: () = assert!(core::mem::size_of::<CommandEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(CommandEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(CommandEffect, commanded_actor) == 0x90);
const _: () = assert!(core::mem::offset_of!(CommandEffect, pad94) == 0x94);

impl RttiType for CommandEffect {
    const RTTI: VariantID = RTTI_CommandEffect;
}

inherit!(CommandEffect : ActiveEffect);

impl CommandEffect {
    pub const RTTI: VariantID = RTTI_CommandEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_CommandEffect;

    // override (ActiveEffect)
    // void OnAdd(MagicTarget*) override;              // 01
    // TESObjectREFR* GetVisualsTarget() override;     // 03
    // void Update(float) override;                    // 04
    // void SaveGame(BGSSaveFormBuffer*) override;     // 08
    // void LoadGame(BGSLoadFormBuffer*) override;     // 09
    // void FinishLoadGame(BGSLoadFormBuffer*) override; // 0A
    // void ClearTargetImpl() override;                // 12
    // ~CommandEffect() override;                      // 13
    // void Start() override;                          // 14
    // void Finish() override;                         // 15
}
