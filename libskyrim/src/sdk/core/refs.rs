//! Shared nullable borrowed reference wrappers for SDK-facing APIs.

use core::fmt;
use core::marker::PhantomData;

use crate::relocation::{RttiType, skyrim_cast, skyrim_cast_const};

/// Nullable borrowed engine reference shared across SDK domains.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GameRef<'a, T> {
    raw: *mut T,
    marker: PhantomData<&'a T>,
}

impl<'a, T> GameRef<'a, T> {
    #[inline(always)]
    pub const fn null() -> Self {
        Self {
            raw: core::ptr::null_mut(),
            marker: PhantomData,
        }
    }

    /// # Safety
    /// The caller must ensure that `raw` is either null or points to a valid
    /// `T` for the lifetime `'a`.
    #[inline(always)]
    pub const unsafe fn from_raw(raw: *mut T) -> Self {
        Self {
            raw,
            marker: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self.raw
    }

    #[inline(always)]
    pub const fn is_null(&self) -> bool {
        self.raw.is_null()
    }

    #[inline(always)]
    pub fn as_ref(&self) -> Option<&'a T> {
        unsafe { self.raw.as_ref() }
    }

    #[inline(always)]
    pub const fn cast<U>(self) -> GameRef<'a, U> {
        unsafe { GameRef::from_raw(self.raw.cast()) }
    }

    #[inline(always)]
    pub fn try_cast<U>(self) -> Option<GameRef<'a, U>>
    where
        T: RttiType,
        U: RttiType,
    {
        let raw = unsafe { skyrim_cast::<T, U>(self.raw.cast()) };
        if raw.is_null() {
            None
        } else {
            Some(unsafe { GameRef::from_raw(raw) })
        }
    }
}

impl<T> Default for GameRef<'_, T> {
    #[inline(always)]
    fn default() -> Self {
        Self::null()
    }
}

impl<'a, T> From<&'a T> for GameRef<'a, T> {
    #[inline(always)]
    fn from(value: &'a T) -> Self {
        unsafe { Self::from_raw(value as *const T as *mut T) }
    }
}

impl<'a, T> From<Option<&'a T>> for GameRef<'a, T> {
    #[inline(always)]
    fn from(value: Option<&'a T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Self::null(),
        }
    }
}

impl<'a, T> From<GameRefMut<'a, T>> for GameRef<'a, T> {
    #[inline(always)]
    fn from(value: GameRefMut<'a, T>) -> Self {
        unsafe { Self::from_raw(value.as_ptr()) }
    }
}

impl<T> fmt::Debug for GameRef<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("GameRef")
            .field(&format_args!("{:p}", self.raw))
            .finish()
    }
}

/// Nullable mutable borrowed engine reference shared across SDK domains.
#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GameRefMut<'a, T> {
    raw: *mut T,
    marker: PhantomData<&'a mut T>,
}

impl<'a, T> GameRefMut<'a, T> {
    #[inline(always)]
    pub const fn null() -> Self {
        Self {
            raw: core::ptr::null_mut(),
            marker: PhantomData,
        }
    }

    /// # Safety
    /// The caller must ensure that `raw` is either null or points to a valid
    /// `T` for the lifetime `'a` with mutable access semantics appropriate for
    /// the surrounding API.
    #[inline(always)]
    pub const unsafe fn from_raw(raw: *mut T) -> Self {
        Self {
            raw,
            marker: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self.raw
    }

    #[inline(always)]
    pub const fn is_null(&self) -> bool {
        self.raw.is_null()
    }

    #[inline(always)]
    pub fn as_ref(&self) -> Option<&T> {
        unsafe { self.raw.as_ref() }
    }

    #[inline(always)]
    pub fn as_mut(&mut self) -> Option<&mut T> {
        unsafe { self.raw.as_mut() }
    }

    #[inline(always)]
    pub fn as_const(&self) -> GameRef<'_, T> {
        unsafe { GameRef::from_raw(self.raw) }
    }

    #[inline(always)]
    pub fn reborrow(&mut self) -> GameRefMut<'_, T> {
        unsafe { GameRefMut::from_raw(self.raw) }
    }

    #[inline(always)]
    pub const fn cast<U>(self) -> GameRefMut<'a, U> {
        unsafe { GameRefMut::from_raw(self.raw.cast()) }
    }

    #[inline(always)]
    pub fn try_cast<U>(self) -> Option<GameRefMut<'a, U>>
    where
        T: RttiType,
        U: RttiType,
    {
        let raw = unsafe { skyrim_cast::<T, U>(self.raw.cast()) };
        if raw.is_null() {
            None
        } else {
            Some(unsafe { GameRefMut::from_raw(raw) })
        }
    }
}

impl<T> Default for GameRefMut<'_, T> {
    #[inline(always)]
    fn default() -> Self {
        Self::null()
    }
}

impl<'a, T> From<&'a mut T> for GameRefMut<'a, T> {
    #[inline(always)]
    fn from(value: &'a mut T) -> Self {
        unsafe { Self::from_raw(value as *mut T) }
    }
}

impl<'a, T> From<Option<&'a mut T>> for GameRefMut<'a, T> {
    #[inline(always)]
    fn from(value: Option<&'a mut T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Self::null(),
        }
    }
}

impl<T> fmt::Debug for GameRefMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("GameRefMut")
            .field(&format_args!("{:p}", self.raw))
            .finish()
    }
}

impl<T> GameRef<'_, T>
where
    T: RttiType,
{
    #[inline(always)]
    pub fn cast_const<U>(self) -> *const U
    where
        U: RttiType,
    {
        unsafe { skyrim_cast_const::<T, U>(self.raw.cast_const()) }
    }
}
