#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_hkSimpleLocalFrame;
use crate::offsets::offsets_vtable::VTABLE_hkSimpleLocalFrame;
use crate::re::{
    hkArray, hkLocalFrame, hkLocalFrameGroup, hkRefPtr, hkReferencedObject, hkStringPtr,
    hkTransform,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::hkSimpleLocalFrame`
#[repr(C)]
pub struct hkSimpleLocalFrame {
    pub base: hkLocalFrame,                        // 00
    pub transform: hkTransform,                    // 10
    pub children: hkArray<hkRefPtr<hkLocalFrame>>, // 50
    pub group: *mut hkLocalFrameGroup,             // 60
    pub parent_frame: hkRefPtr<hkLocalFrame>,      // 68
    pub name: hkStringPtr,                         // 70
    pub pad78: u64,                                // 78
}

const _: () = assert!(core::mem::size_of::<hkSimpleLocalFrame>() == 0x80);
const _: () = assert!(core::mem::offset_of!(hkSimpleLocalFrame, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkSimpleLocalFrame, transform) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkSimpleLocalFrame, children) == 0x50);
const _: () = assert!(core::mem::offset_of!(hkSimpleLocalFrame, group) == 0x60);
const _: () = assert!(core::mem::offset_of!(hkSimpleLocalFrame, parent_frame) == 0x68);
const _: () = assert!(core::mem::offset_of!(hkSimpleLocalFrame, name) == 0x70);
const _: () = assert!(core::mem::offset_of!(hkSimpleLocalFrame, pad78) == 0x78);

impl RttiType for hkSimpleLocalFrame {
    const RTTI: VariantID = RTTI_hkSimpleLocalFrame;
}

inherit!(hkSimpleLocalFrame : hkLocalFrame, base);

impl AsRef<hkReferencedObject> for hkSimpleLocalFrame {
    #[inline(always)]
    fn as_ref(&self) -> &hkReferencedObject {
        self.base.as_ref()
    }
}

impl AsMut<hkReferencedObject> for hkSimpleLocalFrame {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut hkReferencedObject {
        self.base.as_mut()
    }
}

impl hkSimpleLocalFrame {
    pub const RTTI: VariantID = RTTI_hkSimpleLocalFrame;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkSimpleLocalFrame;
}
