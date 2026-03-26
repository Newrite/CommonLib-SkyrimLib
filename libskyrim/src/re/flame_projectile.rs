#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_FlameProjectile;
use crate::offsets::offsets_vtable::VTABLE_FlameProjectile;
use crate::re::{FormCastable, FormType, Projectile};
use crate::relocation::{RttiType, VariantID, VariantOffset};

/// C++ `RE::FlameProjectile::RecordFlags`
pub struct FlameProjectileRecordFlags;

/// C++ `RE::FlameProjectile::FLAME_RUNTIME_DATA`
#[repr(C)]
pub struct FlameProjectileRuntimeData {
    pub expiration_timer: f32, // 00
    pub cone_angle: f32,       // 04
}

const _: () = assert!(core::mem::size_of::<FlameProjectileRuntimeData>() == 0x08);
const _: () = assert!(core::mem::offset_of!(FlameProjectileRuntimeData, expiration_timer) == 0x00);
const _: () = assert!(core::mem::offset_of!(FlameProjectileRuntimeData, cone_angle) == 0x04);

/// C++ `RE::FlameProjectile`
#[repr(C)]
pub struct FlameProjectile {
    pub base: Projectile, // 00
}

const _: () = assert!(core::mem::size_of::<FlameProjectile>() == 0x80);
const _: () = assert!(core::mem::offset_of!(FlameProjectile, base) == 0x00);

core_util::inherit!(FlameProjectile : Projectile, base);

impl RttiType for FlameProjectile {
    const RTTI: VariantID = RTTI_FlameProjectile;
}

impl FormCastable for FlameProjectile {
    const TARGET_FORM_TYPE: FormType = FormType::ProjectileFlame;
}

impl FlameProjectile {
    pub const RTTI: VariantID = RTTI_FlameProjectile;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FlameProjectile;
    pub const FORMTYPE: FormType = FormType::ProjectileFlame;
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x1D8, 0x1E0, 0x1D8);
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0x1E0, 0x1E8, 0x1E0);

    crate::runtime_data_accessor! {
        pub fn get_flame_runtime_data() -> FlameProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn get_flame_runtime_data_mut() -> FlameProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }
}
