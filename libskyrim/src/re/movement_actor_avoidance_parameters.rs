use crate::core_util::EnumSet;
use crate::re::BSTSmallArray;

/// C++ `RE::MovementActorAvoidanceParameters::Type`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MovementActorAvoidanceParametersType {
    AllActors = 0x0,
    NoActor = 0x1,
    Unk2 = 0x2,
    IgnoreOneActor = 0x3,
}

core_util::impl_enumset_type!(MovementActorAvoidanceParametersType => u32);

impl MovementActorAvoidanceParametersType {
    pub const MASK: u32 = 0x3;
}

/// C++ `RE::MovementActorAvoidanceParameters`
#[repr(C)]
pub struct MovementActorAvoidanceParameters {
    pub actors: BSTSmallArray<u32, { core::mem::size_of::<u32>() }>, // 00
    pub flags: EnumSet<MovementActorAvoidanceParametersType, u32>,   // 18
    pub field_1c: i32,                                               // 1C
}

const _: () = assert!(core::mem::size_of::<MovementActorAvoidanceParameters>() == 0x20);
const _: () = assert!(core::mem::offset_of!(MovementActorAvoidanceParameters, actors) == 0x00);
const _: () = assert!(core::mem::offset_of!(MovementActorAvoidanceParameters, flags) == 0x18);
const _: () = assert!(core::mem::offset_of!(MovementActorAvoidanceParameters, field_1c) == 0x1C);
