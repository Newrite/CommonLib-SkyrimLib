use crate::offsets::offsets_rtti::RTTI_BGSListForm;
use crate::offsets::offsets_vtable::VTABLE_BGSListForm;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::relocation::{RttiType, VariantID};

// AUTO-STUB: Full translation pending
// TODO: VERIFY - replace with full translation when layout is needed
core_util::abstract_type! { pub type BGSListForm; }

impl RttiType for BGSListForm {
    const RTTI: VariantID = RTTI_BGSListForm;
}

impl FormCastable for BGSListForm {
    const TARGET_FORM_TYPE: FormType = FormType::FormList;
}

impl BGSListForm {
    pub const RTTI: VariantID = RTTI_BGSListForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSListForm;
    pub const FORMTYPE: FormType = FormType::FormList;
}
