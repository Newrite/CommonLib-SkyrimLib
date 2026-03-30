use core_util::EnumSet;

use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_NiAVObject;
use crate::offsets::offsets_rtti::RTTI_NiAVObject;
use crate::offsets::offsets_vtable::VTABLE_NiAVObject;
use crate::re::bs_visit::{BSVisitControl, traverse_scenegraph_geometries};
use crate::re::{
    BSFixedString, BSLightingShaderMaterialFacegenTint, BSLightingShaderMaterialHairTint,
    BSShaderMaterialFeature, ColLayer, NiAlphaProperty, NiBound, NiColor, NiCullingProcess, NiNode,
    NiObject, NiObjectNET, NiPointer, NiTransform, TESObjectREFR, bhkCollisionObject,
    bhkWorldObject, hkpMotionMotionType, hkpRigidBody,
};
use crate::relocation::{RelocationID, RttiType, VariantID, VariantOffset};

/// C++ `RE::NiUpdateData::Flag`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NiUpdateDataFlag {
    Dirty = 1 << 0,
    DisableCollision = 1 << 13,
}

core_util::impl_enumset_type!(NiUpdateDataFlag => u32);

/// C++ `RE::NiUpdateData`
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct NiUpdateData {
    pub time: f32,                             // 00
    pub flags: EnumSet<NiUpdateDataFlag, u32>, // 04
}

const _: () = assert!(core::mem::size_of::<NiUpdateData>() == 0x8);
const _: () = assert!(core::mem::offset_of!(NiUpdateData, time) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiUpdateData, flags) == 0x04);

/// C++ `RE::PerformOpFunc`
#[repr(C)]
pub struct PerformOpFunc {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<PerformOpFunc>() == 0x8);
const _: () = assert!(core::mem::offset_of!(PerformOpFunc, vtable) == 0x00);

impl PerformOpFunc {
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_CALL: usize = 0x01;
        pub fn call(&mut self, object: *mut NiAVObject) -> bool
    }
}

/// C++ `RE::NiAVObject::Flag`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NiAVObjectFlag {
    Hidden = 1 << 0,
    SelectiveUpdate = 1 << 1,
    SelectiveUpdateTransforms = 1 << 2,
    SelectiveUpdateController = 1 << 3,
    SelectiveUpdateRigid = 1 << 4,
    DisplayObject = 1 << 5,
    DisableSorting = 1 << 6,
    SelectiveUpdateTransformsOverride = 1 << 7,
    SaveExternalGeometryData = 1 << 9,
    NoDecals = 1 << 10,
    AlwaysDraw = 1 << 11,
    PreProcessedNode = 1 << 12,
    FixedBound = 1 << 13,
    TopFadeNode = 1 << 14,
    IgnoreFade = 1 << 15,
    NoAnimSyncX = 1 << 16,
    NoAnimSyncY = 1 << 17,
    NoAnimSyncZ = 1 << 18,
    NoAnimSyncS = 1 << 19,
    NotVisible = 1 << 20,
    NoDismemberValidity = 1 << 21,
    RenderUse = 1 << 22,
    ShadowReceiver = 1 << 23,
    HighDetail = 1 << 24,
    ForceUpdate = 1 << 25,
    Accumulated = 1 << 26,
    MeshLOD = 1 << 27,
    Unk28 = 1 << 28,
    ShadowCaster = 1 << 29,
}

core_util::impl_enumset_type!(NiAVObjectFlag => u32);

pub type NiAVObjectFlags = EnumSet<NiAVObjectFlag, u32>;

