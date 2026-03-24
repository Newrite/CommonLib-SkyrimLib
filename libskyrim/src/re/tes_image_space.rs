use crate::offsets::offsets_rtti::RTTI_TESImageSpace;
use crate::offsets::offsets_vtable::VTABLE_TESImageSpace;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type TESImageSpace; }

impl RttiType for TESImageSpace {
    const RTTI: VariantID = RTTI_TESImageSpace;
}

impl FormCastable for TESImageSpace {
    const TARGET_FORM_TYPE: FormType = FormType::ImageSpace;
}

impl TESImageSpace {
    pub const RTTI: VariantID = RTTI_TESImageSpace;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESImageSpace;
    pub const FORMTYPE: FormType = FormType::ImageSpace;
}
