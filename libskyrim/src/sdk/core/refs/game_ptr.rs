use core::fmt;
use core::hash::{Hash, Hasher};
use core::ptr::NonNull;

use crate::relocation::{RttiType, skyrim_cast};

use super::game_ref::GameRef;

/// Nullable engine object wrapper.
///
/// This is the pointer-shaped companion to [`GameRef`]: it preserves nullability
/// for stable engine pointers while still offering the same read-only
/// convenience once the pointee is known to be valid.
#[repr(transparent)]
#[must_use]
pub struct GamePtr<T> {
    raw: *mut T,
}

impl<T> GamePtr<T> {
    #[inline(always)]
    pub const fn null() -> Self {
        Self {
            raw: core::ptr::null_mut(),
        }
    }

    /// # Safety
    /// If `raw` is non-null it must point to a valid live `T` for all future
    /// safe accesses performed through this wrapper.
    #[inline(always)]
    pub const unsafe fn from_raw(raw: *mut T) -> Self {
        Self { raw }
    }

    /// # Safety
    /// `raw` must point to a valid live `T` for all future safe accesses
    /// performed through this wrapper.
    #[inline(always)]
    pub const unsafe fn from_non_null(raw: NonNull<T>) -> Self {
        Self { raw: raw.as_ptr() }
    }

    #[inline(always)]
    pub const fn as_ptr(self) -> *mut T {
        self.raw
    }

    #[inline(always)]
    pub const fn is_null(self) -> bool {
        self.raw.is_null()
    }

    #[inline(always)]
    pub const fn is_some(self) -> bool {
        !self.is_null()
    }

    #[inline(always)]
    pub fn as_non_null(self) -> Option<NonNull<T>> {
        NonNull::new(self.raw)
    }

    #[inline(always)]
    pub fn as_ref(&self) -> Option<&T> {
        unsafe { self.raw.as_ref() }
    }

    #[inline(always)]
    pub fn into_option(self) -> Option<GameRef<T>> {
        self.as_non_null()
            .map(|raw| unsafe { GameRef::from_non_null(raw) })
    }

    #[inline(always)]
    pub fn unwrap(self) -> GameRef<T> {
        self.expect("called GamePtr::unwrap() on a null pointer")
    }

    #[inline(always)]
    pub fn expect(self, msg: &str) -> GameRef<T> {
        self.into_option().expect(msg)
    }

    #[inline(always)]
    pub fn with<R>(self, f: impl FnOnce(&T) -> R) -> Option<R> {
        self.as_ref().map(f)
    }

    #[inline(always)]
    pub fn map<R>(self, f: impl FnOnce(&T) -> R) -> Option<R> {
        self.with(f)
    }

    #[inline(always)]
    pub fn map_or<R>(self, default: R, f: impl FnOnce(&T) -> R) -> R {
        self.with(f).unwrap_or(default)
    }

    #[inline(always)]
    pub fn map_or_else<R>(self, default: impl FnOnce() -> R, f: impl FnOnce(&T) -> R) -> R {
        self.with(f).unwrap_or_else(default)
    }

    #[inline(always)]
    pub fn and_then<U>(self, f: impl FnOnce(&T) -> Option<U>) -> Option<U> {
        self.as_ref().and_then(f)
    }

    /// # Safety
    /// The caller must ensure no aliasing or other engine invariants are
    /// violated by creating a mutable borrow from this engine pointer.
    #[inline(always)]
    pub unsafe fn with_mut_unchecked<R>(self, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        let value = unsafe { self.raw.as_mut() }?;
        Some(f(value))
    }

    #[inline(always)]
    pub fn base<B>(self) -> GamePtr<B>
    where
        T: AsRef<B>,
    {
        let raw = self.with(|value| value.as_ref() as *const B as *mut B);
        match raw {
            Some(raw) => unsafe { GamePtr::from_raw(raw) },
            None => GamePtr::null(),
        }
    }

    /// # Safety
    /// The caller must uphold the same aliasing guarantees as
    /// [`GamePtr::with_mut_unchecked`].
    #[inline(always)]
    pub unsafe fn base_mut_unchecked<B>(self) -> GamePtr<B>
    where
        T: AsMut<B>,
    {
        let raw = unsafe { self.with_mut_unchecked(|value| value.as_mut() as *mut B) };
        match raw {
            Some(raw) => unsafe { GamePtr::from_raw(raw) },
            None => GamePtr::null(),
        }
    }

    #[inline(always)]
    pub fn try_cast<U>(self) -> GamePtr<U>
    where
        T: RttiType,
        U: RttiType,
    {
        if self.raw.is_null() {
            GamePtr::null()
        } else {
            let raw = unsafe { skyrim_cast::<T, U>(self.raw) };
            unsafe { GamePtr::from_raw(raw) }
        }
    }
}

impl<T> From<GameRef<T>> for GamePtr<T> {
    #[inline(always)]
    fn from(value: GameRef<T>) -> Self {
        value.into_ptr()
    }
}

impl<T> Copy for GamePtr<T> {}

impl<T> Clone for GamePtr<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> PartialEq for GamePtr<T> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<T> Eq for GamePtr<T> {}

impl<T> Default for GamePtr<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::null()
    }
}

impl<T> Hash for GamePtr<T> {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<T> fmt::Debug for GamePtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("GamePtr")
            .field(&format_args!("{:p}", self.raw))
            .finish()
    }
}
