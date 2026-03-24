use crate::offsets::offsets_rtti::RTTI_BSISoundDescriptor__BSIPlaybackCharacteristics;
use crate::offsets::offsets_vtable::VTABLE_BSISoundDescriptor__BSIPlaybackCharacteristics;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::BSISoundDescriptor::BSIPlaybackCharacteristics`
#[repr(C)]
pub struct BSIPlaybackCharacteristics {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<BSIPlaybackCharacteristics>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSIPlaybackCharacteristics, vtable) == 0x00);

impl RttiType for BSIPlaybackCharacteristics {
    const RTTI: VariantID = RTTI_BSISoundDescriptor__BSIPlaybackCharacteristics;
}

impl BSIPlaybackCharacteristics {
    pub const RTTI: VariantID = RTTI_BSISoundDescriptor__BSIPlaybackCharacteristics;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSISoundDescriptor__BSIPlaybackCharacteristics;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_GET_FREQUENCY_SHIFT: usize = 0x01;
        pub fn get_frequency_shift() -> u8
    }

    virtual_method! {
        pub const VFUNC_GET_FREQUENCY_VARIANCE: usize = 0x02;
        pub fn get_frequency_variance() -> u8
    }

    virtual_method! {
        pub const VFUNC_GET_PRIORITY: usize = 0x03;
        pub fn get_priority() -> u8
    }

    virtual_method! {
        pub const VFUNC_GET_STATIC_ATTENUATION: usize = 0x04;
        pub fn get_static_attenuation() -> u16
    }

    virtual_method! {
        pub const VFUNC_GET_DB_VARIANCE: usize = 0x05;
        pub fn get_db_variance() -> u8
    }
}

pub trait BSIPlaybackCharacteristicsExt {
    fn get_frequency_shift(&self) -> u8;
    fn get_frequency_variance(&self) -> u8;
    fn get_priority(&self) -> u8;
    fn get_static_attenuation(&self) -> u16;
    fn get_db_variance(&self) -> u8;
}

impl<T: AsRef<BSIPlaybackCharacteristics>> BSIPlaybackCharacteristicsExt for T {
    fn get_frequency_shift(&self) -> u8 {
        self.as_ref().get_frequency_shift()
    }

    fn get_frequency_variance(&self) -> u8 {
        self.as_ref().get_frequency_variance()
    }

    fn get_priority(&self) -> u8 {
        self.as_ref().get_priority()
    }

    fn get_static_attenuation(&self) -> u16 {
        self.as_ref().get_static_attenuation()
    }

    fn get_db_variance(&self) -> u8 {
        self.as_ref().get_db_variance()
    }
}
