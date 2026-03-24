use crate::offsets::offsets_rtti::RTTI_BSISoundCategory;
use crate::offsets::offsets_vtable::VTABLE_BSISoundCategory;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::BSISoundCategory`
#[repr(C)]
pub struct BSISoundCategory {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<BSISoundCategory>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSISoundCategory, vtable) == 0x00);

impl RttiType for BSISoundCategory {
    const RTTI: VariantID = RTTI_BSISoundCategory;
}

impl BSISoundCategory {
    pub const RTTI: VariantID = RTTI_BSISoundCategory;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSISoundCategory;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_MATCHES: usize = 0x01;
        pub fn matches(category: *const BSISoundCategory) -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_CATEGORY_VOLUME: usize = 0x02;
        pub fn get_category_volume() -> f32
    }

    virtual_method! {
        pub const VFUNC_SET_CATEGORY_VOLUME: usize = 0x03;
        pub fn set_category_volume(value: f32)
    }

    virtual_method! {
        pub const VFUNC_GET_CATEGORY_FREQUENCY: usize = 0x04;
        pub fn get_category_frequency() -> f32
    }

    virtual_method! {
        pub const VFUNC_SET_CATEGORY_FREQUENCY: usize = 0x05;
        pub fn set_category_frequency(value: f32)
    }

    virtual_method! {
        pub const VFUNC_GET_CATEGORY_ATTENUATION: usize = 0x06;
        pub fn get_category_attenuation() -> u16
    }

    virtual_method! {
        pub const VFUNC_SET_CATEGORY_ATTENUATION: usize = 0x07;
        pub fn set_category_attenuation(value: u16)
    }

    virtual_method! {
        pub const VFUNC_UNK_08: usize = 0x08;
        pub fn unk_08()
    }

    virtual_method! {
        pub const VFUNC_UNK_09: usize = 0x09;
        pub fn unk_09()
    }

    virtual_method! {
        pub const VFUNC_UNK_0A: usize = 0x0A;
        pub fn unk_0a()
    }

    virtual_method! {
        pub const VFUNC_UNK_0B: usize = 0x0B;
        pub fn unk_0b()
    }

    #[inline(always)]
    pub fn matches_category(&self, category: &BSISoundCategory) -> bool {
        self.matches(category as *const BSISoundCategory)
    }
}

pub trait BSISoundCategoryExt {
    fn matches(&self, category: *const BSISoundCategory) -> bool;
    fn matches_category(&self, category: &BSISoundCategory) -> bool;
    fn get_category_volume(&self) -> f32;
    fn set_category_volume(&mut self, value: f32);
    fn get_category_frequency(&self) -> f32;
    fn set_category_frequency(&mut self, value: f32);
    fn get_category_attenuation(&self) -> u16;
    fn set_category_attenuation(&mut self, value: u16);
    fn unk_08(&mut self);
    fn unk_09(&mut self);
    fn unk_0a(&mut self);
    fn unk_0b(&mut self);
}

impl<T: AsRef<BSISoundCategory> + AsMut<BSISoundCategory>> BSISoundCategoryExt for T {
    fn matches(&self, category: *const BSISoundCategory) -> bool {
        self.as_ref().matches(category)
    }

    fn matches_category(&self, category: &BSISoundCategory) -> bool {
        self.as_ref().matches_category(category)
    }

    fn get_category_volume(&self) -> f32 {
        self.as_ref().get_category_volume()
    }

    fn set_category_volume(&mut self, value: f32) {
        self.as_mut().set_category_volume(value)
    }

    fn get_category_frequency(&self) -> f32 {
        self.as_ref().get_category_frequency()
    }

    fn set_category_frequency(&mut self, value: f32) {
        self.as_mut().set_category_frequency(value)
    }

    fn get_category_attenuation(&self) -> u16 {
        self.as_ref().get_category_attenuation()
    }

    fn set_category_attenuation(&mut self, value: u16) {
        self.as_mut().set_category_attenuation(value)
    }

    fn unk_08(&mut self) {
        self.as_mut().unk_08()
    }

    fn unk_09(&mut self) {
        self.as_mut().unk_09()
    }

    fn unk_0a(&mut self) {
        self.as_mut().unk_0a()
    }

    fn unk_0b(&mut self) {
        self.as_mut().unk_0b()
    }
}
