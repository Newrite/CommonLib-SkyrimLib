use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::{RTTI_MagicCaster, RTTI_MagicCaster__PostCreationCallback};
use crate::offsets::offsets_vtable::{
    VTABLE_MagicCaster, VTABLE_MagicCaster__PostCreationCallback,
};
use crate::re::ActiveEffect;
use crate::re::Actor;
use crate::re::BGSLoadGameBuffer;
use crate::re::BGSSaveGameBuffer;
use crate::re::BSSoundHandle;
use crate::re::Effect;
use crate::re::MagicItem;
use crate::re::MagicTarget;
use crate::re::MagicTargetPostCreationModification;
use crate::re::NiNode;
use crate::re::NiPoint3;
use crate::re::ObjectRefHandle;
use crate::re::TESBoundObject;
use crate::re::TESObjectCELL;
use crate::re::TESObjectREFR;
use crate::re::bst_array::BSTArray;
use crate::re::magic_system::{CannotCastReason, CastingSource, SoundID};
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::COL_LAYER`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColLayer {
    Static = 1,
    Terrain = 13,
    Ground = 17,
}

/// C++ `RE::CFilter`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CFilter {
    pub filter: u32, // 00
}

impl CFilter {
    #[inline(always)]
    pub fn get_collision_layer(self) -> ColLayer {
        unsafe { core::mem::transmute((self.filter & 0x7F) as i32) }
    }
}

#[repr(C)]
pub struct hkpCollidableCollisionView {
    pub pad00: [u8; 0x2C],              // 00
    pub collision_filter_info: CFilter, // 2C
}

const _: () = assert!(core::mem::size_of::<hkpCollidableCollisionView>() == 0x30);
const _: () =
    assert!(core::mem::offset_of!(hkpCollidableCollisionView, collision_filter_info) == 0x2C);

/// Source-backed partial layout of `RE::bhkPickData` for projectile validation.
#[repr(C)]
pub struct bhkPickData {
    pub pad00: [u8; 0x80],                                  // 00
    pub root_collidable: *const hkpCollidableCollisionView, // 80
    pub pad88: [u8; 0x38],                                  // 88
    pub pick_failed: bool,                                  // C0
    pub padc1: u8,                                          // C1
    pub padc2: u16,                                         // C2
    pub padc4: u32,                                         // C4
    pub padc8: u32,                                         // C8
    pub padcc: u32,                                         // CC
}

const _: () = assert!(core::mem::size_of::<bhkPickData>() == 0xD0);
const _: () = assert!(core::mem::offset_of!(bhkPickData, root_collidable) == 0x80);
const _: () = assert!(core::mem::offset_of!(bhkPickData, pick_failed) == 0xC0);

/// C++ `RE::MagicCaster::State`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MagicCasterState {
    None = 0,
    Unk01 = 1,
    Unk02 = 2,
    Ready = 3,
    Unk04 = 4,
    Charging = 5,
    Casting = 6,
    Unk07 = 7,
    Unk08 = 8,
    Unk09 = 9,
}

core_util::impl_enumset_type!(MagicCasterState => u32);

/// C++ `RE::MagicCaster::PostCreationCallback`
#[repr(C)]
pub struct MagicCasterPostCreationCallback {
    pub base: MagicTargetPostCreationModification, // 00
    pub unk08: [u64; 8],                           // 08
}

const _: () = assert!(core::mem::size_of::<MagicCasterPostCreationCallback>() == 0x48);

inherit!(MagicCasterPostCreationCallback : MagicTargetPostCreationModification);

impl RttiType for MagicCasterPostCreationCallback {
    const RTTI: VariantID = RTTI_MagicCaster__PostCreationCallback;
}

impl MagicCasterPostCreationCallback {
    pub const RTTI: VariantID = RTTI_MagicCaster__PostCreationCallback;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MagicCaster__PostCreationCallback;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_MODIFY_ACTIVE_EFFECT: usize = 0x01;
        pub fn modify_active_effect(effect: *mut ActiveEffect)
    }
}

/// C++ `RE::MagicCaster`
#[repr(C)]
pub struct MagicCaster {
    pub vtable: *const usize,                  // 00
    pub sounds: BSTArray<BSSoundHandle>,       // 08
    pub desired_target: ObjectRefHandle,       // 20
    pub pad24: u32,                            // 24
    pub current_spell: *mut MagicItem,         // 28
    pub state: EnumSet<MagicCasterState, u32>, // 30
    pub casting_timer: f32,                    // 34
    pub current_spell_cost: f32,               // 38
    pub magnitude_override: f32,               // 3C
    pub next_target_update: f32,               // 40
    pub projectile_timer: f32,                 // 44
}

const _: () = assert!(core::mem::size_of::<MagicCaster>() == 0x48);
const _: () = assert!(core::mem::offset_of!(MagicCaster, sounds) == 0x08);
const _: () = assert!(core::mem::offset_of!(MagicCaster, desired_target) == 0x20);
const _: () = assert!(core::mem::offset_of!(MagicCaster, current_spell) == 0x28);
const _: () = assert!(core::mem::offset_of!(MagicCaster, state) == 0x30);

impl RttiType for MagicCaster {
    const RTTI: VariantID = RTTI_MagicCaster;
}

impl MagicCaster {
    pub const RTTI: VariantID = RTTI_MagicCaster;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MagicCaster;

    virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    virtual_method! { pub const VFUNC_CAST_SPELL_IMMEDIATE: usize = 0x01; pub fn cast_spell_immediate(spell: *mut MagicItem, no_hit_effect_art: bool, target: *mut TESObjectREFR, effectiveness: f32, hostile_effectiveness_only: bool, magnitude_override: f32, blame_actor: *mut Actor) }
    virtual_method! { pub const VFUNC_FIND_TOUCH_TARGET: usize = 0x02; pub fn find_touch_target() }
    virtual_method! { pub const VFUNC_REQUEST_CAST_IMPL: usize = 0x03; pub fn request_cast_impl() }
    virtual_method! { pub const VFUNC_START_CHARGE_IMPL: usize = 0x04; pub fn start_charge_impl() -> bool }
    virtual_method! { pub const VFUNC_START_READY_IMPL: usize = 0x05; pub fn start_ready_impl() }
    virtual_method! { pub const VFUNC_START_CAST_IMPL: usize = 0x06; pub fn start_cast_impl() }
    virtual_method! { pub const VFUNC_FINISH_CAST_IMPL: usize = 0x07; pub fn finish_cast_impl() }
    virtual_method! { pub const VFUNC_INTERRUPT_CAST_IMPL: usize = 0x08; pub fn interrupt_cast_impl(deplete_energy: bool) }
    virtual_method! { pub const VFUNC_SPELL_CAST: usize = 0x09; pub fn spell_cast(do_cast: bool, arg2: u32, spell: *mut MagicItem) }
    virtual_method! { pub const VFUNC_CHECK_CAST: usize = 0x0A; pub fn check_cast(spell: *mut MagicItem, dual_cast: bool, effect_strength: *mut f32, reason: *mut CannotCastReason, use_base_value_for_cost: bool) -> bool }
    virtual_method! { pub const VFUNC_GET_CASTER_STATS_OBJECT: usize = 0x0B; pub fn get_caster_stats_object() -> *mut TESObjectREFR }
    virtual_method! { pub const VFUNC_GET_CASTER_AS_ACTOR: usize = 0x0C; pub fn get_caster_as_actor() -> *mut Actor }
    virtual_method! { pub const VFUNC_GET_CASTER_OBJECT_REFERENCE: usize = 0x0D; pub fn get_caster_object_reference(out_caster: *mut *mut Actor) -> *mut TESObjectREFR }
    virtual_method! { pub const VFUNC_GET_MAGIC_NODE: usize = 0x0E; pub fn get_magic_node() -> *mut NiNode }
    virtual_method! { pub const VFUNC_CLEAR_MAGIC_NODE: usize = 0x0F; pub fn clear_magic_node() }
    virtual_method! { pub const VFUNC_SET_CURRENT_SPELL_IMPL: usize = 0x10; pub fn set_current_spell_impl(spell: *mut MagicItem) }
    virtual_method! { pub const VFUNC_SELECT_SPELL_IMPL: usize = 0x11; pub fn select_spell_impl() }
    virtual_method! { pub const VFUNC_DESELECT_SPELL_IMPL: usize = 0x12; pub fn deselect_spell_impl() }
    virtual_method! { pub const VFUNC_SET_SKIP_CHECK_CAST: usize = 0x13; pub fn set_skip_check_cast() }
    virtual_method! { pub const VFUNC_SET_CASTING_TIMER_FOR_CHARGE: usize = 0x14; pub fn set_casting_timer_for_charge() }
    virtual_method! { pub const VFUNC_GET_CASTING_SOURCE: usize = 0x15; pub fn get_casting_source() -> CastingSource }
    virtual_method! { pub const VFUNC_GET_IS_DUAL_CASTING: usize = 0x16; pub fn get_is_dual_casting() -> bool }
    virtual_method! { pub const VFUNC_SET_DUAL_CASTING: usize = 0x17; pub fn set_dual_casting(set: bool) }
    virtual_method! { pub const VFUNC_SAVE_GAME: usize = 0x18; pub fn save_game(buf: *mut BGSSaveGameBuffer) }
    virtual_method! { pub const VFUNC_LOAD_GAME: usize = 0x19; pub fn load_game(buf: *mut BGSLoadGameBuffer) }
    virtual_method! { pub const VFUNC_FINISH_LOAD_GAME: usize = 0x1A; pub fn finish_load_game(buf: *mut BGSLoadGameBuffer) }
    virtual_method! { pub const VFUNC_PREPARE_SOUND: usize = 0x1B; pub fn prepare_sound(sound: SoundID, spell: *mut MagicItem) }
    virtual_method! { pub const VFUNC_ADJUST_ACTIVE_EFFECT: usize = 0x1C; pub fn adjust_active_effect(active_effect: *mut ActiveEffect, power: f32, arg3: bool) }

    crate::relocation_func! {
        pub fn find_pick_target(&mut self, target_location: &mut NiPoint3, target_cell: *mut *mut TESObjectCELL, pick_data: &mut bhkPickData) -> *mut MagicTarget => RelocationID::new(33676, 34456)
    }

    crate::relocation_func! {
        pub fn find_targets(&mut self, effectiveness_mult: f32, target_count: &mut u32, source: *mut TESBoundObject, load_cast: bool, adjust_only_hostile_effectiveness: bool) -> bool => RelocationID::new(33632, 34410)
    }

    crate::relocation_func! {
        pub fn finish_cast(&mut self) => RelocationID::new(33657, 34435)
    }

    crate::relocation_func! {
        pub fn get_current_spell_cost(&mut self) -> f32 => RelocationID::new(33426, 34204)
    }

    crate::relocation_func! {
        pub fn interrupt_cast(&mut self, refund: bool) => RelocationID::new(33630, 34408)
    }

    crate::relocation_func! {
        pub fn play_release_sound(&mut self, item: *mut MagicItem) => RelocationID::new(33675, 34448)
    }

    crate::relocation_func! {
        pub fn set_current_spell(&mut self, item: *mut MagicItem) => RelocationID::new(33644, 34422)
    }

    #[inline]
    pub fn test_projectile_placement(effect: &Effect, pick_data: &bhkPickData) -> bool {
        let Some(base_effect) = (unsafe { effect.base_effect.as_ref() }) else {
            return true;
        };

        if base_effect.data.delivery == crate::re::magic_system::Delivery::TargetLocation
            && !base_effect.data.projectile_base.is_null()
        {
            if pick_data.pick_failed || pick_data.root_collidable.is_null() {
                return false;
            }

            let col_layer =
                unsafe { (*pick_data.root_collidable).collision_filter_info }.get_collision_layer();

            return matches!(
                col_layer,
                ColLayer::Static | ColLayer::Terrain | ColLayer::Ground
            );
        }

        true
    }

    crate::relocation_func! {
        pub fn update_impl(&mut self, delta: f32) => RelocationID::new(33622, 34400)
    }
}
