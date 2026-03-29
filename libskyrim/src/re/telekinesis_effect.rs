use core::ffi::c_void;

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TelekinesisEffect;
use crate::offsets::offsets_vtable::VTABLE_TelekinesisEffect;
use crate::re::{ActiveEffect, ObjectRefHandle};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::TelekinesisEffect::STATE`
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
// TODO: replace this transparent wrapper with the real enum once CommonLibVR
// source exposes the definition of `TelekinesisEffect::STATE`; the current
// header only forward-declares the nested type, so the intended end state is
// the exact source-backed enum with named variants.
pub struct TelekinesisEffectState(pub i32);

const _: () = assert!(core::mem::size_of::<TelekinesisEffectState>() == 0x4);

/// C++ `RE::TelekinesisEffect`
#[repr(C)]
pub struct TelekinesisEffect {
    pub base: ActiveEffect, // 00
    // TODO: replace this raw pointer stand-in with the real smart-pointer type once
    // CommonLibVR source identifies the pointee and ownership contract behind
    // `TelekinesisEffect::unk90`'s header-only `void* // smart ptr` comment;
    // intended end state: the exact source-backed smart-pointer wrapper at 0x90.
    pub unk90: *mut c_void,              // 90 - smart ptr
    pub move_velocity: f32,              // 98
    pub throw_velocity: f32,             // 9C
    pub grabbed_object: ObjectRefHandle, // A0
    pub state: TelekinesisEffectState,   // A4
    pub throw_object: bool,              // A8
    pub first_update: bool,              // A9
    pub unk_aa: u8,                      // AA
    pub unk_ab: u8,                      // AB
    pub unk_ac: u32,                     // AC
}

const _: () = assert!(core::mem::size_of::<TelekinesisEffect>() == 0xB0);
const _: () = assert!(core::mem::offset_of!(TelekinesisEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(TelekinesisEffect, unk90) == 0x90);
const _: () = assert!(core::mem::offset_of!(TelekinesisEffect, move_velocity) == 0x98);
const _: () = assert!(core::mem::offset_of!(TelekinesisEffect, throw_velocity) == 0x9C);
const _: () = assert!(core::mem::offset_of!(TelekinesisEffect, grabbed_object) == 0xA0);
const _: () = assert!(core::mem::offset_of!(TelekinesisEffect, state) == 0xA4);
const _: () = assert!(core::mem::offset_of!(TelekinesisEffect, throw_object) == 0xA8);
const _: () = assert!(core::mem::offset_of!(TelekinesisEffect, first_update) == 0xA9);

impl RttiType for TelekinesisEffect {
    const RTTI: VariantID = RTTI_TelekinesisEffect;
}

inherit!(TelekinesisEffect : ActiveEffect);

impl TelekinesisEffect {
    pub const RTTI: VariantID = RTTI_TelekinesisEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TelekinesisEffect;

    // override (ActiveEffect)
    // void Update(float) override;  // 04
    // ~TelekinesisEffect() override; // 13
    // void Start() override;        // 14
    // void Finish() override;       // 15
}
