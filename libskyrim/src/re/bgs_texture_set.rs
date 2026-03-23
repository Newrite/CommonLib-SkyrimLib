use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_BGSTextureSet;
use crate::offsets::offsets_vtable::VTABLE_BGSTextureSet;
use crate::re::BSResourceID;
use crate::re::BSTEXTURE_SET_USED_TOTAL;
use crate::re::BSTextureSet;
use crate::re::DecalData;
use crate::re::FormCastable;
use crate::re::FormType;
use crate::re::TESBoundObject;
use crate::re::TESTexture;
use crate::relocation::{RttiType, VariantID};
use core_util::EnumSet;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSTextureSetFlag {
    None = 0,
    NoSpecularMap = 1 << 0,
    FacegenTextures = 1 << 1,
    HasModelSpaceNormalMap = 1 << 2,
}

core_util::impl_enumset_type!(BGSTextureSetFlag => u16);

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TextureSetRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(C)]
pub struct BGSTextureSet {
    pub base: TESBoundObject,                                       // 0x000
    pub texture_set: BSTextureSet,                                  // 0x030
    pub textures: [TESTexture; BSTEXTURE_SET_USED_TOTAL],           // 0x040
    pub decal_data: *mut DecalData,                                 // 0x0C0
    pub flags: EnumSet<BGSTextureSetFlag, u16>,                     // 0x0C8
    pub pad0ca: u16,                                                // 0x0CA
    pub texture_file_ids: [BSResourceID; BSTEXTURE_SET_USED_TOTAL], // 0x0CC
    pub pad12c: u32,                                                // 0x12C
}

const _: () = assert!(core::mem::size_of::<BGSTextureSet>() == 0x130);
const _: () = assert!(core::mem::offset_of!(BGSTextureSet, texture_set) == 0x30);

impl RttiType for BGSTextureSet {
    const RTTI: VariantID = RTTI_BGSTextureSet;
}

impl FormCastable for BGSTextureSet {
    const TARGET_FORM_TYPE: FormType = FormType::TextureSet;
}

inherit!(BGSTextureSet : TESBoundObject);
inherit!(BGSTextureSet => BSTextureSet, texture_set);

impl BGSTextureSet {
    pub const RTTI: VariantID = RTTI_BGSTextureSet;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSTextureSet;
    pub const FORMTYPE: FormType = FormType::TextureSet;

    // override (TESBoundObject)
    // void        InitializeData() override;                            // 04
    // void        ClearData() override;                                 // 05
    // bool        Load(TESFile* a_mod) override;                        // 06
    // NiAVObject* Clone3D(TESObjectREFR* a_ref, bool a_arg3) override;  // 40
    // void        UnClone3D(TESObjectREFR* a_ref) override;             // 41
}
