use core::ffi::c_void;

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_GrabActorEffect;
use crate::offsets::offsets_vtable::VTABLE_GrabActorEffect;
use crate::re::{ActorHandle, ValueModifierEffect};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::GrabActorEffect`
#[repr(C)]
pub struct GrabActorEffect {
    pub base: ValueModifierEffect, // 00
    // TODO: replace this raw pointer stand-in with the real smart-pointer type once
    // CommonLibVR source identifies the pointee and ownership contract behind
    // `GrabActorEffect::unk98`'s header-only `void* // smart ptr` comment; intended
    // end state: the exact source-backed smart-pointer wrapper at offset 0x98.
    pub unk98: *mut c_void,         // 98 - smart ptr
    pub grabbed_actor: ActorHandle, // A0
    pub unk_a4: u32,                // A4
    pub unk_a8: bool,               // A8
    pub grabbed: bool,              // A9
    pub unk_aa: u16,                // AA
    pub unk_ac: u32,                // AC
}

const _: () = assert!(core::mem::size_of::<GrabActorEffect>() == 0xB0);
const _: () = assert!(core::mem::offset_of!(GrabActorEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(GrabActorEffect, unk98) == 0x98);
const _: () = assert!(core::mem::offset_of!(GrabActorEffect, grabbed_actor) == 0xA0);
const _: () = assert!(core::mem::offset_of!(GrabActorEffect, unk_a4) == 0xA4);
const _: () = assert!(core::mem::offset_of!(GrabActorEffect, unk_a8) == 0xA8);
const _: () = assert!(core::mem::offset_of!(GrabActorEffect, grabbed) == 0xA9);
const _: () = assert!(core::mem::offset_of!(GrabActorEffect, unk_aa) == 0xAA);
const _: () = assert!(core::mem::offset_of!(GrabActorEffect, unk_ac) == 0xAC);

impl RttiType for GrabActorEffect {
    const RTTI: VariantID = RTTI_GrabActorEffect;
}

inherit!(GrabActorEffect : ValueModifierEffect);

impl GrabActorEffect {
    pub const RTTI: VariantID = RTTI_GrabActorEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_GrabActorEffect;

    // override (ValueModifierEffect)
    // void Update(float) override;                           // 04
    // void SaveGame(BGSSaveFormBuffer*) override;            // 08
    // ~GrabActorEffect() override;                           // 13
    // void Start() override;                                 // 14
    // void Finish() override;                                // 15
    // void ModifyActorValue(Actor*, float, ActorValue) override; // 20
}
