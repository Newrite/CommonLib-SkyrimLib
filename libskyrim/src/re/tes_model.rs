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
const _: () = assert!(core::mem::offset_of!(TESModel, model) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESModel, textures) == 0x10);
const _: () = assert!(core::mem::offset_of!(TESModel, addons) == 0x18);
const _: () = assert!(core::mem::offset_of!(TESModel, num_textures) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESModel, num_addons) == 0x22);
const _: () = assert!(core::mem::offset_of!(TESModel, pad24) == 0x24);

impl RttiType for TESModel {
    const RTTI: VariantID = RTTI_TESModel;
}

impl TESModel {
    pub const RTTI: VariantID = RTTI_TESModel;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESModel;

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
    fn dtor(&mut self);
    fn initialize_data_component(&mut self);
    fn clear_data_component(&mut self);
    fn copy_component(&mut self, rhs: *mut BaseFormComponent);
    fn get_model(&self) -> *const core::ffi::c_char;
    fn set_model(&mut self, model: *const core::ffi::c_char);
    fn get_as_model_texture_swap(&mut self) -> *mut crate::re::TESModelTextureSwap;
    fn get_model_as_str(&self) -> &str;
}

impl<T: AsRef<TESModel> + AsMut<TESModel>> TESModelExt for T {
    fn dtor(&mut self) {
        TESModel::dtor(self.as_mut())
    }

    fn initialize_data_component(&mut self) {
        TESModel::initialize_data_component(self.as_mut())
    }

    fn clear_data_component(&mut self) {
        TESModel::clear_data_component(self.as_mut())
    }

    fn copy_component(&mut self, rhs: *mut BaseFormComponent) {
        TESModel::copy_component(self.as_mut(), rhs)
    }

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
