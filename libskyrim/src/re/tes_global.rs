use crate::offsets::offsets_rtti::RTTI_TESGlobal;
use crate::offsets::offsets_vtable::VTABLE_TESGlobal;
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! { pub type TESGlobal; }

impl RttiType for TESGlobal {
    const RTTI: VariantID = RTTI_TESGlobal;
}

impl TESGlobal {
    pub const RTTI: VariantID = RTTI_TESGlobal;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESGlobal;
}
