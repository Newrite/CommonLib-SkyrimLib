use crate::re::TESFaction;

/// C++ `RE::FACTION_RANK`
#[repr(C)]
pub struct FactionRank {
    pub faction: *mut TESFaction, // 00
    pub rank: i8,                 // 08
    pub pad09: u8,                // 09
    pub pad0a: u16,               // 0A
    pub pad0c: u32,               // 0C
}

const _: () = assert!(core::mem::size_of::<FactionRank>() == 0x10);
const _: () = assert!(core::mem::offset_of!(FactionRank, faction) == 0x00);
const _: () = assert!(core::mem::offset_of!(FactionRank, rank) == 0x08);

pub use FactionRank as FACTION_RANK;
