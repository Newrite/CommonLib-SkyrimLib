use crate::offsets::offsets_rtti::RTTI_TESTexture;
use crate::offsets::offsets_vtable::VTABLE_TESTexture;
use crate::re::tes_texture::TESTexture;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::inherit;

#[repr(C)]
pub struct TESIcon {
    pub base: TESTexture,
}

const _: () = assert!(core::mem::size_of::<TESIcon>() == 0x10);

impl RttiType for TESIcon {
    const RTTI: VariantID = RTTI_TESTexture;
}

inherit!(TESIcon : TESTexture);

impl TESIcon {
    pub const RTTI: VariantID = RTTI_TESTexture;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESTexture;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    virtual_method! {
        pub const GET_DEFAULT_PATH: usize = 0x06;
        pub fn get_default_path() -> *const core::ffi::c_char
    }
}

pub trait TESIconExt {
    fn dtor(&mut self);
    fn get_default_path(&self) -> *const core::ffi::c_char;
}

impl<T: AsRef<TESIcon> + AsMut<TESIcon>> TESIconExt for T {
    fn dtor(&mut self) {
        TESIcon::dtor(self.as_mut())
    }

    fn get_default_path(&self) -> *const core::ffi::c_char {
        self.as_ref().get_default_path()
    }
}
