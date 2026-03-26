#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_bhkShape;
use crate::offsets::offsets_rtti::RTTI_bhkShape;
use crate::offsets::offsets_vtable::VTABLE_bhkShape;
use crate::re::{MATERIAL_ID, NiRef, bhkRefObject, bhkSerializable};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::bhkShape`
#[repr(C)]
pub struct bhkShape {
    pub base: bhkSerializable,    // 00
    pub material_id: MATERIAL_ID, // 20
    pub filter_info: u32,         // 24
}

const _: () = assert!(core::mem::size_of::<bhkShape>() == 0x28);
const _: () = assert!(core::mem::offset_of!(bhkShape, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(bhkShape, material_id) == 0x20);
const _: () = assert!(core::mem::offset_of!(bhkShape, filter_info) == 0x24);

impl RttiType for bhkShape {
    const RTTI: VariantID = RTTI_bhkShape;
}

inherit!(bhkShape : bhkSerializable, base);

impl bhkShape {
    pub const RTTI: VariantID = RTTI_bhkShape;
    pub const NI_RTTI: VariantID = NiRTTI_bhkShape;
    pub const VTABLE: &'static [VariantID] = &VTABLE_bhkShape;

    crate::relocation_func! {
        pub fn get_material_id(&self, key: u32) -> MATERIAL_ID => RelocationID::new(76799, 78676)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_32: usize = 0x32;
        pub fn unk_32(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_33: usize = 0x33;
        pub fn unk_33(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_34: usize = 0x34;
        pub fn unk_34(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_35: usize = 0x35;
        pub fn unk_35(&mut self)
    }
}

impl NiRef for bhkShape {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const _ as *mut bhkRefObject)).adjust_ref_count(true) }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const _ as *mut bhkRefObject)).adjust_ref_count(false) }
    }
}

pub trait bhkShapeExt {
    fn get_material_id(&self, key: u32) -> MATERIAL_ID;
    fn unk_32(&mut self);
    fn unk_33(&mut self);
    fn unk_34(&mut self);
    fn unk_35(&mut self);
}

impl<T: AsRef<bhkShape> + AsMut<bhkShape>> bhkShapeExt for T {
    #[inline(always)]
    fn get_material_id(&self, key: u32) -> MATERIAL_ID {
        bhkShape::get_material_id(self.as_ref(), key)
    }

    #[inline(always)]
    fn unk_32(&mut self) {
        bhkShape::unk_32(self.as_mut())
    }

    #[inline(always)]
    fn unk_33(&mut self) {
        bhkShape::unk_33(self.as_mut())
    }

    #[inline(always)]
    fn unk_34(&mut self) {
        bhkShape::unk_34(self.as_mut())
    }

    #[inline(always)]
    fn unk_35(&mut self) {
        bhkShape::unk_35(self.as_mut())
    }
}
