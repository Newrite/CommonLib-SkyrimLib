use crate::offsets::offsets_rtti::RTTI_TESModelRDT;
use crate::offsets::offsets_vtable::VTABLE_TESModelRDT;

use crate::re::TESModel;

#[repr(C)]
pub struct TESModelRDT {
    pub base: TESModel, // 00
}
const _: () = assert!(core::mem::size_of::<TESModelRDT>() == 0x28);

impl crate::relocation::RttiType for TESModelRDT {
    const RTTI: crate::relocation::VariantID = RTTI_TESModelRDT;
}

impl TESModelRDT {
    pub const RTTI: crate::relocation::VariantID = RTTI_TESModelRDT;
    pub const VTABLE: &'static [crate::relocation::VariantID] = &VTABLE_TESModelRDT;

    // override (TESModel)
    // ~TESModelRDT() override; // 00
}
