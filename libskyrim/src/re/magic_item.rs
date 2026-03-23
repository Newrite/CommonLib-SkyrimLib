use crate::offsets::offsets_rtti::RTTI_MagicItem;
use crate::re::actor::Actor;
use crate::relocation::{RelocationID, RttiType, VariantID};

core_util::abstract_type! {
    pub type MagicItem;
}

impl RttiType for MagicItem {
    const RTTI: VariantID = RTTI_MagicItem;
}

impl MagicItem {
    // RELOCATION_ID SE: 11213, AE: 11321
    crate::relocation_func! {
        fn calculate_cost(&self, caster: *mut Actor) -> f32 => RelocationID::new(11213, 11321)
    }

    #[inline]
    pub fn calculate_magicka_cost(&self, caster: *mut Actor) -> f32 {
        self.calculate_cost(caster)
    }
}
