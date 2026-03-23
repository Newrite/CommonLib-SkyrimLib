use crate::offsets::offsets_rtti::RTTI_TESFullName;
use crate::offsets::offsets_vtable::VTABLE_TESFullName;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::bs_fixed_string::BSFixedString;
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::virtual_method;
use core_util::inherit;

/// C++ `RE::TESFullName`
#[repr(C)]
pub struct TESFullName {
    pub base: BaseFormComponent,  // 0x00
    pub full_name: BSFixedString, // 0x08 - FULL
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

    // RELOCATION_ID SE: 22318, AE: 22791
    crate::relocation_func! {
        pub fn set_full_name(this: &mut TESFullName, name: *const core::ffi::c_char) => RelocationID::new(22318, 22791)
    }

    #[inline]
    pub fn get_name_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_full_name())
    }
}

pub trait TESFullNameExt {
    fn get_name_as_str(&self) -> &str;
    fn get_full_name_length(&self) -> u32;
    fn set_full_name(&mut self, name: *const core::ffi::c_char);
    fn get_full_name(&self) -> *const core::ffi::c_char;
}

impl<T: AsRef<TESFullName> + AsMut<TESFullName>> TESFullNameExt for T {
    fn get_name_as_str(&self) -> &str {
        self.as_ref().get_name_as_str()
    }

    fn get_full_name_length(&self) -> u32 {
        self.as_ref().get_full_name_length()
    }

    fn set_full_name(&mut self, name: *const core::ffi::c_char) {
        TESFullName::set_full_name(self.as_mut(), name)
    }

    fn get_full_name(&self) -> *const core::ffi::c_char {
        self.as_ref().get_full_name()
    }
}

inherit!(TESFullName : BaseFormComponent);
