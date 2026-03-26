#![allow(non_camel_case_types)]

use core::ffi::c_void;
use core::ptr;

use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::{RTTI_Projectile, RTTI_Projectile__LaunchData};
use crate::offsets::offsets_vtable::{VTABLE_Projectile, VTABLE_Projectile__LaunchData};
use crate::re::bst_singleton::BSTSingletonSDM;
use crate::re::magic_system::{CastingSource, Delivery};
use crate::re::{
    Actor, ActorCause, AlchemyItem, BGSExplosion, BGSKeyword, BGSLoadFormBuffer, BGSMaterialType,
    BGSProjectile, BGSSaveFormBuffer, BSLight, BSSimpleList, BSSoundHandle, BSSpinLock, BSTArray,
    ColLayer, CombatController, EffectSetting, EnchantmentItem, ImpactResult, MagicItem,
    ModelDBHandle, NiAVObject, NiMatrix3, NiPoint3, NiPointer, NiTransform, ObjectRefHandle,
    ProjectileHandle, QueuedFile, SpellItem, TESAmmo, TESBoundObject, TESFile, TESObjectCELL,
    TESObjectREFR, TESObjectWEAP, bhkCollisionObject, bhkShape, bhkSimpleShapePhantom,
    hkpCollidable,
};
use crate::relocation::{RelocationID, RttiType, VariantID, VariantOffset, skyrim_cast};

