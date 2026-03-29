use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_EtherealizationEffect;
use crate::offsets::offsets_vtable::VTABLE_EtherealizationEffect;
use crate::re::ActiveEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::EtherealizationEffect`
#[repr(C)]
pub struct EtherealizationEffect {
    pub base: ActiveEffect, // 00
}

const _: () = assert!(core::mem::size_of::<EtherealizationEffect>() == 0x90);
const _: () = assert!(core::mem::offset_of!(EtherealizationEffect, base) == 0x00);

impl RttiType for EtherealizationEffect {
    const RTTI: VariantID = RTTI_EtherealizationEffect;
}

inherit!(EtherealizationEffect : ActiveEffect);

impl EtherealizationEffect {
    pub const RTTI: VariantID = RTTI_EtherealizationEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_EtherealizationEffect;

    // override (ActiveEffect)
    // ~EtherealizationEffect() override;  // 13
    // void Start() override;              // 14
    // void Finish() override;             // 15
}
