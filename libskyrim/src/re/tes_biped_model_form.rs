use crate::offsets::offsets_rtti::RTTI_TESBipedModelForm;
use crate::offsets::offsets_vtable::VTABLE_TESBipedModelForm;

use crate::core_util::inherit;
use crate::re::BGSMessageIcon;
use crate::re::BaseFormComponent;
use crate::re::TESIcon;
use crate::re::TESModelRDT;
use crate::re::TESModelTextureSwap;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sexes {
    Male = 0,
    Female = 1,
    Total = 2,
}

#[repr(C)]
pub struct TESBipedModelForm {
    pub base: BaseFormComponent,                // 00
    pub world_models: [TESModelTextureSwap; 2], // 08
    pub inventory_icons: [TESIcon; 2],          // 78
    pub message_icons: [BGSMessageIcon; 2],     // 98
    pub constraint_template: TESModelRDT,       // C8
}
const _: () = assert!(core::mem::size_of::<TESBipedModelForm>() == 0xF0);

impl crate::relocation::RttiType for TESBipedModelForm {
    const RTTI: crate::relocation::VariantID = RTTI_TESBipedModelForm;
}

inherit!(TESBipedModelForm : BaseFormComponent);

impl TESBipedModelForm {
    pub const RTTI: crate::relocation::VariantID = RTTI_TESBipedModelForm;
    pub const VTABLE: &'static [crate::relocation::VariantID] = &VTABLE_TESBipedModelForm;

    // override (BaseFormComponent)
    // ~TESBipedModelForm() override;  // 00
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03
}
