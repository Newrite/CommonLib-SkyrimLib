use core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_TESTexture;
use crate::offsets::offsets_vtable::VTABLE_TESTexture;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::bs_string::BSString;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

#[repr(C)]
pub struct TESTexture {
    pub base: BaseFormComponent, // 00
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
        pub fn get_max_allowed_size(this: &TESTexture) -> u32
    }

    virtual_method! {
        pub const GET_AS_NORMAL_FILE: usize = 0x05;
        pub fn get_as_normal_file(this: &TESTexture, a_out: &mut BSString) -> *const core::ffi::c_char
    }

    virtual_method! {
        pub const GET_DEFAULT_PATH: usize = 0x06;
        pub fn get_default_path(this: &TESTexture) -> *const core::ffi::c_char
    }
}