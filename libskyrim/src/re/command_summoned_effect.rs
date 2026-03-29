use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_CommandSummonedEffect;
use crate::offsets::offsets_vtable::VTABLE_CommandSummonedEffect;
use crate::re::ActiveEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::CommandSummonedEffect`
#[repr(C)]
pub struct CommandSummonedEffect {
    pub base: ActiveEffect, // 00
}

const _: () = assert!(core::mem::size_of::<CommandSummonedEffect>() == 0x90);
const _: () = assert!(core::mem::offset_of!(CommandSummonedEffect, base) == 0x00);

impl RttiType for CommandSummonedEffect {
    const RTTI: VariantID = RTTI_CommandSummonedEffect;
}

inherit!(CommandSummonedEffect : ActiveEffect);

impl CommandSummonedEffect {
    pub const RTTI: VariantID = RTTI_CommandSummonedEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_CommandSummonedEffect;

    // override (ActiveEffect)
    // ~CommandSummonedEffect() override;  // 13
    // void Start() override;              // 14
}
