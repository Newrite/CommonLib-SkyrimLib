#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_hkBaseObject;
use crate::offsets::offsets_vtable::VTABLE_hkBaseObject;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::hkBaseObject`
#[repr(C)]
pub struct hkBaseObject {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<hkBaseObject>() == 0x8);
const _: () = assert!(core::mem::offset_of!(hkBaseObject, vtable) == 0x00);

impl RttiType for hkBaseObject {
    const RTTI: VariantID = RTTI_hkBaseObject;
}

impl AsRef<hkBaseObject> for hkBaseObject {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<hkBaseObject> for hkBaseObject {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl hkBaseObject {
    pub const RTTI: VariantID = RTTI_hkBaseObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkBaseObject;

    virtual_method! {
        pub const VFUNC_DESTRUCTOR: usize = 0x00;
        pub fn destructor(&mut self)
    }
}

pub trait hkBaseObjectExt {
    fn destructor(&mut self);
}

impl<T: AsMut<hkBaseObject>> hkBaseObjectExt for T {
    #[inline(always)]
    fn destructor(&mut self) {
        hkBaseObject::destructor(self.as_mut())
    }
}
