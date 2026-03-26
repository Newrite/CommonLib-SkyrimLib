#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_bhkCharacterController;
use crate::offsets::offsets_vtable::VTABLE_bhkCharacterController;
use crate::re::{
    BSBound, BSTEventSource, BSTHashMap, CFilter, DamageImpactData, MATERIAL_ID, NiPointer, NiRef,
    NiRefObject, bhkCharacterMoveFinishEvent, bhkICharOrientationController, bhkRigidBody,
    bhkShape, bhkWorld, hkRefPtr, hkStepInfo, hkTransform, hkVector4, hkpCharacterContext,
    hkpCharacterStateType, hkpRigidBody, hkpSurfaceInfo,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::CHARACTER_FLAGS`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CHARACTER_FLAGS {
    None = 0,
    Quadruped = 1 << 0,
    NoGravityOnGround = 1 << 1,
    TryStep = 1 << 2,
    NoFriction = 1 << 3,
    AllowJumpNoContact = 1 << 4,
    StuckQuad = 1 << 5,
    AnimAngleMod = 1 << 6,
    HitDamage = 1 << 7,
    Support = 1 << 8,
    HasPotentialSupportManifold = 1 << 9,
    CanJump = 1 << 10,
    ChaseBip = 1 << 11,
    FollowRagdoll = 1 << 12,
    Jumping = 1 << 13,
    NotPushable = 1 << 14,
    FloatLand = 1 << 15,
    CheckSupport = 1 << 16,
    NoSim = 1 << 17,
    FarAway = 1 << 18,
    OnStilts = 1 << 19,
    QuickSimulate = 1 << 20,
    RecordHits = 1 << 21,
    ComputeTiltPreIntegrate = 1 << 22,
    ShouldersUnderWater = 1 << 23,
    OnStairs = 1 << 24,
    CanPitch = 1 << 25,
    CanRoll = 1 << 26,
    NoCharacterCollisions = 1 << 27,
    NotPushablePermanent = 1 << 28,
    PossiblePathObstacle = 1 << 29,
    ShapeRequiresZRot = 1 << 30,
    SwimAtWaterSurface = 1 << 31,
}

core_util::impl_enumset_type!(CHARACTER_FLAGS => u32);

impl CHARACTER_FLAGS {
    pub const HIT_FLAGS: Self = Self::HitDamage;
}

/// C++ `RE::bhkCharacterController`
#[repr(C)]
pub struct bhkCharacterController {
    pub base: NiRefObject,                                         // 000
    pub event_source: BSTEventSource<bhkCharacterMoveFinishEvent>, // 010
    pub pad068: [u8; 0x8],                                         // 068
    pub forward_vec: hkVector4,                                    // 070
    pub step_info: hkStepInfo,                                     // 080
    pub out_velocity: hkVector4,                                   // 090
    pub initial_velocity: hkVector4,                               // 0A0
    pub velocity_mod: hkVector4,                                   // 0B0
    pub direction: hkVector4,                                      // 0C0
    pub rot_center: hkVector4,                                     // 0D0
    pub push_delta: hkVector4,                                     // 0E0
    pub fake_support_start: hkVector4,                             // 0F0
    pub up: hkVector4,                                             // 100
    pub support_norm: hkVector4,                                   // 110
    pub collision_bound: BSBound,                                  // 120
    pub bumper_collision_bound: BSBound,                           // 150
    pub delta_pos: hkVector4,                                      // 180
    pub orientation_ctrl: *mut bhkICharOrientationController,      // 190
    pub pad198: u64,                                               // 198
    pub surface_info: hkpSurfaceInfo,                              // 1A0
    pub context: hkpCharacterContext,                              // 1E0
    pub flags: core_util::EnumSet<CHARACTER_FLAGS, u32>,           // 218
    pub want_state: hkpCharacterStateType,                         // 21C
    pub velocity_time: f32,                                        // 220
    pub rot_mod: f32,                                              // 224
    pub rot_mod_time: f32,                                         // 228
    pub calculate_pitch_timer: f32,                                // 22C
    pub acrobatics: f32,                                           // 230
    pub center: f32,                                               // 234
    pub water_height: f32,                                         // 238
    pub jump_height: f32,                                          // 23C
    pub fall_start_height: f32,                                    // 240
    pub fall_time: f32,                                            // 244
    pub gravity: f32,                                              // 248
    pub pitch_angle: f32,                                          // 24C
    pub roll_angle: f32,                                           // 250
    pub pitch_mult: f32,                                           // 254
    pub scale: f32,                                                // 258
    pub swim_float_height: f32,                                    // 25C
    pub actor_height: f32,                                         // 260
    pub speed_pct: f32,                                            // 264
    pub push_count: u32,                                           // 268
    pub unk26c: u32,                                               // 26C
    pub unk270: u64,                                               // 270
    pub unk278: u64,                                               // 278
    pub shapes: [NiPointer<bhkShape>; 2],                          // 280
    pub radius: f32,                                               // 290
    pub height: f32,                                               // 294
    pub dest_radius: f32,                                          // 298
    pub lod_distance: f32,                                         // 29C
    pub size: u32,                                                 // 2A0
    pub priority: u32,                                             // 2A4
    pub support_count: i32,                                        // 2A8
    pub pad2ac: u32,                                               // 2AC
    pub support_body: hkRefPtr<hkpRigidBody>,                      // 2B0
    pub bumped_force: f32,                                         // 2B8
    pub pad2bc: u32,                                               // 2BC
    pub bumped_body: hkRefPtr<hkpRigidBody>,                       // 2C0
    pub bumped_char_collision_object: hkRefPtr<hkpRigidBody>,      // 2C8
    pub damage_impacts: BSTHashMap<NiPointer<bhkRigidBody>, *mut DamageImpactData>, // 2D0
    pub z_normal: f32,                                             // 300
    pub surface_material: MATERIAL_ID,                             // 304
    pub unk308: u32,                                               // 308
    pub unk30c: f32,                                               // 30C
    pub unk310: f32,                                               // 310
    pub contact_points_count: u32,                                 // 314
    pub collision_filter_group: u32,                               // 318
    pub unk31c: bool,                                              // 31C
    pub unk31d: u8,                                                // 31D
    pub unk31e: u8,                                                // 31E
    pub unk31f: u8,                                                // 31F
    pub unk320: u16,                                               // 320
    pub unk322: u16,                                               // 322
    pub unk324: u32,                                               // 324
    pub unk328: u64,                                               // 328
}

const _: () = assert!(core::mem::size_of::<bhkCharacterController>() == 0x330);
const _: () = assert!(core::mem::offset_of!(bhkCharacterController, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(bhkCharacterController, event_source) == 0x10);
const _: () = assert!(core::mem::offset_of!(bhkCharacterController, forward_vec) == 0x70);
const _: () = assert!(core::mem::offset_of!(bhkCharacterController, context) == 0x1E0);
const _: () = assert!(core::mem::offset_of!(bhkCharacterController, shapes) == 0x280);
const _: () = assert!(core::mem::offset_of!(bhkCharacterController, damage_impacts) == 0x2D0);

impl RttiType for bhkCharacterController {
    const RTTI: VariantID = RTTI_bhkCharacterController;
}

inherit!(bhkCharacterController : NiRefObject);
inherit!(bhkCharacterController => BSTEventSource<bhkCharacterMoveFinishEvent>, event_source);

impl NiRef for bhkCharacterController {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref()
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref()
    }
}

impl bhkCharacterController {
    pub const RTTI: VariantID = RTTI_bhkCharacterController;
    pub const VTABLE: &'static [VariantID] = &VTABLE_bhkCharacterController;

    crate::virtual_method! {
        pub const VFUNC_GET_POSITION_IMPL: usize = 0x02;
        pub fn get_position_impl(pos: &mut hkVector4, apply_center_offset: bool)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_POSITION_IMPL: usize = 0x03;
        pub fn set_position_impl(&mut self,
            pos: &hkVector4,
            apply_center_offset: bool,
            force_warp: bool
        )
    }

    crate::virtual_method! {
        pub const VFUNC_GET_TRANSFORM_IMPL: usize = 0x04;
        pub fn get_transform_impl(transform: &mut hkTransform)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_TRANSFORM_IMPL: usize = 0x05;
        pub fn set_transform_impl(&mut self, transform: &hkTransform)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_LINEAR_VELOCITY_IMPL: usize = 0x06;
        pub fn get_linear_velocity_impl(velocity: &mut hkVector4)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_LINEAR_VELOCITY_IMPL: usize = 0x07;
        pub fn set_linear_velocity_impl(&mut self, velocity: &hkVector4)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_COLLISION_FILTER_INFO: usize = 0x08;
        pub fn get_collision_filter_info(collision_filter_info: &mut CFilter)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_09: usize = 0x09;
        pub fn unk_09(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_0A: usize = 0x0A;
        pub fn unk_0a(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_0B: usize = 0x0B;
        pub fn unk_0b(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_0C: usize = 0x0C;
        pub fn unk_0c(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_CHECK_SUPPORT_IMPL: usize = 0x0D;
        pub fn check_support_impl(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_0E: usize = 0x0E;
        pub fn unk_0e(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_HAVOK_WORLD: usize = 0x0F;
        pub fn get_havok_world() -> *mut bhkWorld
    }

    crate::virtual_method! {
        pub const VFUNC_GET_RIGID_BODY: usize = 0x10;
        pub fn get_rigid_body() -> *mut hkpRigidBody
    }

    crate::virtual_method! {
        pub const VFUNC_GET_VDB_ALPHA: usize = 0x11;
        pub fn get_vdb_alpha() -> f32
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_12: usize = 0x12;
        pub fn unk_12(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_ROTATE_IMPL: usize = 0x13;
        pub fn rotate_impl(&mut self, transform: &mut hkTransform)
    }

    #[inline(always)]
    pub fn get_position(&self, pos: &mut hkVector4, apply_center_offset: bool) {
        let mut_self = self as *const Self as *mut Self;
        unsafe { (*mut_self).get_position_impl(pos, apply_center_offset) };
    }

    crate::relocation_func! {
        pub fn is_hurtful_body(body: *mut hkpRigidBody) -> bool => RelocationID::new(76456, 78298)
    }

    crate::relocation_func! {
        pub fn process_hurtful_body(
            &mut self,
            body: *mut hkpRigidBody,
            contact_point: *const core::ffi::c_void
        ) => RelocationID::new(76460, 78302)
    }
}
