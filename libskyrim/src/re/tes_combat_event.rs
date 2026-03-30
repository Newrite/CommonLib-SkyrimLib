use core_util::EnumSet;

use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::ACTOR_COMBAT_STATE`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorCombatState {
    None = 0,
    Combat = 1,
    Searching = 2,
}

core_util::impl_enumset_type!(ActorCombatState => u32);

/// C++ `RE::TESCombatEvent`
#[repr(C)]
pub struct TESCombatEvent {
    pub actor: NiPointer<TESObjectREFR>,           // 00
    pub target_actor: NiPointer<TESObjectREFR>,    // 08
    pub new_state: EnumSet<ActorCombatState, u32>, // 10
    pub pad14: u32,                                // 14
}

const _: () = assert!(core::mem::size_of::<TESCombatEvent>() == 0x18);