/// C++ `RE::NiAVObject`
#[repr(C)]
pub struct NiAVObject {
    pub base: NiObjectNET,                                         // 000
    pub parent: *mut NiNode,                                       // 030
    pub parent_index: u32,                                         // 038
    pub unk03c: u32,                                               // 03C
    pub collision_object: NiPointer<crate::re::NiCollisionObject>, // 040
    pub local: NiTransform,                                        // 048
    pub world: NiTransform,                                        // 07C
    pub previous_world: NiTransform,                               // 0B0
    pub world_bound: NiBound,                                      // 0E4
    // TODO: `NiAVObject` has a runtime-divergent tail in `NiAVObject.h`:
    // SE/AE keep `flags` at `0x0F4`, while VR inserts extra floats and moves
    // `flags`/`userData` to `0x10C`/`0x110`. The in-struct SE/AE tail remains
    // here only because existing derived Rust translations such as `NiNode`
    // still embed the flat `0x110` prefix directly. End state: split the
    // common prefix from runtime-specific tail accessors and migrate
    // descendants off the fixed flat layout.
    pub flags: NiAVObjectFlags,          // 0F4
    pub user_data: *mut TESObjectREFR,   // 0F8
    pub fade_amount: f32,                // 100
    pub last_updated_frame_counter: u32, // 104
    pub unk108: u8,                      // 108
    pub flags02: u8,                     // 109
    pub unk10a: u16,                     // 10A
    pub pad10c: u32,                     // 10C
}

const _: () = assert!(core::mem::size_of::<NiAVObject>() == 0x110);
const _: () = assert!(core::mem::offset_of!(NiAVObject, parent) == 0x030);
const _: () = assert!(core::mem::offset_of!(NiAVObject, collision_object) == 0x040);
const _: () = assert!(core::mem::offset_of!(NiAVObject, local) == 0x048);
const _: () = assert!(core::mem::offset_of!(NiAVObject, world) == 0x07C);
const _: () = assert!(core::mem::offset_of!(NiAVObject, previous_world) == 0x0B0);
const _: () = assert!(core::mem::offset_of!(NiAVObject, world_bound) == 0x0E4);

impl RttiType for NiAVObject {
    const RTTI: VariantID = RTTI_NiAVObject;
}

impl crate::re::ni_ref_object::NiRef for NiAVObject {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(NiAVObject : NiObjectNET, base);

impl NiAVObject {
    pub const RTTI: VariantID = RTTI_NiAVObject;
    pub const NI_RTTI: VariantID = NiRTTI_NiAVObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiAVObject;

    pub const FLAGS_OFFSET: VariantOffset = VariantOffset::new(0x0F4, 0x0F4, 0x10C);
    pub const USER_DATA_OFFSET: VariantOffset = VariantOffset::new(0x0F8, 0x0F8, 0x110);
    pub const FADE_AMOUNT_OFFSET: VariantOffset = VariantOffset::new(0x100, 0x100, 0x100);
    pub const LAST_UPDATED_FRAME_COUNTER_OFFSET: VariantOffset =
        VariantOffset::new(0x104, 0x104, 0x104);

    // override (NiObjectNET)
    // const NiRTTI* GetRTTI() const override;                            // 02
    // void          LoadBinary(NiStream& a_stream) override;             // 18
    // void          LinkObject(NiStream& a_stream) override;             // 19
    // bool          RegisterStreamables(NiStream& a_stream) override;    // 1A
    // void          SaveBinary(NiStream& a_stream) override;             // 1B
    // bool          IsEqual(NiObject* a_object) override;                // 1C
    // void          ProcessClone(NiCloningProcess& a_cloning) override;  // 1D

    crate::virtual_method! {
        pub const VFUNC_UPDATE_CONTROLLERS: usize = 0x25;
        pub fn update_controllers(&mut self, data: &mut NiUpdateData)
    }

