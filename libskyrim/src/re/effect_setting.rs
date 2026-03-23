use crate::offsets::offsets_rtti::RTTI_EffectSetting;
use crate::offsets::offsets_vtable::VTABLE_EffectSetting;
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! {
    pub type EffectSetting;
}

impl RttiType for EffectSetting {
    const RTTI: VariantID = RTTI_EffectSetting;
}

impl EffectSetting {
    pub const RTTI: VariantID = RTTI_EffectSetting;
    pub const VTABLE: &'static [VariantID] = &VTABLE_EffectSetting;
    pub const FORMTYPE: crate::re::FormType = crate::re::FormType::MagicEffect;
}
