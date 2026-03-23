use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSBehaviorGraphModel;
use crate::offsets::offsets_vtable::VTABLE_BGSBehaviorGraphModel;
use crate::re::tes_model::TESModel;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct BGSBehaviorGraphModel {
    pub base: TESModel,  // 0x00
}

const _: () = assert!(core::mem::size_of::<BGSBehaviorGraphModel>() == 0x28);

impl RttiType for BGSBehaviorGraphModel {
    const RTTI: VariantID = RTTI_BGSBehaviorGraphModel;
}

inherit!(BGSBehaviorGraphModel : TESModel);

impl BGSBehaviorGraphModel {
    pub const RTTI: VariantID = RTTI_BGSBehaviorGraphModel;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSBehaviorGraphModel;

    // override (TESModel)
    // void SetModel(const char* a_model) override;  // 05
}
