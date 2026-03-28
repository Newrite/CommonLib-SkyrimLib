#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_bhkWorldObject;
use crate::offsets::offsets_rtti::RTTI_bhkWorldObject;
use crate::offsets::offsets_vtable::VTABLE_bhkWorldObject;
use crate::re::{NiRTTI, ahkpWorld, bhkRefObject, bhkSerializable, bhkWorld, hkpWorld};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::bhkWorldObject`
#[repr(C)]
pub struct bhkWorldObject {
    pub base: bhkSerializable, // 00
    pub world: *mut hkpWorld,  // 20
}

const _: () = assert!(core::mem::size_of::<bhkWorldObject>() == 0x28);
const _: () = assert!(core::mem::offset_of!(bhkWorldObject, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(bhkWorldObject, world) == 0x20);

impl RttiType for bhkWorldObject {
    const RTTI: VariantID = RTTI_bhkWorldObject;
}

impl crate::re::NiRef for bhkWorldObject {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const Self as *mut bhkRefObject)).adjust_ref_count(true) }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const Self as *mut bhkRefObject)).adjust_ref_count(false) }
    }
}

inherit!(bhkWorldObject : bhkSerializable, base);

impl bhkWorldObject {
    pub const RTTI: VariantID = RTTI_bhkWorldObject;
    pub const NI_RTTI: VariantID = NiRTTI_bhkWorldObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_bhkWorldObject;

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
        pub const VFUNC_UNK_32: usize = 0x32;
        pub fn unk_32(&mut self)
    }
}
