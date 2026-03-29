use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TurnUndeadEffect;
use crate::offsets::offsets_vtable::VTABLE_TurnUndeadEffect;
use crate::re::DemoralizeEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::TurnUndeadEffect`
#[repr(C)]
pub struct TurnUndeadEffect {
    pub base: DemoralizeEffect, // 00
}

const _: () = assert!(core::mem::size_of::<TurnUndeadEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(TurnUndeadEffect, base) == 0x00);

impl RttiType for TurnUndeadEffect {
    const RTTI: VariantID = RTTI_TurnUndeadEffect;
}

inherit!(TurnUndeadEffect : DemoralizeEffect);

impl TurnUndeadEffect {
    pub const RTTI: VariantID = RTTI_TurnUndeadEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TurnUndeadEffect;

    // override (ActiveEffect)
    // ~TurnUndeadEffect() override;  // 13
}
