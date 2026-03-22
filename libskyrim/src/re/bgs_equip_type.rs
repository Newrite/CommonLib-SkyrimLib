use core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_BGSEquipType;
use crate::offsets::offsets_vtable::VTABLE_BGSEquipType;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct BGSEquipType {
    pub base: BaseFormComponent, // 00
    pub equip_slot: *mut core::ffi::c_void, // 08 - BGSEquipSlot*
}

const _: () = assert!(core::mem::size_of::<BGSEquipType>() == 0x10);

impl RttiType for BGSEquipType {
    const RTTI: VariantID = RTTI_BGSEquipType;
}

inherit!(BGSEquipType : BaseFormComponent);

impl BGSEquipType {
    pub const RTTI: VariantID = RTTI_BGSEquipType;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSEquipType;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03
}
