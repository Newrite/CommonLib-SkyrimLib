use core::mem::MaybeUninit;

mod commonlib;
mod skse;

pub use commonlib::*;
pub use skse::*;

/// Constructs a C++ object directly into caller-provided out storage and returns the
/// initialized value on success.
///
/// This is the Rust-side half of the ABI-safe out-param bridge pattern used for
/// smart-pointer helpers such as `make_hkref`, `make_nismart`, and `make_smart`.
#[inline(always)]
pub unsafe fn try_construct_out_param<T>(construct: impl FnOnce(*mut T) -> bool) -> Option<T> {
    let mut out = MaybeUninit::<T>::uninit();
    if construct(out.as_mut_ptr()) {
        Some(unsafe { out.assume_init() })
    } else {
        None
    }
}
