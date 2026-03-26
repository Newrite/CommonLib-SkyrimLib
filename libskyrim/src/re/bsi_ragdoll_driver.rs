#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_BSIRagdollDriver;
use crate::offsets::offsets_vtable::VTABLE_BSIRagdollDriver;
use crate::re::{bhkWorld, hkpMotionMotionType};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSIRagdollDriver`
#[repr(C)]
pub struct BSIRagdollDriver {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<BSIRagdollDriver>() == 0x08);
const _: () = assert!(core::mem::offset_of!(BSIRagdollDriver, vtable) == 0x00);

impl RttiType for BSIRagdollDriver {
    const RTTI: VariantID = RTTI_BSIRagdollDriver;
}

impl AsRef<BSIRagdollDriver> for BSIRagdollDriver {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BSIRagdollDriver> for BSIRagdollDriver {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSIRagdollDriver {
    pub const RTTI: VariantID = RTTI_BSIRagdollDriver;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSIRagdollDriver;

    // ~BSIRagdollDriver() override;  // 00

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_HAS_RAGDOLL: usize = 0x01;
        pub fn has_ragdoll() -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_ADD_RAGDOLL_TO_WORLD: usize = 0x02;
        pub fn add_ragdoll_to_world() -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_REMOVE_RAGDOLL_FROM_WORLD: usize = 0x03;
        pub fn remove_ragdoll_from_world() -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SET_WORLD: usize = 0x04;
        pub fn set_world(&mut self, world: *mut bhkWorld)
    }

    crate::virtual_method! {
        pub const VFUNC_RESET_RAGDOLL: usize = 0x05;
        pub fn reset_ragdoll(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_06: usize = 0x06;
        pub fn unk_06(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_RAGDOLL_CONSTRAINTS_FROM_BHK_CONSTRAINTS: usize = 0x07;
        pub fn set_ragdoll_constraints_from_bhk_constraints(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_MOTION_TYPE: usize = 0x08;
        pub fn set_motion_type(&mut self, motion_type: hkpMotionMotionType)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_09: usize = 0x09;
        pub fn unk_09(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_TOGGLE_SYNC_ON_UPDATE: usize = 0x0A;
        pub fn toggle_sync_on_update(&mut self, disable: bool)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_0B: usize = 0x0B;
        pub fn unk_0b(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_TOGGLE_CONSTRAINTS: usize = 0x0C;
        pub fn toggle_constraints(&mut self, disable: bool)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_0D: usize = 0x0D;
        pub fn unk_0d(&mut self)
    }
}

pub trait BSIRagdollDriverExt {
    fn has_ragdoll(&self) -> bool;
    fn add_ragdoll_to_world(&self) -> bool;
    fn remove_ragdoll_from_world(&self) -> bool;
    fn set_world(&mut self, world: *mut bhkWorld);
    fn reset_ragdoll(&mut self);
    fn unk_06(&mut self);
    fn set_ragdoll_constraints_from_bhk_constraints(&mut self);
    fn set_motion_type(&mut self, motion_type: hkpMotionMotionType);
    fn unk_09(&mut self);
    fn toggle_sync_on_update(&mut self, disable: bool);
    fn unk_0b(&mut self);
    fn toggle_constraints(&mut self, disable: bool);
    fn unk_0d(&mut self);
}

impl<T: AsRef<BSIRagdollDriver> + AsMut<BSIRagdollDriver>> BSIRagdollDriverExt for T {
    #[inline(always)]
    fn has_ragdoll(&self) -> bool {
        BSIRagdollDriver::has_ragdoll(self.as_ref())
    }

    #[inline(always)]
    fn add_ragdoll_to_world(&self) -> bool {
        BSIRagdollDriver::add_ragdoll_to_world(self.as_ref())
    }

    #[inline(always)]
    fn remove_ragdoll_from_world(&self) -> bool {
        BSIRagdollDriver::remove_ragdoll_from_world(self.as_ref())
    }

    #[inline(always)]
    fn set_world(&mut self, world: *mut bhkWorld) {
        BSIRagdollDriver::set_world(self.as_mut(), world)
    }

    #[inline(always)]
    fn reset_ragdoll(&mut self) {
        BSIRagdollDriver::reset_ragdoll(self.as_mut())
    }

    #[inline(always)]
    fn unk_06(&mut self) {
        BSIRagdollDriver::unk_06(self.as_mut())
    }

    #[inline(always)]
    fn set_ragdoll_constraints_from_bhk_constraints(&mut self) {
        BSIRagdollDriver::set_ragdoll_constraints_from_bhk_constraints(self.as_mut())
    }

    #[inline(always)]
    fn set_motion_type(&mut self, motion_type: hkpMotionMotionType) {
        BSIRagdollDriver::set_motion_type(self.as_mut(), motion_type)
    }

    #[inline(always)]
    fn unk_09(&mut self) {
        BSIRagdollDriver::unk_09(self.as_mut())
    }

    #[inline(always)]
    fn toggle_sync_on_update(&mut self, disable: bool) {
        BSIRagdollDriver::toggle_sync_on_update(self.as_mut(), disable)
    }

    #[inline(always)]
    fn unk_0b(&mut self) {
        BSIRagdollDriver::unk_0b(self.as_mut())
    }

    #[inline(always)]
    fn toggle_constraints(&mut self, disable: bool) {
        BSIRagdollDriver::toggle_constraints(self.as_mut(), disable)
    }

    #[inline(always)]
    fn unk_0d(&mut self) {
        BSIRagdollDriver::unk_0d(self.as_mut())
    }
}
