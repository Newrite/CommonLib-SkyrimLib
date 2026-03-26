#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_MissileProjectile;
use crate::offsets::offsets_vtable::VTABLE_MissileProjectile;
use crate::re::{FormCastable, FormType, ImpactResult, Projectile};
use crate::relocation::{RttiType, VariantID, VariantOffset};

/// C++ `RE::MissileProjectile::RecordFlags`
pub struct MissileProjectileRecordFlags;

/// C++ `RE::MissileProjectile::MISSILE_RUNTIME_DATA`
#[repr(C)]
pub struct MissileProjectileRuntimeData {
    pub impact_result: ImpactResult,    // 00
    pub waiting_to_initialize_3d: bool, // 04
    pub unk1dd: u8,                     // 05
    pub unk1de: u16,                    // 06
}

const _: () = assert!(core::mem::size_of::<MissileProjectileRuntimeData>() == 0x08);
const _: () = assert!(core::mem::offset_of!(MissileProjectileRuntimeData, impact_result) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(MissileProjectileRuntimeData, waiting_to_initialize_3d) == 0x04);

/// C++ `RE::MissileProjectile`
#[repr(C)]
pub struct MissileProjectile {
    pub base: Projectile, // 00
}

const _: () = assert!(core::mem::size_of::<MissileProjectile>() == 0x80);
const _: () = assert!(core::mem::offset_of!(MissileProjectile, base) == 0x00);

core_util::inherit!(MissileProjectile : Projectile, base);

impl RttiType for MissileProjectile {
    const RTTI: VariantID = RTTI_MissileProjectile;
}

impl FormCastable for MissileProjectile {
    const TARGET_FORM_TYPE: FormType = FormType::ProjectileMissile;
}

impl MissileProjectile {
    pub const RTTI: VariantID = RTTI_MissileProjectile;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MissileProjectile;
    pub const FORMTYPE: FormType = FormType::ProjectileMissile;
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x1D8, 0x1E0, 0x1D8);
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0x1E0, 0x1E8, 0x1E0);

    crate::runtime_data_accessor! {
        pub fn get_missile_runtime_data() -> MissileProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn get_missile_runtime_data_mut() -> MissileProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_C2: VariantOffset = VariantOffset::new(0xC2, 0xC2, 0xC3);
        pub fn unk_c2(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_C3: VariantOffset = VariantOffset::new(0xC3, 0xC3, 0xC4);
        pub fn unk_c3(&mut self)
    }
}

pub trait MissileProjectileExt {
    fn get_missile_runtime_data(&self) -> &MissileProjectileRuntimeData;
    fn get_missile_runtime_data_mut(&mut self) -> &mut MissileProjectileRuntimeData;
    fn unk_c2(&mut self);
    fn unk_c3(&mut self);
}

impl<T: AsRef<MissileProjectile> + AsMut<MissileProjectile>> MissileProjectileExt for T {
    #[inline(always)]
    fn get_missile_runtime_data(&self) -> &MissileProjectileRuntimeData {
        MissileProjectile::get_missile_runtime_data(self.as_ref())
    }

    #[inline(always)]
    fn get_missile_runtime_data_mut(&mut self) -> &mut MissileProjectileRuntimeData {
        MissileProjectile::get_missile_runtime_data_mut(self.as_mut())
    }

    #[inline(always)]
    fn unk_c2(&mut self) {
        MissileProjectile::unk_c2(self.as_mut())
    }

    #[inline(always)]
    fn unk_c3(&mut self) {
        MissileProjectile::unk_c3(self.as_mut())
    }
}
