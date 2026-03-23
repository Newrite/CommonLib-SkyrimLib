use crate::offsets::offsets_rtti::RTTI_TESWeightForm;
use crate::offsets::offsets_vtable::VTABLE_TESWeightForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{RttiType, VariantID};
use core_util::inherit;

/// C++ `RE::TESWeightForm`
#[repr(C)]
pub struct TESWeightForm {
    pub base: BaseFormComponent, // 00
    pub weight: f32,             // 08
    pub pad0c: u32,              // 0C
}

const _: () = assert!(core::mem::size_of::<TESWeightForm>() == 0x10);

impl RttiType for TESWeightForm {
    const RTTI: VariantID = RTTI_TESWeightForm;
}

impl TESWeightForm {
    pub const RTTI: VariantID = RTTI_TESWeightForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESWeightForm;
}

inherit!(TESWeightForm : BaseFormComponent);
