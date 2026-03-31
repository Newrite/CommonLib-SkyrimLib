use crate::offsets::offsets_rtti::RTTI_IMovementParameters;
use crate::offsets::offsets_vtable::VTABLE_IMovementParameters;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type IMovementParameters;
}

impl RttiType for IMovementParameters {
    const RTTI: VariantID = RTTI_IMovementParameters;
}

impl IMovementParameters {
    pub const RTTI: VariantID = RTTI_IMovementParameters;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMovementParameters;
}

// TODO: SOURCE - `BSPathingRequest` currently needs `IMovementParameters` only
// as pointer-sized storage. Replace this opaque stand-in with the real
// interface once movement-parameter calls or smart-pointer ownership are needed.
