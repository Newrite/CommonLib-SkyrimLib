use crate::re::ActorHandle;
use crate::re::TESFaction;
use crate::re::bs_atomic::BSReadWriteLock;
use crate::re::bst_array::BSTArray;

/// C++ `RE::PackageNS::CRIME_TYPES::CRIME_TYPE`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrimeType {
    None = u32::MAX,
    Steal = 0,
    Pickpocket = 1,
    Trespass = 2,
    Attack = 3,
    Murder = 4,
    Escape = 5,
    Werewolf = 6,
}

impl CrimeType {
    pub const TOTAL: usize = 7;
}

/// C++ `RE::Crime`
#[repr(C)]
pub struct Crime {
    pub unk00: u64,                                  // 00
    pub unk08: u64,                                  // 08
    pub unk10: u64,                                  // 10
    pub unk18: u64,                                  // 18
    pub unk20: u64,                                  // 20
    pub actors_know_of_crime: BSTArray<ActorHandle>, // 28
    pub unk40: u64,                                  // 40
    pub unk48: u64,                                  // 48
    pub unk50: u64,                                  // 50
    pub unk58: u64,                                  // 58
    pub crime_faction: *mut TESFaction,              // 60
    pub unk68: u32,                                  // 68
    pub lock: BSReadWriteLock,                       // 6C
    pub unk74: u32,                                  // 74
}

const _: () = assert!(core::mem::size_of::<Crime>() == 0x78);
const _: () = assert!(core::mem::offset_of!(Crime, actors_know_of_crime) == 0x28);
const _: () = assert!(core::mem::offset_of!(Crime, crime_faction) == 0x60);
const _: () = assert!(core::mem::offset_of!(Crime, unk68) == 0x68);
const _: () = assert!(core::mem::offset_of!(Crime, lock) == 0x6C);
const _: () = assert!(core::mem::offset_of!(Crime, unk74) == 0x74);
