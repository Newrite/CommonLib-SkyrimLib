#![allow(non_camel_case_types)]

use core_util::EnumSet;

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_hkpCharacterContext;
use crate::offsets::offsets_vtable::VTABLE_hkpCharacterContext;
use crate::re::{hkReferencedObject, hkpCharacterStateType};
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type hkpCharacterStateManager;
}

/// C++ `RE::hkpCharacterContext::CharacterType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpCharacterContextCharacterType {
    Proxy = 0,
    RigidBody = 1,
}

core_util::impl_enumset_type!(hkpCharacterContextCharacterType => u32);

/// C++ `RE::hkpCharacterContext`
#[repr(C)]
pub struct hkpCharacterContext {
    pub base: hkReferencedObject,                                       // 00
    pub character_type: EnumSet<hkpCharacterContextCharacterType, u32>, // 10
    pub pad14: u32,                                                     // 14
    pub state_manager: *const hkpCharacterStateManager,                 // 18
    pub current_state: hkpCharacterStateType,                           // 20
    pub previous_state: hkpCharacterStateType,                          // 24
    pub filter_enable: bool,                                            // 28
    pub pad29: u8,                                                      // 29
    pub pad2a: u16,                                                     // 2A
    pub max_linear_acceleration: f32,                                   // 2C
    pub max_linear_velocity: f32,                                       // 30
    pub gain: f32,                                                      // 34
}

const _: () = assert!(core::mem::size_of::<hkpCharacterContext>() == 0x38);
const _: () = assert!(core::mem::offset_of!(hkpCharacterContext, character_type) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkpCharacterContext, state_manager) == 0x18);
const _: () = assert!(core::mem::offset_of!(hkpCharacterContext, current_state) == 0x20);
const _: () = assert!(core::mem::offset_of!(hkpCharacterContext, previous_state) == 0x24);

impl RttiType for hkpCharacterContext {
    const RTTI: VariantID = RTTI_hkpCharacterContext;
}

inherit!(hkpCharacterContext : hkReferencedObject);

impl hkpCharacterContext {
    pub const RTTI: VariantID = RTTI_hkpCharacterContext;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpCharacterContext;
}
