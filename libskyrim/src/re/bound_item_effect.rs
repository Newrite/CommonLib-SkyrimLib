use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BoundItemEffect;
use crate::offsets::offsets_vtable::VTABLE_BoundItemEffect;
use crate::re::{
    ActiveEffect, ActorInventoryEvent, BSEventNotifyControl, BSTArray, BSTEventSink,
    BSTEventSource, SpellItem,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BoundItemEffect`
#[repr(C)]
pub struct BoundItemEffect {
    pub base: ActiveEffect,                                      // 00
    pub inventory_event_sink: BSTEventSink<ActorInventoryEvent>, // 90
    pub spells: BSTArray<*mut SpellItem>,                        // 98
    pub unk_b0: bool,                                            // B0
    pub unk_b1: bool,                                            // B1
    pub unk_b2: bool,                                            // B2
}

const _: () = assert!(core::mem::size_of::<BoundItemEffect>() == 0xB8);
const _: () = assert!(core::mem::offset_of!(BoundItemEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BoundItemEffect, inventory_event_sink) == 0x90);
const _: () = assert!(core::mem::offset_of!(BoundItemEffect, spells) == 0x98);
const _: () = assert!(core::mem::offset_of!(BoundItemEffect, unk_b0) == 0xB0);
const _: () = assert!(core::mem::offset_of!(BoundItemEffect, unk_b1) == 0xB1);
const _: () = assert!(core::mem::offset_of!(BoundItemEffect, unk_b2) == 0xB2);

impl RttiType for BoundItemEffect {
    const RTTI: VariantID = RTTI_BoundItemEffect;
}

inherit!(BoundItemEffect : ActiveEffect);
inherit!(
    BoundItemEffect => BSTEventSink<ActorInventoryEvent>,
    inventory_event_sink
);

impl BoundItemEffect {
    pub const RTTI: VariantID = RTTI_BoundItemEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BoundItemEffect;

    // override (ActiveEffect)
    // void Update(float) override;                     // 04
    // void SaveGame(BGSSaveFormBuffer*) override;      // 08
    // void LoadGame(BGSLoadFormBuffer*) override;      // 09
    // void FinishLoadGame(BGSLoadFormBuffer*) override; // 0A
    // void Revert(BGSLoadFormBuffer*) override;        // 0B
    // void ClearTargetImpl() override;                 // 12
    // ~BoundItemEffect() override;                     // 13
    // void Start() override;                           // 14
    // void Finish() override;                          // 15
    // bool CanFinish() override;                       // 16

    // override (BSTEventSink<ActorInventoryEvent>)
    // BSEventNotifyControl ProcessEvent(...) override; // 01

    #[inline(always)]
    pub fn process_inventory_event(
        &mut self,
        event: *const ActorInventoryEvent,
        event_source: *mut BSTEventSource<ActorInventoryEvent>,
    ) -> BSEventNotifyControl {
        unsafe { self.inventory_event_sink.process_event(event, event_source) }
    }
}
