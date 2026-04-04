use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use core::ptr::NonNull;

use crate::relocation::{RttiType, skyrim_cast};

use super::game_ptr::GamePtr;

/// Guaranteed non-null engine object wrapper.
///
/// This type intentionally stays weaker than `&T`: callers only get safe
/// shared access by default, while arbitrary mutable access remains explicit
/// and unsafe through `with_mut_unchecked`.
#[repr(transparent)]
#[must_use]
pub struct GameRef<T> {
    raw: NonNull<T>,
}

impl<T> GameRef<T> {
    /// # Safety
    /// `value` must refer to a valid live `T` for all future safe accesses
    /// performed through the returned wrapper.
    #[inline(always)]
    pub unsafe fn from_ref_unchecked(value: &T) -> Self {
        let raw = NonNull::from(value).cast::<T>();
        unsafe { Self::from_non_null(raw) }
    }

    /// # Safety
    /// `value` must refer to a valid live `T` for all future safe accesses
    /// performed through the returned wrapper.
    #[inline(always)]
    pub unsafe fn from_mut_unchecked(value: &mut T) -> Self {
        let raw = NonNull::from(value);
        unsafe { Self::from_non_null(raw) }
    }

    /// # Safety
    /// `raw` must point to a valid live `T` for all future safe accesses
    /// performed through this wrapper.
    #[inline(always)]
    pub const unsafe fn from_non_null(raw: NonNull<T>) -> Self {
        Self { raw }
    }

    /// # Safety
    /// `raw` must be non-null and point to a valid live `T` for all future
    /// safe accesses performed through this wrapper.
    #[inline(always)]
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        let raw = NonNull::new(raw).expect("GameRef requires a non-null pointer");
        unsafe { Self::from_non_null(raw) }
    }

    /// # Safety
    /// If `raw` is non-null it must point to a valid live `T` for all future
    /// safe accesses performed through the returned wrapper.
    #[inline(always)]
    pub unsafe fn try_from_raw(raw: *mut T) -> Option<Self> {
        NonNull::new(raw).map(|raw| unsafe { Self::from_non_null(raw) })
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
    pub fn into_ptr(self) -> GamePtr<T> {
        unsafe { GamePtr::from_non_null(self.raw) }
    }

    #[inline(always)]
    pub fn with<R>(self, f: impl FnOnce(&T) -> R) -> R {
        f(self.as_ref())
    }

    /// # Safety
    /// The caller must ensure no aliasing or other engine invariants are
    /// violated by creating a mutable borrow from this shared engine pointer.
    #[inline(always)]
    pub unsafe fn with_mut_unchecked<R>(self, f: impl FnOnce(&mut T) -> R) -> R {
        let value = unsafe { &mut *self.as_ptr() };
        f(value)
    }

    #[inline(always)]
    pub fn base<B>(self) -> GameRef<B>
    where
        T: AsRef<B>,
    {
        let raw = self.with(|value| value.as_ref() as *const B as *mut B);
        unsafe { GameRef::from_raw(raw) }
    }

    /// # Safety
    /// The caller must uphold the same aliasing guarantees as
    /// [`GameRef::with_mut_unchecked`].
    #[inline(always)]
    pub unsafe fn base_mut_unchecked<B>(self) -> GameRef<B>
    where
        T: AsMut<B>,
    {
        let raw = unsafe { self.with_mut_unchecked(|value| value.as_mut() as *mut B) };
        unsafe { GameRef::from_raw(raw) }
    }

    #[inline(always)]
    pub fn try_cast<U>(self) -> Option<GameRef<U>>
    where
        T: RttiType,
        U: RttiType,
    {
        let raw = unsafe { skyrim_cast::<T, U>(self.as_ptr()) };
        NonNull::new(raw).map(|raw| unsafe { GameRef::from_non_null(raw) })
    }
}

impl<T> AsRef<T> for GameRef<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        unsafe { self.raw.as_ref() }
    }
}

impl<T> Copy for GameRef<T> {}

impl<T> Clone for GameRef<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> PartialEq for GameRef<T> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<T> Eq for GameRef<T> {}

impl<T> Deref for GameRef<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> Hash for GameRef<T> {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<T> fmt::Debug for GameRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("GameRef")
            .field(&format_args!("{:p}", self.as_ptr()))
            .finish()
    }
}
