//! Stable plugin-form wrappers for config-driven caches.
//!
//! Plugin-loaded data-layer forms such as spells, keywords, globals, and base
//! effects typically stay alive for the whole game session. Callers often want
//! to resolve them once from config and then keep a stronger, sendable handle
//! than a generic `GamePtr`.

use core::any::type_name;
use core::fmt;
use core::ops::Deref;
use core::ptr::NonNull;

use crate::sdk::core::{GamePtr, GameRef};
use crate::skse::log;

/// Guaranteed loaded persistent form reference.
///
/// This is the ergonomic wrapper for plugin-loaded forms after the caller has
/// already decided that missing data is fatal or otherwise impossible.
#[repr(transparent)]
pub struct PersistentForm<T> {
    raw: NonNull<T>,
}

impl<T> PersistentForm<T> {
    /// # Safety
    /// `raw` must point to a live plugin-loaded form that remains valid for the
    /// whole session where this wrapper is used.
    #[inline(always)]
    pub const unsafe fn from_non_null(raw: NonNull<T>) -> Self {
        Self { raw }
    }

    #[inline(always)]
    pub fn from_game_ref(raw: GameRef<T>) -> Self {
        unsafe { Self::from_non_null(raw.as_non_null()) }
    }

    #[inline(always)]
    pub const fn as_non_null(self) -> NonNull<T> {
        self.raw
    }

    #[inline(always)]
    pub const fn as_ptr(self) -> *mut T {
        self.raw.as_ptr()
    }

    #[inline(always)]
    pub const fn cast<U>(self) -> *mut U {
        self.as_ptr().cast::<U>()
    }

    #[inline(always)]
    pub fn as_game_ref(self) -> GameRef<T> {
        unsafe { GameRef::from_non_null(self.raw) }
    }

    #[inline(always)]
    pub fn with<R>(self, f: impl FnOnce(&T) -> R) -> R {
        f(self.as_ref())
    }
}

impl<T> AsRef<T> for PersistentForm<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        unsafe { self.raw.as_ref() }
    }
}

impl<T> Copy for PersistentForm<T> {}

impl<T> Clone for PersistentForm<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Deref for PersistentForm<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> fmt::Debug for PersistentForm<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PersistentForm")
            .field(&format_args!("{:p}", self.as_ptr()))
            .finish()
    }
}

// SAFETY: This wrapper is restricted to stable data-layer forms loaded from
// plugins. It carries a pointer only and does not itself provide mutable
// access; callers later resolve it back into `GameRef` on the game thread.
unsafe impl<T> Send for PersistentForm<T> {}
unsafe impl<T> Sync for PersistentForm<T> {}

/// Nullable plugin-form lookup result.
///
/// Use this at lookup/config boundaries, then convert to [`PersistentForm<T>`]
/// once the caller has chosen whether missing data is allowed.
#[repr(transparent)]
pub struct PersistentFormPtr<T> {
    raw: Option<NonNull<T>>,
}

impl<T> PersistentFormPtr<T> {
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
    pub const fn as_ptr(self) -> *mut T {
        match self.raw {
            Some(raw) => raw.as_ptr(),
            None => core::ptr::null_mut(),
        }
    }

    #[inline(always)]
    pub fn try_get(self) -> Option<PersistentForm<T>> {
        self.raw
            .map(|raw| unsafe { PersistentForm::from_non_null(raw) })
    }

    #[inline(always)]
    pub fn try_ref(self) -> Option<GameRef<T>> {
        self.try_get().map(PersistentForm::as_game_ref)
    }

    #[inline(always)]
    pub fn require(self, context: &str) -> PersistentForm<T> {
        self.try_get().unwrap_or_else(|| {
            log::fatal_runtime(format_args!(
                "required persistent form is missing: {} ({})",
                context,
                type_name::<T>()
            ))
        })
    }
}

impl<T> Default for PersistentFormPtr<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::missing()
    }
}

impl<T> Copy for PersistentFormPtr<T> {}

impl<T> Clone for PersistentFormPtr<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> fmt::Debug for PersistentFormPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PersistentFormPtr")
            .field(&format_args!("{:p}", self.as_ptr()))
            .finish()
    }
}

// SAFETY: same contract as `PersistentForm<T>`, but nullable while the caller
// is still deciding how to handle missing plugin data.
unsafe impl<T> Send for PersistentFormPtr<T> {}
unsafe impl<T> Sync for PersistentFormPtr<T> {}
