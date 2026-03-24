use crate::offsets::offsets_nirtti::NiRTTI_ModelReferenceEffect;
use crate::offsets::offsets_rtti::RTTI_ModelReferenceEffect;
use crate::offsets::offsets_vtable::VTABLE_ModelReferenceEffect;
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! { pub type ModelReferenceEffect; }

impl RttiType for ModelReferenceEffect {
    const RTTI: VariantID = RTTI_ModelReferenceEffect;
}

impl ModelReferenceEffect {
    pub const RTTI: VariantID = RTTI_ModelReferenceEffect;
    pub const NI_RTTI: VariantID = NiRTTI_ModelReferenceEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ModelReferenceEffect;
}
