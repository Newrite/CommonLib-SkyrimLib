use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_AccumulatingValueModifierEffect;
use crate::offsets::offsets_vtable::VTABLE_AccumulatingValueModifierEffect;
use crate::re::ValueModifierEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::AccumulatingValueModifierEffect`
#[repr(C)]
pub struct AccumulatingValueModifierEffect {
    pub base: ValueModifierEffect,  // 00
    pub accumulated_magnitude: f32, // 98
    pub maximum_magnitude: f32,     // 9C
    pub hold_timer: f32,            // A0
}

const _: () = assert!(core::mem::size_of::<AccumulatingValueModifierEffect>() == 0xA8);
const _: () = assert!(core::mem::offset_of!(AccumulatingValueModifierEffect, base) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(AccumulatingValueModifierEffect, accumulated_magnitude) == 0x98);
const _: () =
    assert!(core::mem::offset_of!(AccumulatingValueModifierEffect, maximum_magnitude) == 0x9C);
const _: () = assert!(core::mem::offset_of!(AccumulatingValueModifierEffect, hold_timer) == 0xA0);

impl RttiType for AccumulatingValueModifierEffect {
    const RTTI: VariantID = RTTI_AccumulatingValueModifierEffect;
}

inherit!(AccumulatingValueModifierEffect : ValueModifierEffect);

impl AccumulatingValueModifierEffect {
    pub const RTTI: VariantID = RTTI_AccumulatingValueModifierEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_AccumulatingValueModifierEffect;

    // override (ActiveEffect)
    // void SaveGame(BGSSaveFormBuffer*) override;  // 08
    // void LoadGame(BGSLoadFormBuffer*) override;  // 09
    // ~AccumulatingValueModifierEffect() override; // 13
    // void Start() override;                       // 14
    // void Finish() override;                      // 15

    // override (ValueModifierEffect)
    // bool ShouldModifyOnStart() override;                           // 1A
    // bool ShouldModifyOnUpdate() const override;                    // 1C
    // void ModifyOnUpdate(float) override;                           // 1D
    // void ModifyOnFinish(Actor*, Actor*, float) override;           // 1F
}
