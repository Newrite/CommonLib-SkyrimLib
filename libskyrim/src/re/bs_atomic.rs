use core::ffi::c_void;
use core::sync::atomic::{AtomicU32, Ordering, fence};

use windows_sys::Win32::System::Threading::{GetCurrentThreadId, Sleep};

/// Source-backed subset of `RE::BSAtomic.h` required by event/container code.

/// C++ `RE::BSEventFlag`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BSEventFlag {
    pub event: *mut c_void, // 00
}

const _: () = assert!(core::mem::size_of::<BSEventFlag>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSEventFlag, event) == 0x0);

/// C++ `RE::BSNonReentrantSpinLock`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSNonReentrantSpinLock {
    pub lock: u32, // 00
}

const _: () = assert!(core::mem::size_of::<BSNonReentrantSpinLock>() == 0x4);
const _: () = assert!(core::mem::offset_of!(BSNonReentrantSpinLock, lock) == 0x0);

/// C++ `RE::BSSemaphoreBase`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSSemaphoreBase {
    pub semaphore: *mut c_void, // 00
}

const _: () = assert!(core::mem::size_of::<BSSemaphoreBase>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSSemaphoreBase, semaphore) == 0x0);

/// C++ `RE::BSSemaphore`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSSemaphore {
    pub base: BSSemaphoreBase, // 00
}

const _: () = assert!(core::mem::size_of::<BSSemaphore>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSSemaphore, base) == 0x0);

/// C++ `RE::BSSpinLock`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BSSpinLock {
    pub owning_thread: u32, // 00
    pub lock_count: u32,    // 04
}

const _: () = assert!(core::mem::size_of::<BSSpinLock>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSSpinLock, owning_thread) == 0x0);
const _: () = assert!(core::mem::offset_of!(BSSpinLock, lock_count) == 0x4);

impl BSSpinLock {
    pub const FAST_SPIN_THRESHOLD: u32 = 10_000;

    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            owning_thread: 0,
            lock_count: 0,
        }
    }

    #[inline(always)]
    fn owning_thread_atomic(&self) -> &AtomicU32 {
        unsafe { &*(&self.owning_thread as *const u32 as *const AtomicU32) }
    }

    #[inline(always)]
    fn lock_count_atomic(&self) -> &AtomicU32 {
        unsafe { &*(&self.lock_count as *const u32 as *const AtomicU32) }
    }

    pub fn lock(&mut self, pause_attempts: u32) {
        let my_thread_id = unsafe { GetCurrentThreadId() };
        let owning_thread = self.owning_thread_atomic();
        let lock_count = self.lock_count_atomic();

        fence(Ordering::Acquire);
        if owning_thread.load(Ordering::Relaxed) == my_thread_id {
            lock_count.fetch_add(1, Ordering::Relaxed);
            return;
        }

        let mut attempts = 0;
        if lock_count
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            loop {
                attempts += 1;
                core::hint::spin_loop();

                if attempts >= pause_attempts {
                    let mut spin_count = 0;
                    while lock_count
                        .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
                        .is_err()
                    {
                        spin_count += 1;
                        unsafe {
                            Sleep(if spin_count < Self::FAST_SPIN_THRESHOLD {
                                0
                            } else {
                                1
                            });
                        }
                    }
                    break;
                }

                if lock_count
                    .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
                {
                    break;
                }
            }

            fence(Ordering::Acquire);
        }

        owning_thread.store(my_thread_id, Ordering::Relaxed);
        fence(Ordering::Release);
    }

    pub fn unlock(&mut self) {
        let my_thread_id = unsafe { GetCurrentThreadId() };
        let owning_thread = self.owning_thread_atomic();
        let lock_count = self.lock_count_atomic();

        fence(Ordering::Acquire);
        if owning_thread.load(Ordering::Relaxed) != my_thread_id {
            return;
        }

        if lock_count.load(Ordering::Relaxed) == 1 {
            owning_thread.store(0, Ordering::Relaxed);
            fence(Ordering::SeqCst);
            let _ = lock_count.compare_exchange(1, 0, Ordering::Release, Ordering::Relaxed);
        } else {
            lock_count.fetch_sub(1, Ordering::Relaxed);
        }
    }
}

impl Default for BSSpinLock {
    fn default() -> Self {
        Self::new()
    }
}

/// C++ `RE::BSReadWriteLock`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSReadWriteLock {
    pub writer_thread: u32, // 00
    pub lock: u32,          // 04
}

const _: () = assert!(core::mem::size_of::<BSReadWriteLock>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSReadWriteLock, writer_thread) == 0x0);
const _: () = assert!(core::mem::offset_of!(BSReadWriteLock, lock) == 0x4);

/// C++ `RE::BSSpinLockGuard`
#[repr(C)]
#[must_use]
pub struct BSSpinLockGuard {
    lock: *mut BSSpinLock, // 00
}

const _: () = assert!(core::mem::size_of::<BSSpinLockGuard>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSSpinLockGuard, lock) == 0x0);

impl BSSpinLockGuard {
    #[inline(always)]
    pub fn new(lock: &mut BSSpinLock) -> Self {
        unsafe { Self::from_ptr(lock as *mut BSSpinLock) }
    }

    /// # Safety
    /// `lock` must be a valid pointer to a live `BSSpinLock` for the full
    /// lifetime of the returned guard.
    #[inline(always)]
    pub unsafe fn from_ptr(lock: *mut BSSpinLock) -> Self {
        debug_assert!(!lock.is_null());
        unsafe {
            (*lock).lock(0);
        }
        Self { lock }
    }
}

impl Drop for BSSpinLockGuard {
    fn drop(&mut self) {
        if !self.lock.is_null() {
            unsafe {
                (*self.lock).unlock();
            }
        }
    }
}
