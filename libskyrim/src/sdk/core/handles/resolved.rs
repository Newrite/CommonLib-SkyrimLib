use core::fmt;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use crate::relocation::{RttiType, skyrim_cast, skyrim_cast_const};

use super::super::owners::{NativeOwner, NativeOwnerCastExt, NativeOwnerFamily};
use super::super::refs::{GamePtr, GameRef};
use super::traits::{HandleFamilyTarget, ResolvableHandle};

/// A resolved native handle paired with the engine-owned object it currently
/// refers to.
pub struct ResolvedHandle<H, O = <H as ResolvableHandle>::Owner>
where
    H: ResolvableHandle,
    O: NativeOwner,
{
    handle: H,
    owner: O,
}

/// Typed ergonomic facade over a resolved Bethesda handle family.
///
/// `Resolved<T>` keeps the canonical handle-family ownership internally while
/// exposing a typed view of the resolved runtime object.
pub struct Resolved<T>
where
    T: HandleFamilyTarget,
{
    inner: ResolvedHandle<T::Handle>,
    marker: PhantomData<fn() -> T>,
}

impl<H, O> ResolvedHandle<H, O>
where
    H: ResolvableHandle,
    O: NativeOwner,
{
    #[inline(always)]
    pub fn new(handle: H, owner: O) -> Option<Self> {
        if handle.is_null() || owner.is_null() {
            None
        } else {
            Some(Self { handle, owner })
        }
    }

    #[inline(always)]
    pub fn handle(&self) -> H {
        self.handle
    }

    #[inline(always)]
    pub fn owner(&self) -> &O {
        &self.owner
    }

    #[inline(always)]
    pub fn owner_mut(&mut self) -> &mut O {
        &mut self.owner
    }

    #[inline(always)]
    pub fn into_owner(self) -> O {
        self.owner
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *mut O::Target {
        self.owner.as_ptr()
    }

    #[inline(always)]
    pub fn as_game_ptr(&self) -> GamePtr<O::Target> {
        unsafe { GamePtr::from_raw(self.as_ptr()) }
    }

    #[inline(always)]
    pub fn as_ref(&self) -> &O::Target {
        debug_assert!(!self.owner.is_null());
        unsafe { &*self.owner.as_ptr() }
    }

    #[inline(always)]
    pub fn as_game_ref(&self) -> GameRef<O::Target> {
        debug_assert!(!self.owner.is_null());
        unsafe { GameRef::from_raw(self.as_ptr()) }
    }

    #[inline(always)]
    pub fn as_mut(&mut self) -> &mut O::Target {
        debug_assert!(!self.owner.is_null());
        unsafe { &mut *self.owner.as_ptr() }
    }

    #[inline(always)]
    pub fn try_cast<U>(&self) -> Option<ResolvedHandle<H, <O as NativeOwnerFamily<U>>::Owner>>
    where
        H: Copy,
        O: Clone + NativeOwnerFamily<U>,
        O::Target: RttiType,
        U: RttiType,
    {
        let owner = self.owner.try_cast_owner::<U>()?;
        ResolvedHandle::new(self.handle, owner)
    }
}

impl<H> ResolvedHandle<H>
where
    H: ResolvableHandle,
{
    #[inline(always)]
    pub fn from_handle(handle: H) -> Option<Self> {
        let owner = handle.resolve_owned()?;
        Self::new(handle, owner)
    }
}

impl<H, O> Clone for ResolvedHandle<H, O>
where
    H: ResolvableHandle + Copy,
    O: NativeOwner + Clone,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            handle: self.handle,
            owner: self.owner.clone(),
        }
    }
}

impl<H, O> AsRef<O::Target> for ResolvedHandle<H, O>
where
    H: ResolvableHandle,
    O: NativeOwner,
{
    #[inline(always)]
    fn as_ref(&self) -> &O::Target {
        ResolvedHandle::as_ref(self)
    }
}

impl<H, O> AsMut<O::Target> for ResolvedHandle<H, O>
where
    H: ResolvableHandle,
    O: NativeOwner,
{
    #[inline(always)]
    fn as_mut(&mut self) -> &mut O::Target {
        ResolvedHandle::as_mut(self)
    }
}

impl<H, O> Deref for ResolvedHandle<H, O>
where
    H: ResolvableHandle,
    O: NativeOwner,
{
    type Target = O::Target;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<H, O> DerefMut for ResolvedHandle<H, O>
where
    H: ResolvableHandle,
    O: NativeOwner,
{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<H, O> fmt::Debug for ResolvedHandle<H, O>
where
    H: ResolvableHandle + fmt::Debug,
    O: NativeOwner,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResolvedHandle")
            .field("handle", &self.handle)
            .field("ptr", &format_args!("{:p}", self.owner.as_ptr()))
            .finish()
    }
}

