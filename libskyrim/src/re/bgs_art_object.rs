use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_BGSArtObject;
use crate::offsets::offsets_vtable::VTABLE_BGSArtObject;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_bound_object::TESBoundObject;
use crate::re::tes_model_texture_swap::TESModelTextureSwap;
use crate::relocation::{RttiType, VariantID};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSArtType {
    MagicCastingArt = 0,
    MagicHitEffect = 1,
    MagicEnchantEffect = 2,
}

core_util::impl_enumset_type!(BGSArtType => u32);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSArtObjectRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(C)]
pub struct BGSArtObjectData {
    pub art_type: EnumSet<BGSArtType, u32>, // 0x00
}

const _: () = assert!(core::mem::size_of::<BGSArtObjectData>() == 0x4);
const _: () = assert!(core::mem::offset_of!(BGSArtObjectData, art_type) == 0x00);

/// C++ `RE::BGSArtObject`
#[repr(C)]
pub struct BGSArtObject {
    pub base: TESBoundObject,                    // 0x00
    pub model_texture_swap: TESModelTextureSwap, // 0x30
    pub data: BGSArtObjectData,                  // 0x68 - DNAM
    pub pad6c: u32,                              // 0x6C
}

const _: () = assert!(core::mem::size_of::<BGSArtObject>() == 0x70);
const _: () = assert!(core::mem::offset_of!(BGSArtObject, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSArtObject, model_texture_swap) == 0x30);
const _: () = assert!(core::mem::offset_of!(BGSArtObject, data) == 0x68);
const _: () = assert!(core::mem::offset_of!(BGSArtObject, pad6c) == 0x6C);

impl RttiType for BGSArtObject {
    const RTTI: VariantID = RTTI_BGSArtObject;
}

impl FormCastable for BGSArtObject {
    const TARGET_FORM_TYPE: FormType = FormType::ArtObject;
}

inherit!(BGSArtObject : TESBoundObject);
inherit!(BGSArtObject => TESModelTextureSwap, model_texture_swap);

impl BGSArtObject {
    pub const RTTI: VariantID = RTTI_BGSArtObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSArtObject;
    pub const FORMTYPE: FormType = FormType::ArtObject;

    // override (TESBoundObject)
    // bool Load(TESFile* a_mod) override;  // 06
    // void InitItemImpl() override;        // 13
}
