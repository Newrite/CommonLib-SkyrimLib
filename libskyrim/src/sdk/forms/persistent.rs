//! Stable plugin-form wrappers for config-driven caches.
//!
//! Unlike arbitrary world objects, data-layer forms loaded from plugins
//! typically stay alive for the whole game session. Plugin authors often want
//! to resolve those forms once from config and then keep a sendable cache in a
//! global plugin state. `GamePtr`/`GameRef` intentionally do not make that
//! stronger cross-thread/session claim on their own, so this wrapper keeps that
//! contract explicit and narrow.

use core::any::type_name;
use core::fmt;
use core::ptr::NonNull;

use crate::sdk::core::{GamePtr, GameRef};
use crate::skse::log;

/// Cached pointer to a plugin-loaded persistent form.
///
/// This wrapper is intended for stable data-layer forms such as spells,
/// keywords, globals, base effects, and other records loaded from
/// `ESP`/`ESM` files. It is not a general-purpose world-object cache.
#[repr(transparent)]
pub struct PersistentForm<T> {
    raw: Option<NonNull<T>>,
}

impl<T> PersistentForm<T> {
    #[inline(always)]
    pub const fn missing() -> Self {
        Self { raw: None }
    }

    #[inline(always)]
    pub fn from_game_ptr(raw: GamePtr<T>) -> Self {
        Self {
            raw: raw.as_non_null(),
        }
    }

    #[inline(always)]
    pub const fn is_loaded(self) -> bool {
        self.raw.is_some()
    }

    #[inline(always)]
    pub const fn is_missing(self) -> bool {
        self.raw.is_none()
    }

    #[inline(always)]
    pub fn as_non_null(self) -> Option<NonNull<T>> {
        self.raw
    }

    #[inline(always)]
    pub fn try_ref(self) -> Option<GameRef<T>> {
        self.raw.map(|raw| unsafe { GameRef::from_non_null(raw) })
    }

    #[inline(always)]
    pub fn require_ref(self, context: &str) -> GameRef<T> {
        self.try_ref().unwrap_or_else(|| {
            log::fatal_runtime(format_args!(
                "required persistent form is missing: {} ({})",
                context,
                type_name::<T>()
            ))
        })
    }

    #[inline(always)]
    pub fn require_non_null(self, context: &str) -> NonNull<T> {
        self.as_non_null().unwrap_or_else(|| {
            log::fatal_runtime(format_args!(
                "required persistent form is missing: {} ({})",
                context,
                type_name::<T>()
            ))
        })
    }

    #[inline(always)]
    pub const fn as_ptr(self) -> *mut T {
        match self.raw {
            Some(raw) => raw.as_ptr(),
            None => core::ptr::null_mut(),
        }
    }
}

impl<T> Default for PersistentForm<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::missing()
    }
}

impl<T> Copy for PersistentForm<T> {}

impl<T> Clone for PersistentForm<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> fmt::Debug for PersistentForm<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PersistentForm")
            .field(&format_args!("{:p}", self.as_ptr()))
            .finish()
    }
}

// SAFETY: This wrapper is intentionally restricted to stable data-layer forms
// loaded from plugins. It carries only an optional non-null pointer and does
// not by itself grant mutable access; callers later resolve it back into
// `GameRef` on the game thread.
unsafe impl<T> Send for PersistentForm<T> {}
unsafe impl<T> Sync for PersistentForm<T> {}
