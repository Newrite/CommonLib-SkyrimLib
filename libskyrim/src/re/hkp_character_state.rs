#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_hkpCharacterState;
use crate::offsets::offsets_vtable::VTABLE_hkpCharacterState;
use crate::re::hkReferencedObject;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type hkpCharacterInput;
    pub type hkpCharacterOutput;
}

/// C++ `RE::hkpCharacterStateTypes::hkpCharacterStateType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpCharacterStateType {
    OnGround = 0,
    Jumping = 1,
    InAir = 2,
    Climbing = 3,
    Flying = 4,
    UserState0 = 5,
    UserState1 = 6,
    UserState2 = 7,
    UserState3 = 8,
    UserState4 = 9,
    UserState5 = 10,
    Total = 11,
}

impl hkpCharacterStateType {
    pub const SWIMMING: Self = Self::UserState0;
}

/// C++ `RE::hkpCharacterState`
#[repr(C)]
pub struct hkpCharacterState {
    pub base: hkReferencedObject, // 00
}

const _: () = assert!(core::mem::size_of::<hkpCharacterState>() == 0x10);

impl RttiType for hkpCharacterState {
    const RTTI: VariantID = RTTI_hkpCharacterState;
}

inherit!(hkpCharacterState : hkReferencedObject);

impl hkpCharacterState {
    pub const RTTI: VariantID = RTTI_hkpCharacterState;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpCharacterState;

    crate::virtual_method! {
        pub const VFUNC_GET_TYPE: usize = 0x03;
        pub fn get_type() -> hkpCharacterStateType
    }
}
