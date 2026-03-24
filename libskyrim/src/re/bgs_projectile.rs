use crate::core_util::{EnumSet, inherit};
use crate::offsets::offsets_rtti::RTTI_BGSProjectile;
use crate::offsets::offsets_vtable::VTABLE_BGSProjectile;
use crate::re::BGSCollisionLayer;
use crate::re::BGSDestructibleObjectForm;
use crate::re::BGSExplosion;
use crate::re::BGSPreloadable;
use crate::re::BGSSoundDescriptorForm;
use crate::re::BGSTextureSet;
use crate::re::FormCastable;
use crate::re::FormType;
use crate::re::SOUND_LEVEL;
use crate::re::TESBoundObject;
use crate::re::TESFullName;
use crate::re::TESModel;
use crate::re::TESObjectLIGH;
use crate::re::TESObjectWEAP;
use crate::relocation::{RttiType, VariantID};

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSProjectileFlags {
    None = 0,
    HitScan = 1 << 0,
    Explosion = 1 << 1,
    ExplosionAltTrigger = 1 << 2,
    MuzzleFlash = 1 << 3,
    CanTurnOff = 1 << 5,
    CanPickUp = 1 << 6,
    Supersonic = 1 << 7,
    PinsLimbs = 1 << 8,
    PassSMTransparent = 1 << 9,
    DisableCombatAimCorrection = 1 << 10,
    ContinuousUpdate = 1 << 11,
}

core_util::impl_enumset_type!(BGSProjectileFlags => u16);

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSProjectileType {
    Missile = 1 << 0,
    Grenade = 1 << 1,
    Beam = 1 << 2,
    Flamethrower = 1 << 3,
    Cone = 1 << 4,
    Barrier = 1 << 5,
    Arrow = 1 << 6,
}

core_util::impl_enumset_type!(BGSProjectileType => u16);

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ProjectileRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

unsafe impl bytemuck::Zeroable for ProjectileRecordFlags {}

#[repr(C)]
pub struct BGSProjectileData {
    pub flags: EnumSet<BGSProjectileFlags, u16>, // 0x00
    pub types: EnumSet<BGSProjectileType, u16>,  // 0x02
    pub gravity: f32,                            // 0x04
    pub speed: f32,                              // 0x08
    pub range: f32,                              // 0x0C
    pub light: *mut TESObjectLIGH,               // 0x10
    pub muzzle_flash_light: *mut TESObjectLIGH,  // 0x18
    pub tracer_chance: f32,                      // 0x20
    pub explosion_proximity: f32,                // 0x24
    pub explosion_timer: f32,                    // 0x28
    pub pad2c: u32,                              // 0x2C
    pub explosion_type: *mut BGSExplosion,       // 0x30
    pub active_sound_loop: *mut BGSSoundDescriptorForm, // 0x38
    pub muzzle_flash_duration: f32,              // 0x40
    pub fade_out_time: f32,                      // 0x44
    pub force: f32,                              // 0x48
    pub pad4c: u32,                              // 0x4C
    pub countdown_sound: *mut BGSSoundDescriptorForm, // 0x50
    pub deactivate_sound: *mut BGSSoundDescriptorForm, // 0x58
    pub default_weapon_source: *mut TESObjectWEAP, // 0x60
    pub cone_spread: f32,                        // 0x68
    pub collision_radius: f32,                   // 0x6C
    pub lifetime: f32,                           // 0x70
    pub relaunch_interval: f32,                  // 0x74
    pub decal_data: *mut BGSTextureSet,          // 0x78
    pub collision_layer: *mut BGSCollisionLayer, // 0x80
}

const _: () = assert!(core::mem::size_of::<BGSProjectileData>() == 0x88);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, flags) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, types) == 0x02);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, gravity) == 0x04);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, speed) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, range) == 0x0C);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, light) == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, muzzle_flash_light) == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, tracer_chance) == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, explosion_proximity) == 0x24);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, explosion_timer) == 0x28);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, pad2c) == 0x2C);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, explosion_type) == 0x30);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, active_sound_loop) == 0x38);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, muzzle_flash_duration) == 0x40);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, fade_out_time) == 0x44);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, force) == 0x48);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, pad4c) == 0x4C);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, countdown_sound) == 0x50);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, deactivate_sound) == 0x58);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, default_weapon_source) == 0x60);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, cone_spread) == 0x68);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, collision_radius) == 0x6C);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, lifetime) == 0x70);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, relaunch_interval) == 0x74);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, decal_data) == 0x78);
const _: () = assert!(core::mem::offset_of!(BGSProjectileData, collision_layer) == 0x80);

