//! Native handle helpers shared across SDK domains.

use core::fmt;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use crate::re::{
    Actor, ActorHandle, ArrowProjectile, BarrierProjectile, BeamProjectile, Character,
    ConeProjectile, Explosion, Hazard, MissileProjectile, ObjectRefHandle, PlayerCharacter,
    Projectile, ProjectileHandle, TESObjectREFR,
};
use crate::relocation::{RttiType, skyrim_cast, skyrim_cast_const};

use super::owners::{NativeOwner, NativeOwnerCastExt, NativeOwnerFamily};
use super::refs::{GamePtr, GameRef};
use super::sealed;

/// Trait for native handle types that can resolve into a native owner.
pub trait ResolvableHandle: Copy + sealed::Sealed {
    type Target;
    type Owner: NativeOwner<Target = Self::Target>;

    fn is_null(self) -> bool;

    fn resolve_owned(self) -> Option<Self::Owner>;

    #[inline(always)]
    fn resolve(self) -> Option<ResolvedHandle<Self>>
    where
        Self: Sized,
    {
        ResolvedHandle::from_handle(self)
    }

    #[inline(always)]
    fn resolve_cast<U>(
        self,
    ) -> Option<ResolvedHandle<Self, <Self::Owner as NativeOwnerFamily<U>>::Owner>>
    where
        Self: Sized,
        Self::Target: RttiType,
        U: RttiType,
        Self::Owner: Clone + NativeOwnerFamily<U>,
    {
        self.resolve()?.try_cast::<U>()
    }
}

/// Trait for canonical Bethesda handles that can be reconstructed from a raw
/// target pointer.
pub trait CanonicalHandle: ResolvableHandle + sealed::Sealed {
    fn from_target_ptr(ptr: *mut Self::Target) -> Self;
}

/// Trait for engine object types that have a canonical Bethesda handle family.
pub trait HandleTarget: sealed::Sealed + Sized {
    type Handle: ResolvableHandle<Target = Self>;

    fn canonical_handle(ptr: *mut Self) -> Self::Handle;

    #[inline(always)]
    fn resolve_handle(ptr: *mut Self) -> Option<ResolvedHandle<Self::Handle>> {
        if ptr.is_null() {
            return None;
        }

        Self::canonical_handle(ptr).resolve()
    }
}

/// Trait for runtime object types that belong to one of the supported
/// Bethesda handle families.
///
/// This is intentionally wider than `HandleTarget`: family members such as
/// `Character`, `Hazard`, or `ArrowProjectile` can map themselves onto a
/// canonical root object family without pretending to own a bespoke handle
/// type.
pub trait HandleFamilyTarget: sealed::Sealed + RttiType + Sized {
    type CanonicalTarget: HandleTarget<Handle = Self::Handle> + RttiType;
    type Handle: CanonicalHandle<Target = Self::CanonicalTarget>;

    /// Convert a pointer to `Self` into the canonical root pointer used by the
    /// corresponding Bethesda handle family.
    fn to_canonical_ptr(ptr: *mut Self) -> *mut Self::CanonicalTarget;

    #[inline(always)]
    fn canonical_handle(ptr: *mut Self) -> Self::Handle {
        Self::Handle::from_target_ptr(Self::to_canonical_ptr(ptr))
    }

    #[inline(always)]
    fn resolve(ptr: *mut Self) -> Option<Resolved<Self>> {
        if ptr.is_null() {
            return None;
        }

        Resolved::from_ptr(ptr)
    }
}

#[macro_export]
macro_rules! impl_handle_family_target {
    ($ty:path => $canonical:path, $handle:path) => {
        impl $crate::sdk::core::HandleFamilyTarget for $ty {
            type CanonicalTarget = $canonical;
            type Handle = $handle;

            #[inline(always)]
            fn to_canonical_ptr(ptr: *mut Self) -> *mut Self::CanonicalTarget {
                ptr.cast::<$canonical>()
            }
        }
    };
}

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

