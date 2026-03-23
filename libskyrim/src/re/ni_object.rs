use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_NiObject;
use crate::offsets::offsets_rtti::RTTI_NiObject;
use crate::offsets::offsets_vtable::VTABLE_NiObject;
use crate::re::BSDynamicTriShape;
use crate::re::BSFadeNode;
use crate::re::BSGeometry;
use crate::re::BSLines;
use crate::re::BSMultiBoundNode;
use crate::re::BSSegmentedTriShape;
use crate::re::BSSubIndexTriShape;
use crate::re::BSTriShape;
use crate::re::NiCloningProcess;
use crate::re::NiControllerManager;
use crate::re::NiGeometry;
use crate::re::NiNode;
use crate::re::NiObjectGroup;
use crate::re::NiParticles;
use crate::re::NiPointer;
use crate::re::NiRTTI;
use crate::re::NiRef;
use crate::re::NiRefObject;
use crate::re::NiStream;
use crate::re::NiSwitchNode;
use crate::re::NiTriBasedGeom;
use crate::re::NiTriShape;
use crate::re::NiTriStrips;
use crate::re::bhkAttachmentCollisionObject;
use crate::re::bhkBlendCollisionObject;
use crate::re::bhkLimitedHingeConstraint;
use crate::re::bhkNiCollisionObject;
use crate::re::bhkRigidBody;
use crate::relocation::{RelocationID, VariantID};
use crate::relocation_func;
use crate::virtual_method;

#[repr(C)]
pub struct NiObject {
    pub base: NiRefObject, // 00
}

const _: () = assert!(core::mem::size_of::<NiObject>() == 0x10);

impl crate::relocation::RttiType for NiObject {
    const RTTI: VariantID = RTTI_NiObject;
}

impl NiRef for NiObject {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }
    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(NiObject : NiRefObject);

impl NiObject {
    pub const RTTI: VariantID = RTTI_NiObject;
    pub const NI_RTTI: VariantID = NiRTTI_NiObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiObject;

    // override (NiRefObject)
    // ~NiObject() override = default;  // 00

    virtual_method! {
        pub const VFUNC_DESTRUCTOR: usize = 0x00;
        pub fn destructor()
    }

    virtual_method! {
        pub const VFUNC_GET_RTTI: usize = 0x02;
        pub fn get_rtti() -> *const NiRTTI
    }

    virtual_method! {
        pub const VFUNC_AS_NODE: usize = 0x03;
        pub fn as_node() -> *mut NiNode
    }

    virtual_method! {
        pub const VFUNC_AS_SWITCH_NODE: usize = 0x04;
        pub fn as_switch_node() -> *mut NiSwitchNode
    }

    virtual_method! {
        pub const VFUNC_AS_FADE_NODE: usize = 0x05;
        pub fn as_fade_node() -> *mut BSFadeNode
    }

    virtual_method! {
        pub const VFUNC_AS_MULTI_BOUND_NODE: usize = 0x06;
        pub fn as_multi_bound_node() -> *mut BSMultiBoundNode
    }

    virtual_method! {
        pub const VFUNC_AS_GEOMETRY: usize = 0x07;
        pub fn as_geometry() -> *mut BSGeometry
    }

    virtual_method! {
        pub const VFUNC_AS_TRI_STRIPS: usize = 0x08;
        pub fn as_tri_strips() -> *mut NiTriStrips
    }

    virtual_method! {
        pub const VFUNC_AS_TRI_SHAPE: usize = 0x09;
        pub fn as_tri_shape() -> *mut BSTriShape
    }

    virtual_method! {
        pub const VFUNC_AS_SEGMENTED_TRI_SHAPE: usize = 0x0A;
        pub fn as_segmented_tri_shape() -> *mut BSSegmentedTriShape
    }

    virtual_method! {
        pub const VFUNC_AS_SUB_INDEX_TRI_SHAPE: usize = 0x0B;
        pub fn as_sub_index_tri_shape() -> *mut BSSubIndexTriShape
    }

    virtual_method! {
        pub const VFUNC_AS_DYNAMIC_TRI_SHAPE: usize = 0x0C;
        pub fn as_dynamic_tri_shape() -> *mut BSDynamicTriShape
    }

    virtual_method! {
        pub const VFUNC_AS_NI_GEOMETRY: usize = 0x0D;
        pub fn as_ni_geometry() -> *mut NiGeometry
    }

    virtual_method! {
        pub const VFUNC_AS_NI_TRI_BASED_GEOM: usize = 0x0E;
        pub fn as_ni_tri_based_geom() -> *mut NiTriBasedGeom
    }

