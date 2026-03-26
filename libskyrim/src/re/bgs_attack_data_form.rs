use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSAttackDataForm;
use crate::offsets::offsets_vtable::VTABLE_BGSAttackDataForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::bgs_attack_data_map::BGSAttackDataMap;
use crate::re::ni_smart_pointer::NiPointer;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct BGSAttackDataForm {
    pub base: BaseFormComponent,                      // 0x00
    pub attack_data_map: NiPointer<BGSAttackDataMap>, // 0x08
}

const _: () = assert!(core::mem::size_of::<BGSAttackDataForm>() == 0x10);

impl RttiType for BGSAttackDataForm {
    const RTTI: VariantID = RTTI_BGSAttackDataForm;
}

inherit!(BGSAttackDataForm : BaseFormComponent);

impl BGSAttackDataForm {
    pub const RTTI: VariantID = RTTI_BGSAttackDataForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSAttackDataForm;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03
}
