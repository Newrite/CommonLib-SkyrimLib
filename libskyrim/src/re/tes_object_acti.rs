use crate::offsets::offsets_rtti::RTTI_TESObjectACTI;
use crate::offsets::offsets_vtable::VTABLE_TESObjectACTI;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type TESObjectACTI; }

impl RttiType for TESObjectACTI {
    const RTTI: VariantID = RTTI_TESObjectACTI;
}

impl FormCastable for TESObjectACTI {
    const TARGET_FORM_TYPE: FormType = FormType::Activator;
}

impl TESObjectACTI {
    pub const RTTI: VariantID = RTTI_TESObjectACTI;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESObjectACTI;
    pub const FORMTYPE: FormType = FormType::Activator;
}