    virtual_method! {
        pub const VFUNC_AS_NI_TRI_SHAPE: usize = 0x0F;
        pub fn as_ni_tri_shape() -> *mut NiTriShape
    }

    virtual_method! {
        pub const VFUNC_AS_PARTICLES_GEOM: usize = 0x10;
        pub fn as_particles_geom() -> *mut NiParticles
    }

    virtual_method! {
        pub const VFUNC_AS_LINES_GEOM: usize = 0x11;
        pub fn as_lines_geom() -> *mut BSLines
    }

    virtual_method! {
        pub const VFUNC_AS_BHK_NI_COLLISION_OBJECT: usize = 0x12;
        pub fn as_bhk_ni_collision_object() -> *mut bhkNiCollisionObject
    }

    virtual_method! {
        pub const VFUNC_AS_BHK_BLEND_COLLISION_OBJECT: usize = 0x13;
        pub fn as_bhk_blend_collision_object() -> *mut bhkBlendCollisionObject
    }

    virtual_method! {
        pub const VFUNC_AS_BHK_ATTACHMENT_COLLISION_OBJECT: usize = 0x14;
        pub fn as_bhk_attachment_collision_object() -> *mut bhkAttachmentCollisionObject
    }

    virtual_method! {
        pub const VFUNC_AS_BHK_RIGID_BODY: usize = 0x15;
        pub fn as_bhk_rigid_body() -> *mut bhkRigidBody
    }

    virtual_method! {
        pub const VFUNC_AS_BHK_LIMITED_HINGE_CONSTRAINT: usize = 0x16;
        pub fn as_bhk_limited_hinge_constraint() -> *mut bhkLimitedHingeConstraint
    }

    virtual_method! {
        pub const VFUNC_CREATE_CLONE: usize = 0x17;
        pub fn create_clone(a_cloning: *mut NiCloningProcess) -> *mut NiObject
    }

    virtual_method! {
        pub const VFUNC_LOAD_BINARY: usize = 0x18;
        pub fn load_binary(a_stream: *mut NiStream)
    }

    virtual_method! {
        pub const VFUNC_LINK_OBJECT: usize = 0x19;
        pub fn link_object(a_stream: *mut NiStream)
    }

    virtual_method! {
        pub const VFUNC_REGISTER_STREAMABLES: usize = 0x1A;
        pub fn register_streamables(a_stream: *mut NiStream) -> bool
    }

    virtual_method! {
        pub const VFUNC_SAVE_BINARY: usize = 0x1B;
        pub fn save_binary(a_stream: *mut NiStream)
    }

    virtual_method! {
        pub const VFUNC_IS_EQUAL: usize = 0x1C;
        pub fn is_equal(a_object: *mut NiObject) -> bool
    }

    // RELOCATION_ID SE: 68838, AE: 70190
    relocation_func! {
        pub fn process_clone(&mut self, a_cloning: *mut NiCloningProcess) => RelocationID::new(68838, 70190)
    }

    virtual_method! {
        pub const VFUNC_PROCESS_CLONE: usize = 0x1D;
        pub fn process_clone_v(a_cloning: *mut NiCloningProcess)
    }

    virtual_method! {
        pub const VFUNC_POST_LINK_OBJECT: usize = 0x1E;
        pub fn post_link_object(a_stream: *mut NiStream)
    }

    virtual_method! {
        pub const VFUNC_STREAM_CAN_SKIP: usize = 0x1F;
        pub fn stream_can_skip() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_STREAMABLE_RTTI: usize = 0x20;
        pub fn get_streamable_rtti() -> *const NiRTTI
    }

    virtual_method! {
        pub const VFUNC_GET_BLOCK_ALLOCATION_SIZE: usize = 0x21;
        pub fn get_block_allocation_size() -> u32
    }

    virtual_method! {
        pub const VFUNC_GET_GROUP: usize = 0x22;
        pub fn get_group() -> *mut NiObjectGroup
    }

    virtual_method! {
        pub const VFUNC_SET_GROUP: usize = 0x23;
        pub fn set_group(a_group: *mut NiObjectGroup)
    }

    virtual_method! {
        pub const VFUNC_AS_NI_CONTROLLER_MANAGER: usize = 0x24;
        pub fn as_ni_controller_manager() -> *mut NiControllerManager
    }

    // RELOCATION_ID SE: 68835, AE: 70187
    relocation_func! {
        pub fn clone(&mut self) -> *mut NiObject => RelocationID::new(68835, 70187)
    }

