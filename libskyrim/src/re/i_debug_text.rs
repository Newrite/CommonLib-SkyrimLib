use crate::offsets::offsets_rtti::RTTI_IDebugText;
use crate::offsets::offsets_vtable::VTABLE_IDebugText;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type IDebugText;
}

impl RttiType for IDebugText {
    const RTTI: VariantID = RTTI_IDebugText;
}

impl IDebugText {
    pub const RTTI: VariantID = RTTI_IDebugText;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IDebugText;
}

// TODO: SOURCE - `BSPathingRequest` currently uses `IDebugText` only as a raw
// callback sink pointer. Replace this opaque stand-in with the real interface
// when the debug-text callback surface is needed.
