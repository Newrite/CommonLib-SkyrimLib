#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_BarrierProjectile;
use crate::offsets::offsets_vtable::VTABLE_BarrierProjectile;
use crate::re::{FormCastable, FormType, ObjectRefHandle, Projectile, bst_array::BSTArray};
use crate::relocation::{RttiType, VariantID, VariantOffset};

/// C++ `RE::BarrierProjectile::RecordFlags`
pub struct BarrierProjectileRecordFlags;

/// C++ `RE::BarrierProjectile::CollisionData`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BarrierProjectileCollisionData {
    pub refr: ObjectRefHandle, // 00
    pub count: u32,            // 04
}

const _: () = assert!(core::mem::size_of::<BarrierProjectileCollisionData>() == 0x08);
const _: () = assert!(core::mem::offset_of!(BarrierProjectileCollisionData, refr) == 0x00);
const _: () = assert!(core::mem::offset_of!(BarrierProjectileCollisionData, count) == 0x04);

/// C++ `RE::BarrierProjectile::BARRIER_RUNTIME_DATA`
#[repr(C)]
pub struct BarrierProjectileRuntimeData {
    pub width: f32,                                               // 00
    pub pad1dc: u32,                                              // 04
    pub collision_data: BSTArray<BarrierProjectileCollisionData>, // 08
}

const _: () = assert!(core::mem::size_of::<BarrierProjectileRuntimeData>() == 0x20);
const _: () = assert!(core::mem::offset_of!(BarrierProjectileRuntimeData, width) == 0x00);
const _: () = assert!(core::mem::offset_of!(BarrierProjectileRuntimeData, collision_data) == 0x08);

/// C++ `RE::BarrierProjectile`
#[repr(C)]
pub struct BarrierProjectile {
    pub base: Projectile, // 00
}

const _: () = assert!(core::mem::size_of::<BarrierProjectile>() == 0x80);
const _: () = assert!(core::mem::offset_of!(BarrierProjectile, base) == 0x00);

core_util::inherit!(BarrierProjectile : Projectile, base);

impl RttiType for BarrierProjectile {
    const RTTI: VariantID = RTTI_BarrierProjectile;
}

impl FormCastable for BarrierProjectile {
    const TARGET_FORM_TYPE: FormType = FormType::ProjectileBarrier;
}

impl BarrierProjectile {
    pub const RTTI: VariantID = RTTI_BarrierProjectile;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BarrierProjectile;
    pub const FORMTYPE: FormType = FormType::ProjectileBarrier;
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x1D8, 0x1E0, 0x1D8);
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0x1F8, 0x200, 0x1F8);

    crate::runtime_data_accessor! {
        pub fn get_barrier_runtime_data() -> BarrierProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn get_barrier_runtime_data_mut() -> BarrierProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }
}
