use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSLocationRefType;
use crate::offsets::offsets_vtable::VTABLE_BGSLocationRefType;
use crate::re::bgs_keyword::BGSKeyword;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BGSLocationRefType`
#[repr(C)]
pub struct BGSLocationRefType {
    pub base: BGSKeyword, // 00
}

const _: () = assert!(core::mem::size_of::<BGSLocationRefType>() == 0x28);

impl RttiType for BGSLocationRefType {
    const RTTI: VariantID = RTTI_BGSLocationRefType;
}

impl FormCastable for BGSLocationRefType {
    const TARGET_FORM_TYPE: FormType = FormType::LocationRefType;
}

inherit!(BGSLocationRefType : BGSKeyword);

impl BGSLocationRefType {
    pub const RTTI: VariantID = RTTI_BGSLocationRefType;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSLocationRefType;
    pub const FORMTYPE: FormType = FormType::LocationRefType;

    // override (BGSKeyword)
    // void InitItemImpl() override;  // 13
}
