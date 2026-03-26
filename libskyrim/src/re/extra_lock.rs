use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_ExtraLock;
use crate::offsets::offsets_vtable::VTABLE_ExtraLock;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped, TESObjectREFR};
use crate::relocation::{RelocationID, RttiType, VariantID};

crate::core_util::abstract_type! { pub type TESKey; }

/// C++ `RE::LOCK_LEVEL`
#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LOCK_LEVEL {
    Unlocked = -1,
    VeryEasy = 0,
    Easy = 1,
    Average = 2,
    Hard = 3,
    VeryHard = 4,
    RequiresKey = 5,
}

/// C++ `RE::REFR_LOCK::Flag`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum REFRLockFlag {
    None = 0,
    Locked = 1 << 0,
    Leveled = 1 << 2,
}

core_util::impl_enumset_type!(REFRLockFlag => u8);

/// C++ `RE::REFR_LOCK`
#[repr(C)]
pub struct REFR_LOCK {
    pub base_level: i8,                   // 00
    pub pad01: u8,                        // 01
    pub pad02: u16,                       // 02
    pub pad04: u32,                       // 04
    pub key: *mut TESKey,                 // 08
    pub flags: EnumSet<REFRLockFlag, u8>, // 10
    pub pad11: u8,                        // 11
    pub pad12: u16,                       // 12
    pub num_tries: u32,                   // 14
    pub unk18: u32,                       // 18
    pub pad1c: u32,                       // 1C
}

const _: () = assert!(core::mem::size_of::<REFR_LOCK>() == 0x20);
const _: () = assert!(core::mem::offset_of!(REFR_LOCK, base_level) == 0x00);
const _: () = assert!(core::mem::offset_of!(REFR_LOCK, key) == 0x08);
const _: () = assert!(core::mem::offset_of!(REFR_LOCK, flags) == 0x10);
const _: () = assert!(core::mem::offset_of!(REFR_LOCK, num_tries) == 0x14);
const _: () = assert!(core::mem::offset_of!(REFR_LOCK, unk18) == 0x18);

impl REFR_LOCK {
    crate::relocation_func! {
        fn get_lock_level_impl(
            &self,
            container_ref: *const TESObjectREFR
        ) -> LOCK_LEVEL => RelocationID::new(12272, 12399)
    }

    #[inline(always)]
    pub fn get_lock_level(&self, container_ref: *const TESObjectREFR) -> LOCK_LEVEL {
        if self.is_locked() {
            self.get_lock_level_impl(container_ref)
        } else {
            LOCK_LEVEL::Unlocked
        }
    }

    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        self.flags.all(REFRLockFlag::Locked)
    }

    #[inline(always)]
    pub fn set_locked(&mut self, locked: bool) {
        self.flags.set_enabled(locked, REFRLockFlag::Locked);
        if !locked {
            self.num_tries = 0;
        }
    }
}

/// C++ `RE::ExtraLock`
#[repr(C)]
pub struct ExtraLock {
    pub base: BSExtraData,    // 00
    pub lock: *mut REFR_LOCK, // 10
}

const _: () = assert!(core::mem::size_of::<ExtraLock>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraLock, lock) == 0x10);

impl RttiType for ExtraLock {
    const RTTI: VariantID = RTTI_ExtraLock;
}

impl ExtraDataTyped for ExtraLock {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::Lock;
}

inherit!(ExtraLock : BSExtraData);

impl ExtraLock {
    pub const RTTI: VariantID = RTTI_ExtraLock;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraLock;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::Lock;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kLock; }
    // bool IsNotEqual(const BSExtraData* a_rhs) const override;  // 02

    #[inline(always)]
    pub fn is_not_equal_impl(&self, rhs: *const BSExtraData) -> bool {
        let rhs = rhs.cast::<Self>();
        rhs.is_null() || self.lock != unsafe { (*rhs).lock }
    }
}
