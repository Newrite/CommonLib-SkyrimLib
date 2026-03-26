use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_ActorMagicCaster;
use crate::offsets::offsets_vtable::VTABLE_ActorMagicCaster;
use crate::re::{
    Actor, BGSArtObject, BGSArtObjectCloneTask, BGSLoadGameSubBuffer, BSAnimationGraphEvent,
    BSEventNotifyControl, BSLight, BSTEventSink, BSTEventSource, MagicCaster,
    RefAttachTechniqueInput, ReferenceEffectController, SimpleAnimationGraphManagerHolder,
    magic_system::CastingSource,
};
use crate::relocation::{RelocationID, RttiType, VariantID, VariantOffset};

pub type ActorMagicCasterInterruptHandler = unsafe extern "C" fn(*mut Actor);

/// C++ `RE::ActorMagicCaster::Flags`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorMagicCasterFlags {
    None = 0,
    DualCasting = 1 << 0,
    SkipCheckCast = 1 << 1,
    StartCloneTask = 1 << 2,
    CastingArtAttached = 1 << 3,
    CheckDeferredInterrupt = 1 << 4,
    DeferInterrupt = 1 << 5,
}

core_util::impl_enumset_type!(ActorMagicCasterFlags => u32);

/// C++ `RE::ActorMagicCaster`
#[repr(C)]
pub struct ActorMagicCaster {
    pub base: MagicCaster,                                                 // 00
    pub animation_graph_manager_holder: SimpleAnimationGraphManagerHolder, // 48
    pub animation_graph_event_sink: BSTEventSink<BSAnimationGraphEvent>,   // 60
    pub casting_art_data: RefAttachTechniqueInput,                         // 68
    // TODO: `ActorMagicCaster.h` proves this member is `NiPointer<BGSArtObjectCloneTask>`, but
    // the vendored CommonLib tree only forward-declares `BGSArtObjectCloneTask`. Replace this raw
    // pointer stand-in with `NiPointer<BGSArtObjectCloneTask>` once that pointee's inheritance and
    // `NiRef` contract are source-backed instead of guessed.
    pub clone_task: *mut BGSArtObjectCloneTask, // B0
    pub actor: *mut Actor,                      // B8
    pub magic_node: *mut crate::re::NiNode,     // C0
    pub light: crate::re::NiPointer<BSLight>,   // C8
    pub interrupt_handler: Option<ActorMagicCasterInterruptHandler>, // D0
    pub load_game_sub_buffer: BGSLoadGameSubBuffer, // D8
    pub casting_art: *mut BGSArtObject,         // E0
    pub weapon_enchantment_controller: *mut ReferenceEffectController, // E8
    pub cost_charged: f32,                      // F0
    pub casting_source: CastingSource,          // F4
    pub flags: EnumSet<ActorMagicCasterFlags, u32>, // F8
}

const _: () = assert!(core::mem::size_of::<ActorMagicCaster>() == 0x100);
const _: () = assert!(core::mem::offset_of!(ActorMagicCaster, base) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(ActorMagicCaster, animation_graph_manager_holder) == 0x48);
const _: () = assert!(core::mem::offset_of!(ActorMagicCaster, animation_graph_event_sink) == 0x60);
const _: () = assert!(core::mem::offset_of!(ActorMagicCaster, casting_art_data) == 0x68);
const _: () = assert!(core::mem::offset_of!(ActorMagicCaster, clone_task) == 0xB0);
const _: () = assert!(core::mem::offset_of!(ActorMagicCaster, actor) == 0xB8);
const _: () = assert!(core::mem::offset_of!(ActorMagicCaster, magic_node) == 0xC0);
const _: () = assert!(core::mem::offset_of!(ActorMagicCaster, light) == 0xC8);
const _: () = assert!(core::mem::offset_of!(ActorMagicCaster, interrupt_handler) == 0xD0);
const _: () = assert!(core::mem::offset_of!(ActorMagicCaster, load_game_sub_buffer) == 0xD8);
const _: () = assert!(core::mem::offset_of!(ActorMagicCaster, casting_art) == 0xE0);
const _: () =
    assert!(core::mem::offset_of!(ActorMagicCaster, weapon_enchantment_controller) == 0xE8);
const _: () = assert!(core::mem::offset_of!(ActorMagicCaster, cost_charged) == 0xF0);
const _: () = assert!(core::mem::offset_of!(ActorMagicCaster, casting_source) == 0xF4);
const _: () = assert!(core::mem::offset_of!(ActorMagicCaster, flags) == 0xF8);

inherit!(ActorMagicCaster : MagicCaster);
inherit!(
    ActorMagicCaster => SimpleAnimationGraphManagerHolder,
    animation_graph_manager_holder
);
inherit!(
    ActorMagicCaster => BSTEventSink<BSAnimationGraphEvent>,
    animation_graph_event_sink
);

impl RttiType for ActorMagicCaster {
    const RTTI: VariantID = RTTI_ActorMagicCaster;
}

impl ActorMagicCaster {
    pub const RTTI: VariantID = RTTI_ActorMagicCaster;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ActorMagicCaster;
    pub const VFUNC_UPDATE: VariantOffset = VariantOffset::new(0x1D, 0x1D, 0x1F);
    pub const VFUNC_UNK_VR_1D: VariantOffset = VariantOffset::new(0, 0, 0x1D);
    pub const VFUNC_UNK_VR_1E: VariantOffset = VariantOffset::new(0, 0, 0x1E);

    // override (MagicCaster)
    // 00 ~ActorMagicCaster
    // 03 RequestCastImpl
    // 04 StartChargeImpl
    // 05 StartReadyImpl
    // 06 StartCastImpl
    // 07 FinishCastImpl
    // 08 InterruptCastImpl
    // 09 SpellCast
    // 0A CheckCast
    // 0B GetCasterStatsObject
    // 0C GetCasterAsActor
    // 0E GetMagicNode
    // 0F ClearMagicNode
    // 10 SetCurrentSpellImpl
    // 11 SelectSpellImpl
    // 12 DeselectSpellImpl
    // 13 SetSkipCheckCast
    // 14 SetCastingTimerForCharge
    // 15 GetCastingSource
    // 16 GetIsDualCasting
    // 17 SetDualCasting
    // 18 SaveGame
    // 19 LoadGame
    // 1A FinishLoadGame
    // 1B PrepareSound
    // 1C AdjustActiveEffect

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE_SLOT: VariantOffset = Self::VFUNC_UPDATE;
        pub fn update(&mut self, delta: f32)
    }

    #[inline(always)]
    pub fn unk_vr_1d(&mut self) {
        crate::runtime::require_vr("ActorMagicCaster::unk_vr_1d");
        crate::relocate_virtual!(
            extern "C" fn(*mut Self),
            self as *mut Self,
            Self::VFUNC_UNK_VR_1D
        )
    }

    #[inline(always)]
    pub fn unk_vr_1e(&mut self) {
        crate::runtime::require_vr("ActorMagicCaster::unk_vr_1e");
        crate::relocate_virtual!(
            extern "C" fn(*mut Self),
            self as *mut Self,
            Self::VFUNC_UNK_VR_1E
        )
    }

    crate::relocation_func! {
        pub fn check_attach_casting_art(&mut self) => RelocationID::new(33403, 34185)
    }

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
