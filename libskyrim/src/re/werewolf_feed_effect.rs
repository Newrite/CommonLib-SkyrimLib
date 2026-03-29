use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_WerewolfFeedEffect;
use crate::offsets::offsets_vtable::VTABLE_WerewolfFeedEffect;
use crate::re::ActiveEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::WerewolfFeedEffect`
#[repr(C)]
pub struct WerewolfFeedEffect {
    pub base: ActiveEffect, // 00
}

const _: () = assert!(core::mem::size_of::<WerewolfFeedEffect>() == 0x90);
const _: () = assert!(core::mem::offset_of!(WerewolfFeedEffect, base) == 0x00);

impl RttiType for WerewolfFeedEffect {
    const RTTI: VariantID = RTTI_WerewolfFeedEffect;
}

inherit!(WerewolfFeedEffect : ActiveEffect);

impl WerewolfFeedEffect {
    pub const RTTI: VariantID = RTTI_WerewolfFeedEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_WerewolfFeedEffect;

    // override (ActiveEffect)
    // ~WerewolfFeedEffect() override;  // 13
    // void Start() override;           // 14
}
