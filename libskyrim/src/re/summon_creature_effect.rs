use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_SummonCreatureEffect;
use crate::offsets::offsets_vtable::VTABLE_SummonCreatureEffect;
use crate::re::{ActiveEffect, ActorHandle, NiPoint3, SummonPlacementEffect};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::SummonCreatureEffect`
#[repr(C)]
pub struct SummonCreatureEffect {
    pub base: ActiveEffect,                                  // 00
    pub location: NiPoint3,                                  // 90
    pub rotation: NiPoint3,                                  // 9C
    pub commanded_actor: ActorHandle,                        // A8
    pub unk_ac: u32,                                         // AC
    pub summon_placement_effect: *mut SummonPlacementEffect, // B0
    pub unk_b8: bool,                                        // B8
    pub unk_b9: bool,                                        // B9
    pub pad_ba: u16,                                         // BA
    pub pad_bc: u32,                                         // BC
}

const _: () = assert!(core::mem::size_of::<SummonCreatureEffect>() == 0xC0);
const _: () = assert!(core::mem::offset_of!(SummonCreatureEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(SummonCreatureEffect, location) == 0x90);
const _: () = assert!(core::mem::offset_of!(SummonCreatureEffect, rotation) == 0x9C);
const _: () = assert!(core::mem::offset_of!(SummonCreatureEffect, commanded_actor) == 0xA8);
const _: () = assert!(core::mem::offset_of!(SummonCreatureEffect, summon_placement_effect) == 0xB0);
const _: () = assert!(core::mem::offset_of!(SummonCreatureEffect, unk_b8) == 0xB8);
const _: () = assert!(core::mem::offset_of!(SummonCreatureEffect, unk_b9) == 0xB9);

impl RttiType for SummonCreatureEffect {
    const RTTI: VariantID = RTTI_SummonCreatureEffect;
}

inherit!(SummonCreatureEffect : ActiveEffect);

impl SummonCreatureEffect {
    pub const RTTI: VariantID = RTTI_SummonCreatureEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SummonCreatureEffect;

    // override (ActiveEffect)
    // void Update(float) override;                    // 04
    // void SetLocation(const NiPoint3&) override;     // 07
    // void SaveGame(BGSSaveFormBuffer*) override;     // 08
    // void LoadGame(BGSLoadFormBuffer*) override;     // 09
    // void FinishLoadGame(BGSLoadFormBuffer*) override;  // 0A
    // void HandleEvent(const BSFixedString&) override;   // 0D
    // void ClearTargetImpl() override;               // 12
    // ~SummonCreatureEffect() override;              // 13
    // void Start() override;                         // 14
    // void Finish() override;                        // 15
}
