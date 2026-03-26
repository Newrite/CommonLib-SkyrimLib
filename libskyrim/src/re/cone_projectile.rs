#![allow(non_camel_case_types)]

use core::ffi::c_void;

use crate::offsets::offsets_rtti::RTTI_ConeProjectile;
use crate::offsets::offsets_vtable::VTABLE_ConeProjectile;
use crate::re::{
    FormCastable, FormType, ImpactResult, NiPoint3, Projectile, bst_array::BSTArray, hkRefPtr,
    hkpSphereShape,
};
use crate::relocation::{RttiType, VariantID, VariantOffset};

/// C++ `RE::ConeProjectile::RecordFlags`
pub struct ConeProjectileRecordFlags;

/// C++ `RE::ConeProjectile::CONE_RUNTIME_DATA`
#[repr(C)]
pub struct ConeProjectileRuntimeData {
    pub impact_result: ImpactResult,               // 00
    pub environment_timer: f32,                    // 04
    pub cone_angle_tangent: f32,                   // 08
    pub initial_collision_sphere_radius: f32,      // 0C
    pub origin: NiPoint3,                          // 10
    pub pad1f4: u32,                               // 1C
    pub collision_shape: hkRefPtr<hkpSphereShape>, // 20
    pub collisions: BSTArray<*mut c_void>,         // 28
}

const _: () = assert!(core::mem::size_of::<ConeProjectileRuntimeData>() == 0x40);
const _: () = assert!(core::mem::offset_of!(ConeProjectileRuntimeData, impact_result) == 0x00);
const _: () = assert!(
    core::mem::offset_of!(ConeProjectileRuntimeData, initial_collision_sphere_radius) == 0x0C
);
const _: () = assert!(core::mem::offset_of!(ConeProjectileRuntimeData, origin) == 0x10);
const _: () = assert!(core::mem::offset_of!(ConeProjectileRuntimeData, collision_shape) == 0x20);
const _: () = assert!(core::mem::offset_of!(ConeProjectileRuntimeData, collisions) == 0x28);

/// C++ `RE::ConeProjectile`
#[repr(C)]
pub struct ConeProjectile {
    pub base: Projectile, // 00
}

const _: () = assert!(core::mem::size_of::<ConeProjectile>() == 0x80);
const _: () = assert!(core::mem::offset_of!(ConeProjectile, base) == 0x00);

core_util::inherit!(ConeProjectile : Projectile, base);

impl RttiType for ConeProjectile {
    const RTTI: VariantID = RTTI_ConeProjectile;
}

impl FormCastable for ConeProjectile {
    const TARGET_FORM_TYPE: FormType = FormType::ProjectileCone;
}

impl ConeProjectile {
    pub const RTTI: VariantID = RTTI_ConeProjectile;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ConeProjectile;
    pub const FORMTYPE: FormType = FormType::ProjectileCone;
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x1D8, 0x1E0, 0x1D8);
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0x218, 0x220, 0x218);

    crate::runtime_data_accessor! {
        pub fn get_cone_runtime_data() -> ConeProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn get_cone_runtime_data_mut() -> ConeProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_C2: VariantOffset = VariantOffset::new(0xC2, 0xC2, 0xC3);
        pub fn unk_c2(&mut self)
    }

    #[inline(always)]
    pub fn get_height(&self) -> f32 {
        self.get_cone_runtime_data().initial_collision_sphere_radius * 2.0
    }
}
