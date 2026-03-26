#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_BGSNote;
use crate::offsets::offsets_vtable::VTABLE_BGSNote;
use crate::re::{FormCastable, FormType};
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type BGSNote; }

impl RttiType for BGSNote {
    const RTTI: VariantID = RTTI_BGSNote;
}

impl FormCastable for BGSNote {
    const TARGET_FORM_TYPE: FormType = FormType::Note;
}

impl BGSNote {
    pub const RTTI: VariantID = RTTI_BGSNote;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSNote;
    pub const FORMTYPE: FormType = FormType::Note;
}
