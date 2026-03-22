use crate::offsets::offsets_rtti::RTTI_TESRaceForm;
use crate::offsets::offsets_vtable::VTABLE_TESRaceForm;

use crate::re::BaseFormComponent;
use crate::re::TESRace;

#[repr(C)]
pub struct TESRaceForm {
    pub base: BaseFormComponent, // 00
    pub race: *mut TESRace,      // 08 - RNAM
}
const _: () = assert!(core::mem::size_of::<TESRaceForm>() == 0x10);

impl crate::relocation::RttiType for TESRaceForm { const RTTI: crate::relocation::VariantID = RTTI_TESRaceForm; }

impl TESRaceForm {
    pub const RTTI: crate::relocation::VariantID = RTTI_TESRaceForm;
    pub const VTABLE: &'static [crate::relocation::VariantID] = &VTABLE_TESRaceForm;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01 - { race = 0; }
    // void ClearDataComponent() override;                     // 02 - { return; }
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03
}
