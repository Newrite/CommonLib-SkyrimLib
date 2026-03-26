#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_GrenadeProjectile;
use crate::offsets::offsets_vtable::VTABLE_GrenadeProjectile;
use crate::re::{BGSDecalGroup, FormCastable, FormType, Projectile};
use crate::relocation::{RttiType, VariantID, VariantOffset};

/// C++ `RE::GrenadeProjectile::RecordFlags`
pub struct GrenadeProjectileRecordFlags;

/// C++ `RE::GrenadeProjectile::GRENADE_RUNTIME_DATA`
#[repr(C)]
pub struct GrenadeProjectileRuntimeData {
    pub decal_group: *mut BGSDecalGroup, // 00
    pub collision_group_reset: bool,     // 08
    pub pad1e1: u8,                      // 09
    pub pad1e2: u16,                     // 0A
    pub pad1e4: u32,                     // 0C
}

const _: () = assert!(core::mem::size_of::<GrenadeProjectileRuntimeData>() == 0x10);
const _: () = assert!(core::mem::offset_of!(GrenadeProjectileRuntimeData, decal_group) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(GrenadeProjectileRuntimeData, collision_group_reset) == 0x08);

/// C++ `RE::GrenadeProjectile`
#[repr(C)]
pub struct GrenadeProjectile {
    pub base: Projectile, // 00
}

const _: () = assert!(core::mem::size_of::<GrenadeProjectile>() == 0x80);
const _: () = assert!(core::mem::offset_of!(GrenadeProjectile, base) == 0x00);

core_util::inherit!(GrenadeProjectile : Projectile, base);

impl RttiType for GrenadeProjectile {
    const RTTI: VariantID = RTTI_GrenadeProjectile;
}

impl FormCastable for GrenadeProjectile {
    const TARGET_FORM_TYPE: FormType = FormType::ProjectileGrenade;
}

impl GrenadeProjectile {
    pub const RTTI: VariantID = RTTI_GrenadeProjectile;
    pub const VTABLE: &'static [VariantID] = &VTABLE_GrenadeProjectile;
    pub const FORMTYPE: FormType = FormType::ProjectileGrenade;
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x1D8, 0x1E0, 0x1D8);
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0x1E8, 0x1F0, 0x1E8);

    crate::runtime_data_accessor! {
        pub fn get_grenade_runtime_data() -> GrenadeProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn get_grenade_runtime_data_mut() -> GrenadeProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }
}
