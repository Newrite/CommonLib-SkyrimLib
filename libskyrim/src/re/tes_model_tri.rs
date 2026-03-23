use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESModelTri;
use crate::offsets::offsets_vtable::VTABLE_TESModelTri;
use crate::re::tes_model::TESModel;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct TESModelTri {
    pub base: TESModel, // 0x00
}

const _: () = assert!(core::mem::size_of::<TESModelTri>() == 0x28);

impl RttiType for TESModelTri {
    const RTTI: VariantID = RTTI_TESModelTri;
}

inherit!(TESModelTri : TESModel);

impl TESModelTri {
    pub const RTTI: VariantID = RTTI_TESModelTri;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESModelTri;

    // override (TESModel)
    // void SetModel(const char* a_model) override;  // 05
}
