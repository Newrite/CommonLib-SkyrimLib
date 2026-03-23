use crate::offsets::offsets_rtti::RTTI_TESTexture;
use crate::offsets::offsets_vtable::VTABLE_TESTexture;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::bs_string::BSString;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::inherit;

#[repr(C)]
pub struct TESTexture {
    pub base: BaseFormComponent,     // 00
    pub texture_name: BSFixedString, // 08 - ICON
}

const _: () = assert!(core::mem::size_of::<TESTexture>() == 0x10);

impl RttiType for TESTexture {
    const RTTI: VariantID = RTTI_TESTexture;
}

inherit!(TESTexture : BaseFormComponent);

impl TESTexture {
    pub const RTTI: VariantID = RTTI_TESTexture;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESTexture;

    virtual_method! {
        pub const GET_MAX_ALLOWED_SIZE: usize = 0x04;
        pub fn get_max_allowed_size() -> u32
    }

    virtual_method! {
        pub const GET_AS_NORMAL_FILE: usize = 0x05;
        pub fn get_as_normal_file(a_out: &mut BSString) -> *const core::ffi::c_char
    }

    virtual_method! {
        pub const GET_DEFAULT_PATH: usize = 0x06;
        pub fn get_default_path() -> *const core::ffi::c_char
    }
}

pub trait TESTextureExt {
    fn get_max_allowed_size(&self) -> u32;
    fn get_as_normal_file(&self, a_out: &mut BSString) -> *const core::ffi::c_char;
    fn get_default_path(&self) -> *const core::ffi::c_char;
}

impl<T: AsRef<TESTexture>> TESTextureExt for T {
    fn get_max_allowed_size(&self) -> u32 {
        self.as_ref().get_max_allowed_size()
    }

    fn get_as_normal_file(&self, a_out: &mut BSString) -> *const core::ffi::c_char {
        self.as_ref().get_as_normal_file(a_out)
    }

    fn get_default_path(&self) -> *const core::ffi::c_char {
        self.as_ref().get_default_path()
    }
}
