#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_bhkSerializable;
use crate::offsets::offsets_rtti::RTTI_bhkSerializable;
use crate::offsets::offsets_vtable::VTABLE_bhkSerializable;
use crate::re::{NiRTTI, ahkpWorld, bhkRefObject, bhkWorld, hkpWorld};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::bhkSerializable`
#[repr(C)]
pub struct bhkSerializable {
    pub base: bhkRefObject,                 // 00
    pub serializable: *mut bhkSerializable, // 18
}

const _: () = assert!(core::mem::size_of::<bhkSerializable>() == 0x20);
const _: () = assert!(core::mem::offset_of!(bhkSerializable, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(bhkSerializable, serializable) == 0x18);

impl RttiType for bhkSerializable {
    const RTTI: VariantID = RTTI_bhkSerializable;
}

inherit!(bhkSerializable : bhkRefObject);

impl bhkSerializable {
    pub const RTTI: VariantID = RTTI_bhkSerializable;
    pub const NI_RTTI: VariantID = NiRTTI_bhkSerializable;
    pub const VTABLE: &'static [VariantID] = &VTABLE_bhkSerializable;

    crate::virtual_method! {
        pub const VFUNC_GET_RTTI: usize = 0x02;
        pub fn get_rtti() -> *const NiRTTI
    }

    crate::virtual_method! {
        pub const VFUNC_GET_WORLD1: usize = 0x27;
        pub fn get_world1(&mut self) -> *mut hkpWorld
    }

    crate::virtual_method! {
        pub const VFUNC_GET_WORLD2: usize = 0x28;
        pub fn get_world2(&mut self) -> *mut ahkpWorld
    }

    crate::virtual_method! {
        pub const VFUNC_MOVE_TO_WORLD: usize = 0x29;
        pub fn move_to_world(&mut self, world: *mut bhkWorld)
    }

    crate::virtual_method! {
        pub const VFUNC_REMOVE_FROM_CURRENT_WORLD: usize = 0x2A;
        pub fn remove_from_current_world(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_2B: usize = 0x2B;
        pub fn unk_2b(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_2C: usize = 0x2C;
        pub fn unk_2c(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_2D: usize = 0x2D;
        pub fn unk_2d(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_2E: usize = 0x2E;
        pub fn unk_2e(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_2F: usize = 0x2F;
        pub fn unk_2f(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_30: usize = 0x30;
        pub fn unk_30(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_31: usize = 0x31;
        pub fn unk_31(&mut self)
    }
}

pub trait bhkSerializableExt {
    fn get_rtti(&mut self) -> *const NiRTTI;
    fn get_world1(&mut self) -> *mut hkpWorld;
    fn get_world2(&mut self) -> *mut ahkpWorld;
    fn move_to_world(&mut self, world: *mut bhkWorld);
    fn remove_from_current_world(&mut self);
    fn unk_2b(&mut self);
    fn unk_2c(&mut self);
    fn unk_2d(&mut self);
    fn unk_2e(&mut self);
    fn unk_2f(&mut self);
    fn unk_30(&mut self);
    fn unk_31(&mut self);
}

impl<T: AsMut<bhkSerializable>> bhkSerializableExt for T {
    #[inline(always)]
    fn get_rtti(&mut self) -> *const NiRTTI {
        bhkSerializable::get_rtti(self.as_mut())
    }

    #[inline(always)]
    fn get_world1(&mut self) -> *mut hkpWorld {
        bhkSerializable::get_world1(self.as_mut())
    }

    #[inline(always)]
    fn get_world2(&mut self) -> *mut ahkpWorld {
        bhkSerializable::get_world2(self.as_mut())
    }

    #[inline(always)]
    fn move_to_world(&mut self, world: *mut bhkWorld) {
        bhkSerializable::move_to_world(self.as_mut(), world)
    }

    #[inline(always)]
    fn remove_from_current_world(&mut self) {
        bhkSerializable::remove_from_current_world(self.as_mut())
    }

    #[inline(always)]
    fn unk_2b(&mut self) {
        bhkSerializable::unk_2b(self.as_mut())
    }

    #[inline(always)]
    fn unk_2c(&mut self) {
        bhkSerializable::unk_2c(self.as_mut())
    }

    #[inline(always)]
    fn unk_2d(&mut self) {
        bhkSerializable::unk_2d(self.as_mut())
    }

    #[inline(always)]
    fn unk_2e(&mut self) {
        bhkSerializable::unk_2e(self.as_mut())
    }

    #[inline(always)]
    fn unk_2f(&mut self) {
        bhkSerializable::unk_2f(self.as_mut())
    }

    #[inline(always)]
    fn unk_30(&mut self) {
        bhkSerializable::unk_30(self.as_mut())
    }

    #[inline(always)]
    fn unk_31(&mut self) {
        bhkSerializable::unk_31(self.as_mut())
    }
}
