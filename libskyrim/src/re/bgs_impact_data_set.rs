use bitflags::bitflags;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSImpactDataSet;
use crate::offsets::offsets_vtable::VTABLE_BGSImpactDataSet;
use crate::re::bgs_impact_data::BGSImpactData;
use crate::re::bgs_material_type::BGSMaterialType;
use crate::re::bgs_preloadable::BGSPreloadable;
use crate::re::bst_hash_map::BSTHashMap;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_form::TESForm;
use crate::relocation::{RttiType, VariantID};

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSImpactDataSetRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::BGSImpactDataSet`
#[repr(C)]
pub struct BGSImpactDataSet {
    pub base: TESForm,                                                      // 0x00
    pub preloadable: BGSPreloadable,                                        // 0x20
    pub impact_map: BSTHashMap<*const BGSMaterialType, *mut BGSImpactData>, // 0x28 - PNAM
}

const _: () =
    assert!(core::mem::size_of::<BSTHashMap<*const BGSMaterialType, *mut BGSImpactData>>() == 0x30);
const _: () = assert!(core::mem::size_of::<BGSImpactDataSet>() == 0x58);
const _: () = assert!(core::mem::offset_of!(BGSImpactDataSet, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSImpactDataSet, preloadable) == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSImpactDataSet, impact_map) == 0x28);

impl RttiType for BGSImpactDataSet {
    const RTTI: VariantID = RTTI_BGSImpactDataSet;
}

impl FormCastable for BGSImpactDataSet {
    const TARGET_FORM_TYPE: FormType = FormType::ImpactDataSet;
}

inherit!(BGSImpactDataSet : TESForm);
inherit!(BGSImpactDataSet => BGSPreloadable, preloadable);

impl BGSImpactDataSet {
    pub const RTTI: VariantID = RTTI_BGSImpactDataSet;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSImpactDataSet;
    pub const FORMTYPE: FormType = FormType::ImpactDataSet;

    // override (TESForm)
    // void ClearData() override;           // 05
    // bool Load(TESFile* a_mod) override;  // 06
    // void InitItemImpl() override;        // 13
}
