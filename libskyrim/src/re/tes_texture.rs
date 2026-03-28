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
const _: () = assert!(core::mem::offset_of!(TESTexture, texture_name) == 0x08);

impl RttiType for TESTexture {
    const RTTI: VariantID = RTTI_TESTexture;
}

inherit!(TESTexture : BaseFormComponent);

impl TESTexture {
    pub const RTTI: VariantID = RTTI_TESTexture;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESTexture;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    virtual_method! {
        pub const VFUNC_INITIALIZE_DATA_COMPONENT: usize = 0x01;
        pub fn initialize_data_component(&mut self)
    }

    virtual_method! {
        pub const VFUNC_CLEAR_DATA_COMPONENT: usize = 0x02;
        pub fn clear_data_component(&mut self)
    }

    virtual_method! {
        pub const VFUNC_COPY_COMPONENT: usize = 0x03;
        pub fn copy_component(&mut self, rhs: *mut BaseFormComponent)
    }

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

    #[inline]
    pub fn get_as_normal_file_as_str<'a>(&self, a_out: &'a mut BSString) -> &'a str {
        let _ = self.get_as_normal_file(a_out);
        a_out.as_str()
    }

    #[inline]
    pub fn get_default_path_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_default_path())
    }
}

pub trait TESTextureExt {
    fn dtor(&mut self);
    fn initialize_data_component(&mut self);
    fn clear_data_component(&mut self);
    fn copy_component(&mut self, rhs: *mut BaseFormComponent);
    fn get_max_allowed_size(&self) -> u32;
    fn get_as_normal_file(&self, a_out: &mut BSString) -> *const core::ffi::c_char;
    fn get_as_normal_file_as_str<'a>(&self, a_out: &'a mut BSString) -> &'a str;
    fn get_default_path(&self) -> *const core::ffi::c_char;
    fn get_default_path_as_str(&self) -> &str;
}

impl<T: AsRef<TESTexture> + AsMut<TESTexture>> TESTextureExt for T {
    fn dtor(&mut self) {
        TESTexture::dtor(self.as_mut())
    }

    fn initialize_data_component(&mut self) {
        TESTexture::initialize_data_component(self.as_mut())
    }

    fn clear_data_component(&mut self) {
        TESTexture::clear_data_component(self.as_mut())
    }

    fn copy_component(&mut self, rhs: *mut BaseFormComponent) {
        TESTexture::copy_component(self.as_mut(), rhs)
    }

    fn get_max_allowed_size(&self) -> u32 {
        self.as_ref().get_max_allowed_size()
    }

    fn get_as_normal_file(&self, a_out: &mut BSString) -> *const core::ffi::c_char {
        self.as_ref().get_as_normal_file(a_out)
    }

    fn get_as_normal_file_as_str<'a>(&self, a_out: &'a mut BSString) -> &'a str {
        let _ = self.get_as_normal_file(a_out);
        a_out.as_str()
    }

    fn get_default_path(&self) -> *const core::ffi::c_char {
        self.as_ref().get_default_path()
    }

    fn get_default_path_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_default_path())
    }
}