    // RELOCATION_ID SE: 68839, AE: 70191
    relocation_func! {
        pub fn create_deep_copy(&mut self, a_object: &mut NiPointer<NiObject>) => RelocationID::new(68839, 70191)
    }
}

impl AsRef<NiObject> for NiObject {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<NiObject> for NiObject {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

pub trait NiObjectExt {
    fn destructor(&mut self);
    fn get_rtti(&self) -> *const NiRTTI;
    fn as_node(&mut self) -> *mut NiNode;
    fn as_switch_node(&mut self) -> *mut NiSwitchNode;
    fn as_fade_node(&mut self) -> *mut BSFadeNode;
    fn as_multi_bound_node(&mut self) -> *mut BSMultiBoundNode;
    fn as_geometry(&mut self) -> *mut BSGeometry;
    fn as_tri_strips(&mut self) -> *mut NiTriStrips;
    fn as_tri_shape(&mut self) -> *mut BSTriShape;
    fn as_segmented_tri_shape(&mut self) -> *mut BSSegmentedTriShape;
    fn as_sub_index_tri_shape(&mut self) -> *mut BSSubIndexTriShape;
    fn as_dynamic_tri_shape(&mut self) -> *mut BSDynamicTriShape;
    fn as_ni_geometry(&mut self) -> *mut NiGeometry;
    fn as_ni_tri_based_geom(&mut self) -> *mut NiTriBasedGeom;
    fn as_ni_tri_shape(&mut self) -> *mut NiTriShape;
    fn as_particles_geom(&mut self) -> *mut NiParticles;
    fn as_lines_geom(&mut self) -> *mut BSLines;
    fn as_bhk_ni_collision_object(&mut self) -> *mut bhkNiCollisionObject;
    fn as_bhk_blend_collision_object(&mut self) -> *mut bhkBlendCollisionObject;
    fn as_bhk_attachment_collision_object(&mut self) -> *mut bhkAttachmentCollisionObject;
    fn as_bhk_rigid_body(&mut self) -> *mut bhkRigidBody;
    fn as_bhk_limited_hinge_constraint(&mut self) -> *mut bhkLimitedHingeConstraint;
    fn create_clone(&mut self, a_cloning: *mut NiCloningProcess) -> *mut NiObject;
    fn load_binary(&mut self, a_stream: *mut NiStream);
    fn link_object(&mut self, a_stream: *mut NiStream);
    fn register_streamables(&mut self, a_stream: *mut NiStream) -> bool;
    fn save_binary(&mut self, a_stream: *mut NiStream);
    fn is_equal(&mut self, a_object: *mut NiObject) -> bool;
    fn process_clone(&mut self, a_cloning: *mut NiCloningProcess);
    fn process_clone_v(&mut self, a_cloning: *mut NiCloningProcess);
    fn post_link_object(&mut self, a_stream: *mut NiStream);
    fn stream_can_skip(&self) -> bool;
    fn get_streamable_rtti(&self) -> *const NiRTTI;
    fn get_block_allocation_size(&self) -> u32;
    fn get_group(&mut self) -> *mut NiObjectGroup;
    fn set_group(&mut self, a_group: *mut NiObjectGroup);
    fn as_ni_controller_manager(&mut self) -> *mut NiControllerManager;
    fn clone(&mut self) -> *mut NiObject;
    fn create_deep_copy(&mut self, a_object: &mut NiPointer<NiObject>);
}

impl<T: AsRef<NiObject> + AsMut<NiObject>> NiObjectExt for T {
    fn destructor(&mut self) {
        NiObject::destructor(self.as_mut())
    }

    fn get_rtti(&self) -> *const NiRTTI {
        NiObject::get_rtti(self.as_ref())
    }

    fn as_node(&mut self) -> *mut NiNode {
        NiObject::as_node(self.as_mut())
    }

    fn as_switch_node(&mut self) -> *mut NiSwitchNode {
        NiObject::as_switch_node(self.as_mut())
    }

    fn as_fade_node(&mut self) -> *mut BSFadeNode {
        NiObject::as_fade_node(self.as_mut())
    }

    fn as_multi_bound_node(&mut self) -> *mut BSMultiBoundNode {
        NiObject::as_multi_bound_node(self.as_mut())
    }

    fn as_geometry(&mut self) -> *mut BSGeometry {
        NiObject::as_geometry(self.as_mut())
    }

    fn as_tri_strips(&mut self) -> *mut NiTriStrips {
        NiObject::as_tri_strips(self.as_mut())
    }

    fn as_tri_shape(&mut self) -> *mut BSTriShape {
        NiObject::as_tri_shape(self.as_mut())
    }