crate::relocation_func! {
    fn combat_utilities_get_angle_to_projected_target(
        angles: &mut NiPoint3,
        cur_pos: &NiPoint3,
        target: *mut TESObjectREFR,
        speed: f32,
        gravity: f32,
        los_loc: u32,
    ) -> *mut NiPoint3 => RelocationID::new(46022, 0)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProjectileWobbleControl {
    pub unk00: NiMatrix3,
    pub handle: ProjectileHandle,
    pub wobble: f32,
}

const _: () = assert!(core::mem::size_of::<ProjectileWobbleControl>() == 0x2C);
const _: () = assert!(core::mem::offset_of!(ProjectileWobbleControl, unk00) == 0x00);
const _: () = assert!(core::mem::offset_of!(ProjectileWobbleControl, handle) == 0x24);
const _: () = assert!(core::mem::offset_of!(ProjectileWobbleControl, wobble) == 0x28);

#[repr(C)]
pub struct ProjectileManager {
    pub base: BSTSingletonSDM<ProjectileManager>,
    pub pad01: [u8; 7],
    pub unlimited: BSTArray<ProjectileHandle>,
    pub limited: BSTArray<ProjectileHandle>,
    pub pending: BSTArray<ProjectileHandle>,
    pub projectile_lock: BSSpinLock,
    pub wobble: BSTArray<ProjectileWobbleControl>,
}

const _: () = assert!(core::mem::size_of::<ProjectileManager>() == 0x70);
const _: () = assert!(core::mem::offset_of!(ProjectileManager, unlimited) == 0x08);
const _: () = assert!(core::mem::offset_of!(ProjectileManager, limited) == 0x20);
const _: () = assert!(core::mem::offset_of!(ProjectileManager, pending) == 0x38);
const _: () = assert!(core::mem::offset_of!(ProjectileManager, projectile_lock) == 0x50);
const _: () = assert!(core::mem::offset_of!(ProjectileManager, wobble) == 0x58);

impl ProjectileManager {
    crate::relocation_variable! {
        fn singleton() -> *mut ProjectileManager => RelocationID::new(514313, 400473), is_ptr
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut ProjectileManager {
        Self::singleton()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ProjectileRot {
    pub x: f32,
    pub z: f32,
}

const _: () = assert!(core::mem::size_of::<ProjectileRot>() == 0x08);

#[repr(C)]
pub struct ProjectileLaunchData {
    pub vtable: *const usize,
    pub origin: NiPoint3,
    pub contact_normal: NiPoint3,
    pub projectile_base: *mut BGSProjectile,
    pub shooter: *mut TESObjectREFR,
    pub combat_controller: *mut CombatController,
    pub weapon_source: *mut TESObjectWEAP,
    pub ammo_source: *mut TESAmmo,
    pub angle_z: f32,
    pub angle_x: f32,
    pub unk50: *mut c_void,
    pub desired_target: *mut TESObjectREFR,
    pub unk60: f32,
    pub unk64: f32,
    pub parent_cell: *mut TESObjectCELL,
    pub spell: *mut MagicItem,
    pub casting_source: CastingSource,
    pub pad7c: u32,
    pub enchant_item: *mut EnchantmentItem,
    pub poison: *mut AlchemyItem,
    pub area: i32,
    pub power: f32,
    pub scale: f32,
    pub always_hit: bool,
    pub no_damage_outside_combat: bool,
    pub auto_aim: bool,
    pub chain_shatter: bool,
    pub use_origin: bool,
    pub defer_initialization: bool,
    pub force_cone_of_fire: bool,
    pub pada3: [u8; 5],
}

const _: () = assert!(core::mem::size_of::<ProjectileLaunchData>() == 0xA8);
const _: () = assert!(core::mem::offset_of!(ProjectileLaunchData, vtable) == 0x00);
const _: () = assert!(core::mem::offset_of!(ProjectileLaunchData, origin) == 0x08);
const _: () = assert!(core::mem::offset_of!(ProjectileLaunchData, projectile_base) == 0x20);
const _: () = assert!(core::mem::offset_of!(ProjectileLaunchData, shooter) == 0x28);
const _: () = assert!(core::mem::offset_of!(ProjectileLaunchData, combat_controller) == 0x30);
const _: () = assert!(core::mem::offset_of!(ProjectileLaunchData, spell) == 0x70);
const _: () = assert!(core::mem::offset_of!(ProjectileLaunchData, casting_source) == 0x78);
const _: () = assert!(core::mem::offset_of!(ProjectileLaunchData, force_cone_of_fire) == 0xA2);

impl RttiType for ProjectileLaunchData {
    const RTTI: VariantID = RTTI_Projectile__LaunchData;
}

impl ProjectileLaunchData {
    pub const RTTI: VariantID = RTTI_Projectile__LaunchData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_Projectile__LaunchData;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    #[inline(always)]
    pub fn new(
        projectile_base: *mut BGSProjectile,
        shooter: *mut Actor,
        origin: &NiPoint3,
        angles: &ProjectileRot,
    ) -> Self {
        let combat_controller = if shooter.is_null() {
            ptr::null_mut()
        } else {
            unsafe { (*shooter).get_actor_runtime_data().combat_controller }
        };
        let parent_cell = if shooter.is_null() {
            ptr::null_mut()
        } else {
            unsafe { (*shooter).base.get_parent_cell() }
        };

        Self {
            vtable: Self::VTABLE[0].address() as *const usize,
            origin: *origin,
            contact_normal: NiPoint3::default(),
            projectile_base,
            shooter: shooter.cast(),
            combat_controller,
            weapon_source: ptr::null_mut(),
            ammo_source: ptr::null_mut(),
            angle_z: angles.z,
            angle_x: angles.x,
            unk50: ptr::null_mut(),
            desired_target: ptr::null_mut(),
            unk60: 0.0,
            unk64: 0.0,
            parent_cell,
            spell: ptr::null_mut(),
            casting_source: CastingSource::Other,
            pad7c: 0,
            enchant_item: ptr::null_mut(),
            poison: ptr::null_mut(),
            area: 0,
            power: 1.0,
            scale: 1.0,
            always_hit: false,
            no_damage_outside_combat: false,
            auto_aim: true,
            chain_shatter: false,
            use_origin: false,
            defer_initialization: false,
            force_cone_of_fire: false,
            pada3: [0; 5],
        }
    }

    #[inline(always)]
    pub fn from_spell(
        shooter: *mut Actor,
        origin: &NiPoint3,
        angles: &ProjectileRot,
        spell: *mut MagicItem,
    ) -> Self {
        let av_effect = unsafe { spell.as_ref() }.map(|spell| spell.get_av_effect());
        let projectile_base = av_effect
            .and_then(|effect| unsafe { effect.as_ref() })
            .map(|effect| effect.data.projectile_base)
            .unwrap_or(ptr::null_mut());
        let costliest_effect = unsafe { spell.as_ref() }
            .map(|spell| spell.get_costliest_effect_item(Delivery::None, false))
            .unwrap_or(ptr::null_mut());

        let mut data = Self::new(projectile_base, shooter, origin, angles);
        data.spell = spell;
        data.area = unsafe { costliest_effect.as_ref() }
            .map(|effect| effect.get_area() as i32)
            .unwrap_or(0);
        data.use_origin = true;
        data.auto_aim = false;
        data
    }

    #[inline(always)]
    pub fn from_ammo(
        shooter: *mut Actor,
        origin: &NiPoint3,
        angles: &ProjectileRot,
        ammo: *mut TESAmmo,
        weap: *mut TESObjectWEAP,
    ) -> Self {
        let projectile_base = unsafe { ammo.as_ref() }
            .map(|ammo| ammo.get_runtime_data().data.projectile)
            .unwrap_or(ptr::null_mut());

        let mut data = Self::new(projectile_base, shooter, origin, angles);
        data.weapon_source = weap;
        data.ammo_source = ammo;
        data.use_origin = true;
        data.auto_aim = false;
        data
    }
}

#[repr(C)]
pub struct ProjectileImpactData {
    pub desired_target_loc: NiPoint3,
    pub negative_velocity: NiPoint3,
    pub collidee: ObjectRefHandle,
    pub col_obj: NiPointer<bhkCollisionObject>,
    pub material: *mut BGSMaterialType,
    pub damage_root_node_type: i32,
    pub collided_layer: EnumSet<ColLayer, i32>,
    pub damage_root_node: *mut crate::re::NiNode,
    pub impact_result: ImpactResult,
    pub unk44: u16,
    pub unk46: u16,
    pub unk48: u8,
    pub unk49: u8,
    pub pad4a: [u8; 6],
}

const _: () = assert!(core::mem::size_of::<ProjectileImpactData>() == 0x50);
const _: () = assert!(core::mem::offset_of!(ProjectileImpactData, collidee) == 0x18);
const _: () = assert!(core::mem::offset_of!(ProjectileImpactData, col_obj) == 0x20);
const _: () = assert!(core::mem::offset_of!(ProjectileImpactData, collided_layer) == 0x34);
const _: () = assert!(core::mem::offset_of!(ProjectileImpactData, impact_result) == 0x40);

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectileFlags {
    None = 0,
    Unk0 = 1 << 0,
    NotAddThreat = 1 << 1,
    Unk2 = 1 << 2,
    Unk3 = 1 << 3,
    IsTracer = 1 << 4,
    Fading = 1 << 5,
    GravityUpdateModel = 1 << 6,
    Unk7 = 1 << 7,
    Inited = 1 << 8,
    ChainShatter = 1 << 9,
    Unk10 = 1 << 10,
    Unk11 = 1 << 11,
    AlwaysHit = 1 << 12,
    HitScan = 1 << 13,
    Unk14 = 1 << 14,
    DestroyAfterHit = 1 << 15,
    AddedToManager = 1 << 16,
    NoDamageOutsideCombat = 1 << 17,
    CanStartTrails = 1 << 18,
    AggressiveActor = 1 << 19,
    AddedVisualEffectOnGround = 1 << 20,
    AutoAim = 1 << 21,
    ProcessedImpacts = 1 << 22,
    Unk23 = 1 << 23,
    Unk24 = 1 << 24,
    Destroyed = 1 << 25,
    Unk26 = 1 << 26,
    Unk27 = 1 << 27,
    IsDual = 1 << 28,
    UseOrigin = 1 << 29,
    Unk30 = 1 << 30,
    Moved = 1u32 << 31,
}

core_util::impl_enumset_type!(ProjectileFlags => u32);

#[repr(C)]
pub struct ProjectileRuntimeData {
    pub impacts: BSSimpleList<*mut ProjectileImpactData>,
    pub unk0a8: NiTransform,
    pub unk0dc: f32,
    // TODO: CommonLib marks `unk0E0` as a smart pointer to `bhkSimpleShapePhantom`, but the
    // vendored source still only forward-declares `bhkSimpleShapePhantom`. Keep the raw pointer
    // until its ownership/refcount contract is translated instead of inventing a fake smart layer.
    pub unk0e0: *mut bhkSimpleShapePhantom,
    pub unk0e8: BSSpinLock,
    pub velocity: NiPoint3,
    pub linear_velocity: NiPoint3,
    pub light: NiPointer<BSLight>,
    // TODO: CommonLib documents `unk110` as a smart pointer, but its pointee type is still
    // erased to `void*` in the vendored header/source. Keep the raw pointer until the real
    // pointee and ownership layer are source-backed.
    pub unk110: *mut c_void,
    pub actor_cause: NiPointer<ActorCause>,
    pub shooter: ObjectRefHandle,
    pub desired_target: ObjectRefHandle,
    pub snd_handle: BSSoundHandle,
    pub snd_countdown: BSSoundHandle,
    pub unk140: *mut u32,
    pub unk148: *mut crate::re::InventoryEntryData,
    pub explosion: *mut BGSExplosion,
    pub spell: *mut MagicItem,
    pub casting_source: CastingSource,
    pub pad164: u32,
    pub av_effect: *mut EffectSetting,
    // TODO: CommonLib uses `NiPointer<QueuedFile>` here, but `QueuedFile` is still only
    // forward-declared in the vendored tree. Replace this raw pointer once `QueuedFile` gets a
    // source-backed `NiRef`-compatible translation instead of faking `NiPointer<QueuedFile>`.
    pub projectile_db_files: *mut QueuedFile,
    pub muzzle_flash_db_handle: ModelDBHandle,
    pub unk180: u64,
    pub power: f32,
    pub speed_mult: f32,
    pub range: f32,
    pub living_time: f32,
    pub weapon_damage: f32,
    pub transparency: f32,
    pub explosion_timer: f32,
    pub unk1a4: u32,
    pub unk1a8: f32,
    pub unk1ac: f32,
    pub weapon_source: *mut TESObjectWEAP,
    pub ammo_source: *mut TESAmmo,
    pub distance_moved: f32,
    pub unk1c4: u32,
    pub scale: f32,
    pub flags: EnumSet<ProjectileFlags, u32>,
    pub unk1d0: bool,
    pub unk1d1: bool,
    pub unk1d2: [u8; 6],
}

const _: () = assert!(core::mem::size_of::<ProjectileRuntimeData>() == 0x140);
const _: () = assert!(core::mem::offset_of!(ProjectileRuntimeData, impacts) == 0x00);
const _: () = assert!(core::mem::offset_of!(ProjectileRuntimeData, velocity) == 0x58);
const _: () = assert!(core::mem::offset_of!(ProjectileRuntimeData, light) == 0x70);
const _: () = assert!(core::mem::offset_of!(ProjectileRuntimeData, actor_cause) == 0x80);
const _: () = assert!(core::mem::offset_of!(ProjectileRuntimeData, spell) == 0xC0);
const _: () = assert!(core::mem::offset_of!(ProjectileRuntimeData, projectile_db_files) == 0xD8);
const _: () = assert!(core::mem::offset_of!(ProjectileRuntimeData, power) == 0xF0);
const _: () = assert!(core::mem::offset_of!(ProjectileRuntimeData, weapon_source) == 0x118);
const _: () = assert!(core::mem::offset_of!(ProjectileRuntimeData, flags) == 0x134);

#[repr(C)]
pub struct Projectile {
    pub base: TESObjectREFR,
}

const _: () = assert!(core::mem::size_of::<Projectile>() == 0x80);
const _: () = assert!(core::mem::offset_of!(Projectile, base) == 0x00);

inherit!(Projectile : TESObjectREFR, base);

impl RttiType for Projectile {
    const RTTI: VariantID = RTTI_Projectile;
}

impl AsRef<Projectile> for Projectile {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Projectile> for Projectile {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Projectile {
    pub const RTTI: VariantID = RTTI_Projectile;
    pub const VTABLE: &'static [VariantID] = &VTABLE_Projectile;
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x98, 0xA0, 0x98);
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0x1D8, 0x1E0, 0x1D8);

    crate::runtime_data_accessor! {
        pub fn get_projectile_runtime_data() -> ProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn get_projectile_runtime_data_mut() -> ProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    // override (TESObjectREFR)
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD: usize = 0x06;
        pub fn load(mod_: *mut TESFile) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SAVE_GAME: usize = 0x0E;
        pub fn save_game(buf: *mut BGSSaveFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD_GAME: usize = 0x0F;
        pub fn load_game(buf: *mut BGSLoadFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_INIT_LOAD_GAME: usize = 0x10;
        pub fn init_load_game(buf: *mut BGSLoadFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_FINISH_LOAD_GAME: usize = 0x11;
        pub fn finish_load_game(buf: *mut BGSLoadFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_REVERT: usize = 0x12;
        pub fn revert(buf: *mut BGSLoadFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ALLOW_PROMOTE_TO_PERSISTENT: usize = 0x47;
        pub fn get_allow_promote_to_persistent(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_HAS_KEYWORD_HELPER: usize = 0x48;
        pub fn has_keyword_helper(&self, keyword: *const BGSKeyword) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SET_ACTOR_CAUSE: usize = 0x50;
        pub fn set_actor_cause(cause: *mut ActorCause)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ACTOR_CAUSE: usize = 0x51;
        pub fn get_actor_cause(&self) -> *mut ActorCause
    }

    crate::virtual_method! {
        pub const VFUNC_GET_MAGIC_CASTER: usize = 0x5C;
        pub fn get_magic_caster(source: CastingSource) -> *mut crate::re::MagicCaster
    }

    crate::virtual_method! {
        pub const VFUNC_DETACH_HAVOK: usize = 0x65;
        pub fn detach_havok(obj_3d: *mut NiAVObject) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_INIT_HAVOK: usize = 0x66;
        pub fn init_havok()
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD_3D: usize = 0x6A;
        pub fn load_3d(background_loading: bool) -> *mut NiAVObject
    }

    crate::virtual_method! {
        pub const VFUNC_SET_3D: usize = 0x6C;
        pub fn set_3d(object: *mut NiAVObject, queue_3d_tasks: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_IS_MISSILE_PROJECTILE: VariantOffset = VariantOffset::new(0xA2, 0xA2, 0xA3);
        pub fn is_missile_projectile(&mut self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_IS_GRENADE_PROJECTILE: VariantOffset = VariantOffset::new(0xA3, 0xA3, 0xA4);
        pub fn is_grenade_projectile(&mut self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_IS_FLAME_PROJECTILE: VariantOffset = VariantOffset::new(0xA4, 0xA4, 0xA5);
        pub fn is_flame_projectile(&mut self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_IS_BEAM_PROJECTILE: VariantOffset = VariantOffset::new(0xA5, 0xA5, 0xA6);
        pub fn is_beam_projectile(&mut self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_A6: VariantOffset = VariantOffset::new(0xA6, 0xA6, 0xA7);
        pub fn unk_a6(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_IS_BARRIER_PROJECTILE: VariantOffset = VariantOffset::new(0xA7, 0xA7, 0xA8);
        pub fn is_barrier_projectile(&mut self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_ON_KILL: VariantOffset = VariantOffset::new(0xA8, 0xA8, 0xA9);
        pub fn on_kill(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_PROCESS_3D: VariantOffset = VariantOffset::new(0xA9, 0xA9, 0xAA);
        pub fn process_3d(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_POST_LOAD_3D: VariantOffset = VariantOffset::new(0xAA, 0xAA, 0xAB);
        pub fn post_load_3d(&mut self, root: *mut NiAVObject)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE_IMPL: VariantOffset = VariantOffset::new(0xAB, 0xAB, 0xAC);
        pub fn update_impl(&mut self, delta: f32)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_PROCESS_IMPACTS: VariantOffset = VariantOffset::new(0xAC, 0xAC, 0xAD);
        pub fn process_impacts(&mut self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE_3D: VariantOffset = VariantOffset::new(0xAD, 0xAD, 0xAE);
        pub fn update_3d(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_AE: VariantOffset = VariantOffset::new(0xAE, 0xAE, 0xAF);
        pub fn unk_ae(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_POWER_SPEED_MULT: VariantOffset = VariantOffset::new(0xAF, 0xAF, 0xB0);
        pub fn get_power_speed_mult(&self) -> f32
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_WEAPON_SPEED_MULT: VariantOffset = VariantOffset::new(0xB0, 0xB0, 0xB1);
        pub fn get_weapon_speed_mult(&self) -> f32
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_STOP_MAIN_SOUND_AFTER_IMPACT: VariantOffset =
            VariantOffset::new(0xB1, 0xB1, 0xB2);
        pub fn get_stop_main_sound_after_impact(&mut self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_REPORT_HAVOK_DEACTIVATION: VariantOffset =
            VariantOffset::new(0xB2, 0xB2, 0xB3);
        pub fn report_havok_deactivation(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_TURN_OFF: VariantOffset = VariantOffset::new(0xB3, 0xB3, 0xB4);
        pub fn turn_off(&mut self, owner: *mut Actor, no_deactivate_sound: bool) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_IS_PERMANENT: VariantOffset = VariantOffset::new(0xB4, 0xB4, 0xB5);
        pub fn is_permanent(&self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_GRAVITY: VariantOffset = VariantOffset::new(0xB5, 0xB5, 0xB6);
        pub fn get_gravity(&mut self) -> f32
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_CLEAN_UP_POINTERS_ON_DISABLE: VariantOffset =
            VariantOffset::new(0xB6, 0xB6, 0xB7);
        pub fn clean_up_pointers_on_disable(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_RUN_TARGET_PICK: VariantOffset = VariantOffset::new(0xB7, 0xB7, 0xB8);
        pub fn run_target_pick(&mut self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_KILL_ON_COLLISION: VariantOffset = VariantOffset::new(0xB8, 0xB8, 0xB9);
        pub fn get_kill_on_collision(&mut self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SHOULD_BE_LIMITED: VariantOffset = VariantOffset::new(0xB9, 0xB9, 0xBA);
        pub fn should_be_limited(&mut self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_TARGETS_WHOLE_BODY: VariantOffset = VariantOffset::new(0xBA, 0xBA, 0xBB);
        pub fn targets_whole_body(&mut self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_COLLISION_GROUP: VariantOffset = VariantOffset::new(0xBB, 0xBB, 0xBC);
        pub fn get_collision_group(&mut self) -> u32
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_COLLISION_SHAPE: VariantOffset = VariantOffset::new(0xBC, 0xBC, 0xBD);
        pub fn get_collision_shape(&mut self) -> *mut bhkShape
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_ADD_IMPACT: VariantOffset = VariantOffset::new(0xBD, 0xBD, 0xBE);
        pub fn add_impact(
            &mut self,
            refr: *mut TESObjectREFR,
            target_loc: &NiPoint3,
            velocity: &NiPoint3,
            collidable: *mut hkpCollidable,
            arg6: i32,
            arg7: u32
        )
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_HANDLE_HITS: VariantOffset = VariantOffset::new(0xBE, 0xBE, 0xBF);
        pub fn handle_hits(&mut self, collidable: *mut hkpCollidable) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_ON_TRIGGER_ENTER: VariantOffset = VariantOffset::new(0xBF, 0xBF, 0xC0);
        pub fn on_trigger_enter(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_HANDLE_3D_LOADED: VariantOffset = VariantOffset::new(0xC0, 0xC0, 0xC1);
        pub fn handle_3d_loaded(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SHOULD_USE_DESIRED_TARGET: VariantOffset = VariantOffset::new(0xC1, 0xC1, 0xC2);
        pub fn should_use_desired_target(&mut self) -> bool
    }

    crate::relocation_func! {
        pub fn launch(
            result: &mut ProjectileHandle,
            data: &mut ProjectileLaunchData
        ) -> *mut ProjectileHandle => RelocationID::new(42928, 44108)
    }

    crate::relocation_func! {
        pub fn kill(&mut self) => RelocationID::new(42930, 44110)
    }

    #[inline(always)]
    pub fn get_projectile_base(&self) -> *mut BGSProjectile {
        let object = self.base.get_object_reference();
        if object.is_null() {
            ptr::null_mut()
        } else {
            unsafe { skyrim_cast::<TESBoundObject, BGSProjectile>(object) }
        }
    }

    #[inline(always)]
    pub fn get_height(&self) -> f32 {
        let projectile = self.get_projectile_base();
        if projectile.is_null() {
            0.0
        } else {
            unsafe { (*projectile).data.collision_radius * 2.0 }
        }
    }

    #[inline(always)]
    pub fn get_speed(&self) -> f32 {
        let projectile = self.get_projectile_base();
        if projectile.is_null() {
            0.0
        } else {
            unsafe {
                (*projectile).data.speed
                    * self.get_power_speed_mult()
                    * self.get_weapon_speed_mult()
                    * self.get_projectile_runtime_data().speed_mult
            }
        }
    }

    #[inline(always)]
    pub fn launch_spell(
        result: &mut ProjectileHandle,
        shooter: *mut Actor,
        spell: *mut SpellItem,
        origin: &NiPoint3,
        angles: &ProjectileRot,
    ) -> *mut ProjectileHandle {
        let mut launch_data =
            ProjectileLaunchData::from_spell(shooter, origin, angles, spell.cast::<MagicItem>());
        Self::launch(result, &mut launch_data)
    }

    #[inline(always)]
    pub fn launch_spell_from_source(
        result: &mut ProjectileHandle,
        shooter: *mut Actor,
        spell: *mut SpellItem,
        source: CastingSource,
    ) -> *mut ProjectileHandle {
        if shooter.is_null() || spell.is_null() {
            return result;
        }

        let caster = unsafe { (*shooter).base.get_magic_caster(source) };
        if caster.is_null() {
            return result;
        }

        let mut origin = unsafe { (*shooter).base.get_position() };
        let magic_node = unsafe { (*caster).get_magic_node() };
        if !magic_node.is_null() {
            origin = unsafe { (&*magic_node).world.translate };
        } else {
            let bound_max = unsafe { (*shooter).base.get_bound_max() };
            let bound_min = unsafe { (*shooter).base.get_bound_min() };
            origin.z += (bound_max.z - bound_min.z) * 0.7;
        }

        let angles =
            unsafe { skyrim_cast::<TESObjectREFR, Actor>(shooter.cast::<TESObjectREFR>()) };
        let angles = if !angles.is_null() {
            ProjectileRot {
                x: unsafe { (*angles).get_aim_angle() },
                z: unsafe { (*angles).get_aim_heading() },
            }
        } else {
            let desired_target = unsafe { (*caster).desired_target.get().get() };
            if !desired_target.is_null() {
                let projectile_base = unsafe {
                    spell
                        .cast::<MagicItem>()
                        .as_ref()
                        .and_then(|spell| spell.get_av_effect().as_ref())
                        .map(|effect| effect.data.projectile_base)
                        .unwrap_or(ptr::null_mut())
                };
                if !projectile_base.is_null() {
                    let mut vec_angles = NiPoint3::default();
                    combat_utilities_get_angle_to_projected_target(
                        &mut vec_angles,
                        &origin,
                        desired_target,
                        unsafe { (*projectile_base).data.speed },
                        0.0,
                        2,
                    );
                    ProjectileRot {
                        x: vec_angles.x,
                        z: vec_angles.z,
                    }
                } else {
                    ProjectileRot::default()
                }
            } else {
                ProjectileRot::default()
            }
        };

        Self::launch_spell(result, shooter, spell, &origin, &angles)
    }

    #[inline(always)]
    pub fn launch_arrow(
        result: &mut ProjectileHandle,
        shooter: *mut Actor,
        ammo: *mut TESAmmo,
        weap: *mut TESObjectWEAP,
        origin: &NiPoint3,
        angles: &ProjectileRot,
    ) -> *mut ProjectileHandle {
        let mut launch_data = ProjectileLaunchData::from_ammo(shooter, origin, angles, ammo, weap);
        Self::launch(result, &mut launch_data)
    }

    #[inline(always)]
    pub fn launch_arrow_auto(
        result: &mut ProjectileHandle,
        shooter: *mut Actor,
        ammo: *mut TESAmmo,
        weap: *mut TESObjectWEAP,
    ) -> *mut ProjectileHandle {
        if shooter.is_null() || weap.is_null() {
            return result;
        }

        let fire_node = unsafe {
            let current_process = (*shooter).get_actor_runtime_data().current_process;
            if let Some(current_process) = current_process.as_ref() {
                let biped = (*shooter).base.get_biped().get();
                if (*weap).is_crossbow() {
                    current_process.get_magic_node(biped)
                } else {
                    current_process.get_weapon_node(biped)
                }
            } else {
                let root = if (*shooter).base.base.is_player_ref() {
                    (*shooter).base.get_current_3d()
                } else {
                    (*shooter).base.get_3d2()
                };
                (*weap).get_fire_node_ptr(root)
            }
        };

        let mut origin;
        let mut angles = ProjectileRot::default();
        if !fire_node.is_null() {
            origin = unsafe { (&*fire_node).world.translate };
            unsafe {
                (*shooter)
                    .base
                    .unk_a0(fire_node, &mut angles.x, &mut angles.z, &mut origin);
            }
        } else {
            let shooter_char =
                unsafe { skyrim_cast::<TESObjectREFR, Actor>(shooter.cast::<TESObjectREFR>()) };
            if !shooter_char.is_null() {
                origin = unsafe { (*shooter).base.get_position() };
                origin.z += 96.0;
                angles.x = unsafe { (*shooter_char).get_aim_angle() };
                angles.z = unsafe { (*shooter_char).get_aim_heading() };
            } else {
                origin = unsafe { (*shooter).base.get_position() };
                angles.x = unsafe { (*shooter).base.get_angle_x() };
                angles.z = unsafe { (*shooter).base.get_angle_z() };
            }
        }

        Self::launch_arrow(result, shooter, ammo, weap, &origin, &angles)
    }
}

pub trait ProjectileExt {
    fn get_projectile_runtime_data(&self) -> &ProjectileRuntimeData;
    fn get_projectile_runtime_data_mut(&mut self) -> &mut ProjectileRuntimeData;
    fn dtor(&mut self);
    fn load(&mut self, mod_: *mut TESFile) -> bool;
    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer);
    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer);
    fn init_load_game(&mut self, buf: *mut BGSLoadFormBuffer);
    fn finish_load_game(&mut self, buf: *mut BGSLoadFormBuffer);
    fn revert(&mut self, buf: *mut BGSLoadFormBuffer);
    fn get_allow_promote_to_persistent(&self) -> bool;
    fn has_keyword_helper(&self, keyword: *const BGSKeyword) -> bool;
    fn set_actor_cause(&mut self, cause: *mut ActorCause);
    fn get_actor_cause(&self) -> *mut ActorCause;
    fn get_magic_caster(&mut self, source: CastingSource) -> *mut crate::re::MagicCaster;
    fn detach_havok(&mut self, obj_3d: *mut NiAVObject) -> bool;
    fn init_havok(&mut self);
    fn load_3d(&mut self, background_loading: bool) -> *mut NiAVObject;
    fn set_3d(&mut self, object: *mut NiAVObject, queue_3d_tasks: bool);
    fn is_missile_projectile(&mut self) -> bool;
    fn is_grenade_projectile(&mut self) -> bool;
    fn is_flame_projectile(&mut self) -> bool;
    fn is_beam_projectile(&mut self) -> bool;
    fn unk_a6(&mut self);
    fn is_barrier_projectile(&mut self) -> bool;
    fn on_kill(&mut self);
    fn process_3d(&mut self);
    fn post_load_3d(&mut self, root: *mut NiAVObject);
    fn update_impl(&mut self, delta: f32);
    fn process_impacts(&mut self) -> bool;
    fn update_3d(&mut self);
    fn unk_ae(&mut self);
    fn get_power_speed_mult(&self) -> f32;
    fn get_weapon_speed_mult(&self) -> f32;
    fn get_stop_main_sound_after_impact(&mut self) -> bool;
    fn report_havok_deactivation(&mut self);
    fn turn_off(&mut self, owner: *mut Actor, no_deactivate_sound: bool) -> bool;
    fn is_permanent(&self) -> bool;
    fn get_gravity(&mut self) -> f32;
    fn clean_up_pointers_on_disable(&mut self);
    fn run_target_pick(&mut self) -> bool;
    fn get_kill_on_collision(&mut self) -> bool;
    fn should_be_limited(&mut self) -> bool;
    fn targets_whole_body(&mut self) -> bool;
    fn get_collision_group(&mut self) -> u32;
    fn get_collision_shape(&mut self) -> *mut bhkShape;
    fn add_impact(
        &mut self,
        refr: *mut TESObjectREFR,
        target_loc: &NiPoint3,
        velocity: &NiPoint3,
        collidable: *mut hkpCollidable,
        arg6: i32,
        arg7: u32,
    );
    fn handle_hits(&mut self, collidable: *mut hkpCollidable) -> bool;
    fn on_trigger_enter(&mut self);
    fn handle_3d_loaded(&mut self);
    fn should_use_desired_target(&mut self) -> bool;
    fn get_projectile_base(&self) -> *mut BGSProjectile;
    fn get_height(&self) -> f32;
    fn get_speed(&self) -> f32;
    fn kill(&mut self);
}

impl<T: AsRef<Projectile> + AsMut<Projectile>> ProjectileExt for T {
    #[inline(always)]
    fn get_projectile_runtime_data(&self) -> &ProjectileRuntimeData {
        Projectile::get_projectile_runtime_data(self.as_ref())
    }

    #[inline(always)]
    fn get_projectile_runtime_data_mut(&mut self) -> &mut ProjectileRuntimeData {
        Projectile::get_projectile_runtime_data_mut(self.as_mut())
    }

    #[inline(always)]
    fn dtor(&mut self) {
        Projectile::dtor(self.as_mut())
    }

    #[inline(always)]
    fn load(&mut self, mod_: *mut TESFile) -> bool {
        Projectile::load(self.as_mut(), mod_)
    }

    #[inline(always)]
    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer) {
        Projectile::save_game(self.as_mut(), buf)
    }

    #[inline(always)]
    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer) {
        Projectile::load_game(self.as_mut(), buf)
    }

    #[inline(always)]
    fn init_load_game(&mut self, buf: *mut BGSLoadFormBuffer) {
        Projectile::init_load_game(self.as_mut(), buf)
    }

    #[inline(always)]
    fn finish_load_game(&mut self, buf: *mut BGSLoadFormBuffer) {
        Projectile::finish_load_game(self.as_mut(), buf)
    }

    #[inline(always)]
    fn revert(&mut self, buf: *mut BGSLoadFormBuffer) {
        Projectile::revert(self.as_mut(), buf)
    }

    #[inline(always)]
    fn get_allow_promote_to_persistent(&self) -> bool {
        Projectile::get_allow_promote_to_persistent(self.as_ref())
    }

    #[inline(always)]
    fn has_keyword_helper(&self, keyword: *const BGSKeyword) -> bool {
        Projectile::has_keyword_helper(self.as_ref(), keyword)
    }

    #[inline(always)]
    fn set_actor_cause(&mut self, cause: *mut ActorCause) {
        Projectile::set_actor_cause(self.as_mut(), cause)
    }

    #[inline(always)]
    fn get_actor_cause(&self) -> *mut ActorCause {
        Projectile::get_actor_cause(self.as_ref())
    }

    #[inline(always)]
    fn get_magic_caster(&mut self, source: CastingSource) -> *mut crate::re::MagicCaster {
        Projectile::get_magic_caster(self.as_mut(), source)
    }

    #[inline(always)]
    fn detach_havok(&mut self, obj_3d: *mut NiAVObject) -> bool {
        Projectile::detach_havok(self.as_mut(), obj_3d)
    }

    #[inline(always)]
    fn init_havok(&mut self) {
        Projectile::init_havok(self.as_mut())
    }

    #[inline(always)]
    fn load_3d(&mut self, background_loading: bool) -> *mut NiAVObject {
        Projectile::load_3d(self.as_mut(), background_loading)
    }

    #[inline(always)]
    fn set_3d(&mut self, object: *mut NiAVObject, queue_3d_tasks: bool) {
        Projectile::set_3d(self.as_mut(), object, queue_3d_tasks)
    }

    #[inline(always)]
    fn is_missile_projectile(&mut self) -> bool {
        Projectile::is_missile_projectile(self.as_mut())
    }

    #[inline(always)]
    fn is_grenade_projectile(&mut self) -> bool {
        Projectile::is_grenade_projectile(self.as_mut())
    }

    #[inline(always)]
    fn is_flame_projectile(&mut self) -> bool {
        Projectile::is_flame_projectile(self.as_mut())
    }

    #[inline(always)]
    fn is_beam_projectile(&mut self) -> bool {
        Projectile::is_beam_projectile(self.as_mut())
    }

    #[inline(always)]
    fn unk_a6(&mut self) {
        Projectile::unk_a6(self.as_mut())
    }

    #[inline(always)]
    fn is_barrier_projectile(&mut self) -> bool {
        Projectile::is_barrier_projectile(self.as_mut())
    }

    #[inline(always)]
    fn on_kill(&mut self) {
        Projectile::on_kill(self.as_mut())
    }

    #[inline(always)]
    fn process_3d(&mut self) {
        Projectile::process_3d(self.as_mut())
    }

    #[inline(always)]
    fn post_load_3d(&mut self, root: *mut NiAVObject) {
        Projectile::post_load_3d(self.as_mut(), root)
    }

    #[inline(always)]
    fn update_impl(&mut self, delta: f32) {
        Projectile::update_impl(self.as_mut(), delta)
    }

    #[inline(always)]
    fn process_impacts(&mut self) -> bool {
        Projectile::process_impacts(self.as_mut())
    }

    #[inline(always)]
    fn update_3d(&mut self) {
        Projectile::update_3d(self.as_mut())
    }

    #[inline(always)]
    fn unk_ae(&mut self) {
        Projectile::unk_ae(self.as_mut())
    }

    #[inline(always)]
    fn get_power_speed_mult(&self) -> f32 {
        Projectile::get_power_speed_mult(self.as_ref())
    }

    #[inline(always)]
    fn get_weapon_speed_mult(&self) -> f32 {
        Projectile::get_weapon_speed_mult(self.as_ref())
    }

    #[inline(always)]
    fn get_stop_main_sound_after_impact(&mut self) -> bool {
        Projectile::get_stop_main_sound_after_impact(self.as_mut())
    }

    #[inline(always)]
    fn report_havok_deactivation(&mut self) {
        Projectile::report_havok_deactivation(self.as_mut())
    }

    #[inline(always)]
    fn turn_off(&mut self, owner: *mut Actor, no_deactivate_sound: bool) -> bool {
        Projectile::turn_off(self.as_mut(), owner, no_deactivate_sound)
    }

    #[inline(always)]
    fn is_permanent(&self) -> bool {
        Projectile::is_permanent(self.as_ref())
    }

    #[inline(always)]
    fn get_gravity(&mut self) -> f32 {
        Projectile::get_gravity(self.as_mut())
    }

    #[inline(always)]
    fn clean_up_pointers_on_disable(&mut self) {
        Projectile::clean_up_pointers_on_disable(self.as_mut())
    }

    #[inline(always)]
    fn run_target_pick(&mut self) -> bool {
        Projectile::run_target_pick(self.as_mut())
    }

    #[inline(always)]
    fn get_kill_on_collision(&mut self) -> bool {
        Projectile::get_kill_on_collision(self.as_mut())
    }

    #[inline(always)]
    fn should_be_limited(&mut self) -> bool {
        Projectile::should_be_limited(self.as_mut())
    }

    #[inline(always)]
    fn targets_whole_body(&mut self) -> bool {
        Projectile::targets_whole_body(self.as_mut())
    }

    #[inline(always)]
    fn get_collision_group(&mut self) -> u32 {
        Projectile::get_collision_group(self.as_mut())
    }

    #[inline(always)]
    fn get_collision_shape(&mut self) -> *mut bhkShape {
        Projectile::get_collision_shape(self.as_mut())
    }

    #[inline(always)]
    fn add_impact(
        &mut self,
        refr: *mut TESObjectREFR,
        target_loc: &NiPoint3,
        velocity: &NiPoint3,
        collidable: *mut hkpCollidable,
        arg6: i32,
        arg7: u32,
    ) {
        Projectile::add_impact(
            self.as_mut(),
            refr,
            target_loc,
            velocity,
            collidable,
            arg6,
            arg7,
        )
    }

    #[inline(always)]
    fn handle_hits(&mut self, collidable: *mut hkpCollidable) -> bool {
        Projectile::handle_hits(self.as_mut(), collidable)
    }

    #[inline(always)]
    fn on_trigger_enter(&mut self) {
        Projectile::on_trigger_enter(self.as_mut())
    }

    #[inline(always)]
    fn handle_3d_loaded(&mut self) {
        Projectile::handle_3d_loaded(self.as_mut())
    }

    #[inline(always)]
    fn should_use_desired_target(&mut self) -> bool {
        Projectile::should_use_desired_target(self.as_mut())
    }

    #[inline(always)]
    fn get_projectile_base(&self) -> *mut BGSProjectile {
        Projectile::get_projectile_base(self.as_ref())
    }

    #[inline(always)]
    fn get_height(&self) -> f32 {
        Projectile::get_height(self.as_ref())
    }

    #[inline(always)]
    fn get_speed(&self) -> f32 {
        Projectile::get_speed(self.as_ref())
    }

    #[inline(always)]
    fn kill(&mut self) {
        Projectile::kill(self.as_mut())
    }
}
