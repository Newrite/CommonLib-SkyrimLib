#![allow(non_camel_case_types)]

use core::mem::MaybeUninit;

use core_util::inherit;

use crate::rex::W32::{
    CRITICAL_SECTION, DeleteCriticalSection, EnterCriticalSection,
    InitializeCriticalSectionAndSpinCount, LeaveCriticalSection,
};

/// C++ `RE::GLock::Locker`
#[repr(C)]
pub struct GLockLocker {
    pub lock: *mut GLock, // 00
}

const _: () = assert!(core::mem::size_of::<GLockLocker>() == 0x8);
const _: () = assert!(core::mem::offset_of!(GLockLocker, lock) == 0x0);

impl GLockLocker {
    #[inline(always)]
    pub unsafe fn new(lock: *mut GLock) -> Self {
        debug_assert!(!lock.is_null());
        unsafe {
            (*lock).lock();
        }
        Self { lock }
    }
}

impl Drop for GLockLocker {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            (*self.lock).unlock();
        }
    }
}

/// C++ `RE::GLock`
#[repr(C)]
pub struct GLock {
    pub cs: CRITICAL_SECTION, // 00
}

const _: () = assert!(core::mem::size_of::<GLock>() == 0x28);
const _: () = assert!(core::mem::offset_of!(GLock, cs) == 0x0);

impl GLock {
    #[inline(always)]
    pub fn new(spin_count: u32) -> Self {
        let mut lock = MaybeUninit::<Self>::uninit();
        unsafe {
            InitializeCriticalSectionAndSpinCount(
                core::ptr::addr_of_mut!((*lock.as_mut_ptr()).cs),
                spin_count,
            );
            lock.assume_init()
        }
    }

    #[inline(always)]
    pub fn lock(&mut self) {
        unsafe {
            EnterCriticalSection(core::ptr::addr_of_mut!(self.cs));
        }
    }

    #[inline(always)]
    pub fn unlock(&mut self) {
        unsafe {
            LeaveCriticalSection(core::ptr::addr_of_mut!(self.cs));
        }
    }
}

impl Default for GLock {
    #[inline(always)]
    fn default() -> Self {
        Self::new(0)
    }
}

impl Drop for GLock {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            DeleteCriticalSection(core::ptr::addr_of_mut!(self.cs));
        }
    }
}

/// C++ `RE::GAtomicValueBase<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GAtomicValueBase<T> {
    // TODO: C++ stores this as `volatile T value`. Rust has no field-level
    // `volatile` qualifier, so the layout stays source-backed while volatile
    // access is carried by the helper methods below when a consumer needs it.
    pub value: T, // 00
}

const _: () = assert!(core::mem::size_of::<GAtomicValueBase<i32>>() == 0x4);
const _: () = assert!(core::mem::offset_of!(GAtomicValueBase<i32>, value) == 0x0);

impl<T> GAtomicValueBase<T> {
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub fn read_volatile(&self) -> T
    where
        T: Copy,
    {
        unsafe { core::ptr::read_volatile(core::ptr::addr_of!(self.value)) }
    }

    #[inline(always)]
    pub fn write_volatile(&mut self, value: T) {
        unsafe {
            core::ptr::write_volatile(core::ptr::addr_of_mut!(self.value), value);
        }
    }
}

/// C++ `RE::GAtomicInt<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GAtomicInt<T> {
    pub base: GAtomicValueBase<T>, // 00
}

const _: () = assert!(core::mem::size_of::<GAtomicInt<i32>>() == 0x4);
const _: () = assert!(core::mem::offset_of!(GAtomicInt<i32>, base) == 0x0);

inherit!(for[T] GAtomicInt<T> : GAtomicValueBase<T>, base);

impl<T> GAtomicInt<T> {
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            base: GAtomicValueBase::new(value),
        }
    }
}
