use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ValueAndConditionsEffect;
use crate::offsets::offsets_vtable::VTABLE_ValueAndConditionsEffect;
use crate::re::{ActorValue, ValueModifierEffect};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ValueAndConditionsEffect`
#[repr(C)]
pub struct ValueAndConditionsEffect {
    pub base: ValueModifierEffect, // 00
    pub actor_value: ActorValue,   // 98
    pub pad9c: u32,                // 9C
}

const _: () = assert!(core::mem::size_of::<ValueAndConditionsEffect>() == 0xA0);
const _: () = assert!(core::mem::offset_of!(ValueAndConditionsEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ValueAndConditionsEffect, actor_value) == 0x98);

impl RttiType for ValueAndConditionsEffect {
    const RTTI: VariantID = RTTI_ValueAndConditionsEffect;
}

inherit!(ValueAndConditionsEffect : ValueModifierEffect);

impl ValueAndConditionsEffect {
    pub const RTTI: VariantID = RTTI_ValueAndConditionsEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ValueAndConditionsEffect;

    // override (ActiveEffect)
    // void SaveGame(BGSSaveFormBuffer*) override;  // 08
    // void LoadGame(BGSLoadFormBuffer*) override;  // 09
    // ~ValueAndConditionsEffect() override;        // 13

    // override (ValueModifierEffect)
    // void ModifyActorValue(Actor*, float, ActorValue) override;  // 20
}
