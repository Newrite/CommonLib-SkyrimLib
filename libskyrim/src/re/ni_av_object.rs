use bitflags::bitflags;
use crate::core_util::inherit;
use crate::virtual_method;
use crate::relocation_func;
use crate::runtime_data_accessor;

use crate::offsets::offsets_rtti::RTTI_NiAVObject;
use crate::offsets::offsets_vtable::VTABLE_NiAVObject;
use crate::relocation::{VariantID, RttiType};

use crate::re::ni_object_net::NiObjectNET;
use crate::re::ni_node::NiNode;
use crate::re::ni_collision_object::NiCollisionObject;
use crate::re::ni_transform::NiTransform;
use crate::re::ni_bound::NiBound;
use crate::re::ni_smart_pointer::NiPointer;
use crate::re::bs_fixed_string::BSFixedString;

use crate::re::tes_object_refr::TESObjectREFR;
use crate::re::ni_culling_process::NiCullingProcess;
use crate::re::ni_alpha_property::NiAlphaProperty;

#[repr(C)]
#[derive(bytemuck::Zeroable, Clone, Copy, PartialEq)]
pub struct NiUpdateData {
    pub time: f32, // 0
    pub flags: u32, // 4
}
const _: () = assert!(core::mem::size_of::<NiUpdateData>() == 0x8);

#[repr(C)]
pub struct PerformOpFunc {
    pub vtable: *const (),
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NiAVObjectFlags: u32 {
        const NONE = 0;
        const HIDDEN = 1 << 0;
        const SELECTIVE_UPDATE = 1 << 1;
        const SELECTIVE_UPDATE_TRANSFORMS = 1 << 2;
        const SELECTIVE_UPDATE_CONTROLLER = 1 << 3;
        const SELECTIVE_UPDATE_RIGID = 1 << 4;
        const DISPLAY_OBJECT = 1 << 5;
        const DISABLE_SORTING = 1 << 6;
        const SELECTIVE_UPDATE_TRANSFORMS_OVERRIDE = 1 << 7;
        const SAVE_EXTERNAL_GEOMETRY_DATA = 1 << 9;
        const NO_DECALS = 1 << 10;
        const ALWAYS_DRAW = 1 << 11;
        const PRE_PROCESSED_NODE = 1 << 12;
        const FIXED_BOUND = 1 << 13;
        const TOP_FADE_NODE = 1 << 14;
        const IGNORE_FADE = 1 << 15;
        const NO_ANIM_SYNC_X = 1 << 16;
        const NO_ANIM_SYNC_Y = 1 << 17;
        const NO_ANIM_SYNC_Z = 1 << 18;
        const NO_ANIM_SYNC_S = 1 << 19;
        const NOT_VISIBLE = 1 << 20;
        const NO_DISMEMBER_VALIDITY = 1 << 21;
        const RENDER_USE = 1 << 22;
        const SHADOW_RECEIVER = 1 << 23;
        const HIGH_DETAIL = 1 << 24;
        const FORCE_UPDATE = 1 << 25;
        const ACCUMULATED = 1 << 26;
        const MESH_LOD = 1 << 27;
        const UNK28 = 1 << 28;
        const SHADOW_CASTER = 1 << 29;
    }
}
unsafe impl bytemuck::Zeroable for NiAVObjectFlags {}

#[repr(C)]
pub struct NiAVObject {
    pub base: NiObjectNET,                        // 000
    pub parent: *mut NiNode,                      // 030
    pub parent_index: u32,                        // 038
    pub unk03c: u32,                              // 03C
    pub collision_object: NiPointer<NiCollisionObject>, // 040
    pub local: NiTransform,                       // 048
    pub world: NiTransform,                       // 07C
    pub previous_world: NiTransform,              // 0B0
    pub world_bound: NiBound,                     // 0E4
    pub flags: NiAVObjectFlags,                   // 0F4
    pub user_data: *mut TESObjectREFR,            // 0F8
    pub fade_amount: f32,                         // 100
    pub last_updated_frame_counter: u32,          // 104
    pub unk108: u8,                               // 108
    pub flags02: u8,                              // 109
    pub unk10a: u16,                              // 10A
    pub pad10c: u32,                              // 10C
}

const _: () = assert!(core::mem::size_of::<NiAVObject>() == 0x110);

impl RttiType for NiAVObject {
    const RTTI: VariantID = RTTI_NiAVObject;
}

