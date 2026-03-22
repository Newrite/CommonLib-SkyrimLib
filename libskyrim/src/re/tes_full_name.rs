use crate::offsets::offsets_rtti::RTTI_TESFullName;
use crate::offsets::offsets_vtable::VTABLE_TESFullName;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{VariantID, RttiType};
use core_util::inherit;
use crate::virtual_method;

/// C++ `RE::TESFullName`
#[repr(C)]
pub struct TESFullName {
    pub base: BaseFormComponent,  // 00
    pub full_name: BSFixedString, // 08 - FULL
}

const _: () = assert!(core::mem::size_of::<TESFullName>() == 0x10);

impl RttiType for TESFullName {
    const RTTI: VariantID = RTTI_TESFullName;
}

impl TESFullName {
    pub const RTTI: VariantID = RTTI_TESFullName;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESFullName;

    virtual_method! {
        pub const VFUNC_GET_FULL_NAME_LENGTH: usize = 0x04;
        pub fn get_full_name_length() -> u32
    }

    virtual_method! {
        pub const VFUNC_GET_FULL_NAME: usize = 0x05;
        pub fn get_full_name() -> *const core::ffi::c_char
    }

    crate::relocation_func! {
        pub fn set_full_name(this: &mut TESFullName, name: *const core::ffi::c_char) => VariantID::new(22318, 22791, 0)
    }
}

inherit!(TESFullName : BaseFormComponent);
