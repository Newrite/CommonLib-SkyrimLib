use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::sync::atomic::{AtomicBool, Ordering};

/// A small once-initialized cell that panics on re-initialization.
pub struct Later<T> {
    is_init: AtomicBool,
    value: UnsafeCell<MaybeUninit<T>>,
}

/// An `UnsafeCell` wrapper that explicitly implements `Sync`.
#[repr(transparent)]
pub struct RacyCell<T>(UnsafeCell<T>);

impl<T> Later<T> {
    /// Creates an uninitialized cell.
    pub const fn new() -> Self {
        Self {
            is_init: AtomicBool::new(false),
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    /// Initializes the cell.
    ///
    /// Panics if the cell has already been initialized.
    pub fn init(&self, value: T) {
        assert!(!self.is_init.swap(true, Ordering::Relaxed));
        unsafe {
            (*self.value.get()).write(value);
        }
    }

    /// Returns whether the cell has already been initialized.
    pub fn is_init(&self) -> bool {
        self.is_init.load(Ordering::Relaxed)
    }
}

impl<T> Default for Later<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for Later<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        assert!(self.is_init.load(Ordering::Relaxed));
        unsafe { (*self.value.get()).assume_init_ref() }
    }
}

impl<T> Drop for Later<T> {
    fn drop(&mut self) {
        if *self.is_init.get_mut() {
            unsafe {
                (*self.value.get()).assume_init_drop();
            }
        }
    }
}

unsafe impl<T: Sync> Sync for Later<T> {}

impl<T> RacyCell<T> {
    /// Creates a new `RacyCell`.
    pub const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }

    /// Returns a raw pointer to the stored value.
    pub fn get(&self) -> *mut T {
        self.0.get()
    }
}

unsafe impl<T> Sync for RacyCell<T> {}
