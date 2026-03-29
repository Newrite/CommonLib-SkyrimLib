use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_LightEffect;
use crate::offsets::offsets_vtable::VTABLE_LightEffect;
use crate::re::{ActiveEffect, NiPointLight, NiPointer};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::LightEffect`
#[repr(C)]
pub struct LightEffect {
    pub base: ActiveEffect,             // 00
    pub light: NiPointer<NiPointLight>, // 90
}

const _: () = assert!(core::mem::size_of::<LightEffect>() == 0x98);
const _: () = assert!(core::mem::offset_of!(LightEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(LightEffect, light) == 0x90);

impl RttiType for LightEffect {
    const RTTI: VariantID = RTTI_LightEffect;
}

inherit!(LightEffect : ActiveEffect);

impl LightEffect {
    pub const RTTI: VariantID = RTTI_LightEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_LightEffect;

    // override (ActiveEffect)
    // void Update(float) override;                           // 04
    // void FinishLoadGame(BGSLoadFormBuffer*) override;     // 0A
    // void Revert(BGSLoadFormBuffer*) override;             // 0B
    // void SwitchAttachedRoot(NiNode*, NiNode*) override;   // 0E
    // ~LightEffect() override;                              // 13
    // void Start() override;                                // 14
    // void Finish() override;                               // 15
}