    #[inline(always)]
    pub fn apply_local_transform_to_world(&mut self) {
        crate::runtime::require_vr("NiAVObject::apply_local_transform_to_world");
        crate::relocate_virtual!(extern "C" fn(*mut Self), self as *mut Self, 0x26);
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_PERFORM_OP: VariantOffset = VariantOffset::new_se_ae(0x26, 0x27);
        pub fn perform_op(&mut self, func: &mut PerformOpFunc)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_ATTACH_PROPERTY: VariantOffset = VariantOffset::new_se_ae(0x27, 0x28);
        pub fn attach_property(&mut self, property: *mut NiAlphaProperty)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_MATERIAL_NEEDS_UPDATE: VariantOffset = VariantOffset::new_se_ae(0x28, 0x29);
        pub fn set_material_needs_update(&mut self, needs_update: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_DEFAULT_MATERIAL_NEEDS_UPDATE_FLAG: VariantOffset = VariantOffset::new_se_ae(0x29, 0x2A);
        pub fn set_default_material_needs_update_flag(&mut self, flag: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_OBJECT_BY_NAME: VariantOffset = VariantOffset::new_se_ae(0x2A, 0x2B);
        pub fn get_object_by_name(&mut self, name: *const BSFixedString) -> *mut NiAVObject
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_SELECTIVE_UPDATE_FLAGS: VariantOffset = VariantOffset::new_se_ae(0x2B, 0x2C);
        pub fn set_selective_update_flags(&mut self, selective_update: &mut bool, selective_update_transforms: bool, rigid: &mut bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE_DOWNWARD_PASS: VariantOffset = VariantOffset::new_se_ae(0x2C, 0x2D);
        pub fn update_downward_pass(&mut self, data: &mut NiUpdateData, arg2: u32)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE_SELECTED_DOWNWARD_PASS: VariantOffset = VariantOffset::new_se_ae(0x2D, 0x2E);
        pub fn update_selected_downward_pass(&mut self, data: &mut NiUpdateData, arg2: u32)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE_RIGID_DOWNWARD_PASS: VariantOffset = VariantOffset::new_se_ae(0x2E, 0x2F);
        pub fn update_rigid_downward_pass(&mut self, data: &mut NiUpdateData, arg2: u32)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE_WORLD_BOUND: VariantOffset = VariantOffset::new_se_ae(0x2F, 0x30);
        pub fn update_world_bound(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE_WORLD_DATA: VariantOffset = VariantOffset::new_se_ae(0x30, 0x31);
        pub fn update_world_data(&mut self, data: *mut NiUpdateData)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE_TRANSFORM_AND_BOUNDS: VariantOffset = VariantOffset::new_se_ae(0x31, 0x32);
        pub fn update_transform_and_bounds(&mut self, data: &mut NiUpdateData)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_PRE_ATTACH_UPDATE: VariantOffset = VariantOffset::new_se_ae(0x32, 0x33);
        pub fn pre_attach_update(&mut self, parent: *mut NiNode, data: &mut NiUpdateData)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_POST_ATTACH_UPDATE: VariantOffset = VariantOffset::new_se_ae(0x33, 0x34);
        pub fn post_attach_update(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_ON_VISIBLE: VariantOffset = VariantOffset::new_se_ae(0x34, 0x35);
        pub fn on_visible(&mut self, process: &mut NiCullingProcess, alpha_group_index: i32)
    }

    crate::relocation_func! {
        pub fn get_collision_object(&self) -> *mut bhkCollisionObject => RelocationID::new(25482, 26022)
    }

    crate::relocation_func! {
        pub fn remove_decals(&mut self) => RelocationID::new(15547, 15723)
    }

    crate::relocation_func! {
        pub fn set_collision_layer(&mut self, collision_layer: ColLayer) => RelocationID::new(76170, 77998)
    }

    crate::relocation_func! {
        pub fn set_collision_layer_and_group(&mut self, collision_layer: ColLayer, group: u32) => RelocationID::new(76171, 77999)
    }

    crate::relocation_func! {
        pub fn set_motion_type(&mut self, motion_type: u32, recurse: bool, force: bool, allow_activate: bool) -> bool
            => RelocationID::new(76033, 77866)
    }

    crate::relocation_func! {
        pub fn update(&mut self, data: &mut NiUpdateData) => RelocationID::new(68900, 70251)
    }

    crate::relocation_func! {
        pub fn update_rigid_constraints(&mut self, enable: bool, arg2: u8, arg3: u32) => RelocationID::new(76271, 78103)
    }

    crate::runtime_data_accessor! {
        pub fn runtime_flags() -> NiAVObjectFlags {
            offset: Self::FLAGS_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn runtime_flags_mut() -> NiAVObjectFlags {
            offset: Self::FLAGS_OFFSET
        }
    }

    crate::runtime_data_accessor! {
        pub fn runtime_user_data() -> *mut TESObjectREFR {
            offset: Self::USER_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn runtime_user_data_mut() -> *mut TESObjectREFR {
            offset: Self::USER_DATA_OFFSET
        }
    }

    crate::runtime_data_accessor! {
        pub fn runtime_fade_amount() -> f32 {
            offset: Self::FADE_AMOUNT_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn runtime_fade_amount_mut() -> f32 {
            offset: Self::FADE_AMOUNT_OFFSET
        }
    }

    crate::runtime_data_accessor! {
        pub fn runtime_last_updated_frame_counter() -> u32 {
            offset: Self::LAST_UPDATED_FRAME_COUNTER_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn runtime_last_updated_frame_counter_mut() -> u32 {
            offset: Self::LAST_UPDATED_FRAME_COUNTER_OFFSET
        }
    }

    #[inline(always)]
    pub fn get_app_culled(&self) -> bool {
        self.runtime_flags().all(NiAVObjectFlag::Hidden)
    }

    #[inline(always)]
    pub fn get_collision_layer(&self) -> ColLayer {
        let collision_object = self.collision_object.get();
        if collision_object.is_null() {
            return ColLayer::Unidentified;
        }

        let col_obj =
            unsafe { (*collision_object.cast::<NiObject>()).as_bhk_ni_collision_object() };
        if col_obj.is_null() {
            return ColLayer::Unidentified;
        }

        let body = unsafe { (*col_obj).body.get() };
        if body.is_null() {
            return ColLayer::Unidentified;
        }

        let rigid_body = unsafe { (*body.cast::<NiObject>()).as_bhk_rigid_body() };
        if rigid_body.is_null() {
            return ColLayer::Unidentified;
        }

        let referenced_object = unsafe {
            (*rigid_body.cast::<bhkWorldObject>())
                .base
                .base
                .referenced_object
                .get()
        };
        if referenced_object.is_null() {
            return ColLayer::Unidentified;
        }

        let havok_rigid_body = referenced_object.cast::<hkpRigidBody>();
        let collidable = unsafe { (*havok_rigid_body).base.base.get_collidable() };
        if collidable.is_null() {
            return ColLayer::Unidentified;
        }

        unsafe { (*collidable).get_collision_layer() }
    }

    #[inline(always)]
    pub fn get_user_data(&self) -> *mut TESObjectREFR {
        let user_data = *self.runtime_user_data();
        if !user_data.is_null() {
            return user_data;
        }

        if !self.parent.is_null() {
            return unsafe { (*self.parent).base.get_user_data() };
        }

        core::ptr::null_mut()
    }

    #[inline(always)]
    pub fn set_user_data(&mut self, reference: *mut TESObjectREFR) {
        *self.runtime_user_data_mut() = reference;
    }

    #[inline(always)]
    pub fn set_app_culled(&mut self, cull: bool) {
        if cull {
            self.runtime_flags_mut().set(NiAVObjectFlag::Hidden);
        } else {
            self.runtime_flags_mut().reset(NiAVObjectFlag::Hidden);
        }
    }

    #[inline(always)]
    pub fn get_object_by_name_ref(&mut self, name: &BSFixedString) -> *mut NiAVObject {
        self.get_object_by_name(name as *const _)
    }

    #[inline(always)]
    pub fn set_motion_type_enum(
        &mut self,
        motion_type: hkpMotionMotionType,
        recurse: bool,
        force: bool,
        allow_activate: bool,
    ) -> bool {
        self.set_motion_type(motion_type as u32, recurse, force, allow_activate)
    }

    #[inline(always)]
    pub fn is_visual_object_i(&self) -> i32 {
        i32::from_ne_bytes(self.world_bound.radius.to_ne_bytes())
    }

    #[inline(always)]
    pub fn cull(&mut self, culler: *mut NiCullingProcess, alpha_group_index: i32) {
        if !self.get_app_culled() {
            unsafe {
                (*culler).process1(self, alpha_group_index);
            }
        }
    }

    #[inline(always)]
    pub fn update_body_tint(&mut self, color: NiColor) {
        let _ = traverse_scenegraph_geometries(self, &mut |geometry| {
            let Some(lighting_shader) =
                (unsafe { (*geometry).lighting_shader_prop_cast().as_mut() })
            else {
                return BSVisitControl::Continue;
            };

            let material = lighting_shader.base.material;
            if !material.is_null()
                && unsafe { (*material).get_feature() == BSShaderMaterialFeature::FaceGenRGBTint }
            {
                unsafe {
                    (*material.cast::<BSLightingShaderMaterialFacegenTint>()).tint_color = color;
                }
            }

            BSVisitControl::Continue
        });
    }

    #[inline(always)]
    pub fn update_hair_color(&mut self, color: NiColor) {
        let _ = traverse_scenegraph_geometries(self, &mut |geometry| {
            let Some(lighting_shader) =
                (unsafe { (*geometry).lighting_shader_prop_cast().as_mut() })
            else {
                return BSVisitControl::Continue;
            };

            let material = lighting_shader.base.material;
            if !material.is_null()
                && unsafe { (*material).get_feature() == BSShaderMaterialFeature::HairTint }
            {
                unsafe {
                    (*material.cast::<BSLightingShaderMaterialHairTint>()).tint_color = color;
                }
            }

            BSVisitControl::Continue
        });
    }

    // TODO: `CullGeometry`, `CullNode`, and `GetMass` can now reuse translated
    // `BSVisit`, but they still need a follow-up audit against the newly added
    // `NiNode` / `BSGeometry` / collision surfaces before porting the exact
    // `NiAVObject.cpp` bodies here.
    // TODO: `HasAnimation` now only depends on the missing
    // `NiObjectNET::GetExtraData(...)` helper surface; `BSXFlags` itself is
    // translated.
    // TODO: `GetFirstGeometryOfShaderType`, `HasShaderType`, `SetProjectedUVData`,
    // `TintScenegraph`, and `UpdateMaterialAlpha` still need the remaining source-backed shader
    // material subtype and graphics-state helpers from `NiAVObject.cpp`.
}

impl AsRef<NiAVObject> for NiAVObject {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<NiAVObject> for NiAVObject {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

pub trait NiAVObjectExt {
    fn update_controllers(&mut self, data: &mut NiUpdateData);
    fn apply_local_transform_to_world(&mut self);
    fn perform_op(&mut self, func: &mut PerformOpFunc);
    fn attach_property(&mut self, property: *mut NiAlphaProperty);
    fn set_material_needs_update(&mut self, needs_update: bool);
    fn set_default_material_needs_update_flag(&mut self, flag: bool);
    fn get_object_by_name(&mut self, name: *const BSFixedString) -> *mut NiAVObject;
    fn get_object_by_name_ref(&mut self, name: &BSFixedString) -> *mut NiAVObject;
    fn set_selective_update_flags(
        &mut self,
        selective_update: &mut bool,
        selective_update_transforms: bool,
        rigid: &mut bool,
    );
    fn update_downward_pass(&mut self, data: &mut NiUpdateData, arg2: u32);
    fn update_selected_downward_pass(&mut self, data: &mut NiUpdateData, arg2: u32);
    fn update_rigid_downward_pass(&mut self, data: &mut NiUpdateData, arg2: u32);
    fn update_world_bound(&mut self);
    fn update_world_data(&mut self, data: *mut NiUpdateData);
    fn update_transform_and_bounds(&mut self, data: &mut NiUpdateData);
    fn pre_attach_update(&mut self, parent: *mut NiNode, data: &mut NiUpdateData);
    fn post_attach_update(&mut self);
    fn on_visible(&mut self, process: &mut NiCullingProcess, alpha_group_index: i32);
    fn get_app_culled(&self) -> bool;
    fn get_collision_object(&self) -> *mut bhkCollisionObject;
    fn get_collision_layer(&self) -> ColLayer;
    fn get_user_data(&self) -> *mut TESObjectREFR;
    fn set_user_data(&mut self, reference: *mut TESObjectREFR);
    fn remove_decals(&mut self);
    fn set_app_culled(&mut self, cull: bool);
    fn set_collision_layer(&mut self, collision_layer: ColLayer);
    fn set_collision_layer_and_group(&mut self, collision_layer: ColLayer, group: u32);
    fn set_motion_type(
        &mut self,
        motion_type: u32,
        recurse: bool,
        force: bool,
        allow_activate: bool,
    ) -> bool;
    fn set_motion_type_enum(
        &mut self,
        motion_type: hkpMotionMotionType,
        recurse: bool,
        force: bool,
        allow_activate: bool,
    ) -> bool;
    fn update(&mut self, data: &mut NiUpdateData);
    fn update_body_tint(&mut self, color: NiColor);
    fn update_hair_color(&mut self, color: NiColor);
    fn update_rigid_constraints(&mut self, enable: bool, arg2: u8, arg3: u32);
    fn get_flags(&self) -> NiAVObjectFlags;
    fn is_visual_object_i(&self) -> i32;
    fn cull(&mut self, culler: *mut NiCullingProcess, alpha_group_index: i32);
}

impl<T: AsRef<NiAVObject> + AsMut<NiAVObject>> NiAVObjectExt for T {
    #[inline(always)]
    fn update_controllers(&mut self, data: &mut NiUpdateData) {
        NiAVObject::update_controllers(self.as_mut(), data)
    }

    #[inline(always)]
    fn apply_local_transform_to_world(&mut self) {
        NiAVObject::apply_local_transform_to_world(self.as_mut())
    }

    #[inline(always)]
    fn perform_op(&mut self, func: &mut PerformOpFunc) {
        NiAVObject::perform_op(self.as_mut(), func)
    }

    #[inline(always)]
    fn attach_property(&mut self, property: *mut NiAlphaProperty) {
        NiAVObject::attach_property(self.as_mut(), property)
    }

    #[inline(always)]
    fn set_material_needs_update(&mut self, needs_update: bool) {
        NiAVObject::set_material_needs_update(self.as_mut(), needs_update)
    }

    #[inline(always)]
    fn set_default_material_needs_update_flag(&mut self, flag: bool) {
        NiAVObject::set_default_material_needs_update_flag(self.as_mut(), flag)
    }

    #[inline(always)]
    fn get_object_by_name(&mut self, name: *const BSFixedString) -> *mut NiAVObject {
        NiAVObject::get_object_by_name(self.as_mut(), name)
    }

    #[inline(always)]
    fn get_object_by_name_ref(&mut self, name: &BSFixedString) -> *mut NiAVObject {
        NiAVObject::get_object_by_name_ref(self.as_mut(), name)
    }

    #[inline(always)]
    fn set_selective_update_flags(
        &mut self,
        selective_update: &mut bool,
        selective_update_transforms: bool,
        rigid: &mut bool,
    ) {
        NiAVObject::set_selective_update_flags(
            self.as_mut(),
            selective_update,
            selective_update_transforms,
            rigid,
        )
    }

    #[inline(always)]
    fn update_downward_pass(&mut self, data: &mut NiUpdateData, arg2: u32) {
        NiAVObject::update_downward_pass(self.as_mut(), data, arg2)
    }

    #[inline(always)]
    fn update_selected_downward_pass(&mut self, data: &mut NiUpdateData, arg2: u32) {
        NiAVObject::update_selected_downward_pass(self.as_mut(), data, arg2)
    }

    #[inline(always)]
    fn update_rigid_downward_pass(&mut self, data: &mut NiUpdateData, arg2: u32) {
        NiAVObject::update_rigid_downward_pass(self.as_mut(), data, arg2)
    }

    #[inline(always)]
    fn update_world_bound(&mut self) {
        NiAVObject::update_world_bound(self.as_mut())
    }

    #[inline(always)]
    fn update_world_data(&mut self, data: *mut NiUpdateData) {
        NiAVObject::update_world_data(self.as_mut(), data)
    }

    #[inline(always)]
    fn update_transform_and_bounds(&mut self, data: &mut NiUpdateData) {
        NiAVObject::update_transform_and_bounds(self.as_mut(), data)
    }

    #[inline(always)]
    fn pre_attach_update(&mut self, parent: *mut NiNode, data: &mut NiUpdateData) {
        NiAVObject::pre_attach_update(self.as_mut(), parent, data)
    }

    #[inline(always)]
    fn post_attach_update(&mut self) {
        NiAVObject::post_attach_update(self.as_mut())
    }

    #[inline(always)]
    fn on_visible(&mut self, process: &mut NiCullingProcess, alpha_group_index: i32) {
        NiAVObject::on_visible(self.as_mut(), process, alpha_group_index)
    }

    #[inline(always)]
    fn get_app_culled(&self) -> bool {
        NiAVObject::get_app_culled(self.as_ref())
    }

    #[inline(always)]
    fn get_collision_object(&self) -> *mut bhkCollisionObject {
        NiAVObject::get_collision_object(self.as_ref())
    }

    #[inline(always)]
    fn get_collision_layer(&self) -> ColLayer {
        NiAVObject::get_collision_layer(self.as_ref())
    }

    #[inline(always)]
    fn get_user_data(&self) -> *mut TESObjectREFR {
        NiAVObject::get_user_data(self.as_ref())
    }

    #[inline(always)]
    fn set_user_data(&mut self, reference: *mut TESObjectREFR) {
        NiAVObject::set_user_data(self.as_mut(), reference)
    }

    #[inline(always)]
    fn remove_decals(&mut self) {
        NiAVObject::remove_decals(self.as_mut())
    }

    #[inline(always)]
    fn set_app_culled(&mut self, cull: bool) {
        NiAVObject::set_app_culled(self.as_mut(), cull)
    }

    #[inline(always)]
    fn set_collision_layer(&mut self, collision_layer: ColLayer) {
        NiAVObject::set_collision_layer(self.as_mut(), collision_layer)
    }

    #[inline(always)]
    fn set_collision_layer_and_group(&mut self, collision_layer: ColLayer, group: u32) {
        NiAVObject::set_collision_layer_and_group(self.as_mut(), collision_layer, group)
    }

    #[inline(always)]
    fn set_motion_type(
        &mut self,
        motion_type: u32,
        recurse: bool,
        force: bool,
        allow_activate: bool,
    ) -> bool {
        NiAVObject::set_motion_type(self.as_mut(), motion_type, recurse, force, allow_activate)
    }

    #[inline(always)]
    fn set_motion_type_enum(
        &mut self,
        motion_type: hkpMotionMotionType,
        recurse: bool,
        force: bool,
        allow_activate: bool,
    ) -> bool {
        NiAVObject::set_motion_type_enum(self.as_mut(), motion_type, recurse, force, allow_activate)
    }

    #[inline(always)]
    fn update(&mut self, data: &mut NiUpdateData) {
        NiAVObject::update(self.as_mut(), data)
    }

    #[inline(always)]
    fn update_body_tint(&mut self, color: NiColor) {
        NiAVObject::update_body_tint(self.as_mut(), color)
    }

    #[inline(always)]
    fn update_hair_color(&mut self, color: NiColor) {
        NiAVObject::update_hair_color(self.as_mut(), color)
    }

    #[inline(always)]
    fn update_rigid_constraints(&mut self, enable: bool, arg2: u8, arg3: u32) {
        NiAVObject::update_rigid_constraints(self.as_mut(), enable, arg2, arg3)
    }

    #[inline(always)]
    fn get_flags(&self) -> NiAVObjectFlags {
        *NiAVObject::runtime_flags(self.as_ref())
    }

    #[inline(always)]
    fn is_visual_object_i(&self) -> i32 {
        NiAVObject::is_visual_object_i(self.as_ref())
    }

    #[inline(always)]
    fn cull(&mut self, culler: *mut NiCullingProcess, alpha_group_index: i32) {
        NiAVObject::cull(self.as_mut(), culler, alpha_group_index)
    }
}
