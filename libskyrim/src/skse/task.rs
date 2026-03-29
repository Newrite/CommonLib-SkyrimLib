use alloc::boxed::Box;
use core::ffi::c_void;

use crate::ffi;

fn task_runner_impl<F: FnOnce()>(data: *mut c_void, context: &str) {
    crate::skse::crash::guard(context, move || {
        let closure = unsafe { Box::from_raw(data.cast::<F>()) };
        closure();
    });
}

/// Executes a queued SKSE task by reclaiming the boxed Rust closure.
extern "C" fn task_runner<F: FnOnce()>(data: *mut c_void) {
    task_runner_impl::<F>(data, "SKSE task callback");
}

/// Executes a queued SKSE UI task by reclaiming the boxed Rust closure.
extern "C" fn ui_task_runner<F: FnOnce()>(data: *mut c_void) {
    task_runner_impl::<F>(data, "SKSE UI task callback");
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
        ffi::commonlib_add_ui_task(ui_task_runner::<F>, data);
    }
}
