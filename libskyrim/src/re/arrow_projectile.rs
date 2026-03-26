#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_ArrowProjectile;
use crate::offsets::offsets_vtable::VTABLE_ArrowProjectile;
use crate::re::{AlchemyItem, EnchantmentItem, FormCastable, FormType, MissileProjectile};
use crate::relocation::{RttiType, VariantID, VariantOffset};

/// C++ `RE::ArrowProjectile::RecordFlags`
pub struct ArrowProjectileRecordFlags;

/// C++ `RE::ArrowProjectile::ARROW_RUNTIME_DATA`
#[repr(C)]
pub struct ArrowProjectileRuntimeData {
    pub enchant_item: *mut EnchantmentItem, // 00
    pub poison: *mut AlchemyItem,           // 08
}

const _: () = assert!(core::mem::size_of::<ArrowProjectileRuntimeData>() == 0x10);
const _: () = assert!(core::mem::offset_of!(ArrowProjectileRuntimeData, enchant_item) == 0x00);
const _: () = assert!(core::mem::offset_of!(ArrowProjectileRuntimeData, poison) == 0x08);

/// C++ `RE::ArrowProjectile`
#[repr(C)]
pub struct ArrowProjectile {
    pub base: MissileProjectile, // 00
}

const _: () = assert!(core::mem::size_of::<ArrowProjectile>() == 0x80);
const _: () = assert!(core::mem::offset_of!(ArrowProjectile, base) == 0x00);

core_util::inherit!(ArrowProjectile : MissileProjectile, base);

impl RttiType for ArrowProjectile {
    const RTTI: VariantID = RTTI_ArrowProjectile;
}

impl FormCastable for ArrowProjectile {
    const TARGET_FORM_TYPE: FormType = FormType::ProjectileArrow;
}

impl ArrowProjectile {
    pub const RTTI: VariantID = RTTI_ArrowProjectile;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ArrowProjectile;
    pub const FORMTYPE: FormType = FormType::ProjectileArrow;
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x1E0, 0x1E8, 0x1E0);
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0x1F0, 0x1F8, 0x1F0);

    crate::runtime_data_accessor! {
        pub fn get_arrow_runtime_data() -> ArrowProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn get_arrow_runtime_data_mut() -> ArrowProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }
}