impl<T> Resolved<T>
where
    T: HandleFamilyTarget,
{
    #[inline(always)]
    pub fn from_handle_owner(
        handle: T::Handle,
        owner: <T::Handle as ResolvableHandle>::Owner,
    ) -> Option<Self> {
        Self::from_inner(ResolvedHandle::new(handle, owner)?)
    }

    #[inline(always)]
    pub fn try_from_ptr(ptr: *mut T) -> Option<Self> {
        Self::from_ptr(ptr)
    }

    #[inline(always)]
    pub fn try_from_ref(value: &T) -> Option<Self> {
        Self::from_ptr(value as *const T as *mut T)
    }

    #[inline(always)]
    pub fn try_from_mut(value: &mut T) -> Option<Self> {
        Self::from_ptr(value as *mut T)
    }

    #[inline(always)]
    pub fn try_from_game_ref(value: GameRef<T>) -> Option<Self> {
        Self::from_ptr(value.as_ptr())
    }

    #[inline(always)]
    pub fn try_from_game_ptr(value: GamePtr<T>) -> Option<Self> {
        Self::from_ptr(value.as_ptr())
    }

    #[inline(always)]
    pub fn from_inner(inner: ResolvedHandle<T::Handle>) -> Option<Self> {
        let typed =
            unsafe { skyrim_cast_const::<T::CanonicalTarget, T>(inner.as_ptr().cast_const()) };
        if typed.is_null() {
            None
        } else {
            Some(Self {
                inner,
                marker: PhantomData,
            })
        }
    }

    #[inline(always)]
    pub fn from_handle(handle: T::Handle) -> Option<Self> {
        Self::from_inner(ResolvedHandle::from_handle(handle)?)
    }

    #[inline(always)]
    pub fn from_ptr(ptr: *mut T) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }

        Self::from_handle(T::canonical_handle(ptr))
    }

    #[inline(always)]
    pub fn inner(&self) -> &ResolvedHandle<T::Handle> {
        &self.inner
    }

    #[inline(always)]
    pub fn inner_mut(&mut self) -> &mut ResolvedHandle<T::Handle> {
        &mut self.inner
    }

    #[inline(always)]
    pub fn into_inner(self) -> ResolvedHandle<T::Handle> {
        self.inner
    }

    #[inline(always)]
    pub fn handle(&self) -> T::Handle {
        self.inner.handle()
    }

    #[inline(always)]
    pub fn owner(&self) -> &<T::Handle as ResolvableHandle>::Owner {
        self.inner.owner()
    }

    #[inline(always)]
    pub fn owner_mut(&mut self) -> &mut <T::Handle as ResolvableHandle>::Owner {
        self.inner.owner_mut()
    }

    #[inline(always)]
    pub fn into_owner(self) -> <T::Handle as ResolvableHandle>::Owner {
        self.inner.into_owner()
    }

    #[inline(always)]
    pub fn as_canonical_ptr(&self) -> *mut T::CanonicalTarget {
        self.inner.as_ptr()
    }

    #[inline(always)]
    pub fn as_canonical_ref(&self) -> &T::CanonicalTarget {
        self.inner.as_ref()
    }

    #[inline(always)]
    pub fn as_canonical_mut(&mut self) -> &mut T::CanonicalTarget {
        self.inner.as_mut()
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *mut T {
        unsafe { skyrim_cast::<T::CanonicalTarget, T>(self.as_canonical_ptr()) }
    }

    #[inline(always)]
    pub fn as_game_ptr(&self) -> GamePtr<T> {
        unsafe { GamePtr::from_raw(self.as_ptr()) }
    }

    #[inline(always)]
    pub fn as_ref(&self) -> &T {
        debug_assert!(!self.as_ptr().is_null());
        unsafe { &*self.as_ptr() }
    }

    #[inline(always)]
    pub fn as_game_ref(&self) -> GameRef<T> {
        debug_assert!(!self.as_ptr().is_null());
        unsafe { GameRef::from_raw(self.as_ptr()) }
    }

    #[inline(always)]
    pub fn as_mut(&mut self) -> &mut T {
        debug_assert!(!self.as_ptr().is_null());
        unsafe { &mut *self.as_ptr() }
    }

    #[inline(always)]
    pub fn try_cast<U>(&self) -> Option<Resolved<U>>
    where
        <T::Handle as ResolvableHandle>::Owner: Clone,
        U: HandleFamilyTarget<Handle = T::Handle, CanonicalTarget = T::CanonicalTarget>,
    {
        Resolved::from_inner(self.inner.clone())
    }
}

impl<T> AsRef<T> for Resolved<T>
where
    T: HandleFamilyTarget,
{
    #[inline(always)]
    fn as_ref(&self) -> &T {
        Resolved::as_ref(self)
    }
}

impl<T> AsMut<T> for Resolved<T>
where
    T: HandleFamilyTarget,
{
    #[inline(always)]
    fn as_mut(&mut self) -> &mut T {
        Resolved::as_mut(self)
    }
}

impl<T> Deref for Resolved<T>
where
    T: HandleFamilyTarget,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for Resolved<T>
where
    T: HandleFamilyTarget,
{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T> Clone for Resolved<T>
where
    T: HandleFamilyTarget,
    <T::Handle as ResolvableHandle>::Owner: Clone,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            marker: PhantomData,
        }
    }
}

impl<T> fmt::Debug for Resolved<T>
where
    T: HandleFamilyTarget,
    T::Handle: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Resolved")
            .field("handle", &self.handle())
            .field(
                "canonical_ptr",
                &format_args!("{:p}", self.as_canonical_ptr()),
            )
            .field("ptr", &format_args!("{:p}", self.as_ptr()))
            .finish()
    }
}
