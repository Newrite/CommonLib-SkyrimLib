use crate::offsets::offsets_rtti::RTTI_BSBound;
use crate::offsets::offsets_vtable::VTABLE_BSBound;
use crate::re::{NiExtraData, NiPoint3};
use crate::relocation::{RttiType, VariantID};
use core_util::inherit;

/// Source-backed layout subset of `RE::BSBound`
#[repr(C)]
pub struct BSBound {
    pub base: NiExtraData, // 00
    pub center: NiPoint3,  // 18
    pub extents: NiPoint3, // 24
}

const _: () = assert!(core::mem::size_of::<BSBound>() == 0x30);

impl RttiType for BSBound {
    const RTTI: VariantID = RTTI_BSBound;
}

inherit!(BSBound : NiExtraData);

impl BSBound {
    pub const RTTI: VariantID = RTTI_BSBound;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSBound;
}
