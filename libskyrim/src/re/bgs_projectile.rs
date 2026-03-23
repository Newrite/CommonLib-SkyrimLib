use crate::offsets::offsets_rtti::RTTI_BGSProjectile;
use crate::offsets::offsets_vtable::VTABLE_BGSProjectile;
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! {
    pub type BGSProjectile;
}

impl RttiType for BGSProjectile {
    const RTTI: VariantID = RTTI_BGSProjectile;
}

impl BGSProjectile {
    pub const RTTI: VariantID = RTTI_BGSProjectile;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSProjectile;
}
