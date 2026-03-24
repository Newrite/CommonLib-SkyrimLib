use crate::offsets::offsets_nirtti::NiRTTI_ShaderReferenceEffect;
use crate::offsets::offsets_rtti::RTTI_ShaderReferenceEffect;
use crate::offsets::offsets_vtable::VTABLE_ShaderReferenceEffect;
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! { pub type ShaderReferenceEffect; }

impl RttiType for ShaderReferenceEffect {
    const RTTI: VariantID = RTTI_ShaderReferenceEffect;
}

impl ShaderReferenceEffect {
    pub const RTTI: VariantID = RTTI_ShaderReferenceEffect;
    pub const NI_RTTI: VariantID = NiRTTI_ShaderReferenceEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ShaderReferenceEffect;
}
