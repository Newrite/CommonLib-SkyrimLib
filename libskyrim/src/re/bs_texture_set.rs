use core::ffi::c_char;

use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_BSTextureSet;
use crate::offsets::offsets_rtti::RTTI_BSTextureSet;
use crate::offsets::offsets_vtable::VTABLE_BSTextureSet;
use crate::re::{NiObject, NiPointer, NiRef, NiRefObject, NiSourceTexture};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

pub const BSTEXTURE_SET_USED_TOTAL: usize = 8;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSTextureType {
    Diffuse = 0,
    Normal = 1,
    EnvironmentMask = 2,
    GlowMap = 3,
    Height = 4,
    Environment = 5,
    Multilayer = 6,
    BacklightMask = 7,
    Unused08 = 8,
    Total = 9,
}

#[repr(C)]
pub struct BSTextureSet {
    pub base: NiObject, // 0x00
}

const _: () = assert!(core::mem::size_of::<BSTextureSet>() == 0x10);

impl RttiType for BSTextureSet {
    const RTTI: VariantID = RTTI_BSTextureSet;
}

impl NiRef for BSTextureSet {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const _ as *const NiRefObject)).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const _ as *const NiRefObject)).dec_ref() }
    }
}

inherit!(BSTextureSet : NiObject);

impl BSTextureSet {
    pub const RTTI: VariantID = RTTI_BSTextureSet;
    pub const NI_RTTI: VariantID = NiRTTI_BSTextureSet;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSTextureSet;

    // override (NiObject)
    // const NiRTTI* GetRTTI() const override;                          // 02
    // void          LoadBinary(NiStream& a_stream) override;           // 18
    // void          LinkObject(NiStream& a_stream) override;           // 19
    // bool          RegisterStreamables(NiStream& a_stream) override;  // 1A
    // void          SaveBinary(NiStream& a_stream) override;           // 1B
    // bool          IsEqual(NiObject* a_object) override;              // 1C

    virtual_method! {
        pub const VFUNC_GET_TEXTURE_PATH: usize = 0x25;
        pub fn get_texture_path(a_texture: BSTextureType) -> *const c_char
    }

    virtual_method! {
        pub const VFUNC_SET_TEXTURE: usize = 0x26;
        pub fn set_texture(a_texture: BSTextureType, a_src_texture: &mut NiPointer<NiSourceTexture>)
    }

    virtual_method! {
        pub const VFUNC_SET_TEXTURE_PATH: usize = 0x27;
        pub fn set_texture_path(a_texture: BSTextureType, a_path: *const c_char)
    }

    #[inline(always)]
    pub fn get_texture_path_as_str(&self, a_texture: BSTextureType) -> &str {
        core_util::ptr_to_str(self.get_texture_path(a_texture))
    }
}

pub trait BSTextureSetExt {
    fn get_texture_path(&self, a_texture: BSTextureType) -> *const c_char;
    fn get_texture_path_as_str(&self, a_texture: BSTextureType) -> &str;
    fn set_texture(
        &mut self,
        a_texture: BSTextureType,
        a_src_texture: &mut NiPointer<NiSourceTexture>,
    );
    fn set_texture_path(&mut self, a_texture: BSTextureType, a_path: *const c_char);
}

impl<T: AsRef<BSTextureSet> + AsMut<BSTextureSet>> BSTextureSetExt for T {
    fn get_texture_path(&self, a_texture: BSTextureType) -> *const c_char {
        self.as_ref().get_texture_path(a_texture)
    }

    fn get_texture_path_as_str(&self, a_texture: BSTextureType) -> &str {
        self.as_ref().get_texture_path_as_str(a_texture)
    }

    fn set_texture(
        &mut self,
        a_texture: BSTextureType,
        a_src_texture: &mut NiPointer<NiSourceTexture>,
    ) {
        self.as_mut().set_texture(a_texture, a_src_texture)
    }

    fn set_texture_path(&mut self, a_texture: BSTextureType, a_path: *const c_char) {
        self.as_mut().set_texture_path(a_texture, a_path)
    }
}
