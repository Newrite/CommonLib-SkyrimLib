use core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_TESDescription;
use crate::offsets::offsets_vtable::VTABLE_TESDescription;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::bgs_localized_string_dl::BGSLocalizedStringDL;
use crate::relocation::{RttiType, VariantID};
use crate::re::bs_string::BSString;
use crate::re::tes_form::TESForm;

#[repr(C)]
pub struct TESDescription {
    pub base: BaseFormComponent,         // 0x00
    pub file_offset: u32,                // 0x08
    pub description_text: BGSLocalizedStringDL,  // 0x0C
}

const _: () = assert!(core::mem::size_of::<TESDescription>() == 0x10);

impl RttiType for TESDescription {
    const RTTI: VariantID = RTTI_TESDescription;
}

inherit!(TESDescription : BaseFormComponent);

impl TESDescription {
    pub const RTTI: VariantID = RTTI_TESDescription;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESDescription;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03

    // RELOCATION_ID SE: 14399, AE: 14552
    // TODO: VERIFY - VR ID unknown
    crate::relocation_func! {
        pub fn get_description(this: &TESDescription, out: &mut BSString, parent: *mut TESForm, field_type: u32) => VariantID::new(14399, 14552, 0)
    }
}
