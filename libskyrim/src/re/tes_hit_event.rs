use core_util::EnumSet;

use crate::re::bs_core_types::FormID;
use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESHitEvent::Flag`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESHitEventFlag {
    kNone = 0,
    kPowerAttack = 1 << 0,
    kSneakAttack = 1 << 1,
    kBashAttack = 1 << 2,
    kHitBlocked = 1 << 3,
}

core_util::impl_enumset_type!(TESHitEventFlag => u8);

/// C++ `RE::TESHitEvent`
#[repr(C)]
pub struct TESHitEvent {
    pub target: NiPointer<TESObjectREFR>,    // 00
    pub cause: NiPointer<TESObjectREFR>,     // 08
    pub source: FormID,                      // 10
    pub projectile: FormID,                  // 14
    pub flags: EnumSet<TESHitEventFlag, u8>, // 18
    pub pad19: u8,                           // 19
    pub pad1a: u16,                          // 1A
    pub pad1c: u32,                          // 1C
}

const _: () = assert!(core::mem::size_of::<TESHitEvent>() == 0x20);

impl Default for TESHitEvent {
    fn default() -> Self {
        Self::new(
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            0,
            0,
            TESHitEventFlag::kNone,
        )
    }
}

impl TESHitEvent {
    #[inline(always)]
    pub fn new(
        target: *mut TESObjectREFR,
        aggressor: *mut TESObjectREFR,
        weapon: FormID,
        projectile: FormID,
        flags: TESHitEventFlag,
    ) -> Self {
        Self {
            target: unsafe { NiPointer::new(target) },
            cause: unsafe { NiPointer::new(aggressor) },
            source: weapon,
            projectile,
            flags: EnumSet::from_underlying(flags as u8),
            pad19: 0,
            pad1a: 0,
            pad1c: 0,
        }
    }
}
