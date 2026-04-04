use crate::impl_handle_family_target;
use crate::re::{
    Actor, ActorHandle, ArrowProjectile, BarrierProjectile, BeamProjectile, Character,
    ConeProjectile, Explosion, Hazard, MissileProjectile, ObjectRefHandle, PlayerCharacter,
    Projectile, ProjectileHandle, TESObjectREFR,
};

use super::super::sealed;
use super::traits::{CanonicalHandle, HandleTarget, ResolvableHandle};

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
