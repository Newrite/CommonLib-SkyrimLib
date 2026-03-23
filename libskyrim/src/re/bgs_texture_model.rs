use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSTextureModel;
use crate::offsets::offsets_vtable::VTABLE_BGSTextureModel;
use crate::re::tes_model::TESModel;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct BGSTextureModel {
    pub base: TESModel,  // 0x00
}

const _: () = assert!(core::mem::size_of::<BGSTextureModel>() == 0x28);

impl RttiType for BGSTextureModel {
    const RTTI: VariantID = RTTI_BGSTextureModel;
}

inherit!(BGSTextureModel : TESModel);

impl BGSTextureModel {
    pub const RTTI: VariantID = RTTI_BGSTextureModel;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSTextureModel;

    // override (TESModel)
    // void SetModel(const char* a_model) override;  // 05
}