    fn as_segmented_tri_shape(&mut self) -> *mut BSSegmentedTriShape {
        NiObject::as_segmented_tri_shape(self.as_mut())
    }

    fn as_sub_index_tri_shape(&mut self) -> *mut BSSubIndexTriShape {
        NiObject::as_sub_index_tri_shape(self.as_mut())
    }

    fn as_dynamic_tri_shape(&mut self) -> *mut BSDynamicTriShape {
        NiObject::as_dynamic_tri_shape(self.as_mut())
    }

    fn as_ni_geometry(&mut self) -> *mut NiGeometry {
        NiObject::as_ni_geometry(self.as_mut())
    }

    fn as_ni_tri_based_geom(&mut self) -> *mut NiTriBasedGeom {
        NiObject::as_ni_tri_based_geom(self.as_mut())
    }

    fn as_ni_tri_shape(&mut self) -> *mut NiTriShape {
        NiObject::as_ni_tri_shape(self.as_mut())
    }

    fn as_particles_geom(&mut self) -> *mut NiParticles {
        NiObject::as_particles_geom(self.as_mut())
    }

    fn as_lines_geom(&mut self) -> *mut BSLines {
        NiObject::as_lines_geom(self.as_mut())
    }

    fn as_bhk_ni_collision_object(&mut self) -> *mut bhkNiCollisionObject {
        NiObject::as_bhk_ni_collision_object(self.as_mut())
    }

    fn as_bhk_blend_collision_object(&mut self) -> *mut bhkBlendCollisionObject {
        NiObject::as_bhk_blend_collision_object(self.as_mut())
    }

    fn as_bhk_attachment_collision_object(&mut self) -> *mut bhkAttachmentCollisionObject {
        NiObject::as_bhk_attachment_collision_object(self.as_mut())
    }

    fn as_bhk_rigid_body(&mut self) -> *mut bhkRigidBody {
        NiObject::as_bhk_rigid_body(self.as_mut())
    }

    fn as_bhk_limited_hinge_constraint(&mut self) -> *mut bhkLimitedHingeConstraint {
        NiObject::as_bhk_limited_hinge_constraint(self.as_mut())
    }

    fn create_clone(&mut self, a_cloning: *mut NiCloningProcess) -> *mut NiObject {
        NiObject::create_clone(self.as_mut(), a_cloning)
    }

    fn load_binary(&mut self, a_stream: *mut NiStream) {
        NiObject::load_binary(self.as_mut(), a_stream)
    }

    fn link_object(&mut self, a_stream: *mut NiStream) {
        NiObject::link_object(self.as_mut(), a_stream)
    }

    fn register_streamables(&mut self, a_stream: *mut NiStream) -> bool {
        NiObject::register_streamables(self.as_mut(), a_stream)
    }

    fn save_binary(&mut self, a_stream: *mut NiStream) {
        NiObject::save_binary(self.as_mut(), a_stream)
    }

    fn is_equal(&mut self, a_object: *mut NiObject) -> bool {
        NiObject::is_equal(self.as_mut(), a_object)
    }

    fn process_clone(&mut self, a_cloning: *mut NiCloningProcess) {
        NiObject::process_clone(self.as_mut(), a_cloning)
    }

    fn process_clone_v(&mut self, a_cloning: *mut NiCloningProcess) {
        NiObject::process_clone_v(self.as_mut(), a_cloning)
    }

    fn post_link_object(&mut self, a_stream: *mut NiStream) {
        NiObject::post_link_object(self.as_mut(), a_stream)
    }

    fn stream_can_skip(&self) -> bool {
        NiObject::stream_can_skip(self.as_ref())
    }

    fn get_streamable_rtti(&self) -> *const NiRTTI {
        NiObject::get_streamable_rtti(self.as_ref())
    }

    fn get_block_allocation_size(&self) -> u32 {
        NiObject::get_block_allocation_size(self.as_ref())
    }

    fn get_group(&mut self) -> *mut NiObjectGroup {
        NiObject::get_group(self.as_mut())
    }

    fn set_group(&mut self, a_group: *mut NiObjectGroup) {
        NiObject::set_group(self.as_mut(), a_group)
    }

    fn as_ni_controller_manager(&mut self) -> *mut NiControllerManager {
        NiObject::as_ni_controller_manager(self.as_mut())
    }

    fn clone(&mut self) -> *mut NiObject {
        NiObject::clone(self.as_mut())
    }

    fn create_deep_copy(&mut self, a_object: &mut NiPointer<NiObject>) {
        NiObject::create_deep_copy(self.as_mut(), a_object)
    }
}
