//! SDK-level logging conveniences built on top of `libskyrim::skse::log`.
//!
//! This module is the recommended logging entrypoint for ordinary SDK users.
//! It keeps the raw SKSE logger surface available, but groups the common
//! plugin-facing decisions in one place:
//!
//! - use [`debug`], [`warning`], [`error`], or the `skse_*` macros for normal
//!   runtime diagnostics;
//! - use [`set_level_from_ini`] or [`set_level_from_str`] during bootstrap when
//!   a plugin wants user-configurable verbosity;
//! - use [`fatal`] or [`fatal_runtime`] only for unrecoverable initialization or
//!   runtime failures that should surface as a blocking message box.

/// Log-level and write helpers forwarded from the low-level SKSE logger.
///
/// These are the right entrypoints when a plugin wants ordinary runtime
/// diagnostics without depending on the raw `crate::skse::log` path directly.
pub use crate::skse::log::{
    LogLevel, LogType, debug, enabled, error, fatal, fatal_runtime, level, set_level,
    set_level_from_ini, set_level_from_str, warning, write,
};

/// Windows message-box icon flags used by [`fatal`] and [`fatal_runtime`].
pub use crate::skse::log::{MB_ICONERROR, MB_ICONWARNING};

/// Macro counterparts to the function-based logging helpers.
///
/// These are convenient when formatting is cheaper to express inline than
/// through [`write`] or the individual function helpers.
pub use crate::{skse_debug, skse_error, skse_fatal, skse_message, skse_warning};
