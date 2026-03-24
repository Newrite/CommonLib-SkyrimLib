use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSSkinForm;
use crate::offsets::offsets_vtable::VTABLE_BGSSkinForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::tes_object_armo::TESObjectARMO;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct BGSSkinForm {
    pub base: BaseFormComponent,  // 0x00
    pub skin: *mut TESObjectARMO, // 0x08 - WNAM
}

const _: () = assert!(core::mem::size_of::<BGSSkinForm>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSSkinForm, skin) == 0x08);

impl RttiType for BGSSkinForm {
    const RTTI: VariantID = RTTI_BGSSkinForm;
}

inherit!(BGSSkinForm : BaseFormComponent);

impl BGSSkinForm {
    pub const RTTI: VariantID = RTTI_BGSSkinForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSSkinForm;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03
}
