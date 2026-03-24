use crate::offsets::offsets_rtti::RTTI_BSISoundOutputModel__BSIAttenuationCharacteristics;
use crate::offsets::offsets_vtable::VTABLE_BSISoundOutputModel__BSIAttenuationCharacteristics;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::BSISoundOutputModel::BSIAttenuationCharacteristics`
#[repr(C)]
pub struct BSIAttenuationCharacteristics {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<BSIAttenuationCharacteristics>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSIAttenuationCharacteristics, vtable) == 0x00);

impl RttiType for BSIAttenuationCharacteristics {
    const RTTI: VariantID = RTTI_BSISoundOutputModel__BSIAttenuationCharacteristics;
}

impl BSIAttenuationCharacteristics {
    pub const RTTI: VariantID = RTTI_BSISoundOutputModel__BSIAttenuationCharacteristics;
    pub const VTABLE: &'static [VariantID] =
        &VTABLE_BSISoundOutputModel__BSIAttenuationCharacteristics;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_GET_MAX_DISTANCE: usize = 0x01;
        pub fn get_max_distance() -> f32
    }

    virtual_method! {
        pub const VFUNC_GET_MIN_DISTANCE: usize = 0x02;
        pub fn get_min_distance() -> f32
    }

    virtual_method! {
        pub const VFUNC_GET_CURVE_VALUE: usize = 0x03;
        pub fn get_curve_value(idx: u32) -> u8
    }
}

pub trait BSIAttenuationCharacteristicsExt {
    fn get_max_distance(&self) -> f32;
    fn get_min_distance(&self) -> f32;
    fn get_curve_value(&self, idx: u32) -> u8;
}

impl<T: AsRef<BSIAttenuationCharacteristics>> BSIAttenuationCharacteristicsExt for T {
    fn get_max_distance(&self) -> f32 {
        self.as_ref().get_max_distance()
    }

    fn get_min_distance(&self) -> f32 {
        self.as_ref().get_min_distance()
    }

    fn get_curve_value(&self, idx: u32) -> u8 {
        self.as_ref().get_curve_value(idx)
    }
}
