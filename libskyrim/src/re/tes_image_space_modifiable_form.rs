use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_TESImageSpaceModifiableForm;
use crate::offsets::offsets_vtable::VTABLE_TESImageSpaceModifiableForm;
use crate::re::BaseFormComponent;
use crate::re::TESImageSpaceModifier;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct TESImageSpaceModifiableForm {
    pub base: BaseFormComponent,                           // 0x00
    pub image_space_modifying: *mut TESImageSpaceModifier, // 0x08 - MNAM
}

const _: () = assert!(core::mem::size_of::<TESImageSpaceModifiableForm>() == 0x10);

impl RttiType for TESImageSpaceModifiableForm {
    const RTTI: VariantID = RTTI_TESImageSpaceModifiableForm;
}

inherit!(TESImageSpaceModifiableForm : BaseFormComponent);

impl TESImageSpaceModifiableForm {
    pub const RTTI: VariantID = RTTI_TESImageSpaceModifiableForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESImageSpaceModifiableForm;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03
}
