use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_NiObject;
use crate::offsets::offsets_nirtti::NiRTTI_NiObject;
use crate::offsets::offsets_vtable::VTABLE_NiObject;
use crate::re::bhkAttachmentCollisionObject;
use crate::re::bhkBlendCollisionObject;
use crate::re::bhkLimitedHingeConstraint;
use crate::re::bhkNiCollisionObject;
use crate::re::bhkRigidBody;
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
use crate::relocation_func;
use crate::virtual_method;
use crate::relocation::VariantID;

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
    fn inc_ref(&self) { self.base.inc_ref(); }
    #[inline(always)]
    fn dec_ref(&self) { self.base.dec_ref(); }
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
        pub fn process_clone(this: &mut NiObject, a_cloning: *mut NiCloningProcess) => VariantID::new(68838, 70190, 0)
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
        pub fn clone(this: &mut NiObject) -> *mut NiObject => VariantID::new(68835, 70187, 0)
    }

    // RELOCATION_ID SE: 68839, AE: 70191
    relocation_func! {
        pub fn create_deep_copy(this: &mut NiObject, a_object: &mut NiPointer<NiObject>) => VariantID::new(68839, 70191, 0)
    }
}
