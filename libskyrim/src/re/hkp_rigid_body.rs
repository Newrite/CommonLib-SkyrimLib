#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_hkpRigidBody;
use crate::offsets::offsets_vtable::VTABLE_hkpRigidBody;
use crate::re::{hkMotionState, hkVector4, hkWorldOperationResult, hkpEntity, hkpShape};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::hkpRigidBody`
#[repr(C)]
pub struct hkpRigidBody {
    pub base: hkpEntity, // 00
}

const _: () = assert!(core::mem::size_of::<hkpRigidBody>() == 0x2D0);
const _: () = assert!(core::mem::offset_of!(hkpRigidBody, base) == 0x00);

impl RttiType for hkpRigidBody {
    const RTTI: VariantID = RTTI_hkpRigidBody;
}

inherit!(hkpRigidBody : hkpEntity, base);

impl AsRef<crate::re::hkReferencedObject> for hkpRigidBody {
    #[inline(always)]
    fn as_ref(&self) -> &crate::re::hkReferencedObject {
        self.base.as_ref()
    }
}

impl AsMut<crate::re::hkReferencedObject> for hkpRigidBody {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut crate::re::hkReferencedObject {
        self.base.as_mut()
    }
}

impl hkpRigidBody {
    pub const RTTI: VariantID = RTTI_hkpRigidBody;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpRigidBody;

    crate::virtual_method! {
        pub const VFUNC_SET_SHAPE: usize = 0x03;
        pub fn set_shape(&mut self, shape: *const hkpShape) -> hkWorldOperationResult
    }

    crate::virtual_method! {
        pub const VFUNC_UPDATE_SHAPE: usize = 0x04;
        pub fn update_shape(
            &mut self,
            shape_modifier: *mut core::ffi::c_void
        ) -> hkWorldOperationResult
    }

    crate::virtual_method! {
        pub const VFUNC_GET_MOTION_STATE: usize = 0x05;
        pub fn get_motion_state(&mut self) -> *mut hkMotionState
    }

    crate::virtual_method! {
        pub const VFUNC_CLONE: usize = 0x07;
        pub fn clone_body() -> *mut hkpRigidBody
    }

    #[inline(always)]
    pub fn apply_linear_impulse(&mut self, impulse: &hkVector4) {
        self.base.activate();
        self.base.motion.base.base.apply_linear_impulse(impulse);
    }

    #[inline(always)]
    pub fn set_linear_velocity(&mut self, new_vel: &hkVector4) {
        self.base.activate();
        self.base.motion.base.base.set_linear_velocity(new_vel);
    }

    #[inline(always)]
    pub fn set_angular_velocity(&mut self, new_vel: &hkVector4) {
        self.base.activate();
        self.base.motion.base.base.set_angular_velocity(new_vel);
    }
}
