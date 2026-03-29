use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_EnhanceWeaponEffect;
use crate::offsets::offsets_vtable::VTABLE_EnhanceWeaponEffect;
use crate::re::{
    ActorInventoryEvent, ActorValue, BSEventNotifyControl, BSTEventSink, BSTEventSource,
    DualValueModifierEffect,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::EnhanceWeaponEffect`
#[repr(C)]
pub struct EnhanceWeaponEffect {
    pub base: DualValueModifierEffect,                           // 00
    pub inventory_event_sink: BSTEventSink<ActorInventoryEvent>, // A0
    pub secondary_actor_value: ActorValue,                       // A8
}

const _: () = assert!(core::mem::size_of::<EnhanceWeaponEffect>() == 0xB0);
const _: () = assert!(core::mem::offset_of!(EnhanceWeaponEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(EnhanceWeaponEffect, inventory_event_sink) == 0xA0);
const _: () = assert!(core::mem::offset_of!(EnhanceWeaponEffect, secondary_actor_value) == 0xA8);

impl RttiType for EnhanceWeaponEffect {
    const RTTI: VariantID = RTTI_EnhanceWeaponEffect;
}

inherit!(EnhanceWeaponEffect : DualValueModifierEffect);
inherit!(
    EnhanceWeaponEffect => BSTEventSink<ActorInventoryEvent>,
    inventory_event_sink
);

impl EnhanceWeaponEffect {
    pub const RTTI: VariantID = RTTI_EnhanceWeaponEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_EnhanceWeaponEffect;

    // override (ActiveEffect)
    // void FinishLoadGame(BGSLoadFormBuffer*) override;  // 0A
    // void Revert(BGSLoadFormBuffer*) override;          // 0B
    // void ClearTargetImpl() override;                   // 12
    // ~EnhanceWeaponEffect() override;                   // 13
    // void Start() override;                             // 14
    // void Finish() override;                            // 15

    // override (DualValueModifierEffect)
    // ActorValue GetAdditionalActorValue() const override;  // 21
    // float GetSecondaryAVWeight() const override;          // 22

    // override (BSTEventSink<ActorInventoryEvent>)
    // BSEventNotifyControl ProcessEvent(...) override;      // 01

    #[inline(always)]
    pub fn process_inventory_event(
        &mut self,
        event: *const ActorInventoryEvent,
        event_source: *mut BSTEventSource<ActorInventoryEvent>,
    ) -> BSEventNotifyControl {
        unsafe { self.inventory_event_sink.process_event(event, event_source) }
    }
}
