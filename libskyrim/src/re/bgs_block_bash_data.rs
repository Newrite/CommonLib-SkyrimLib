use core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_BGSBlockBashData;
use crate::offsets::offsets_vtable::VTABLE_BGSBlockBashData;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::bgs_impact_data_set::BGSImpactDataSet;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

// TODO: create a stub for BGSMaterialType
#[repr(C)]
pub struct BGSBlockBashData {
    pub base: BaseFormComponent,
    pub block_bash_impact_data_set: *mut BGSImpactDataSet, // 08
    pub alt_block_material_type: *mut core::ffi::c_void, // 10
}

const _: () = assert!(core::mem::size_of::<BGSBlockBashData>() == 0x18);

impl RttiType for BGSBlockBashData {
    const RTTI: VariantID = RTTI_BGSBlockBashData;
}

inherit!(BGSBlockBashData : BaseFormComponent);

impl BGSBlockBashData {
    pub const RTTI: VariantID = RTTI_BGSBlockBashData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSBlockBashData;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01 - { impact = 0; material = 0; }
    // void ClearDataComponent() override;                     // 02 - { return; }
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03
}