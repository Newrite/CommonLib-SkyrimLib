#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_hkLocalFrame;
use crate::offsets::offsets_vtable::VTABLE_hkLocalFrame;
use crate::re::{hkLocalFrameGroup, hkReferencedObject, hkStringPtr, hkTransform, hkVector4};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::hkLocalFrame`
#[repr(C)]
pub struct hkLocalFrame {
    pub base: hkReferencedObject, // 00
}

const _: () = assert!(core::mem::size_of::<hkLocalFrame>() == 0x10);
const _: () = assert!(core::mem::offset_of!(hkLocalFrame, base) == 0x00);

impl RttiType for hkLocalFrame {
    const RTTI: VariantID = RTTI_hkLocalFrame;
}

inherit!(hkLocalFrame : hkReferencedObject, base);

impl hkLocalFrame {
    pub const RTTI: VariantID = RTTI_hkLocalFrame;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkLocalFrame;

    // override (hkReferencedObject)
    // ~hkLocalFrame() override;  // 00

    crate::virtual_method! {
        pub const VFUNC_GET_TRANSFORM: usize = 0x03;
        pub fn get_transform(&self, transform: *mut hkTransform)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_TRANSFORM: usize = 0x04;
        pub fn set_transform(&mut self, transform: *const hkTransform)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ORIGIN: usize = 0x05;
        pub fn get_origin(&self, origin: *mut hkVector4)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_NAME: usize = 0x06;
        pub fn get_name(&self) -> hkStringPtr
    }

    crate::virtual_method! {
        pub const VFUNC_GET_GROUP: usize = 0x07;
        pub fn get_group(&self) -> *mut hkLocalFrameGroup
    }

    crate::virtual_method! {
        pub const VFUNC_GET_PARENT_FRAME: usize = 0x08;
        pub fn get_parent_frame(&self) -> *mut hkLocalFrame
    }
}

pub trait hkLocalFrameExt {
    fn get_transform(&self, transform: *mut hkTransform);
    fn set_transform(&mut self, transform: *const hkTransform);
    fn get_origin(&self, origin: *mut hkVector4);
    fn get_name(&self) -> hkStringPtr;
    fn get_group(&self) -> *mut hkLocalFrameGroup;
    fn get_parent_frame(&self) -> *mut hkLocalFrame;
}

impl<T: AsRef<hkLocalFrame> + AsMut<hkLocalFrame>> hkLocalFrameExt for T {
    #[inline(always)]
    fn get_transform(&self, transform: *mut hkTransform) {
        hkLocalFrame::get_transform(self.as_ref(), transform)
    }

    #[inline(always)]
    fn set_transform(&mut self, transform: *const hkTransform) {
        hkLocalFrame::set_transform(self.as_mut(), transform)
    }

    #[inline(always)]
    fn get_origin(&self, origin: *mut hkVector4) {
        hkLocalFrame::get_origin(self.as_ref(), origin)
    }

    #[inline(always)]
    fn get_name(&self) -> hkStringPtr {
        hkLocalFrame::get_name(self.as_ref())
    }

    #[inline(always)]
    fn get_group(&self) -> *mut hkLocalFrameGroup {
        hkLocalFrame::get_group(self.as_ref())
    }

    #[inline(always)]
    fn get_parent_frame(&self) -> *mut hkLocalFrame {
        hkLocalFrame::get_parent_frame(self.as_ref())
    }
}