#[repr(C)]
pub struct BGSProjectile {
    pub base: TESBoundObject,                                // 0x000
    pub full_name: TESFullName,                              // 0x030
    pub model: TESModel,                                     // 0x040
    pub preloadable: BGSPreloadable,                         // 0x068
    pub destructible_object_form: BGSDestructibleObjectForm, // 0x070
    pub data: BGSProjectileData,                             // 0x080
    pub muzzle_flash_model: TESModel,                        // 0x108
    pub sound_level: SOUND_LEVEL,                            // 0x130
    pub pad134: u32,                                         // 0x134
}

const _: () = assert!(core::mem::size_of::<BGSProjectile>() == 0x138);
const _: () = assert!(core::mem::offset_of!(BGSProjectile, full_name) == 0x30);
const _: () = assert!(core::mem::offset_of!(BGSProjectile, model) == 0x40);
const _: () = assert!(core::mem::offset_of!(BGSProjectile, preloadable) == 0x68);
const _: () = assert!(core::mem::offset_of!(BGSProjectile, destructible_object_form) == 0x70);
const _: () = assert!(core::mem::offset_of!(BGSProjectile, data) == 0x80);
const _: () = assert!(core::mem::offset_of!(BGSProjectile, muzzle_flash_model) == 0x108);
const _: () = assert!(core::mem::offset_of!(BGSProjectile, sound_level) == 0x130);
const _: () = assert!(core::mem::offset_of!(BGSProjectile, pad134) == 0x134);

impl RttiType for BGSProjectile {
    const RTTI: VariantID = RTTI_BGSProjectile;
}

impl FormCastable for BGSProjectile {
    const TARGET_FORM_TYPE: FormType = FormType::Projectile;
}

inherit!(BGSProjectile : TESBoundObject);
inherit!(BGSProjectile => TESFullName, full_name);
inherit!(BGSProjectile => TESModel, model);
inherit!(BGSProjectile => BGSPreloadable, preloadable);
inherit!(BGSProjectile => BGSDestructibleObjectForm, destructible_object_form);

impl BGSProjectile {
    pub const RTTI: VariantID = RTTI_BGSProjectile;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSProjectile;
    pub const FORMTYPE: FormType = FormType::Projectile;

    // override (TESBoundObject)
    // void        InitializeData() override;                            // 04
    // void        ClearData() override;                                 // 05
    // bool        Load(TESFile* a_mod) override;                        // 06
    // void        InitItemImpl() override;                              // 13
    // bool        Activate(...) override;                               // 37
    // NiAVObject* Clone3D(TESObjectREFR* a_ref, bool a_arg3) override;  // 40
    // void        UnClone3D(TESObjectREFR* a_ref) override;             // 41
    // bool        GetActivateText(...) override;                        // 4C

    #[inline(always)]
    pub fn is_missile(&self) -> bool {
        self.data.types.all(BGSProjectileType::Missile)
    }

    #[inline(always)]
    pub fn is_grenade(&self) -> bool {
        self.data.types.all(BGSProjectileType::Grenade)
    }

    #[inline(always)]
    pub fn is_beam(&self) -> bool {
        self.data.types.all(BGSProjectileType::Beam)
    }

    #[inline(always)]
    pub fn is_flamethrower(&self) -> bool {
        self.data.types.all(BGSProjectileType::Flamethrower)
    }

    #[inline(always)]
    pub fn is_cone(&self) -> bool {
        self.data.types.all(BGSProjectileType::Cone)
    }

    #[inline(always)]
    pub fn is_barrier(&self) -> bool {
        self.data.types.all(BGSProjectileType::Barrier)
    }

    #[inline(always)]
    pub fn is_arrow(&self) -> bool {
        self.data.types.all(BGSProjectileType::Arrow)
    }
}
