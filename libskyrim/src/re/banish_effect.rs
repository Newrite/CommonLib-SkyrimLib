use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BanishEffect;
use crate::offsets::offsets_vtable::VTABLE_BanishEffect;
use crate::re::DemoralizeEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BanishEffect`
#[repr(C)]
pub struct BanishEffect {
    pub base: DemoralizeEffect, // 00
}

const _: () = assert!(core::mem::size_of::<BanishEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(BanishEffect, base) == 0x00);

impl RttiType for BanishEffect {
    const RTTI: VariantID = RTTI_BanishEffect;
}

inherit!(BanishEffect : DemoralizeEffect);

impl BanishEffect {
    pub const RTTI: VariantID = RTTI_BanishEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BanishEffect;

    // override (ActiveEffect)
    // 13 ~BanishEffect
    // 14 Start
}
