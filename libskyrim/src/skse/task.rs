use alloc::boxed::Box;
use core::ffi::c_void;

use crate::ffi;

/// Executes a queued SKSE task by reclaiming the boxed Rust closure.
extern "C" fn task_runner<F: FnOnce()>(data: *mut c_void) {
    let closure = unsafe { Box::from_raw(data.cast::<F>()) };
    closure();
}

#[inline(always)]
/// Queues work on the SKSE task interface.
pub fn add_task<F>(f: F)
where
    F: FnOnce() + Send + 'static,
{
    let data = Box::into_raw(Box::new(f)).cast::<c_void>();
    unsafe {
        ffi::commonlib_add_task(task_runner::<F>, data);
    }
}

#[inline(always)]
/// Queues work on the SKSE UI task interface.
pub fn add_ui_task<F>(f: F)
where
    F: FnOnce() + Send + 'static,
{
    let data = Box::into_raw(Box::new(f)).cast::<c_void>();
    unsafe {
        ffi::commonlib_add_ui_task(task_runner::<F>, data);
    }
}
