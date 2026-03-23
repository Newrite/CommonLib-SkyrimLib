use crate::offsets::offsets_rtti::RTTI_BGSMessageIcon;
use crate::offsets::offsets_vtable::VTABLE_BGSMessageIcon;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::tes_icon::TESIcon;
use crate::relocation::{RttiType, VariantID};
use core_util::inherit;

#[repr(C)]
pub struct BGSMessageIcon {
    pub base: BaseFormComponent, // 00
    pub icon: TESIcon,           // 08
}

const _: () = assert!(core::mem::size_of::<BGSMessageIcon>() == 0x18);

impl RttiType for BGSMessageIcon {
    const RTTI: VariantID = RTTI_BGSMessageIcon;
}

inherit!(BGSMessageIcon : BaseFormComponent);

impl BGSMessageIcon {
    pub const RTTI: VariantID = RTTI_BGSMessageIcon;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSMessageIcon;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03
}
