use crate::relocation::RelocationID;
use crate::relocation_func;
use core::sync::atomic::{AtomicU16, Ordering};

/// C++ `RE::BSStringPool::Entry`
/// This header resides directly before the actual string data (`data - 0x18`).
#[repr(C)]
pub struct BSStringPoolEntry {
    pub left: *mut BSStringPoolEntry, // 0x00
    pub flags: AtomicU16,             // 0x08
    pub crc: u16,                     // 0x0A
    pub pad0c: u32, // 0x0C (Р В Р’В Р РЋРЎС™Р В Р’В Р вЂ™Р’ВµР В Р Р‹Р В Р РЏР В Р’В Р В РІР‚В Р В Р’В Р В РІР‚В¦Р В Р Р‹Р Р†Р вЂљРІвЂћвЂ“Р В Р’В Р Р†РІР‚С›РІР‚вЂњ C++ Р В Р’В Р РЋРІР‚вЂќР В Р’В Р вЂ™Р’В°Р В Р’В Р СћРІР‚ВР В Р’В Р СћРІР‚ВР В Р’В Р РЋРІР‚ВР В Р’В Р В РІР‚В¦Р В Р’В Р РЋРІР‚вЂњ Р В Р’В Р СћРІР‚ВР В Р’В Р вЂ™Р’В»Р В Р Р‹Р В Р РЏ Р В Р’В Р В РІР‚В Р В Р Р‹Р Р†Р вЂљРІвЂћвЂ“Р В Р Р‹Р В РІР‚С™Р В Р’В Р вЂ™Р’В°Р В Р’В Р В РІР‚В Р В Р’В Р В РІР‚В¦Р В Р’В Р РЋРІР‚ВР В Р’В Р В РІР‚В Р В Р’В Р вЂ™Р’В°Р В Р’В Р В РІР‚В¦Р В Р’В Р РЋРІР‚ВР В Р Р‹Р В Р РЏ union)
    pub length: u32, // 0x10 (Р В Р’В Р РЋРЎС™Р В Р’В Р вЂ™Р’В°Р В Р Р‹Р Р†Р вЂљР Р‹Р В Р’В Р вЂ™Р’В°Р В Р’В Р вЂ™Р’В»Р В Р’В Р РЋРІР‚Сћ union, Р В Р Р‹Р В РЎвЂњР В Р’В Р РЋРІР‚СћР В Р’В Р В РІР‚В Р В Р’В Р РЋРІР‚вЂќР В Р’В Р вЂ™Р’В°Р В Р’В Р СћРІР‚ВР В Р’В Р вЂ™Р’В°Р В Р’В Р вЂ™Р’ВµР В Р Р‹Р Р†Р вЂљРЎв„ў Р В Р Р‹Р В РЎвЂњ _length)
    pub pad14: u32, // 0x14 (Р В Р’В Р Р†Р вЂљРЎСљР В Р’В Р РЋРІР‚СћР В Р’В Р вЂ™Р’В±Р В Р’В Р РЋРІР‚ВР В Р’В Р В РІР‚В Р В Р’В Р вЂ™Р’В°Р В Р’В Р вЂ™Р’ВµР В Р’В Р РЋР’В Р В Р’В Р РЋРІР‚СћР В Р Р‹Р В РЎвЂњР В Р Р‹Р Р†Р вЂљРЎв„ўР В Р’В Р вЂ™Р’В°Р В Р Р‹Р Р†Р вЂљРЎв„ўР В Р’В Р РЋРІР‚СћР В Р’В Р РЋРІР‚Сњ union, Р В Р Р‹Р Р†Р вЂљРЎв„ўР В Р’В Р вЂ™Р’В°Р В Р’В Р РЋРІР‚Сњ Р В Р’В Р РЋРІР‚СњР В Р’В Р вЂ™Р’В°Р В Р’В Р РЋРІР‚Сњ Р В Р’В Р РЋРІР‚СћР В Р’В Р В РІР‚В¦ 8 Р В Р’В Р вЂ™Р’В±Р В Р’В Р вЂ™Р’В°Р В Р’В Р Р†РІР‚С›РІР‚вЂњР В Р Р‹Р Р†Р вЂљРЎв„ў)
}

const _: () = assert!(core::mem::size_of::<BSStringPoolEntry>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BSStringPoolEntry, length) == 0x10);

impl BSStringPoolEntry {
    pub const K_WIDE: u16 = 1 << 15;
    pub const K_REF_COUNT_MASK: u16 = 0x7FFF;
    pub const K_LENGTH_MASK: u32 = 0xFFFFFF;

    #[inline(always)]
    pub fn is_wide(&self) -> bool {
        (self.flags.load(Ordering::Relaxed) & Self::K_WIDE) != 0
    }

    #[inline(always)]
    pub fn length(&self) -> u32 {
        self.length & Self::K_LENGTH_MASK
    }

    #[inline(always)]
    pub fn acquire(&self) {
        let mut expected = self.flags.load(Ordering::Relaxed);
        loop {
            if (expected & Self::K_REF_COUNT_MASK) >= Self::K_REF_COUNT_MASK {
                break;
            }
            match self.flags.compare_exchange_weak(
                expected,
                expected + 1,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(actual) => expected = actual,
            }
        }
    }

    relocation_func! {
        fn release8_impl(entry: *mut *const core::ffi::c_char) => RelocationID::new(67847, 69192)
    }

    relocation_func! {
        fn release16_impl(entry: *mut *const u16) => RelocationID::new(67848, 69193)
    }

    #[inline(always)]
    pub fn release8(entry: &mut *const core::ffi::c_char) {
        Self::release8_impl(entry as *mut _);
    }

    #[inline(always)]
    pub fn release16(entry: &mut *const u16) {
        Self::release16_impl(entry as *mut _);
    }
}
