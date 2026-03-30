//! Shared defensive helpers for SDK-facing guard rails.
//!
//! The low-level RE layer still exposes strict/raw entrypoints where layout and
//! ABI fidelity matter most, but higher-level helpers should prefer returning a
//! safe fallback over crashing the game when callers hit a bad lifecycle phase
//! or pass null / transient engine pointers.

#[inline(always)]
pub const fn defensive_logging_enabled() -> bool {
    cfg!(feature = "defensive-sdk-log")
}

#[macro_export]
macro_rules! defensive_sdk_warn {
    ($($arg:tt)*) => {{
        #[cfg(feature = "defensive-sdk-log")]
        {
            $crate::skse_warning!(log, $($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! defensive_sdk_error {
    ($($arg:tt)*) => {{
        #[cfg(feature = "defensive-sdk-log")]
        {
            $crate::skse_error!(log, $($arg)*);
        }
    }};
}
