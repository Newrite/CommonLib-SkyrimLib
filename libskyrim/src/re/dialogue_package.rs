#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_DialoguePackage;
use crate::offsets::offsets_vtable::VTABLE_DialoguePackage;
use crate::re::{FormCastable, FormType};
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type DialoguePackage; }

impl RttiType for DialoguePackage {
    const RTTI: VariantID = RTTI_DialoguePackage;
}

impl FormCastable for DialoguePackage {
    const TARGET_FORM_TYPE: FormType = FormType::Package;
}

impl DialoguePackage {
    pub const RTTI: VariantID = RTTI_DialoguePackage;
    pub const VTABLE: &'static [VariantID] = &VTABLE_DialoguePackage;
    pub const FORMTYPE: FormType = FormType::Package;
}
