use core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_BGSDestructibleObjectForm;
use crate::offsets::offsets_vtable::VTABLE_BGSDestructibleObjectForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct BGSDestructibleObjectForm {
    pub base: BaseFormComponent, // 00
    pub data: *mut core::ffi::c_void, // 08 - DestructibleObjectData*
}

const _: () = assert!(core::mem::size_of::<BGSDestructibleObjectForm>() == 0x10);

impl RttiType for BGSDestructibleObjectForm {
    const RTTI: VariantID = RTTI_BGSDestructibleObjectForm;
}

inherit!(BGSDestructibleObjectForm : BaseFormComponent);

impl BGSDestructibleObjectForm {
    pub const RTTI: VariantID = RTTI_BGSDestructibleObjectForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSDestructibleObjectForm;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03
}
