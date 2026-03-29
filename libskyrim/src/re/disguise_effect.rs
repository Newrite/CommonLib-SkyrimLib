use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_DisguiseEffect;
use crate::offsets::offsets_vtable::VTABLE_DisguiseEffect;
use crate::re::ActiveEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::DisguiseEffect::State`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisguiseEffectState {
    Initiate = 0,
    Waiting = 1,
    Normal = 2,
    Fail = 3,
}

/// C++ `RE::DisguiseEffect`
#[repr(C)]
pub struct DisguiseEffect {
    pub base: ActiveEffect,         // 00
    pub state: DisguiseEffectState, // 90
}

const _: () = assert!(core::mem::size_of::<DisguiseEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(DisguiseEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(DisguiseEffect, state) == 0x90);

impl RttiType for DisguiseEffect {
    const RTTI: VariantID = RTTI_DisguiseEffect;
}

inherit!(DisguiseEffect : ActiveEffect);

impl DisguiseEffect {
    pub const RTTI: VariantID = RTTI_DisguiseEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_DisguiseEffect;

    // override (ActiveEffect)
    // void EvaluateConditions(float, bool) override;   // 05
    // void SaveGame(BGSSaveFormBuffer*) override;      // 08
    // void LoadGame(BGSLoadFormBuffer*) override;      // 09
    // void FinishLoadGame(BGSLoadFormBuffer*) override; // 0A
    // ~DisguiseEffect() override;                      // 13
    // void Start() override;                           // 14
    // void Finish() override;                          // 15
}
