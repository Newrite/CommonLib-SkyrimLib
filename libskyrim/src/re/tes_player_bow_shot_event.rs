use crate::re::bs_core_types::FormID;

/// C++ `RE::TESPlayerBowShotEvent`
#[repr(C)]
pub struct TESPlayerBowShotEvent {
    pub weapon: FormID,      // 00
    pub ammo: FormID,        // 04
    pub shot_power: f32,     // 08
    pub is_sun_gazing: bool, // 0C
    pub pad0d: u8,           // 0D
    pub pad0e: u16,          // 0E
}

const _: () = assert!(core::mem::size_of::<TESPlayerBowShotEvent>() == 0x10);
