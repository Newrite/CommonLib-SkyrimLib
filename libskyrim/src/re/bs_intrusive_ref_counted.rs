//! Translation of `RE::BSIntrusiveRefCounted.h`.

use core::sync::atomic::{AtomicU32, Ordering};

/// C++ `RE::BSIntrusiveRefCounted`
///
/// Base class for reference counted engine objects (using intrusive tracking).
#[repr(C)]
pub struct BSIntrusiveRefCounted {
    pub ref_count: AtomicU32, // 00
}

const _: () = assert!(core::mem::size_of::<BSIntrusiveRefCounted>() == 0x4);

impl Default for BSIntrusiveRefCounted {
    #[inline]
    fn default() -> Self {
        Self {
            ref_count: AtomicU32::new(0),
        }
    }
}

impl BSIntrusiveRefCounted {
    /// Increments the reference count.
    /// Matches C++ `IncRef()`
    #[inline]
    pub fn inc_ref(&self) -> u32 {
        self.ref_count.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Decrements the reference count.
    /// Matches C++ `DecRef()`
    #[inline]
    pub fn dec_ref(&self) -> u32 {
        self.ref_count.fetch_sub(1, Ordering::SeqCst) - 1
    }
}
