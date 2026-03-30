use core::fmt::Arguments;
use core::hint::spin_loop;

use crate::ffi;
use crate::skse::log::{self, LogType};

pub const RUST_PANIC_EXCEPTION_CODE: u32 = 0xE000_0001u32;

#[inline(always)]
pub fn raise_seh_exception(code: u32) -> ! {
    unsafe {
        ffi::commonlib_raise_seh_exception(code);
    }

    // If control somehow returns here, an upstream frame swallowed the exception.
    // Keep the current thread parked instead of force-killing the process so an
    // attached debugger can still inspect the bad state.
    loop {
        spin_loop();
    }
}

#[track_caller]
pub fn raise_logged_runtime_error(args: Arguments<'_>) -> ! {
    let location = core::panic::Location::caller();
    log::fatal(
        LogType::Both(log::MB_ICONERROR),
        location.file(),
        location.line(),
        args,
    );
    raise_seh_exception(RUST_PANIC_EXCEPTION_CODE)
}

#[cfg(feature = "std")]
mod std_runtime {
    use alloc::boxed::Box;
    use alloc::string::String;
    use core::any::Any;

    use std::cell::Cell;
    use std::panic::{self, AssertUnwindSafe, PanicHookInfo};
    use std::sync::Once;

    use super::{LogType, RUST_PANIC_EXCEPTION_CODE, log, raise_seh_exception};

    static PANIC_HOOK_INSTALLED: Once = Once::new();
    std::thread_local! {
        static PANIC_HOOK_SUPPRESSED: Cell<u32> = const { Cell::new(0) };
    }

    struct HookSuppressionGuard;

    impl HookSuppressionGuard {
        fn enter() -> Self {
            PANIC_HOOK_SUPPRESSED.with(|depth| depth.set(depth.get() + 1));
            Self
        }
    }

    impl Drop for HookSuppressionGuard {
        fn drop(&mut self) {
            PANIC_HOOK_SUPPRESSED.with(|depth| depth.set(depth.get().saturating_sub(1)));
        }
    }

    #[inline(always)]
    fn panic_payload_message(payload: &(dyn Any + Send)) -> String {
        if let Some(message) = payload.downcast_ref::<&'static str>() {
            (*message).into()
        } else if let Some(message) = payload.downcast_ref::<String>() {
            message.clone()
        } else {
            "non-string panic payload".into()
        }
    }

    #[inline(always)]
    fn panic_info_message(info: &PanicHookInfo<'_>) -> String {
        panic_payload_message(info.payload())
    }

    fn log_panic_hook(info: &PanicHookInfo<'_>) {
        let suppressed = PANIC_HOOK_SUPPRESSED.with(|depth| depth.get() != 0);
        if suppressed {
            return;
        }

        let (file, line) = match info.location() {
            Some(location) => (location.file(), location.line()),
            None => ("<panic>", 0),
        };
        let current_thread = std::thread::current();
        let thread = current_thread.name().unwrap_or("<unnamed>");
        let payload = panic_info_message(info);

        log::error(
            LogType::File,
            file,
            line,
            format_args!("unhandled Rust panic on thread '{thread}': {payload}"),
        );

        raise_seh_exception(RUST_PANIC_EXCEPTION_CODE)
    }

    pub fn install_panic_hook() {
        PANIC_HOOK_INSTALLED.call_once(|| {
            panic::set_hook(Box::new(log_panic_hook));
        });
    }

    #[track_caller]
    fn raise_caught_panic(context: &str, payload: Box<dyn Any + Send>) -> ! {
        let location = core::panic::Location::caller();
        let current_thread = std::thread::current();
        let thread = current_thread.name().unwrap_or("<unnamed>");
        let payload = panic_payload_message(payload.as_ref());

        log::error(
            LogType::File,
            location.file(),
            location.line(),
            format_args!("caught Rust panic in {context} on thread '{thread}': {payload}"),
        );

        raise_seh_exception(RUST_PANIC_EXCEPTION_CODE)
    }

    #[track_caller]
    pub fn guard<R>(context: &str, f: impl FnOnce() -> R) -> R {
        install_panic_hook();
        let _suppressed = HookSuppressionGuard::enter();

        match panic::catch_unwind(AssertUnwindSafe(f)) {
            Ok(value) => value,
            Err(payload) => raise_caught_panic(context, payload),
        }
    }
}

#[cfg(feature = "std")]
pub use std_runtime::{guard, install_panic_hook};

#[cfg(not(feature = "std"))]
#[inline(always)]
pub fn install_panic_hook() {}

#[cfg(not(feature = "std"))]
#[inline(always)]
pub fn guard<R>(_: &str, f: impl FnOnce() -> R) -> R {
    f()
}
