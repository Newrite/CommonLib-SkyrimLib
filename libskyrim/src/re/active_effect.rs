use core_util::EnumSet;

use crate::offsets::offsets_rtti::{RTTI_ActiveEffect, RTTI_ActiveEffect__ForEachHitEffectVisitor};
use crate::offsets::offsets_vtable::{
    VTABLE_ActiveEffect, VTABLE_ActiveEffect__ForEachHitEffectVisitor,
};
use crate::re::ActiveEffectReferenceEffectController;
use crate::re::Actor;
use crate::re::ActorHandle;
use crate::re::BGSLoadFormBuffer;
use crate::re::BGSSaveFormBuffer;
use crate::re::BSFixedString;
use crate::re::BSSimpleList;
use crate::re::BSSoundHandle;
use crate::re::Effect;
use crate::re::EffectSetting;
use crate::re::MagicItem;
use crate::re::MagicTarget;
use crate::re::NiNode;
use crate::re::NiPoint3;
use crate::re::NiPointer;
use crate::re::ReferenceEffect;
use crate::re::TESBoundObject;
use crate::re::TESObjectREFR;
use crate::re::bs_container::BSContainerForEachResult;
use crate::re::bs_core_types::VMTypeID;
use crate::re::magic_system::CastingSource;
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::ActiveEffect::ForEachHitEffectVisitor`
#[repr(C)]
pub struct ActiveEffectForEachHitEffectVisitor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<ActiveEffectForEachHitEffectVisitor>() == 0x8);
const _: () = assert!(core::mem::offset_of!(ActiveEffectForEachHitEffectVisitor, vtable) == 0x00);

impl RttiType for ActiveEffectForEachHitEffectVisitor {
    const RTTI: VariantID = RTTI_ActiveEffect__ForEachHitEffectVisitor;
}

impl ActiveEffectForEachHitEffectVisitor {
    pub const RTTI: VariantID = RTTI_ActiveEffect__ForEachHitEffectVisitor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ActiveEffect__ForEachHitEffectVisitor;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_CALL: usize = 0x01;
        pub fn call(hit_effect: *mut ReferenceEffect) -> BSContainerForEachResult
    }
}

/// C++ `RE::ActiveEffect::Flag`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActiveEffectFlag {
    HasConditions = 1 << 7,
    Enchanting = 1 << 8,
    Recovers = 1 << 9,
    Dual = 1 << 12,
    Inactive = 1 << 15,
    Dispelled = 1 << 18,
    CustomSkillUse = 1 << 19,
}

core_util::impl_enumset_type!(ActiveEffectFlag => u32);

/// C++ `RE::ActiveEffect::ConditionStatus`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActiveEffectConditionStatus {
    NA = -1,
    False = 0,
    True = 1,
}

core_util::impl_enumset_type!(ActiveEffectConditionStatus => u32);

/// C++ `RE::ActiveEffect`
#[repr(C)]
pub struct ActiveEffect {
    pub vtable: *const usize,                                         // 00
    pub hit_effect_controller: ActiveEffectReferenceEffectController, // 08
    pub persistent_sound: BSSoundHandle,                              // 28
    pub caster: ActorHandle,                                          // 34
    pub source_node: NiPointer<NiNode>,                               // 38
    pub spell: *mut MagicItem,                                        // 40
    pub effect: *mut Effect,                                          // 48
    pub target: *mut MagicTarget,                                     // 50
    pub source: *mut TESBoundObject,                                  // 58
    pub hit_effects: *mut BSSimpleList<*mut ReferenceEffect>,         // 60
    pub displacement_spell: *mut MagicItem,                           // 68
    pub elapsed_seconds: f32,                                         // 70
    pub duration: f32,                                                // 74
    pub magnitude: f32,                                               // 78
    pub flags: EnumSet<ActiveEffectFlag, u32>,                        // 7C
    pub condition_status: EnumSet<ActiveEffectConditionStatus, u32>,  // 80
    pub us_unique_id: u16,                                            // 84
    pub pad86: u16,                                                   // 86
    pub casting_source: CastingSource,                                // 88
    pub pad8c: u32,                                                   // 8C
}

const _: () = assert!(core::mem::size_of::<ActiveEffect>() == 0x90);
const _: () = assert!(core::mem::offset_of!(ActiveEffect, hit_effect_controller) == 0x08);
const _: () = assert!(core::mem::offset_of!(ActiveEffect, persistent_sound) == 0x28);
const _: () = assert!(core::mem::offset_of!(ActiveEffect, caster) == 0x34);
const _: () = assert!(core::mem::offset_of!(ActiveEffect, effect) == 0x48);
const _: () = assert!(core::mem::offset_of!(ActiveEffect, target) == 0x50);
const _: () = assert!(core::mem::offset_of!(ActiveEffect, hit_effects) == 0x60);
const _: () = assert!(core::mem::offset_of!(ActiveEffect, flags) == 0x7C);
const _: () = assert!(core::mem::offset_of!(ActiveEffect, condition_status) == 0x80);
const _: () = assert!(core::mem::offset_of!(ActiveEffect, casting_source) == 0x88);

impl RttiType for ActiveEffect {
    const RTTI: VariantID = RTTI_ActiveEffect;
}

impl ActiveEffect {
    pub const RTTI: VariantID = RTTI_ActiveEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ActiveEffect;
    pub const VM_TYPE_ID: VMTypeID = 142;

    virtual_method! {
        pub const VFUNC_ADJUST_FOR_PERKS: usize = 0x00;
        pub fn adjust_for_perks(caster: *mut Actor, target: *mut MagicTarget)
    }

    virtual_method! {
        pub const VFUNC_ON_ADD: usize = 0x01;
        pub fn on_add(target: *mut MagicTarget)
    }

    virtual_method! {
        pub const VFUNC_ON_REMOVE: usize = 0x02;
        pub fn on_remove()
    }

    virtual_method! {
        pub const VFUNC_GET_VISUALS_TARGET: usize = 0x03;
        pub fn get_visuals_target() -> *mut TESObjectREFR
    }

    virtual_method! {
        pub const VFUNC_UPDATE: usize = 0x04;
        pub fn update(delta: f32)
    }

    virtual_method! {
        pub const VFUNC_EVALUATE_CONDITIONS: usize = 0x05;
        pub fn evaluate_conditions(delta: f32, force_update: bool)
    }

    virtual_method! {
        pub const VFUNC_IS_CAUSING_HEALTH_DAMAGE: usize = 0x06;
        pub fn is_causing_health_damage() -> bool
    }

    virtual_method! {
        pub const VFUNC_SET_LOCATION: usize = 0x07;
        pub fn set_location(location: *const NiPoint3)
    }

    virtual_method! {
        pub const VFUNC_SAVE_GAME: usize = 0x08;
        pub fn save_game(buf: *mut BGSSaveFormBuffer)
    }

    virtual_method! {
        pub const VFUNC_LOAD_GAME: usize = 0x09;
        pub fn load_game(buf: *mut BGSLoadFormBuffer)
    }

    virtual_method! {
        pub const VFUNC_FINISH_LOAD_GAME: usize = 0x0A;
        pub fn finish_load_game(buf: *mut BGSLoadFormBuffer)
    }

    virtual_method! {
        pub const VFUNC_REVERT: usize = 0x0B;
        pub fn revert(buf: *mut BGSLoadFormBuffer)
    }

    virtual_method! {
        pub const VFUNC_COMPARE: usize = 0x0C;
        pub fn compare(other_effect: *mut ActiveEffect) -> i32
    }

    virtual_method! {
        pub const VFUNC_HANDLE_EVENT: usize = 0x0D;
        pub fn handle_event(event_name: *const BSFixedString)
    }

    virtual_method! {
        pub const VFUNC_SWITCH_ATTACHED_ROOT: usize = 0x0E;
        pub fn switch_attached_root(root: *mut NiNode, attach_root: *mut NiNode)
    }

    virtual_method! {
        pub const VFUNC_HANDLE_QUEUED_START: usize = 0x0F;
        pub fn handle_queued_start()
    }

    virtual_method! {
        pub const VFUNC_SHOULD_DISPEL_ON_DEATH: usize = 0x10;
        pub fn should_dispel_on_death() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_ALLOW_MULTIPLE_CASTING_SOURCE_STACKING: usize = 0x11;
        pub fn get_allow_multiple_casting_source_stacking() -> bool
    }

    virtual_method! {
        pub const VFUNC_CLEAR_TARGET_IMPL: usize = 0x12;
        pub fn clear_target_impl()
    }

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x13;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_START: usize = 0x14;
        pub fn start()
    }

    virtual_method! {
        pub const VFUNC_FINISH: usize = 0x15;
        pub fn finish()
    }

    virtual_method! {
        pub const VFUNC_CAN_FINISH: usize = 0x16;
        pub fn can_finish() -> bool
    }

    virtual_method! {
        pub const VFUNC_CHECK_CUSTOM_SKILL_USE_CONDITIONS: usize = 0x17;
        pub fn check_custom_skill_use_conditions() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_CUSTOM_SKILL_USE_MAGNITUDE_MULTIPLIER: usize = 0x18;
        pub fn get_custom_skill_use_magnitude_multiplier(mult: f32) -> f32
    }

    crate::relocation_func! {
        pub fn dispel(&mut self, force: bool) => RelocationID::new(33286, 34061)
    }

    #[inline(always)]
    pub fn get_base_object(&self) -> *mut EffectSetting {
        unsafe { self.effect.as_ref() }
            .map(|effect| effect.base_effect)
            .unwrap_or(core::ptr::null_mut())
    }

    #[inline(always)]
    pub fn get_caster_actor(&self) -> NiPointer<Actor> {
        self.caster.get()
    }

    crate::relocation_func! {
        pub fn get_magnitude(&self) -> f32 => RelocationID::new(33282, 34057)
    }

    #[inline(always)]
    pub fn get_target_actor(&self) -> *mut Actor {
        if !self.target.is_null() && unsafe { (*self.target).magic_target_is_actor() } {
            self.target.cast::<Actor>()
        } else {
            core::ptr::null_mut()
        }
    }
}
