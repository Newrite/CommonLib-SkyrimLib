use core::sync::atomic::{AtomicU16, Ordering};
use crate::relocation::VariantID;
use crate::relocation_func;

/// C++ `RE::BSStringPool::Entry`
/// This header resides directly before the actual string data (`data - 0x18`).
#[repr(C)]
pub struct BSStringPoolEntry {
    pub left: *mut BSStringPoolEntry, // 0x00
    pub flags: AtomicU16,             // 0x08
    pub crc: u16,                     // 0x0A
    pub pad0c: u32,                   // 0x0C (Неявный C++ паддинг для выравнивания union)
    pub length: u32,                  // 0x10 (Начало union, совпадает с _length)
    pub pad14: u32,                   // 0x14 (Добиваем остаток union, так как он 8 байт)
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
        pub fn release8(entry: *const core::ffi::c_char) => VariantID::new(67847, 69192, 0)
    }

    relocation_func! {
        pub fn release16(entry: *const u16) => VariantID::new(67848, 69193, 0)
    }
}