impl crate::re::ni_ref_object::NiRef for NiAVObject {
    #[inline(always)]
    fn inc_ref(&self) { self.base.inc_ref(); }
    #[inline(always)]
    fn dec_ref(&self) { self.base.dec_ref(); }
}

inherit!(NiAVObject : NiObjectNET);

impl NiAVObject {
    pub const RTTI: VariantID = RTTI_NiAVObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiAVObject;

    // override (NiObject)
    // const NiRTTI* GetRTTI() const override;                            // 02
    // void          LoadBinary(NiStream& a_stream) override;             // 18
    // void          LinkObject(NiStream& a_stream) override;             // 19
    // bool          RegisterStreamables(NiStream& a_stream) override;    // 1A
    // void          SaveBinary(NiStream& a_stream) override;             // 1B
    // bool          IsEqual(NiObject* a_object) override;                // 1C
    // void          ProcessClone(NiCloningProcess& a_cloning) override;  // 1D

    virtual_method! {
        pub const VFUNC_UPDATE_CONTROLLERS: usize = 0x25;
        pub fn update_controllers(a_data: *mut NiUpdateData)
    }

    // vtbl SE: 0x26, AE: 0x27
    virtual_method! {
        pub const VFUNC_PERFORM_OP: usize = 0x26;
        pub fn perform_op(a_func: *mut core::ffi::c_void)
    }

    // vtbl SE: 0x27, AE: 0x28
    virtual_method! {
        pub const VFUNC_ATTACH_PROPERTY: usize = 0x27;
        pub fn attach_property(a_property: *mut NiAlphaProperty)
    }

    // vtbl SE: 0x28, AE: 0x29
    virtual_method! {
        pub const VFUNC_SET_MATERIAL_NEEDS_UPDATE: usize = 0x28;
        pub fn set_material_needs_update(a_needs_update: bool)
    }

    // vtbl SE: 0x29, AE: 0x2A
    virtual_method! {
        pub const VFUNC_SET_DEFAULT_MATERIAL_NEEDS_UPDATE_FLAG: usize = 0x29;
        pub fn set_default_material_needs_update_flag(a_flag: bool)
    }

    // vtbl SE: 0x2A, AE: 0x2B
    virtual_method! {
        pub const VFUNC_GET_OBJECT_BY_NAME: usize = 0x2A;
        pub fn get_object_by_name(a_name: *const BSFixedString) -> *mut NiAVObject
    }

    // vtbl SE: 0x2B, AE: 0x2C
    virtual_method! {
        pub const VFUNC_SET_SELECTIVE_UPDATE_FLAGS: usize = 0x2B;
        pub fn set_selective_update_flags(a_selective_update: *mut bool, a_selective_update_transforms: bool, a_rigid: *mut bool)
    }

    // vtbl SE: 0x2C, AE: 0x2D
    virtual_method! {
        pub const VFUNC_UPDATE_DOWNWARD_PASS: usize = 0x2C;
        pub fn update_downward_pass(a_data: *mut NiUpdateData, a_arg2: u32)
    }

    // vtbl SE: 0x2D, AE: 0x2E
    virtual_method! {
        pub const VFUNC_UPDATE_SELECTED_DOWNWARD_PASS: usize = 0x2D;
        pub fn update_selected_downward_pass(a_data: *mut NiUpdateData, a_arg2: u32)
    }

    // vtbl SE: 0x2E, AE: 0x2F
    virtual_method! {
        pub const VFUNC_UPDATE_RIGID_DOWNWARD_PASS: usize = 0x2E;
        pub fn update_rigid_downward_pass(a_data: *mut NiUpdateData, a_arg2: u32)
    }

    // vtbl SE: 0x2F, AE: 0x30
    virtual_method! {
        pub const VFUNC_UPDATE_WORLD_BOUND: usize = 0x2F;
        pub fn update_world_bound()
    }

    // vtbl SE: 0x30, AE: 0x31
    virtual_method! {
        pub const VFUNC_UPDATE_WORLD_DATA: usize = 0x30;
        pub fn update_world_data(a_data: *mut NiUpdateData)
    }

    // vtbl SE: 0x31, AE: 0x32
    virtual_method! {
        pub const VFUNC_UPDATE_TRANSFORM_AND_BOUNDS: usize = 0x31;
        pub fn update_transform_and_bounds(a_data: *mut NiUpdateData)
    }

    // vtbl SE: 0x32, AE: 0x33
    virtual_method! {
        pub const VFUNC_PRE_ATTACH_UPDATE: usize = 0x32;
        pub fn pre_attach_update(a_parent: *mut NiNode, a_data: *mut NiUpdateData)
    }