impl sealed::Sealed for ActorHandle {}

impl HandleTarget for Actor {
    type Handle = ActorHandle;

    #[inline(always)]
    fn canonical_handle(ptr: *mut Self) -> Self::Handle {
        ActorHandle::from_target_ptr(ptr)
    }
}

impl CanonicalHandle for ActorHandle {
    #[inline(always)]
    fn from_target_ptr(ptr: *mut Self::Target) -> Self {
        Self::from_ptr(ptr)
    }
}

impl ResolvableHandle for ActorHandle {
    type Target = crate::re::Actor;
    type Owner = crate::re::NiPointer<crate::re::Actor>;

    #[inline(always)]
    fn is_null(self) -> bool {
        !self.has_value()
    }

    #[inline(always)]
    fn resolve_owned(self) -> Option<Self::Owner> {
        let owner = self.get();
        if owner.is_null() { None } else { Some(owner) }
    }
}

impl sealed::Sealed for ObjectRefHandle {}

impl HandleTarget for TESObjectREFR {
    type Handle = ObjectRefHandle;

    #[inline(always)]
    fn canonical_handle(ptr: *mut Self) -> Self::Handle {
        ObjectRefHandle::from_target_ptr(ptr)
    }
}

impl CanonicalHandle for ObjectRefHandle {
    #[inline(always)]
    fn from_target_ptr(ptr: *mut Self::Target) -> Self {
        Self::from_ptr(ptr)
    }
}

impl ResolvableHandle for ObjectRefHandle {
    type Target = crate::re::TESObjectREFR;
    type Owner = crate::re::NiPointer<crate::re::TESObjectREFR>;

    #[inline(always)]
    fn is_null(self) -> bool {
        !self.has_value()
    }

    #[inline(always)]
    fn resolve_owned(self) -> Option<Self::Owner> {
        let owner = self.get();
        if owner.is_null() { None } else { Some(owner) }
    }
}

impl sealed::Sealed for ProjectileHandle {}

impl HandleTarget for Projectile {
    type Handle = ProjectileHandle;

    #[inline(always)]
    fn canonical_handle(ptr: *mut Self) -> Self::Handle {
        ProjectileHandle::from_target_ptr(ptr)
    }
}

impl CanonicalHandle for ProjectileHandle {
    #[inline(always)]
    fn from_target_ptr(ptr: *mut Self::Target) -> Self {
        Self::from_ptr(ptr)
    }
}

impl ResolvableHandle for ProjectileHandle {
    type Target = crate::re::Projectile;
    type Owner = crate::re::NiPointer<crate::re::Projectile>;

    #[inline(always)]
    fn is_null(self) -> bool {
        !self.has_value()
    }

    #[inline(always)]
    fn resolve_owned(self) -> Option<Self::Owner> {
        let owner = self.get();
        if owner.is_null() { None } else { Some(owner) }
    }
}

impl_handle_family_target!(Actor => Actor, ActorHandle);
impl_handle_family_target!(Character => Actor, ActorHandle);
impl_handle_family_target!(PlayerCharacter => Actor, ActorHandle);

impl_handle_family_target!(TESObjectREFR => TESObjectREFR, ObjectRefHandle);
impl_handle_family_target!(Hazard => TESObjectREFR, ObjectRefHandle);
impl_handle_family_target!(Explosion => TESObjectREFR, ObjectRefHandle);

impl_handle_family_target!(Projectile => Projectile, ProjectileHandle);
impl_handle_family_target!(ArrowProjectile => Projectile, ProjectileHandle);
impl_handle_family_target!(BarrierProjectile => Projectile, ProjectileHandle);
impl_handle_family_target!(BeamProjectile => Projectile, ProjectileHandle);
impl_handle_family_target!(ConeProjectile => Projectile, ProjectileHandle);
impl_handle_family_target!(MissileProjectile => Projectile, ProjectileHandle);
