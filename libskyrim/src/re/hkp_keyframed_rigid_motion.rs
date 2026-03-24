use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_hkpKeyframedRigidMotion;
use crate::offsets::offsets_vtable::VTABLE_hkpKeyframedRigidMotion;
use crate::re::{hkVector4, hkpMaxSizeMotion, hkpMotion};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::hkpKeyframedRigidMotion`
#[repr(C)]
pub struct hkpKeyframedRigidMotion {
    pub base: hkpMotion, // 00
}

const _: () = assert!(core::mem::size_of::<hkpKeyframedRigidMotion>() == 0x140);
const _: () = assert!(core::mem::offset_of!(hkpKeyframedRigidMotion, base) == 0x00);

impl RttiType for hkpKeyframedRigidMotion {
    const RTTI: VariantID = RTTI_hkpKeyframedRigidMotion;
}

inherit!(hkpKeyframedRigidMotion : hkpMotion, base);

impl hkpKeyframedRigidMotion {
    pub const RTTI: VariantID = RTTI_hkpKeyframedRigidMotion;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpKeyframedRigidMotion;

    // override (hkpMotion)
    // void SetMass(float a_mass) override;  // 03
    // ...
    // void ApplyTorque(float a_deltaTime, const hkVector4& a_torque) override;  // 18

    virtual_method! {
        pub const VFUNC_SET_STEP_POSITION: usize = 0x1A;
        pub fn set_step_position(&mut self, a_position: f32, a_timestep: f32)
    }

    virtual_method! {
        pub const VFUNC_SET_STORED_MOTION: usize = 0x1B;
        pub fn set_stored_motion(&mut self, a_saved_motion: *mut hkpMaxSizeMotion)
    }

    #[inline(always)]
    pub fn get_point_velocity(&self, a_point: &hkVector4) -> hkVector4 {
        let center_of_mass_in_world = self.motion_state.swept_transform.center_of_mass1;
        self.linear_velocity
            + self
                .angular_velocity
                .cross(&(*a_point - center_of_mass_in_world))
    }
}
