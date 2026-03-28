use crate::offsets::offsets_rtti::RTTI_TESModelTextureSwap;
use crate::offsets::offsets_vtable::VTABLE_TESModelTextureSwap;
use crate::re::BaseFormComponent;
use crate::re::bgs_texture_set::BGSTextureSet;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::tes_model::TESModel;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::inherit;

/// C++ `RE::TESModelTextureSwap::AlternateTexture`
#[repr(C)]
pub struct AlternateTexture {
    pub texture_set: *mut BGSTextureSet, // 00
    pub index3d: u32,                    // 08
    pub unk0c: u32,                      // 0C
    pub name3d: BSFixedString,           // 10
}

const _: () = assert!(core::mem::size_of::<AlternateTexture>() == 0x18);

/// C++ `RE::TESModelTextureSwap`
#[repr(C)]
pub struct TESModelTextureSwap {
    pub base: TESModel,                            // 00
    pub alternate_textures: *mut AlternateTexture, // 28 - MODS
    pub num_alternate_textures: u32,               // 30
    pub pad34: u32,                                // 34
}

const _: () = assert!(core::mem::size_of::<TESModelTextureSwap>() == 0x38);
const _: () = assert!(core::mem::offset_of!(TESModelTextureSwap, alternate_textures) == 0x28);
const _: () = assert!(core::mem::offset_of!(TESModelTextureSwap, num_alternate_textures) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESModelTextureSwap, pad34) == 0x34);

impl RttiType for TESModelTextureSwap {
    const RTTI: VariantID = RTTI_TESModelTextureSwap;
}

impl TESModelTextureSwap {
    pub const RTTI: VariantID = RTTI_TESModelTextureSwap;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESModelTextureSwap;

    // override (TESModel)
    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_INITIALIZE_DATA_COMPONENT: usize = 0x01;
        pub fn initialize_data_component()
    }

    virtual_method! {
        pub const VFUNC_CLEAR_DATA_COMPONENT: usize = 0x02;
        pub fn clear_data_component()
    }

    virtual_method! {
        pub const VFUNC_COPY_COMPONENT: usize = 0x03;
        pub fn copy_component(rhs: *mut BaseFormComponent)
    }

    virtual_method! {
        pub const VFUNC_GET_AS_MODEL_TEXTURE_SWAP: usize = 0x06;
        pub fn get_as_model_texture_swap() -> *mut TESModelTextureSwap
    }

    #[inline]
    pub fn get_alternate_textures(&self) -> &[AlternateTexture] {
        if self.alternate_textures.is_null() || self.num_alternate_textures == 0 {
            &[]
        } else {
            unsafe {
                core::slice::from_raw_parts(
                    self.alternate_textures,
                    self.num_alternate_textures as usize,
                )
            }
        }
    }
}

pub trait TESModelTextureSwapExt {
    fn dtor(&mut self);
    fn initialize_data_component(&mut self);
    fn clear_data_component(&mut self);
    fn copy_component(&mut self, rhs: *mut BaseFormComponent);
    fn get_as_model_texture_swap(&self) -> *mut TESModelTextureSwap;
    fn get_alternate_textures(&self) -> &[AlternateTexture];
}

impl<T: AsRef<TESModelTextureSwap> + AsMut<TESModelTextureSwap>> TESModelTextureSwapExt for T {
    fn dtor(&mut self) {
        TESModelTextureSwap::dtor(self.as_mut())
    }

    fn initialize_data_component(&mut self) {
        TESModelTextureSwap::initialize_data_component(self.as_mut())
    }

    fn clear_data_component(&mut self) {
        TESModelTextureSwap::clear_data_component(self.as_mut())
    }

    fn copy_component(&mut self, rhs: *mut BaseFormComponent) {
        TESModelTextureSwap::copy_component(self.as_mut(), rhs)
    }

    fn get_as_model_texture_swap(&self) -> *mut TESModelTextureSwap {
        TESModelTextureSwap::get_as_model_texture_swap(self.as_ref())
    }

    fn get_alternate_textures(&self) -> &[AlternateTexture] {
        TESModelTextureSwap::get_alternate_textures(self.as_ref())
    }
}

inherit!(TESModelTextureSwap : TESModel);
