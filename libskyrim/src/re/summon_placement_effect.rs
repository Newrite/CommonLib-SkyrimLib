use core_util::inherit;

use crate::offsets::offsets_nirtti::NiRTTI_SummonPlacementEffect;
use crate::offsets::offsets_rtti::RTTI_SummonPlacementEffect;
use crate::offsets::offsets_vtable::VTABLE_SummonPlacementEffect;
use crate::re::{
    BGSArtObject, BGSArtObjectCloneTask, BGSLoadGameSubBuffer, BSAnimationGraphEvent,
    BSEventNotifyControl, BSTEventSink, BSTEventSource, NiAVObject, NiPoint3, NiPointer,
    ReferenceEffect, SimpleAnimationGraphManagerHolder, TempEffectType,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::SummonPlacementEffect`
#[repr(C)]
pub struct SummonPlacementEffect {
    pub base: ReferenceEffect,                                             // 00
    pub animation_graph_manager_holder: SimpleAnimationGraphManagerHolder, // 48
    pub animation_graph_event_sink: BSTEventSink<BSAnimationGraphEvent>,   // 60
    pub art_object: *mut BGSArtObject,                                     // 68
    pub location: NiPoint3,                                                // 70
    pub pad7c: u32,                                                        // 7C
    pub art_object_3d: NiPointer<NiAVObject>,                              // 80
    pub loaded_data_sub_buffer: BGSLoadGameSubBuffer,                      // 88
    // TODO: `SummonPlacementEffect.h` proves this field is
    // `BSTSmartPointer<BGSArtObjectCloneTask>`, but the vendored CommonLib tree only exposes
    // RTTI/VTABLE plus forward declarations for `BGSArtObjectCloneTask`. Replace this raw pointer
    // stand-in with the exact smart-pointer field once that pointee's intrusive refcount/delete
    // contract is source-backed instead of guessed.
    pub clone_task: *mut BGSArtObjectCloneTask, // 90
    pub animation_complete: bool,               // 98
}

const _: () = assert!(core::mem::size_of::<SummonPlacementEffect>() == 0xA0);
const _: () = assert!(core::mem::offset_of!(SummonPlacementEffect, base) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(SummonPlacementEffect, animation_graph_manager_holder) == 0x48);
const _: () =
    assert!(core::mem::offset_of!(SummonPlacementEffect, animation_graph_event_sink) == 0x60);
const _: () = assert!(core::mem::offset_of!(SummonPlacementEffect, art_object) == 0x68);
const _: () = assert!(core::mem::offset_of!(SummonPlacementEffect, location) == 0x70);
const _: () = assert!(core::mem::offset_of!(SummonPlacementEffect, art_object_3d) == 0x80);
const _: () = assert!(core::mem::offset_of!(SummonPlacementEffect, loaded_data_sub_buffer) == 0x88);
const _: () = assert!(core::mem::offset_of!(SummonPlacementEffect, clone_task) == 0x90);
const _: () = assert!(core::mem::offset_of!(SummonPlacementEffect, animation_complete) == 0x98);

impl RttiType for SummonPlacementEffect {
    const RTTI: VariantID = RTTI_SummonPlacementEffect;
}

inherit!(SummonPlacementEffect : ReferenceEffect);
inherit!(
    SummonPlacementEffect => SimpleAnimationGraphManagerHolder,
    animation_graph_manager_holder
);
inherit!(
    SummonPlacementEffect => BSTEventSink<BSAnimationGraphEvent>,
    animation_graph_event_sink
);

impl SummonPlacementEffect {
    pub const RTTI: VariantID = RTTI_SummonPlacementEffect;
    pub const NI_RTTI: VariantID = NiRTTI_SummonPlacementEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SummonPlacementEffect;
    pub const TYPE: TempEffectType = TempEffectType::MagicSummon;

    // override (ReferenceEffect)
    // const NiRTTI* GetRTTI() const override;                   // 02
    // bool Update(float) override;                              // 28
    // TEMP_EFFECT_TYPE GetType() const override;                // 2C
    // void SaveGame(BGSSaveGameBuffer*) override;               // 2D
    // void LoadGame(BGSLoadGameBuffer*) override;               // 2E
    // void FinishLoadGame(BGSLoadGameBuffer*) override;         // 2F
    // void Init() override;                                     // 36

    // override (SimpleAnimationGraphManagerHolder)
    // bool SetupAnimEventSinks(const BSTSmartPointer<BShkbAnimationGraph>&) override;  // 08

    // override (BSTEventSink<BSAnimationGraphEvent>)
    // BSEventNotifyControl ProcessEvent(...) override;          // 01

    #[inline(always)]
    pub fn process_animation_graph_event(
        &mut self,
        event: *const BSAnimationGraphEvent,
        event_source: *mut BSTEventSource<BSAnimationGraphEvent>,
    ) -> BSEventNotifyControl {
        unsafe {
            self.animation_graph_event_sink
                .process_event(event, event_source)
        }
    }
}
