use crate::offsets::offsets_rtti::RTTI_TESModelTextureSwap;
use crate::offsets::offsets_vtable::VTABLE_TESModelTextureSwap;
use crate::re::tes_model::TESModel;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::bgs_texture_set::BGSTextureSet;
use crate::relocation::{VariantID, RttiType};
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

impl RttiType for TESModelTextureSwap {
    const RTTI: VariantID = RTTI_TESModelTextureSwap;
}

impl TESModelTextureSwap {
    pub const RTTI: VariantID = RTTI_TESModelTextureSwap;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESModelTextureSwap;
}

inherit!(TESModelTextureSwap : TESModel);
