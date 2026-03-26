#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_bhkRefObject;
use crate::offsets::offsets_rtti::RTTI_bhkRefObject;
use crate::offsets::offsets_vtable::VTABLE_bhkRefObject;
use crate::re::{NiObject, NiRef, hkRefPtr, hkReferencedObject};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::bhkRefObject`
#[repr(C)]
pub struct bhkRefObject {
    pub base: NiObject,                                  // 00
    pub referenced_object: hkRefPtr<hkReferencedObject>, // 10
}

const _: () = assert!(core::mem::size_of::<bhkRefObject>() == 0x18);
const _: () = assert!(core::mem::offset_of!(bhkRefObject, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(bhkRefObject, referenced_object) == 0x10);

impl RttiType for bhkRefObject {
    const RTTI: VariantID = RTTI_bhkRefObject;
}

inherit!(bhkRefObject : NiObject);

impl bhkRefObject {
    pub const RTTI: VariantID = RTTI_bhkRefObject;
    pub const NI_RTTI: VariantID = NiRTTI_bhkRefObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_bhkRefObject;

    crate::virtual_method! {
        pub const VFUNC_SET_REFERENCED_OBJECT: usize = 0x25;
        pub fn set_referenced_object(object: *mut hkReferencedObject)
    }

    crate::virtual_method! {
        pub const VFUNC_ADJUST_REF_COUNT: usize = 0x26;
        pub fn adjust_ref_count(increment: bool)
    }
}

impl NiRef for bhkRefObject {
    #[inline(always)]
    fn inc_ref(&self) {
        let this = self as *const Self as *mut Self;
        unsafe { (*this).adjust_ref_count(true) }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        let this = self as *const Self as *mut Self;
        unsafe { (*this).adjust_ref_count(false) }
    }
}

pub trait bhkRefObjectExt {
    fn set_referenced_object(&mut self, object: *mut hkReferencedObject);
    fn adjust_ref_count(&mut self, increment: bool);
}

impl<T: AsMut<bhkRefObject>> bhkRefObjectExt for T {
    #[inline(always)]
    fn set_referenced_object(&mut self, object: *mut hkReferencedObject) {
        bhkRefObject::set_referenced_object(self.as_mut(), object)
    }

    #[inline(always)]
    fn adjust_ref_count(&mut self, increment: bool) {
        bhkRefObject::adjust_ref_count(self.as_mut(), increment)
    }
}
