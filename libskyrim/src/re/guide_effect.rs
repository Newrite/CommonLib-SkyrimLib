use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_GuideEffect;
use crate::offsets::offsets_vtable::VTABLE_GuideEffect;
use crate::re::tes_quest::TESQuestTarget;
use crate::re::{ActiveEffect, BSTArray, ObjectRefHandle, TESQuest};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::GuideEffect`
#[repr(C)]
pub struct GuideEffect {
    pub base: ActiveEffect,                 // 00
    pub quest: *mut TESQuest,               // 90
    pub quest_target: *mut TESQuestTarget,  // 98
    pub hazards: BSTArray<ObjectRefHandle>, // A0
}

const _: () = assert!(core::mem::size_of::<GuideEffect>() == 0xB8);
const _: () = assert!(core::mem::offset_of!(GuideEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(GuideEffect, quest) == 0x90);
const _: () = assert!(core::mem::offset_of!(GuideEffect, quest_target) == 0x98);
const _: () = assert!(core::mem::offset_of!(GuideEffect, hazards) == 0xA0);

impl RttiType for GuideEffect {
    const RTTI: VariantID = RTTI_GuideEffect;
}

inherit!(GuideEffect : ActiveEffect);

impl GuideEffect {
    pub const RTTI: VariantID = RTTI_GuideEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_GuideEffect;

    // override (ActiveEffect)
    // ~GuideEffect() override;  // 13
    // void Start() override;    // 14
    // void Finish() override;   // 15
}
