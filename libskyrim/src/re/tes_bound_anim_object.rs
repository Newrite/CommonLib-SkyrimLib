use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_TESBoundAnimObject;
use crate::offsets::offsets_vtable::VTABLE_TESBoundAnimObject;
use crate::re::TESBoundObject;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct TESBoundAnimObject {
    pub base: TESBoundObject, // 0x00
}

const _: () = assert!(core::mem::size_of::<TESBoundAnimObject>() == 0x30);

impl RttiType for TESBoundAnimObject {
    const RTTI: VariantID = RTTI_TESBoundAnimObject;
}

inherit!(TESBoundAnimObject : TESBoundObject);

impl TESBoundAnimObject {
    pub const RTTI: VariantID = RTTI_TESBoundAnimObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESBoundAnimObject;

    // override (TESBoundObject)
    // bool IsBoundAnimObject() override;              // 3C - { return true; }
    // bool ReplaceModel(const char* a_str) override;  // 4B
}