    // vtbl SE: 0x33, AE: 0x34
    virtual_method! {
        pub const VFUNC_POST_ATTACH_UPDATE: usize = 0x33;
        pub fn post_attach_update()
    }

    // vtbl SE: 0x34, AE: 0x35
    virtual_method! {
        pub const VFUNC_ON_VISIBLE: usize = 0x34;
        pub fn on_visible(a_process: *mut NiCullingProcess, a_alpha_group_index: i32)
    }

    // RELOCATION_ID SE: 25482, AE: 26022
    relocation_func! {
        pub fn get_collision_object(&self) -> *mut core::ffi::c_void => VariantID::new(25482, 26022, 0)
    }

    // RELOCATION_ID SE: 15547, AE: 15723
    relocation_func! {
        pub fn remove_decals(&mut self) => VariantID::new(15547, 15723, 0)
    }

    // RELOCATION_ID SE: 76170, AE: 77998
    relocation_func! {
        pub fn set_collision_layer(&mut self, a_collision_layer: u32) => VariantID::new(76170, 77998, 0)
    }

    // RELOCATION_ID SE: 76171, AE: 77999
    relocation_func! {
        pub fn set_collision_layer_and_group(&mut self, a_collision_layer: u32, a_group: u32) => VariantID::new(76171, 77999, 0)
    }

    // RELOCATION_ID SE: 76033, AE: 77866
    relocation_func! {
        pub fn set_motion_type(&mut self, a_motion_type: u32, a_recurse: bool, a_force: bool, a_allow_activate: bool) -> bool => VariantID::new(76033, 77866, 0)
    }

    // RELOCATION_ID SE: 68900, AE: 70251
    relocation_func! {
        pub fn update(&mut self, a_data: *mut NiUpdateData) => VariantID::new(68900, 70251, 0)
    }

    // RELOCATION_ID SE: 76271, AE: 78103
    relocation_func! {
        pub fn update_rigid_constraints(&mut self, a_enable: bool, a_arg2: u8, a_arg3: u32) => VariantID::new(76271, 78103, 0)
    }
    
    runtime_data_accessor! {
        pub fn get_flags() -> NiAVObjectFlags {
            se_ae: 0x0F4,
            vr: 0x10C
        }
    }
}

impl AsRef<NiAVObject> for NiAVObject {
    #[inline(always)]
    fn as_ref(&self) -> &Self { self }
}

impl AsMut<NiAVObject> for NiAVObject {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self { self }
}

pub trait NiAVObjectExt {
    fn update_controllers(&mut self, a_data: *mut NiUpdateData);
    fn perform_op(&mut self, a_func: *mut core::ffi::c_void);
    fn attach_property(&mut self, a_property: *mut NiAlphaProperty);
    fn set_material_needs_update(&mut self, a_needs_update: bool);
    fn set_default_material_needs_update_flag(&mut self, a_flag: bool);
    fn get_object_by_name(&mut self, a_name: *const BSFixedString) -> *mut NiAVObject;
    fn set_selective_update_flags(&mut self, a_selective_update: *mut bool, a_selective_update_transforms: bool, a_rigid: *mut bool);
    fn update_downward_pass(&mut self, a_data: *mut NiUpdateData, a_arg2: u32);
    fn update_selected_downward_pass(&mut self, a_data: *mut NiUpdateData, a_arg2: u32);
    fn update_rigid_downward_pass(&mut self, a_data: *mut NiUpdateData, a_arg2: u32);
    fn update_world_bound(&mut self);
    fn update_world_data(&mut self, a_data: *mut NiUpdateData);
    fn update_transform_and_bounds(&mut self, a_data: *mut NiUpdateData);
    fn pre_attach_update(&mut self, a_parent: *mut NiNode, a_data: *mut NiUpdateData);
    fn post_attach_update(&mut self);
    fn on_visible(&mut self, a_process: *mut NiCullingProcess, a_alpha_group_index: i32);
    fn get_collision_object(&self) -> *mut core::ffi::c_void;
    fn remove_decals(&mut self);
    fn set_collision_layer(&mut self, a_collision_layer: u32);
    fn set_collision_layer_and_group(&mut self, a_collision_layer: u32, a_group: u32);
    fn set_motion_type(&mut self, a_motion_type: u32, a_recurse: bool, a_force: bool, a_allow_activate: bool) -> bool;
    fn update(&mut self, a_data: *mut NiUpdateData);
    fn update_rigid_constraints(&mut self, a_enable: bool, a_arg2: u8, a_arg3: u32);
    fn get_flags(&self) -> NiAVObjectFlags;
}

