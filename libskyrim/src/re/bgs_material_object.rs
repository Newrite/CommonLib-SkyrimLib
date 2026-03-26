use crate::offsets::offsets_rtti::RTTI_BGSMaterialObject;
use crate::offsets::offsets_vtable::VTABLE_BGSMaterialObject;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    /// Pointer-compatible partial translation of `RE::BGSMaterialObject`.
    pub type BGSMaterialObject;
}

impl RttiType for BGSMaterialObject {
    const RTTI: VariantID = RTTI_BGSMaterialObject;
}

impl FormCastable for BGSMaterialObject {
    const TARGET_FORM_TYPE: FormType = FormType::MaterialObject;
}

impl BGSMaterialObject {
    pub const RTTI: VariantID = RTTI_BGSMaterialObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSMaterialObject;
    pub const FORMTYPE: FormType = FormType::MaterialObject;
}
