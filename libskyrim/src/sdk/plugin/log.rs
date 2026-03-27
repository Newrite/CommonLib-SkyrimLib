//! SDK-level logging conveniences built on top of `libskyrim::skse::log`.

pub use crate::skse::log::{
    LogLevel, LogType, MB_ICONERROR, MB_ICONWARNING, debug, enabled, error, fatal, fatal_runtime,
    level, set_level, set_level_from_ini, set_level_from_str, warning, write,
};
pub use crate::{skse_debug, skse_error, skse_fatal, skse_message, skse_warning};