impl<T: AsRef<NiAVObject> + AsMut<NiAVObject>> NiAVObjectExt for T {
    fn update_controllers(&mut self, a_data: *mut NiUpdateData) {
        NiAVObject::update_controllers(self.as_mut(), a_data)
    }

    fn perform_op(&mut self, a_func: *mut core::ffi::c_void) {
        NiAVObject::perform_op(self.as_mut(), a_func)
    }

    fn attach_property(&mut self, a_property: *mut NiAlphaProperty) {
        NiAVObject::attach_property(self.as_mut(), a_property)
    }

    fn set_material_needs_update(&mut self, a_needs_update: bool) {
        NiAVObject::set_material_needs_update(self.as_mut(), a_needs_update)
    }

    fn set_default_material_needs_update_flag(&mut self, a_flag: bool) {
        NiAVObject::set_default_material_needs_update_flag(self.as_mut(), a_flag)
    }

    fn get_object_by_name(&mut self, a_name: *const BSFixedString) -> *mut NiAVObject {
        NiAVObject::get_object_by_name(self.as_mut(), a_name)
    }

    fn set_selective_update_flags(&mut self, a_selective_update: *mut bool, a_selective_update_transforms: bool, a_rigid: *mut bool) {
        NiAVObject::set_selective_update_flags(self.as_mut(), a_selective_update, a_selective_update_transforms, a_rigid)
    }

    fn update_downward_pass(&mut self, a_data: *mut NiUpdateData, a_arg2: u32) {
        NiAVObject::update_downward_pass(self.as_mut(), a_data, a_arg2)
    }

    fn update_selected_downward_pass(&mut self, a_data: *mut NiUpdateData, a_arg2: u32) {
        NiAVObject::update_selected_downward_pass(self.as_mut(), a_data, a_arg2)
    }

    fn update_rigid_downward_pass(&mut self, a_data: *mut NiUpdateData, a_arg2: u32) {
        NiAVObject::update_rigid_downward_pass(self.as_mut(), a_data, a_arg2)
    }

    fn update_world_bound(&mut self) {
        NiAVObject::update_world_bound(self.as_mut())
    }

    fn update_world_data(&mut self, a_data: *mut NiUpdateData) {
        NiAVObject::update_world_data(self.as_mut(), a_data)
    }

    fn update_transform_and_bounds(&mut self, a_data: *mut NiUpdateData) {
        NiAVObject::update_transform_and_bounds(self.as_mut(), a_data)
    }

    fn pre_attach_update(&mut self, a_parent: *mut NiNode, a_data: *mut NiUpdateData) {
        NiAVObject::pre_attach_update(self.as_mut(), a_parent, a_data)
    }

    fn post_attach_update(&mut self) {
        NiAVObject::post_attach_update(self.as_mut())
    }

    fn on_visible(&mut self, a_process: *mut NiCullingProcess, a_alpha_group_index: i32) {
        NiAVObject::on_visible(self.as_mut(), a_process, a_alpha_group_index)
    }

    fn get_collision_object(&self) -> *mut core::ffi::c_void {
        NiAVObject::get_collision_object(self.as_ref())
    }

    fn remove_decals(&mut self) {
        NiAVObject::remove_decals(self.as_mut())
    }

    fn set_collision_layer(&mut self, a_collision_layer: u32) {
        NiAVObject::set_collision_layer(self.as_mut(), a_collision_layer)
    }

    fn set_collision_layer_and_group(&mut self, a_collision_layer: u32, a_group: u32) {
        NiAVObject::set_collision_layer_and_group(self.as_mut(), a_collision_layer, a_group)
    }

    fn set_motion_type(&mut self, a_motion_type: u32, a_recurse: bool, a_force: bool, a_allow_activate: bool) -> bool {
        NiAVObject::set_motion_type(self.as_mut(), a_motion_type, a_recurse, a_force, a_allow_activate)
    }

    fn update(&mut self, a_data: *mut NiUpdateData) {
        NiAVObject::update(self.as_mut(), a_data)
    }

    fn update_rigid_constraints(&mut self, a_enable: bool, a_arg2: u8, a_arg3: u32) {
        NiAVObject::update_rigid_constraints(self.as_mut(), a_enable, a_arg2, a_arg3)
    }

    fn get_flags(&self) -> NiAVObjectFlags {
        unsafe { *NiAVObject::get_flags(self.as_ref()) }
    }
}