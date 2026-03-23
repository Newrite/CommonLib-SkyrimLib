use crate::offsets::offsets_rtti::RTTI_TESModel;
use crate::offsets::offsets_vtable::VTABLE_TESModel;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::bs_resource_id::BSResourceID;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::inherit;

/// C++ `RE::TESModel`
#[repr(C)]
pub struct TESModel {
    pub base: BaseFormComponent,     // 00
    pub model: BSFixedString,        // 08 - MODL
    pub textures: *mut BSResourceID, // 10 - MODT
    pub addons: *mut u32,            // 18
    pub num_textures: u16,           // 20
    pub num_addons: u16,             // 22
    pub pad24: u32,                  // 24
}

const _: () = assert!(core::mem::size_of::<TESModel>() == 0x28);

impl RttiType for TESModel {
    const RTTI: VariantID = RTTI_TESModel;
}

impl TESModel {
    pub const RTTI: VariantID = RTTI_TESModel;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESModel;

    virtual_method! {
        pub const VFUNC_GET_MODEL: usize = 0x04;
        pub fn get_model() -> *const core::ffi::c_char
    }

    virtual_method! {
        pub const VFUNC_SET_MODEL: usize = 0x05;
        pub fn set_model(model: *const core::ffi::c_char)
    }

    virtual_method! {
        pub const VFUNC_GET_AS_MODEL_TEXTURE_SWAP: usize = 0x06;
        pub fn get_as_model_texture_swap() -> *mut crate::re::TESModelTextureSwap
    }

    #[inline]
    pub fn get_model_as_str(&self) -> &str {
        crate::core_util::ptr_to_str(self.get_model())
    }
}

pub trait TESModelExt {
    fn get_model(&self) -> *const core::ffi::c_char;
    fn set_model(&mut self, model: *const core::ffi::c_char);
    fn get_as_model_texture_swap(&mut self) -> *mut crate::re::TESModelTextureSwap;
    fn get_model_as_str(&self) -> &str;
}

impl<T: AsRef<TESModel> + AsMut<TESModel>> TESModelExt for T {
    fn get_model(&self) -> *const core::ffi::c_char {
        self.as_ref().get_model()
    }

    fn set_model(&mut self, model: *const core::ffi::c_char) {
        self.as_mut().set_model(model)
    }

    fn get_as_model_texture_swap(&mut self) -> *mut crate::re::TESModelTextureSwap {
        self.as_mut().get_as_model_texture_swap()
    }

    fn get_model_as_str(&self) -> &str {
        self.as_ref().get_model_as_str()
    }
}

inherit!(TESModel : BaseFormComponent);
