use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ReanimateEffect;
use crate::offsets::offsets_vtable::VTABLE_ReanimateEffect;
use crate::re::CommandEffect;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ReanimateEffect`
#[repr(C)]
pub struct ReanimateEffect {
    pub base: CommandEffect, // 00
    pub unk98: bool,         // 98
    pub pad99: u8,           // 99
    pub pad9a: u16,          // 9A
    pub pad9c: u32,          // 9C
}

const _: () = assert!(core::mem::size_of::<ReanimateEffect>() == 0xA0);
const _: () = assert!(core::mem::offset_of!(ReanimateEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ReanimateEffect, unk98) == 0x98);
const _: () = assert!(core::mem::offset_of!(ReanimateEffect, pad99) == 0x99);
const _: () = assert!(core::mem::offset_of!(ReanimateEffect, pad9a) == 0x9A);
const _: () = assert!(core::mem::offset_of!(ReanimateEffect, pad9c) == 0x9C);

impl RttiType for ReanimateEffect {
    const RTTI: VariantID = RTTI_ReanimateEffect;
}

inherit!(ReanimateEffect : CommandEffect);

impl ReanimateEffect {
    pub const RTTI: VariantID = RTTI_ReanimateEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ReanimateEffect;

    // override (CommandEffect)
    // void Update(float) override;                    // 04
    // void SaveGame(BGSSaveFormBuffer*) override;     // 08
    // void LoadGame(BGSLoadFormBuffer*) override;     // 09
    // bool ShouldDispelOnDeath() const override;      // 10
    // ~ReanimateEffect() override;                    // 13
    // void Start() override;                          // 14
    // void Finish() override;                         // 15
}
